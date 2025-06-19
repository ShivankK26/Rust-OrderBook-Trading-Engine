use crate::models::{Order, Orderbook, OrderbookEntry, OrderSide, OrderType, ProcessingResult, Trade};
use rust_decimal::Decimal;
use chrono::Utc;

pub fn process_orders(orders: &[Order]) -> Result<ProcessingResult, Box<dyn std::error::Error>> {
    let mut orderbook = Orderbook::new();
    let mut trades = Vec::new();

    for order in orders {
        match order.order_type {
            OrderType::Create => handle_create_order(order.clone(), &mut orderbook, &mut trades)?,
            OrderType::Delete => handle_delete_order(order.clone(), &mut orderbook)?,
        }
    }

    Ok(ProcessingResult { orderbook, trades })
}

fn handle_create_order(
    order: Order,
    orderbook: &mut Orderbook,
    trades: &mut Vec<Trade>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Try to match the order first
    let remaining_amount = match_order(&order, orderbook, trades)?;

    // If there's remaining amount, add to orderbook
    if remaining_amount > rust_decimal::Decimal::ZERO {
        let price_str = order.price.to_string();
        let side_clone = order.side.clone(); // CLONE here
        let entry = OrderbookEntry {
            order_id: order.order_id,
            account_id: order.account_id,
            amount: remaining_amount,
            price: order.price,
            side: side_clone.clone(),
        };

        // Add to appropriate side of the orderbook
        let side = match side_clone {
            OrderSide::Buy => &mut orderbook.buy,
            OrderSide::Sell => &mut orderbook.sell,
        };

        side.entry(price_str)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    Ok(())
}

fn handle_delete_order(
    order: Order,
    orderbook: &mut Orderbook,
) -> Result<(), Box<dyn std::error::Error>> {
    let side = match order.side {
        OrderSide::Buy => &mut orderbook.buy,
        OrderSide::Sell => &mut orderbook.sell,
    };

    let price_str = order.price.to_string();
    
    if let Some(entries) = side.get_mut(&price_str) {
        // Find and remove the specific order
        if let Some(index) = entries
            .iter()
            .position(|entry| entry.order_id == order.order_id)
        {
            entries.remove(index);
            
            // Remove the price level if no orders remain
            if entries.is_empty() {
                side.remove(&price_str);
            }
        }
    }

    Ok(())
}

fn match_order(
    order: &Order,
    orderbook: &mut Orderbook,
    trades: &mut Vec<Trade>,
) -> Result<Decimal, Box<dyn std::error::Error>> {
    let mut remaining_amount = order.amount;
    
    match order.side {
        // If BUY order, try to match with SELL orders
        OrderSide::Buy => {
            // Sort sell prices in ascending order (this is already done by BTreeMap)
            let mut sell_prices: Vec<String> = orderbook.sell.keys().cloned().collect();
            sell_prices.sort_by(|a, b| {
                let a_dec = a.parse::<Decimal>().unwrap_or(Decimal::MAX);
                let b_dec = b.parse::<Decimal>().unwrap_or(Decimal::MAX);
                a_dec.cmp(&b_dec)
            });
            
            for price_str in sell_prices {
                let price = price_str.parse::<Decimal>()?;
                
                // Only match if the buy order's price is >= the sell order's price
                if order.price >= price {
                    if let Some(entries) = orderbook.sell.get_mut(&price_str) {
                        remaining_amount = process_matching(
                            order,
                            entries,
                            price,
                            remaining_amount,
                            trades,
                        )?;
                        
                        // Remove empty price levels
                        if entries.is_empty() {
                            orderbook.sell.remove(&price_str);
                        }
                        
                        // Stop if order is fully matched
                        if remaining_amount <= Decimal::ZERO {
                            break;
                        }
                    }
                } else {
                    break; // No more potential matches at acceptable prices
                }
            }
        }
        // If SELL order, try to match with BUY orders
        OrderSide::Sell => {
            // Sort buy prices in descending order
            let mut buy_prices: Vec<String> = orderbook.buy.keys().cloned().collect();
            buy_prices.sort_by(|a, b| {
                let a_dec = a.parse::<Decimal>().unwrap_or(Decimal::ZERO);
                let b_dec = b.parse::<Decimal>().unwrap_or(Decimal::ZERO);
                b_dec.cmp(&a_dec) // Descending order
            });
            
            for price_str in buy_prices {
                let price = price_str.parse::<Decimal>()?;
                
                // Only match if the sell order's price is <= the buy order's price
                if order.price <= price {
                    if let Some(entries) = orderbook.buy.get_mut(&price_str) {
                        remaining_amount = process_matching(
                            order,
                            entries,
                            price,
                            remaining_amount,
                            trades,
                        )?;
                        
                        // Remove empty price levels
                        if entries.is_empty() {
                            orderbook.buy.remove(&price_str);
                        }
                        
                        // Stop if order is fully matched
                        if remaining_amount <= Decimal::ZERO {
                            break;
                        }
                    }
                } else {
                    break; // No more potential matches at acceptable prices
                }
            }
        }
    }
    
    Ok(remaining_amount)
}

fn process_matching(
    order: &Order,
    entries: &mut Vec<OrderbookEntry>,
    price: Decimal,
    mut remaining_amount: Decimal,
    trades: &mut Vec<Trade>,
) -> Result<Decimal, Box<dyn std::error::Error>> {
    let mut i = 0;
    
    while i < entries.len() && remaining_amount > Decimal::ZERO {
        let entry = &mut entries[i];
        let entry_amount = entry.amount;
        
        // Calculate the matched amount
        let matched_amount = if remaining_amount < entry_amount {
            remaining_amount
        } else {
            entry_amount
        };
        
        // Create a trade
        let trade = Trade {
            buy_order_id: match order.side {
                OrderSide::Buy => order.order_id.clone(),
                OrderSide::Sell => entry.order_id.clone(),
            },
            sell_order_id: match order.side {
                OrderSide::Buy => entry.order_id.clone(),
                OrderSide::Sell => order.order_id.clone(),
            },
            amount: matched_amount,
            price,
            timestamp: Utc::now(),
        };
        trades.push(trade);
        
        // Update remaining amounts
        remaining_amount -= matched_amount;
        entry.amount -= matched_amount;
        
        // Remove the entry if fully matched
        if entry.amount <= Decimal::ZERO {
            entries.remove(i);
        } else {
            i += 1;
        }
    }
    
    Ok(remaining_amount)
}

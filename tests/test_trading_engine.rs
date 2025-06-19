use rust_trading_engine::{Order, OrderSide, OrderType, process_orders};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_simple_buy_and_sell_match() {
    // Test case: A simple buy and sell order that should match
    let test_orders = vec![
        Order {
            order_type: OrderType::Create,
            account_id: "1".to_string(),
            amount: Decimal::from_str("10.0").unwrap(),
            order_id: "1".to_string(),
            pair: "BTC/USDC".to_string(),
            price: Decimal::from_str("50000.00").unwrap(),
            side: OrderSide::Sell,
        },
        Order {
            order_type: OrderType::Create,
            account_id: "2".to_string(),
            amount: Decimal::from_str("5.0").unwrap(),
            order_id: "2".to_string(),
            pair: "BTC/USDC".to_string(),
            price: Decimal::from_str("50000.00").unwrap(),
            side: OrderSide::Buy,
        },
    ];

    let result = process_orders(&test_orders).unwrap();

    // Verify trades
    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.trades[0].buy_order_id, "2");
    assert_eq!(result.trades[0].sell_order_id, "1");
    assert_eq!(result.trades[0].amount, Decimal::from_str("5.0").unwrap());
    assert_eq!(result.trades[0].price, Decimal::from_str("50000.00").unwrap());

    // Verify orderbook
    // The sell side should have the remainder of the first order (5.0)
    assert_eq!(result.orderbook.sell.len(), 1);
    let sell_price = "50000.00";
    assert!(result.orderbook.sell.contains_key(sell_price));
    assert_eq!(result.orderbook.sell.get(sell_price).unwrap().len(), 1);
    assert_eq!(result.orderbook.sell.get(sell_price).unwrap()[0].amount, Decimal::from_str("5.0").unwrap());
    assert_eq!(result.orderbook.sell.get(sell_price).unwrap()[0].order_id, "1");
    
    // Buy side should be empty as the order was fully executed
    assert_eq!(result.orderbook.buy.len(), 0);
}

#[test]
fn test_delete_order() {
    // Test case: Create then delete an order
    let test_orders = vec![
        Order {
            order_type: OrderType::Create,
            account_id: "1".to_string(),
            amount: Decimal::from_str("10.0").unwrap(),
            order_id: "1".to_string(),
            pair: "BTC/USDC".to_string(),
            price: Decimal::from_str("50000.00").unwrap(),
            side: OrderSide::Sell,
        },
        Order {
            order_type: OrderType::Delete,
            account_id: "1".to_string(),
            amount: Decimal::from_str("10.0").unwrap(),
            order_id: "1".to_string(),
            pair: "BTC/USDC".to_string(),
            price: Decimal::from_str("50000.00").unwrap(),
            side: OrderSide::Sell,
        },
    ];

    let result = process_orders(&test_orders).unwrap();

    // No trades should be executed
    assert_eq!(result.trades.len(), 0);

    // Orderbook should be empty
    assert_eq!(result.orderbook.sell.len(), 0);
    assert_eq!(result.orderbook.buy.len(), 0);
}

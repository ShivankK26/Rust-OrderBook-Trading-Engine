
# Rust Trading Engine

A simple trading engine implementation in Rust that processes orders from a JSON file and generates an orderbook and trades.

## Features

- Process `CREATE` and `DELETE` orders
- Match buy and sell orders based on price
- Generate orderbook and trades
- Price-time priority matching algorithm
- Decimal precision for financial calculations

## Usage

1. Place your orders in a JSON file called `orders.json` in the root directory
2. Run the program:

```bash
cargo run
```

3. Two JSON files will be generated:
   - `orderbook.json`: Contains the current state of the orderbook
   - `trades.json`: Contains all the executed trades

## Testing

To run the tests:

```bash
cargo test
```

## Order Format

Orders should be in the following JSON format:

```json
{
  "type_op": "CREATE",
  "account_id": "1",
  "amount": "0.00230",
  "order_id": "1",
  "pair": "BTC/USDC",
  "limit_price": "63500.00",
  "side": "SELL"
}
```

## Output Format

### Orderbook

```json
{
  "buy": {
    "50000.00": [
      {
        "order_id": "2",
        "account_id": "2",
        "amount": "5.0",
        "price": "50000.00",
        "side": "BUY"
      }
    ]
  },
  "sell": {
    "51000.00": [
      {
        "order_id": "1",
        "account_id": "1",
        "amount": "10.0",
        "price": "51000.00",
        "side": "SELL"
      }
    ]
  }
}
```

### Trades

```json
[
  {
    "buy_order_id": "2",
    "sell_order_id": "1",
    "amount": "5.0",
    "price": "50000.00",
    "timestamp": "2025-05-10T12:34:56.789Z"
  }
]
```

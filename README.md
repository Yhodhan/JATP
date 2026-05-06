# Just Another Transaction Processor 

A simple transaction engine that reads and processes financial transactions from a CSV file, maintaining account state and outputting the final balances to stdout.

# Main design 

The input of the program in a CSV file containing transactions to be executed. The program holds a central data structure called `TxManager` that mantaints the state of the transactions and accounts during the program execution.

The engine deserialize the CSV and read and execute the transactions one by one so avoiding to load and store into the memory program a massive file, as file sizes are unknown it would not be ideal to deserealize and load a 5GB file for instance, especially if several threads with different `TxManager` are created.

Once a transaction is read it is pass to the `TxManager`, which proceeds with execution. Once all the transactions have been executed the final state of the accounts is written in standart output. 

![alt text](<design.png>)

## Usage

```bash
cargo run -- --file transactions.csv
```

## Transactions types

| type | description |
|---|---|
| `deposit` | increases available funds and total for an account |
| `withdrawal` | decreases available funds and total, rejected if insufficient funds |
| `dispute` | moves funds from available to held pending investigation (deposits only) |
| `resolve` | releases held funds back to available, closing the dispute |
| `chargeback` | returns held funds to the client and locks the account |

> **note:** disputes on withdrawals are ignored. reversing a withdrawal would require moving funds into `held` from an already-reduced balance, which introduces ambiguity around the held value. for simplicity, only deposits can be disputed. However disputes are only allowed if the user did not perform a withdrawal after a deposit, in that case the dispute is ignored as it is considered that the previous deposit was a valid operation. 

## Accounts 

Accounts represent a user's funds and status in the system, identified by a `u16` value. Each account tracks:

- `available` — funds the client can use
- `held` — funds frozen during an open dispute
- `total` — `available + held`
- `locked` — set to `true` after a chargeback; locked accounts are ignored by the engine

All monetary amounts are stored as `rust_decimal::Decimal` with 4 decimal places of precision (e.g `1.5000`). Floating point is avoid to prevent rounding errors.

## Notes on efficiency 

The current implementation is a single thread that streams CSV row by row to avoid loading the entire dataset.

In the case of receiving multiple CSV at once through, for example multiple tcp connections, the program could be adapted to use Thread pool where each threads its own `TxManager` instance. An Epoll-based polling strategy coulb be used to pass to dispatch incoming datasets to free workers.  

## Testing

The program uses datasets stored in the folder 'test_csvs'. Unit tests covering the core `TxManager` logic live in `tx_manager.rs`. 

execute unit testing with: 

```bash
cargo test
```

## Notes on the use of AI 

Most of the code has been handwritten but the parsing part of the CSV files and the initial setup of tests and datasets were also created with Claude to speed the process of writing the boiler plate.
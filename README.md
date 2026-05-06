# Just Another Transaction Processor 

The main idea is to develop a simple transacion engine that handles a few kind of transactions.

# Main design 

The input of the program in a CSV file containing the transactions to be executed. The program holds a main data structure called the 'TxManager' that holds the state of the transactions and accounts during the program execution. The engine deserialize the CSV and read and execute the transactions one by one so avoiding to load and store into the memory program a massive file, as file sizes are unknown it would not be ideal to deserealize and load a 5GB file for instance, especially if several threads with different TxManagers are created. Once a transaction is read it is pass to the TxManager itself that proceeds with the execution. Once all the transactions have been executed the final state of the accounts is written in standart output. 

## Transactions types

the type of transactions supported are Deposit, Withdrawal, Dispute, Resolve and ChargeBack. 

A Deposit and a withdrawal only increase or decrease the availability and totality of funds in an account. A Dispute represents a conflict with one of the previous operations where the user account can claim not to have done such transaction, for simplicity in this case it has been assume that only Deposit transactions can be set into Dispute, Withdrawals disputes are ignored because they required more logic to properly handle the held value. Resolve represent the resolution to the conflict and it removes the funds that are in held and are set to be available again. On the other hand a ChargeBack returns the funds to the user and reduces the avaialability and totality of the funds in an account, this also locks the user account.

## Accounts 

The accounts represents a user funds and status (locked or not) in the system. They are identify by a simple u16 value, the operations they can perform throught transactions are the one previously mention for as long as their account is not locked. Accounts that have been locked are not allow to keep operating and transactions that they submit are ignored by the system.  

## Notes on efficiency 

The current implementation is a single thread TxManager that receives the input through standard input, as mention before it streams from the memory the values of the CSV to avoid loading the entire dataset. In the case of receiving multiple CSV at once through, for example multiple tcp connections, the program could be adapted to create a pool of working threads that holds instances of the TxManager and then a polling strategy with an Epoll can be used to pass into free Working threads the datasets.  

## Testing

The program uses some sample datasets, stored in the folder 'test_csvs', and a test module in 'tx_manager.rs' to check the correct behaviour of the operations 

## Notes on the use of AI 

Most of the code has been handwritten but the parsing part of the CSV files and the initial setup of tests were created with Claude to speed the process of writing the boiler plate. Datasets were also generated with Claude at random.  
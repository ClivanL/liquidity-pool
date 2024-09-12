# Staking program
## Preventing congestion on minting of liquidity pool tokens (lp_tokens) by lp_mint
### Using clockwork (to pause and work on trading first)
#### Currently implemented
- Implemented pending_stake_seed_records, which holds the sub_seeds value, each sub_seed value + 'pending_stake' will be used to generate a stake_token_transaction, which consists the pending transaction details
- Existing program logic, confirm_stake will receive required accounts from frontend and process the confirmation -> all the account generation is processed on the frontend startieng from deriving pending_stake_seed_records.

#### Previous attempts
##### Attempt 1:
- Previous attempt on having everything derived on chain by finding the individual pdas and confirming all the stake_token_transactions did not work, as i was stuck at having the pdas derived but unable to retrieve the values

##### Attempt 2:
- Having a chain of programs calling each other using CPIs
- Program A will receive a String value as an argument from frontend, which will allow for the forming pending_stake_seed_records account, read data and loop through to form individual stake_token_transactions pda, which will feed these pdas into program B
- Program B will receive these stake_token_transaction pda, read data and process confirm stake logic (program will be similar to existing stake_token logic)
- Use clockwork to run program A, with a fixed frequency
- *Same issue faced as attempt 1, unable to convert to account to retrieve the values in the account after deriving pda on chain*
- *Faced timeline issues in having instructions calling each other, variables defined in instruction a cannot get to instruction b*
- *learnt that probably there isn't a need to use CPI, can use context when instructions are from the same program*

### Withdraw from stake (Exchange lp_token back to tokens)

### Interest rate generation of staked tokens

# Trading program
## Features
### Market Order
- Bid-Offer spread
- Using liquidity pool (StakeRecords)
#### Process
- When market buy or sell is initiated, look up existing bid offer spread (mock bid offer spread between pairs on oracle, 4+3+2+1(10) oracles needed)
- Increment / decrement user token account and decrement / increment liquidity pool 
#### Concurrency issues
- Updating of liquidity pool by multiple users at the same time initiating a market order (Solution: clockwork?)
### Limit Order
- Order book
#### Process
##### Buy limit order
- Retrieve latest index from OrderBook Account (last_index, orders(Vec), direction, token-pair), seed:orderbook, `token-pair`, buy
- PurchaseOrder Account created (user, amount_to_trade, exchange_rate, created_at , token_pair, closed:boolean), seed: order, buy, order+index
##### Sell limit order
- Retrieve latest index from OrderBook Account (last_index, orders(Vec), direction, token-pair), seed:orderbook, `token-pair`, sell
- SaleOrder Account created (user_pubkey, quantity, price, datetime,from-token, to-token, closed:boolean), seed: order, sell, order+index
##### Processing (Single order-book)
- Retrieve OrderBook Account for purchase (Solution: clockwork)
- Sort order book by closed, exchange_rate, created_at
- Loop from first entry until last entry for SaleOrder, skip if closed is true, if price for saleorder < price for purchase order, fulfil order, update both orderbooks, update quantity, closed
- Retrieve both user token accounts for both types of tokens and perform changes
##### Processing (Multiple order-books)
- Combine order books for sale and purchase on-chain
- Perform sorting by closed, exchange_rate, created_at
- Process as per single order-book logic, locate and update order books using orderbook-sub-seed that is stored in order struct 
- **drawbacks** - computationally expensive to process combination and sorting logic on chain
- *possible solutions* - 
1. offchain computation
2. Store orders in pre-defined arrangement (eg orderbooks with different exchange rates to hold orders, instead of filling up orderbook by orderbook in sequence) **TBE**

##### Account management
###### Multiple order books for same token-pair and direction (Exceed max number of orders that can be stored by a single order book)
- Create order book directory account (orderbook-subseeds(Vec), last_index, direction, token_pair), seed: orderbook_directory, `token-pair`, `direction`
- Introduce additional seed to orderbook account - "OB"+last_index
- Introduce additonal seed to order account and save the additional seed in order struct (for identification of which order book the order belongs to during order processing)
###### Closing of account after order is fulfilled
- Close account when quantity in order account is fulfilled, logic to introduce into processing logic for orders
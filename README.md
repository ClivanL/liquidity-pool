# Staking program
## Preventing congestion on minting of liquidity pool tokens (lp_tokens) by lp_mint
### Using clockwork
#### Currently implemented
- Implemented pending_stake_seed_records, which holds the sub_seeds value, each sub_seed value + 'pending_stake' will be used to generate a stake_token_transaction, which consists the pending transaction details
- Existing program logic, confirm_stake will receive required accounts from frontend and process the confirmation -> all the account generation is processed on the frontend startieng from deriving pending_stake_seed_records.

#### Previous attempts
- Previous attempt on having everything derived on chain by finding the individual pdas and confirming all the stake_token_transactions did not work, as i was stuck at having the pdas derived but unable to retrieve the values

#### To try
- Having a chain of programs calling each other using CPIs
- Program A will receive a String value as an argument from frontend, which will allow for the forming pending_stake_seed_records account, read data and loop through to form individual stake_token_transactions pda, which will feed these pdas into program B
- Program B will receive these stake_token_transaction pda, read data and process confirm stake logic (program will be similar to existing stake_token logic)
- Use clockwork to run program A, with a fixed frequency

use anchor_lang::prelude::*;
use crate::context::*;
use crate::enums::*;
use core::str::FromStr;
use crate::errors::*;
use crate::constants::*;
use crate::state::*;
use solana_program::clock::Clock;

pub fn handler(ctx: Context<ProcessBuyLimitOrder>,pending_transfers_subseed:String) -> Result<()> {

    let buy_limit_order = &mut ctx.accounts.buy_limit_order;

    let remaining_accounts = &ctx.remaining_accounts;

    let mut sell_limit_orders = Vec::new();
    for account_info in remaining_accounts.iter(){
        let account_data: &[u8] = &account_info.data.borrow();
        let sell_limit_order:LimitOrder = LimitOrder::try_from_slice(account_data).map_err(|err| ProgramError::from(err))?;
        sell_limit_orders.push(sell_limit_order);
    }

    sell_limit_orders.sort_by(|a,b| {
        a.closed.cmp(&b.closed)
            .then(b.exchange_rate.partial_cmp(&a.exchange_rate).unwrap())
            .then(a.created_at.cmp(&b.created_at))
    });

    let pending_transfers = &mut ctx.accounts.pending_transfers.transfers;

    let mut balance = buy_limit_order.amount_to_trade;
    let exchange_rate = buy_limit_order.exchange_rate;
    let mut open_index = 0;
    for (index,sell_limit_order) in sell_limit_orders.iter().enumerate() {
        if sell_limit_order.closed != true {
            open_index = index;
            break;
        }
        else{}
    }
    
    for sell_limit_order in &mut sell_limit_orders[open_index..]{
        if sell_limit_order.exchange_rate <= exchange_rate{
            if sell_limit_order.amount_to_trade<balance{
                let transfer_buy =  Transfer {
                    account_from: buy_limit_order.user,
                    account_to: sell_limit_order.user,
                    amount: sell_limit_order.amount_to_trade*exchange_rate,
                    token_name: format!("token_{}",&(sell_limit_order.token_pair.to_string())[0..1]).into_bytes()
                };
                let transfer_sell = Transfer{
                    account_from: sell_limit_order.user,
                    account_to: buy_limit_order.user,
                    amount: sell_limit_order.amount_to_trade,
                    token_name: format!("token_{}",&(sell_limit_order.token_pair.to_string())[1..2]).into_bytes()
                };
                balance -= sell_limit_order.amount_to_trade;
                sell_limit_order.amount_to_trade = 0.0;
                sell_limit_order.closed = true;
                pending_transfers.push(transfer_buy);
                pending_transfers.push(transfer_sell);
            }
            else if sell_limit_order.amount_to_trade > balance{
                let transfer_buy =  Transfer {
                    account_from: buy_limit_order.user,
                    account_to: sell_limit_order.user,
                    amount: balance*exchange_rate,
                    token_name: format!("token_{}",&(sell_limit_order.token_pair.to_string())[0..1]).into_bytes()
                };
                let transfer_sell = Transfer{
                    account_from: sell_limit_order.user,
                    account_to: buy_limit_order.user,
                    amount: balance,
                    token_name: format!("token_{}",&(sell_limit_order.token_pair.to_string())[1..2]).into_bytes()
                };
                sell_limit_order.amount_to_trade -= balance;
                balance = 0.0;
                pending_transfers.push(transfer_buy);
                pending_transfers.push(transfer_sell);
                break;
            }
            else {
                let transfer_buy =  Transfer {
                    account_from: buy_limit_order.user,
                    account_to: sell_limit_order.user,
                    amount: balance*exchange_rate,
                    token_name: format!("token_{}",&(sell_limit_order.token_pair.to_string())[0..1]).into_bytes()
                };
                let transfer_sell = Transfer{
                    account_from: sell_limit_order.user,
                    account_to: buy_limit_order.user,
                    amount: balance,
                    token_name: format!("token_{}",&(sell_limit_order.token_pair.to_string())[1..2]).into_bytes()
                };
                sell_limit_order.amount_to_trade = 0.0;
                balance = 0.0;
                sell_limit_order.closed = true;
                pending_transfers.push(transfer_buy);
                pending_transfers.push(transfer_sell);
                break;
            }
        }
    }

    let pending_transfers_record = &mut ctx.accounts.pending_transfers_record.pending_transfer_subseeds;
    pending_transfers_record.push(pending_transfers_subseed);

    Ok(())
}
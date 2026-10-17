//! Margin position closing handlers. Settlement details are omitted from this source.

use anchor_lang::prelude::*;

use crate::constants::MAX_CLOSE_INSERT_INDICES;
use crate::error::ErrorCode;
use crate::instructions::context_validator::validate_trade_close_context;
use crate::instructions::contexts::TradeClose;
use crate::instructions::orderbook_manager::OrderBookManager;
use crate::instructions::structs::MarginOrder;
use crate::instructions::trade_engine::{buy_amounts, sell_amounts};

fn validate_close_indices(indices: &[u16]) -> Result<()> {
    if indices.is_empty() {
        return err!(ErrorCode::EmptyCloseInsertIndices);
    }
    if indices.len() > MAX_CLOSE_INSERT_INDICES {
        return err!(ErrorCode::TooManyCloseInsertIndices);
    }
    Ok(())
}

fn find_order(book: &AccountInfo<'_>, order_id: u64, indices: &[u16]) -> Result<MarginOrder> {
    let data = book.data.borrow();
    for &index in indices {
        let Ok(order) = OrderBookManager::get_order(&data, index) else {
            continue;
        };
        if order.order_id == order_id {
            return Ok(*order);
        }
        for neighbor in [order.prev_order, order.next_order] {
            if neighbor != u16::MAX {
                if let Ok(adjacent) = OrderBookManager::get_order(&data, neighbor) {
                    if adjacent.order_id == order_id {
                        return Ok(*adjacent);
                    }
                }
            }
        }
    }
    err!(ErrorCode::CloseOrderNotFound)
}

/// Close all or part of a long position.
pub fn close_long_trade(
    ctx: Context<TradeClose>,
    sell_token_amount: u64,
    min_sol_output: u64,
    close_order_id: u64,
    close_order_indices: Vec<u16>,
) -> Result<()> {
    validate_trade_close_context(&ctx)?;
    validate_close_indices(&close_order_indices)?;

    let book = ctx.accounts.down_orderbook.to_account_info();
    let order = find_order(&book, close_order_id, &close_order_indices)?;
    let _quote = sell_amounts(
        &mut ctx.accounts.curve_account,
        &book,
        Some(order.order_id),
        sell_token_amount,
        min_sol_output,
        order.borrow_fee,
    )?;

    // Partial and full settlement, repayment, order updates, and events are omitted.
    Err(ErrorCode::CurveCalculationError.into())
}

/// Close all or part of a short position.
pub fn close_short_trade(
    ctx: Context<TradeClose>,
    buy_token_amount: u64,
    max_sol_amount: u64,
    close_order_id: u64,
    close_order_indices: Vec<u16>,
) -> Result<()> {
    validate_trade_close_context(&ctx)?;
    validate_close_indices(&close_order_indices)?;

    let book = ctx.accounts.up_orderbook.to_account_info();
    let order = find_order(&book, close_order_id, &close_order_indices)?;
    let _quote = buy_amounts(
        &mut ctx.accounts.curve_account,
        &book,
        Some(order.order_id),
        buy_token_amount,
        max_sol_amount,
        order.borrow_fee,
    )?;

    // Partial and full settlement, repayment, order updates, and events are omitted.
    Err(ErrorCode::CurveCalculationError.into())
}

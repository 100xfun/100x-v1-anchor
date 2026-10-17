//! Margin position opening handlers. Position construction and settlement are omitted.

use anchor_lang::prelude::*;

use crate::constants::{MAX_CLOSE_INSERT_INDICES, MIN_TRADE_TOKEN_AMOUNT};
use crate::error::ErrorCode;
use crate::instructions::context_validator::validate_trade_long_short_context;
use crate::instructions::contexts::TradeLongShort;
use crate::instructions::trade_engine::{buy_amounts, sell_amounts};

fn validate_insert_indices(indices: &[u16]) -> Result<()> {
    if indices.is_empty() {
        return err!(ErrorCode::EmptyCloseInsertIndices);
    }
    if indices.len() > MAX_CLOSE_INSERT_INDICES {
        return err!(ErrorCode::TooManyCloseInsertIndices);
    }
    Ok(())
}

/// Open a long position using borrowed SOL.
pub fn long_trade(
    ctx: Context<TradeLongShort>,
    buy_token_amount: u64,
    max_sol_amount: u64,
    margin_sol_max: u64,
    close_price: u128,
    close_insert_indices: Vec<u16>,
) -> Result<()> {
    validate_trade_long_short_context(&ctx)?;
    validate_insert_indices(&close_insert_indices)?;
    if buy_token_amount < MIN_TRADE_TOKEN_AMOUNT {
        return err!(ErrorCode::InsufficientTradeAmount);
    }

    let fee = ctx.accounts.curve_account.borrow_fee;
    let _quote = buy_amounts(
        &mut ctx.accounts.curve_account,
        &ctx.accounts.up_orderbook.to_account_info(),
        None,
        buy_token_amount,
        max_sol_amount,
        fee,
    )?;

    let _ = (margin_sol_max, close_price);
    // Stop validation, collateral, borrowing, order placement, and settlement are omitted.
    Err(ErrorCode::CurveCalculationError.into())
}

/// Open a short position using borrowed tokens.
pub fn short_trade(
    ctx: Context<TradeLongShort>,
    borrow_sell_token_amount: u64,
    min_sol_output: u64,
    margin_sol_max: u64,
    close_price: u128,
    close_insert_indices: Vec<u16>,
) -> Result<()> {
    validate_trade_long_short_context(&ctx)?;
    validate_insert_indices(&close_insert_indices)?;
    if borrow_sell_token_amount < MIN_TRADE_TOKEN_AMOUNT {
        return err!(ErrorCode::InsufficientTradeAmount);
    }
    if borrow_sell_token_amount > ctx.accounts.curve_account.borrow_token_reserve {
        return err!(ErrorCode::InsufficientBorrowingReserve);
    }

    let fee = ctx.accounts.curve_account.borrow_fee;
    let _quote = sell_amounts(
        &mut ctx.accounts.curve_account,
        &ctx.accounts.down_orderbook.to_account_info(),
        None,
        borrow_sell_token_amount,
        min_sol_output,
        fee,
    )?;

    let _ = (margin_sol_max, close_price);
    // Stop validation, collateral, borrowing, order placement, and settlement are omitted.
    Err(ErrorCode::CurveCalculationError.into())
}

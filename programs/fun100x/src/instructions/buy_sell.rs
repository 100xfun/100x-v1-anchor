//! Spot trading handlers. Settlement details are omitted from this public source.

use anchor_lang::prelude::*;

use crate::constants::{MAX_SINGLE_TRADE_SOL, MIN_TRADE_TOKEN_AMOUNT};
use crate::error::ErrorCode;
use crate::instructions::context_validator::validate_trade_buy_sell_context;
use crate::instructions::contexts::TradeBuySell;
use crate::instructions::trade_engine::{buy_amounts, sell_amounts};

/// Buy tokens subject to the user's maximum SOL spend.
pub fn buy_trade(
    ctx: Context<TradeBuySell>,
    buy_token_amount: u64,
    max_sol_amount: u64,
) -> Result<()> {
    validate_trade_buy_sell_context(&ctx)?;

    if ctx.accounts.cooldown.last_trade_time > 0 {
        crate::instructions::validate_trade_cooldown(&ctx.accounts.cooldown)?;
    }
    if buy_token_amount < MIN_TRADE_TOKEN_AMOUNT {
        return err!(ErrorCode::InsufficientTradeAmount);
    }
    if max_sol_amount > MAX_SINGLE_TRADE_SOL {
        return err!(ErrorCode::SolAmountTooLarge);
    }
    if buy_token_amount > ctx.accounts.curve_account.lp_token_reserve {
        return err!(ErrorCode::InsufficientLiquidity);
    }

    let swap_fee = ctx.accounts.curve_account.swap_fee;
    let _quote = buy_amounts(
        &mut ctx.accounts.curve_account,
        &ctx.accounts.up_orderbook.to_account_info(),
        None,
        buy_token_amount,
        max_sol_amount,
        swap_fee,
    )?;

    // Token transfers, liquidation settlement, state updates, and event emission are omitted.
    Err(ErrorCode::CurveCalculationError.into())
}

/// Sell tokens subject to the user's minimum SOL output.
pub fn sell_trade(
    ctx: Context<TradeBuySell>,
    sell_token_amount: u64,
    min_sol_output: u64,
) -> Result<()> {
    validate_trade_buy_sell_context(&ctx)?;

    if ctx.accounts.cooldown.last_trade_time == 0 {
        return err!(ErrorCode::CooldownNotInitialized);
    }
    crate::instructions::validate_trade_cooldown(&ctx.accounts.cooldown)?;
    if sell_token_amount > ctx.accounts.cooldown.approval_token_amount {
        return err!(ErrorCode::ExceedApprovalAmount);
    }
    if sell_token_amount < MIN_TRADE_TOKEN_AMOUNT {
        return err!(ErrorCode::InsufficientTradeAmount);
    }

    let swap_fee = ctx.accounts.curve_account.swap_fee;
    let _quote = sell_amounts(
        &mut ctx.accounts.curve_account,
        &ctx.accounts.down_orderbook.to_account_info(),
        None,
        sell_token_amount,
        min_sol_output,
        swap_fee,
    )?;

    // Token transfers, liquidation settlement, cooldown updates, and event emission are omitted.
    Err(ErrorCode::CurveCalculationError.into())
}

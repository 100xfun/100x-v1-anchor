// Context validation module - used to validate account constraints in transaction contexts
use {
    crate::error::ErrorCode,
    crate::instructions::contexts::{TradeBuySell, TradeClose, TradeLongShort},
    crate::instructions::pdas::BorrowingBondingCurve,
    anchor_lang::prelude::*,
};

/// Validates if fee recipient account addresses match
///
/// # Arguments
/// * `curve_account` - Borrowing liquidity pool account
/// * `fee_recipient_account` - Partner fee recipient account
/// * `base_fee_recipient_account` - Technology provider base fee recipient account
///
/// # Returns
/// * `Result<()>` - Returns Ok on successful validation, error on failure
pub fn validate_fee_recipient_accounts(
    curve_account: &Account<BorrowingBondingCurve>,
    fee_recipient_account: &UncheckedAccount,
    base_fee_recipient_account: &UncheckedAccount,
) -> Result<()> {
    // Verify partner fee recipient account
    require!(
        curve_account.fee_recipient == fee_recipient_account.key(),
        ErrorCode::InvalidPartnerFeeRecipientAccount
    );

    // Verify technology provider base fee recipient account
    require!(
        curve_account.base_fee_recipient == base_fee_recipient_account.key(),
        ErrorCode::InvalidBaseFeeRecipientAccount
    );

    Ok(())
}

/// Validates account constraints in TradeLongShort context
///
/// # Arguments
/// * `ctx` - TradeLongShort context
///
/// # Returns
/// * `Result<()>` - Returns Ok on successful validation, error on failure
pub fn validate_trade_long_short_context(ctx: &Context<TradeLongShort>) -> Result<()> {
    // Verify fee recipient accounts
    validate_fee_recipient_accounts(
        &ctx.accounts.curve_account,
        &ctx.accounts.fee_recipient_account,
        &ctx.accounts.base_fee_recipient_account,
    )?;

    // PDA account ownership verification
    require!(
        ctx.accounts.up_orderbook.to_account_info().owner == ctx.program_id,
        ErrorCode::InvalidAccountOwner
    );
    require!(
        ctx.accounts.down_orderbook.to_account_info().owner == ctx.program_id,
        ErrorCode::InvalidAccountOwner
    );

    // Orderbook address verification
    require!(
        ctx.accounts.curve_account.up_orderbook == ctx.accounts.up_orderbook.key(),
        ErrorCode::InvalidOrderbookAddress
    );
    require!(
        ctx.accounts.curve_account.down_orderbook == ctx.accounts.down_orderbook.key(),
        ErrorCode::InvalidOrderbookAddress
    );

    // Token account ownership verification
    require!(
        ctx.accounts.pool_token_account.owner == ctx.accounts.curve_account.key(),
        ErrorCode::InvalidAccountOwner
    );
    require!(
        ctx.accounts.pool_borrow_token_account.owner == ctx.accounts.curve_account.key(),
        ErrorCode::InvalidAccountOwner
    );

    Ok(())
}

/// Validates account constraints in TradeBuySell context
///
/// # Arguments
/// * `ctx` - TradeBuySell context
///
/// # Returns
/// * `Result<()>` - Returns Ok on successful validation, error on failure
pub fn validate_trade_buy_sell_context(ctx: &Context<TradeBuySell>) -> Result<()> {
    // 1. Verify fee recipient accounts
    validate_fee_recipient_accounts(
        &ctx.accounts.curve_account,
        &ctx.accounts.fee_recipient_account,
        &ctx.accounts.base_fee_recipient_account,
    )?;

    // PDA account ownership verification
    require!(
        ctx.accounts.up_orderbook.to_account_info().owner == ctx.program_id,
        ErrorCode::InvalidAccountOwner
    );
    require!(
        ctx.accounts.down_orderbook.to_account_info().owner == ctx.program_id,
        ErrorCode::InvalidAccountOwner
    );

    // Token account ownership verification
    require!(
        ctx.accounts.pool_token_account.owner == ctx.accounts.curve_account.key(),
        ErrorCode::InvalidAccountOwner
    );
    require!(
        ctx.accounts.pool_borrow_token_account.owner == ctx.accounts.curve_account.key(),
        ErrorCode::InvalidAccountOwner
    );
    require!(
        ctx.accounts.user_token_account.owner == ctx.accounts.payer.key(),
        ErrorCode::InvalidAccountOwner
    );

    // Orderbook address verification
    require!(
        ctx.accounts.curve_account.up_orderbook == ctx.accounts.up_orderbook.key(),
        ErrorCode::InvalidOrderbookAddress
    );
    require!(
        ctx.accounts.curve_account.down_orderbook == ctx.accounts.down_orderbook.key(),
        ErrorCode::InvalidOrderbookAddress
    );

    Ok(())
}

/// Validates account constraints in TradeClose context
///
/// # Arguments
/// * `ctx` - TradeClose context
///
/// # Returns
/// * `Result<()>` - Returns Ok on successful validation, error on failure
pub fn validate_trade_close_context(ctx: &Context<TradeClose>) -> Result<()> {
    // Verify fee recipient accounts
    validate_fee_recipient_accounts(
        &ctx.accounts.curve_account,
        &ctx.accounts.fee_recipient_account,
        &ctx.accounts.base_fee_recipient_account,
    )?;

    // PDA account ownership verification
    require!(
        ctx.accounts.up_orderbook.to_account_info().owner == ctx.program_id,
        ErrorCode::InvalidAccountOwner
    );
    require!(
        ctx.accounts.down_orderbook.to_account_info().owner == ctx.program_id,
        ErrorCode::InvalidAccountOwner
    );

    // Token account ownership verification
    require!(
        ctx.accounts.pool_token_account.owner == ctx.accounts.curve_account.key(),
        ErrorCode::InvalidAccountOwner
    );
    require!(
        ctx.accounts.pool_borrow_token_account.owner == ctx.accounts.curve_account.key(),
        ErrorCode::InvalidAccountOwner
    );

    // Orderbook address verification
    require!(
        ctx.accounts.curve_account.up_orderbook == ctx.accounts.up_orderbook.key(),
        ErrorCode::InvalidOrderbookAddress
    );
    require!(
        ctx.accounts.curve_account.down_orderbook == ctx.accounts.down_orderbook.key(),
        ErrorCode::InvalidOrderbookAddress
    );

    Ok(())
}

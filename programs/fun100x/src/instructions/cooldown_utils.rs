use crate::constants::TRADE_COOLDOWN_SECONDS;
use crate::error::ErrorCode;
use crate::instructions::pdas::TradeCooldown;
use anchor_lang::prelude::*;

/// Validate trade cooldown period
pub fn validate_trade_cooldown(cooldown: &TradeCooldown) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let elapsed_time = current_time.saturating_sub(cooldown.last_trade_time);

    if elapsed_time < TRADE_COOLDOWN_SECONDS as i64 {
        // msg!(
        //     "Cooldown period not expired: elapsed {} seconds, required {} seconds",
        //     elapsed_time,
        //     TRADE_COOLDOWN_SECONDS
        // );
        return Err(ErrorCode::TradeCooldownNotExpired.into());
    }

    Ok(())
}

/// Update cooldown record
pub fn update_cooldown_record(
    cooldown: &mut Account<TradeCooldown>,
    new_token_amount: u64,
    bump: u8,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;

    cooldown.last_trade_time = current_time;
    cooldown.approval_token_amount = new_token_amount;
    cooldown.bump = bump;

    // msg!("Cooldown record updated: time={}, amount={}", current_time, new_token_amount);
    Ok(())
}

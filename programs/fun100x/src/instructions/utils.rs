use crate::error::ErrorCode;
use anchor_lang::prelude::*;

/// Fee split calculation result
#[derive(Debug)]
pub struct FeeSplitResult {
    /// Partner fee amount
    pub partner_fee: u64,
    /// Technology provider base fee amount
    pub base_fee: u64,
}

/// Calculate the fee each party deserves based on fee split ratio
///
/// # Parameters
/// * `total_fee` - Total fee amount
/// * `fee_split` - Fee split ratio (0-100), representing the percentage for partner
///
/// # Returns
/// * `FeeSplitResult` - Struct containing fee amounts for each party
///
/// # Example
/// ```
/// let result = calculate_fee_split(1000, 80);
/// // Partner gets 800, technology provider gets 200
/// assert_eq!(result.partner_fee, 800);
/// assert_eq!(result.base_fee, 200);
/// ```
pub fn calculate_fee_split(total_fee: u64, fee_split: u8) -> Result<FeeSplitResult> {
    // Validate fee_split range
    if fee_split > 100 {
        return Err(ErrorCode::InvalidFeePercentage.into());
    }

    // Handle special case when total_fee is 0
    if total_fee == 0 {
        return Ok(FeeSplitResult {
            partner_fee: 0,
            base_fee: 0,
        });
    }

    // Calculate partner fee (by percentage)
    let partner_fee = if fee_split == 0 {
        0
    } else {
        // Strict check: ensure divisor is not 0
        const DIVISOR: u64 = 100;

        total_fee
            .checked_mul(fee_split as u64)
            .ok_or(ErrorCode::FeeSplitCalculationOverflow)?
            .checked_div(DIVISOR)
            .ok_or(ErrorCode::FeeSplitCalculationOverflow)?
    };

    // Technology provider fee = total fee - partner fee
    let base_fee = total_fee
        .checked_sub(partner_fee)
        .ok_or(ErrorCode::FeeSplitCalculationOverflow)?;

    Ok(FeeSplitResult {
        partner_fee,
        base_fee,
    })
}

/// Minimum balance pool_sol_account must keep after any outflow:
/// the live rent-exempt minimum for its data length (at least 1 lamport).
pub fn pool_sol_required_balance(pool_sol_account: &AccountInfo) -> Result<u64> {
    Ok(Rent::get()?
        .minimum_balance(pool_sol_account.data_len())
        .max(1))
}

/// Ensure pool_sol_account stays at or above the live rent-exempt minimum after a deduction.
/// `balance_after` is computed by the caller with checked_sub (keeps each path's own overflow error).
pub fn ensure_pool_sol_min_balance(
    pool_sol_account: &AccountInfo,
    balance_after: u64,
) -> Result<()> {
    let required_balance = pool_sol_required_balance(pool_sol_account)?;
    if balance_after < required_balance {
        msg!(
            "pool_sol below rent-exempt minimum: balance_after={}, required={}, shortfall={}",
            balance_after,
            required_balance,
            required_balance - balance_after
        );
        return Err(ErrorCode::InsufficientPoolBalance.into());
    }
    Ok(())
}

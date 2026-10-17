//! Transfer and fee operation interfaces used by the instruction handlers.
//! Their execution logic is omitted from this public source.

#[macro_export]
macro_rules! transfer_close_fees_split {
    ($total_fees:expr, $pool_sol_account:expr, $fee_recipient_account:expr, $base_fee_recipient_account:expr, $fee_split:expr) => {{
        Err::<(), anchor_lang::error::Error>(crate::error::ErrorCode::CurveCalculationError.into())
    }};
}

#[macro_export]
macro_rules! apply_fee_discount_if_needed {
    ($ctx:expr) => {{
        Err::<(), anchor_lang::error::Error>(crate::error::ErrorCode::CurveCalculationError.into())
    }};
}

#[macro_export]
macro_rules! transfer_lamports {
    ($amount:expr, $from_account:expr, $to_account:expr, $rent_sysvar:expr, $base_fee_recipient:expr) => {{
        Err::<(), anchor_lang::error::Error>(crate::error::ErrorCode::CurveCalculationError.into())
    }};
}

#[macro_export]
macro_rules! transfer_pool_to_user {
    ($amount:expr, $ctx:expr, $purpose:expr) => {{
        Err::<(), anchor_lang::error::Error>(
            crate::error::ErrorCode::CurveCalculationError.into(),
        )?;
    }};
}

#[macro_export]
macro_rules! transfer_pool_to_user_if_positive {
    ($amount:expr, $ctx:expr) => {{
        Err::<(), anchor_lang::error::Error>(
            crate::error::ErrorCode::CurveCalculationError.into(),
        )?;
    }};
}

#[macro_export]
macro_rules! repay_liquidated_token_to_borrow_pool {
    ($ctx:expr, $amount:expr) => {{
        Err::<(), anchor_lang::error::Error>(
            crate::error::ErrorCode::CurveCalculationError.into(),
        )?;
    }};
}

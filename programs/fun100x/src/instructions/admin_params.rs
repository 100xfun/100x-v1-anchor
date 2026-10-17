// Import necessary modules
use {
    crate::constants::{MAX_BORROW_DURATION, MIN_BORROW_DURATION, MIN_BORROW_FEE, MIN_SWAP_FEE},
    crate::curve::curve_amm::MAX_FEE_RATE,
    crate::error::ErrorCode,
    crate::instructions::contexts::{CreateParams, UpdateAdmin, UpdateParams},
    anchor_lang::prelude::*,
};

// Handler function for updating Admin, supports initialization and update
pub fn update_admin(
    ctx: Context<UpdateAdmin>,
    default_swap_fee: Option<u16>,
    default_borrow_fee: Option<u16>,
    default_borrow_duration: Option<u32>,
    base_fee_recipient: Option<Pubkey>,
    default_fee_split: Option<u8>,
    new_admin: Option<Pubkey>,
) -> Result<()> {
    let admin_account = &mut ctx.accounts.admin_account;

    // Determine if it's a new account: check if admin is default value (most reliable method)
    let is_new_account = admin_account.admin == Pubkey::default();

    // If not a new account, perform double permission verification
    if !is_new_account {
        // First verification: basic permission check
        if admin_account.admin != ctx.accounts.admin.key() {
            // msg!("Unauthorized: Caller is not the stored admin. Expected: {}, Got: {}",
            //      admin_account.admin, ctx.accounts.admin.key());
            return Err(ErrorCode::Unauthorized.into());
        }

        // Second verification: ensure admin account field is not default value (prevent accidental reset)
        // if admin_account.admin == Pubkey::default() {
        //     // msg!("Invalid state: Admin account admin field is default value");
        //     return Err(ErrorCode::Unauthorized.into());
        // }

        // msg!("Admin update verification passed for existing account: {}", ctx.accounts.admin.key());
    } else {
        // msg!("Initializing new admin account for: {}", ctx.accounts.admin.key());
    }

    // If it's a new account, initialize bump value
    if is_new_account {
        admin_account.bump = ctx.bumps.admin_account;
    }

    // Set or update default swap fee
    if let Some(fee) = default_swap_fee {
        // Validate fee is within reasonable range (0.02% - 10%)
        if fee < MIN_SWAP_FEE || fee > MAX_FEE_RATE {
            return Err(ErrorCode::InvalidFeeRate.into());
        }
        admin_account.default_swap_fee = fee;
        // msg!("{} default swap fee: {}", if is_new_account { "Set" } else { "Updated" }, fee);
    } else if is_new_account {
        // New account must set initial value
        return Err(ErrorCode::RequiredParameter.into());
    }

    // Set or update default borrow fee
    if let Some(fee) = default_borrow_fee {
        // Validate fee is within reasonable range (0.05% - 10%)
        if fee < MIN_BORROW_FEE || fee > MAX_FEE_RATE {
            return Err(ErrorCode::InvalidFeeRate.into());
        }
        admin_account.default_borrow_fee = fee;
        // msg!("{} default borrow fee: {}", if is_new_account { "Set" } else { "Updated" }, fee);
    } else if is_new_account {
        // New account must set initial value
        return Err(ErrorCode::RequiredParameter.into());
    }

    // Set or update default borrow duration
    if let Some(duration) = default_borrow_duration {
        // Validate borrow duration is within reasonable range (3 days to 30 days)
        if duration < MIN_BORROW_DURATION || duration > MAX_BORROW_DURATION {
            return Err(ErrorCode::InvalidBorrowDuration.into());
        }
        admin_account.default_borrow_duration = duration;
        // msg!("{} default borrow duration (seconds): {}", if is_new_account { "Set" } else { "Updated" }, duration);
    } else if is_new_account {
        // New account must set initial value
        return Err(ErrorCode::RequiredParameter.into());
    }

    // Set or update base fee recipient address
    if let Some(recipient) = base_fee_recipient {
        admin_account.base_fee_recipient = recipient;
        // msg!("{} base fee recipient: {}", if is_new_account { "Set" } else { "Updated" }, recipient);
    } else if is_new_account {
        // If new account is not set, use default value (current admin address)
        admin_account.base_fee_recipient = ctx.accounts.admin.key();
        // msg!("Set default base fee recipient: {}", ctx.accounts.admin.key());
    }

    // Set or update default fee split ratio
    if let Some(split) = default_fee_split {
        if split > 100 {
            return Err(ErrorCode::InvalidFeePercentage.into());
        }
        admin_account.default_fee_split = split;
        // msg!("{} default fee split: {}%", if is_new_account { "Set" } else { "Updated" }, split);
    } else if is_new_account {
        // Set default value of 80% for new account
        admin_account.default_fee_split = 80;
        // msg!("Set default fee split: 80%");
    }

    // Set or update super admin account
    if let Some(admin_key) = new_admin {
        admin_account.admin = admin_key;
        // msg!("{} admin: {}", if is_new_account { "Set" } else { "Updated" }, admin_key);
    } else if is_new_account {
        // Set current signer as admin for new account
        admin_account.admin = ctx.accounts.admin.key();
        // msg!("Set admin: {}", ctx.accounts.admin.key());
    }

    // msg!("Admin {} successfully!", if is_new_account { "initialized" } else { "updated" });

    Ok(())
}

// Handler function for creating partner parameters
pub fn create_params(ctx: Context<CreateParams>) -> Result<()> {
    let admin_account = &ctx.accounts.admin_account;
    let params = &mut ctx.accounts.params;
    let partner = &ctx.accounts.partner;

    // Initialize bump value
    params.bump = ctx.bumps.params;

    // Copy default values from Admin account (no additional validation needed, as admin account values are validated when set)
    params.base_swap_fee = admin_account.default_swap_fee;
    params.base_borrow_fee = admin_account.default_borrow_fee;
    params.base_borrow_duration = admin_account.default_borrow_duration;
    params.base_fee_recipient = admin_account.base_fee_recipient;
    params.fee_recipient = partner.key(); // Set partner fee recipient account to caller
    params.fee_split = admin_account.default_fee_split;

    // msg!("Created params for partner: {}", partner.key());
    // msg!("Base swap fee: {}", params.base_swap_fee);
    // msg!("Base borrow fee: {}", params.base_borrow_fee);
    // msg!("Base borrow duration: {}", params.base_borrow_duration);
    // msg!("Base fee recipient: {}", params.base_fee_recipient);
    // msg!("Partner fee recipient: {}", params.fee_recipient);
    // msg!("Fee split: {}%", params.fee_split);

    Ok(())
}

// Handler function for updating partner parameters (only super admin can call)
pub fn update_params(
    ctx: Context<UpdateParams>,
    _partner_pubkey: Pubkey,
    base_swap_fee: Option<u16>,
    base_borrow_fee: Option<u16>,
    base_borrow_duration: Option<u32>,
    base_fee_recipient: Option<Pubkey>,
    fee_split: Option<u8>,
) -> Result<()> {
    // Double verification: ensure caller is indeed the super admin
    let admin_account = &ctx.accounts.admin_account;
    let admin_signer = &ctx.accounts.admin;

    // First verification: check if admin account is initialized
    if admin_account.admin == Pubkey::default() {
        // msg!("Admin account not properly initialized");
        return Err(ErrorCode::Unauthorized.into());
    }

    // Second verification: confirm caller is the stored admin
    if admin_account.admin != admin_signer.key() {
        // msg!("Caller is not the stored admin. Expected: {}, Got: {}",
        //      admin_account.admin, admin_signer.key());
        return Err(ErrorCode::Unauthorized.into());
    }

    // msg!("Admin verification passed for: {}", admin_signer.key());

    let params = &mut ctx.accounts.params;

    // Set or update swap fee
    if let Some(fee) = base_swap_fee {
        // Validate fee is within reasonable range (0.02% - 10%)
        if fee < MIN_SWAP_FEE || fee > MAX_FEE_RATE {
            return Err(ErrorCode::InvalidFeeRate.into());
        }
        params.base_swap_fee = fee;
        // msg!("Updated base swap fee: {}", fee);
    }

    // Set or update borrow fee
    if let Some(fee) = base_borrow_fee {
        // Validate fee is within reasonable range (0.05% - 10%)
        if fee < MIN_BORROW_FEE || fee > MAX_FEE_RATE {
            return Err(ErrorCode::InvalidFeeRate.into());
        }
        params.base_borrow_fee = fee;
        // msg!("Updated base borrow fee: {}", fee);
    }

    // Set or update borrow duration
    if let Some(duration) = base_borrow_duration {
        // Validate borrow duration is within reasonable range (3 days to 30 days)
        if duration < MIN_BORROW_DURATION || duration > MAX_BORROW_DURATION {
            return Err(ErrorCode::InvalidBorrowDuration.into());
        }
        params.base_borrow_duration = duration;
        // msg!("Updated base borrow duration (seconds): {}", duration);
    }

    // Set or update base fee recipient address
    if let Some(recipient) = base_fee_recipient {
        params.base_fee_recipient = recipient;
        // msg!("Updated base fee recipient: {}", recipient);
    }

    // Set or update fee split ratio
    if let Some(split) = fee_split {
        if split > 100 {
            return Err(ErrorCode::InvalidFeePercentage.into());
        }
        params.fee_split = split;
        // msg!("Updated fee split: {}%", split);
    }

    // msg!("Parameters updated successfully for partner: {}", partner_pubkey);

    Ok(())
}

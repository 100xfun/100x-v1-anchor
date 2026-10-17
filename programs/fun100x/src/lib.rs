//! Public source snapshot of the 100x.fun contract.
//! Core execution routines are intentionally omitted. Do not deploy this source.

#[cfg(any(target_arch = "bpf", target_os = "solana"))]
compile_error!("This public source omits core execution logic and must not be deployed.");

use anchor_lang::prelude::*;
// Import instruction modules
mod instructions;
use instructions::*;

// Import curve modules
pub mod curve;
//pub mod utils;
pub mod constants;
pub mod error;
pub mod types;

declare_id!("CYzM71ENGdE6Gjx5NWt4r12EbhGX2DnoEua5oWh8xpCz");

#[program]
pub mod fun100x {
    use super::*;

    // Update Admin configuration instruction
    pub fn update_admin(
        ctx: Context<UpdateAdmin>,
        default_swap_fee: Option<u16>,
        default_borrow_fee: Option<u16>,
        default_borrow_duration: Option<u32>,
        base_fee_recipient: Option<Pubkey>,
        default_fee_split: Option<u8>,
        new_admin: Option<Pubkey>,
    ) -> Result<()> {
        instructions::admin_params::update_admin(
            ctx,
            default_swap_fee,
            default_borrow_fee,
            default_borrow_duration,
            base_fee_recipient,
            default_fee_split,
            new_admin,
        )
    }

    // Create partner parameters instruction
    pub fn create_params(ctx: Context<CreateParams>) -> Result<()> {
        instructions::admin_params::create_params(ctx)
    }

    // Update partner parameters instruction (only callable by super admin)
    pub fn update_params(
        ctx: Context<UpdateParams>,
        partner_pubkey: Pubkey,
        base_swap_fee: Option<u16>,
        base_borrow_fee: Option<u16>,
        base_borrow_duration: Option<u32>,
        base_fee_recipient: Option<Pubkey>,
        fee_split: Option<u8>,
    ) -> Result<()> {
        instructions::admin_params::update_params(
            ctx,
            partner_pubkey,
            base_swap_fee,
            base_borrow_fee,
            base_borrow_duration,
            base_fee_recipient,
            fee_split,
        )
    }

    // Create basic token instruction
    pub fn create_token(
        ctx: Context<CreateToken>,
        name: String,
        symbol: String,
        uri: String,
        custom_lp_sol: Option<u64>,
        custom_lp_token: Option<u64>,
        custom_borrow_ratio: Option<u8>,
        custom_borrow_duration: Option<u32>,
        custom_fee: Option<u16>,
    ) -> Result<()> {
        instructions::create_token::create_token(
            ctx,
            name,
            symbol,
            uri,
            custom_lp_sol,
            custom_lp_token,
            custom_borrow_ratio,
            custom_borrow_duration,
            custom_fee,
        )
    }

    // Alias for create basic token instruction (maintain backward compatibility)
    // pub fn create(
    //     ctx: Context<CreateToken>,
    //     name: String,
    //     symbol: String,
    //     uri: String,
    //     custom_lp_sol: Option<u64>,
    //     custom_lp_token: Option<u64>,
    //     custom_borrow_ratio: Option<u8>,
    //     custom_borrow_duration: Option<u32>,
    //     custom_fee: Option<u16>,
    // ) -> Result<()> {
    //     instructions::create_token::create_token(
    //         ctx,
    //         name,
    //         symbol,
    //         uri,
    //         custom_lp_sol,
    //         custom_lp_token,
    //         custom_borrow_ratio,
    //         custom_borrow_duration,
    //         custom_fee,
    //     )
    // }

    // Margin long trading instruction
    pub fn long(
        ctx: Context<TradeLongShort>,
        buy_token_amount: u64,          // Desired token amount to buy
        max_sol_amount: u64,            // Maximum SOL amount willing to pay
        margin_sol: u64,                // Margin amount (SOL)
        close_price: u128,              // Close price
        close_insert_indices: Vec<u16>, // Position indices for inserting into order book when opening
    ) -> Result<()> {
        instructions::long_short::long_trade(
            ctx,
            buy_token_amount,
            max_sol_amount,
            margin_sol,
            close_price,
            close_insert_indices,
        )
    }

    // Margin short trading instruction
    pub fn short(
        ctx: Context<TradeLongShort>,
        borrow_sell_token_amount: u64,  // Desired token amount to sell
        min_sol_output: u64,            // Minimum SOL amount to receive after selling
        margin_sol: u64,                // Margin amount (SOL)
        close_price: u128,              // Close price
        close_insert_indices: Vec<u16>, // Position indices for inserting into order book when opening
    ) -> Result<()> {
        instructions::long_short::short_trade(
            ctx,
            borrow_sell_token_amount,
            min_sol_output,
            margin_sol,
            close_price,
            close_insert_indices,
        )
    }

    // Spot buy trading instruction
    pub fn buy(
        ctx: Context<TradeBuySell>,
        buy_token_amount: u64, // Desired token amount to buy
        max_sol_amount: u64,   // Maximum SOL amount willing to pay
    ) -> Result<()> {
        instructions::buy_sell::buy_trade(ctx, buy_token_amount, max_sol_amount)
    }

    // Spot sell trading instruction
    pub fn sell(
        ctx: Context<TradeBuySell>,
        sell_token_amount: u64, // Desired token amount to sell
        min_sol_output: u64,    // Minimum SOL amount to receive after selling
    ) -> Result<()> {
        instructions::buy_sell::sell_trade(ctx, sell_token_amount, min_sol_output)
    }

    // Approve current token balance for trading
    // Used in the following scenarios:
    // 1. Want to trade immediately after receiving tokens from another address
    // 2. Reactivate cooldown PDA
    pub fn approve_trade(ctx: Context<ApproveCooldown>) -> Result<()> {
        // msg!("Approve tokens for trading");

        // ========== If PDA already exists, validate cooldown time ==========
        if ctx.accounts.cooldown.last_trade_time > 0 {
            instructions::validate_trade_cooldown(&ctx.accounts.cooldown)?;
        }

        // ========== Update approval amount to current balance ==========
        let current_token_balance = ctx.accounts.user_token_account.amount;

        instructions::update_cooldown_record(
            &mut ctx.accounts.cooldown,
            current_token_balance,
            ctx.bumps.cooldown,
        )?;

        // msg!("Approval successful: approval_token_amount = {}", current_token_balance);

        Ok(())
    }

    // Manually close TradeCooldown PDA and reclaim rent
    //
    // Usage conditions:
    // 1. Can only close your own PDA (verified through seeds)
    //
    // Usage scenarios:
    // - User wants to reclaim rent
    // - Clean up unused PDA
    //
    // Security requirements:
    // - Must have passed cooldown time
    // - Token balance must be 0
    // - After PDA is closed, next buy or approve will automatically recreate it
    pub fn close_trade_cooldown(ctx: Context<CloseCooldown>) -> Result<()> {
        // msg!("Manually close TradeCooldown PDA");

        // Validation 1: Check if cooldown period has passed
        validate_trade_cooldown(&ctx.accounts.cooldown)?;

        // Validation 2: Check token balance must be 0
        let token_balance = ctx.accounts.user_token_account.amount;
        if token_balance != 0 {
            return Err(error::ErrorCode::CannotCloseCooldownWithBalance.into());
        }

        // msg!("Current token balance: {}, PDA closed successfully, rent returned to user", token_balance);

        // PDA will be automatically closed (through close = payer constraint)
        Ok(())
    }

    // Close long position trading instruction
    pub fn close_long(
        ctx: Context<TradeClose>,
        sell_token_amount: u64,        // Desired token amount to sell
        min_sol_output: u64,           // Minimum SOL amount to receive after selling
        close_order_id: u64,           // Order's unique identifier
        close_order_indices: Vec<u16>, // Position indices when closing (can have multiple, will auto-find second position if order was moved)
    ) -> Result<()> {
        instructions::close_long_short::close_long_trade(
            ctx,
            sell_token_amount,
            min_sol_output,
            close_order_id,
            close_order_indices,
        )
    }

    // Close short position trading instruction
    pub fn close_short(
        ctx: Context<TradeClose>,
        buy_token_amount: u64,         // Token amount to buy
        max_sol_amount: u64,           // Maximum SOL amount willing to pay
        close_order_id: u64,           // Order's unique identifier
        close_order_indices: Vec<u16>, // Position indices when closing (can have multiple, will auto-find second position if order was moved)
    ) -> Result<()> {
        instructions::close_long_short::close_short_trade(
            ctx,
            buy_token_amount,
            max_sol_amount,
            close_order_id,
            close_order_indices,
        )
    }
}

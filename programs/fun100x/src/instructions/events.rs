// Import necessary modules
use anchor_lang::prelude::*;

// -----------------------------Events------------------------------

// Define basic token creation event
#[event]
pub struct TokenCreatedEvent {
    pub payer: Pubkey,
    pub mint_account: Pubkey,
    pub curve_account: Pubkey,
    pub pool_token_account: Pubkey,
    pub pool_borrow_token_account: Pubkey, // Borrow pool dedicated token account address
    pub pool_sol_account: Pubkey,
    pub fee_recipient: Pubkey,
    pub base_fee_recipient: Pubkey, // Base fee recipient account
    pub params_account: Pubkey,     // Partner parameters account PDA address
    pub swap_fee: u16,              // Spot trading fee
    pub borrow_fee: u16,            // Margin trading fee
    pub fee_discount_flag: u8, // Fee discount flag 0: Original price 1: 50% off 2: 75% off 3: 87.5% off
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub up_orderbook: Pubkey, // Short order book (Up direction) PDA address
    pub down_orderbook: Pubkey, // Long order book (Down direction) PDA address
    pub latest_price: u128,   // Latest price

    // ============ New fields: Dynamic liquidity pool parameters ============
    pub initial_virtual_sol: u64, // Initial virtual SOL reserve (lamports, 9 decimal precision, default 30 SOL)
    pub initial_virtual_token: u64, // Initial virtual Token reserve (smallest unit, 9 decimal precision, default 1.073 billion tokens)
    pub initial_borrow_token_reserve: u64, // Borrow pool initial funding (initial token amount)

    // ============ New fields: Advanced pool parameters ============
    pub pool_type: u8,         // Pool type: 0=standard, 1=advanced
    pub borrow_pool_ratio: u8, // Borrow pool token allocation percentage
}

// Define buy/sell trade event
#[event]
pub struct BuySellEvent {
    pub payer: Pubkey,
    pub mint_account: Pubkey,
    pub is_buy: bool,
    pub token_amount: u64,           // Final token amount bought or sold
    pub sol_amount: u64,             // Final SOL amount spent or received
    pub latest_price: u128,          // Latest price
    pub liquidate_indices: Vec<u16>, // List of order indices to liquidate (indices not order IDs!)
}

// Define margin long/short trade event
#[event]
pub struct LongShortEvent {
    pub payer: Pubkey,
    pub mint_account: Pubkey,

    // Curve trade direction for this position opening (long = buy, short = sell).
    pub is_buy: bool,
    // Executed token amount in the token's smallest unit.
    pub token_amount: u64,
    // Net curve trade amount in lamports, excluding fees.
    pub sol_amount: u64,

    pub order_id: u64,               // Unique ID of the opened order
    pub order_index: u16,            // Index of the opened order in the order book
    pub latest_price: u128,          // Latest price
    pub open_price: u128,            // Opening price
    pub order_type: u8,              // Order type 1: Long 2: Short
    pub lock_lp_start_price: u128,   // Locked liquidity pool range start price
    pub lock_lp_end_price: u128,     // Locked liquidity pool range end price
    pub lock_lp_sol_amount: u64,     // Locked liquidity pool range SOL amount
    pub lock_lp_token_amount: u64,   // Locked liquidity pool range token amount
    pub start_time: i64,             // Order start timestamp (seconds)
    pub end_time: i64,               // Loan expiration timestamp (seconds)
    pub margin_sol_amount: u64,      // Margin SOL amount
    pub borrow_amount: u64,          // Borrowed amount
    pub position_asset_amount: u64,  // Current position asset amount
    pub borrow_fee: u16,             // Margin trading fee
    pub liquidate_indices: Vec<u16>, // List of order indices to liquidate (indices not order IDs!)
}

// Define full close event
#[event]
pub struct FullCloseEvent {
    pub payer: Pubkey,
    pub user_sol_account: Pubkey, // SOL account of the user who opened the close_order
    pub mint_account: Pubkey,
    pub is_close_long: bool,         // Whether closing a long position
    pub final_token_amount: u64,     // Final token amount bought or sold
    pub final_sol_amount: u64,       // Final SOL amount spent or received
    pub user_close_profit: u64,      // User's SOL profit from closing position
    pub latest_price: u128,          // Latest price
    pub order_id: u64,               // Unique ID of the closed order
    pub order_index: u16,            // Index of the closed order in the order book
    pub liquidate_indices: Vec<u16>, // List of order indices to liquidate, including the closed order itself (indices not order IDs!)
}

// Define partial close event
#[event]
pub struct PartialCloseEvent {
    pub payer: Pubkey,
    pub user_sol_account: Pubkey, // SOL account of the user who opened the close_order
    pub mint_account: Pubkey,
    pub is_close_long: bool,     // Whether closing a long position
    pub final_token_amount: u64, // Final token amount bought or sold
    pub final_sol_amount: u64,   // Final SOL amount spent or received
    pub user_close_profit: u64,  // User's SOL profit from closing position
    pub latest_price: u128,      // Latest price
    pub order_id: u64,           // Unique ID of the closed order
    pub order_index: u16,        // Index of the opened order in the order book
    // Partial close order parameters (updated values)
    pub order_type: u8,              // Order type 1: Long 2: Short
    pub user: Pubkey,                // User who opened position
    pub lock_lp_start_price: u128,   // Locked liquidity pool range start price
    pub lock_lp_end_price: u128,     // Locked liquidity pool range end price
    pub lock_lp_sol_amount: u64,     // Locked liquidity pool range SOL amount
    pub lock_lp_token_amount: u64,   // Locked liquidity pool range token amount
    pub start_time: i64,             // Order start timestamp (seconds)
    pub end_time: i64,               // Loan expiration timestamp (seconds)
    pub margin_sol_amount: u64,      // Margin SOL amount
    pub borrow_amount: u64,          // Borrowed amount
    pub position_asset_amount: u64,  // Current position amount
    pub borrow_fee: u16,             // Margin trading fee
    pub realized_sol_amount: u64,    // Realized profit/loss SOL amount
    pub liquidate_indices: Vec<u16>, // List of order indices to liquidate (indices not order IDs!)
}

// Trading milestone discount event
#[event]
pub struct MilestoneDiscountEvent {
    pub payer: Pubkey,
    pub mint_account: Pubkey,
    pub curve_account: Pubkey,
    pub swap_fee: u16,         // Spot trading fee
    pub borrow_fee: u16,       // Margin trading fee
    pub fee_discount_flag: u8, // Fee discount flag 0: Original price 1: 50% off 2: 75% off 3: 87.5% off
}

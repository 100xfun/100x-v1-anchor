// Program-derived account layouts
use anchor_lang::prelude::*;

// Admin is mainly used to allocate super administrator
#[account]
#[derive(InitSpace)]
pub struct Admin {
    #[max_len(2)]
    pub default_swap_fee: u16, // Default spot trading fee swap_fee
    #[max_len(2)]
    pub default_borrow_fee: u16, // Default margin trading fee borrow_fee
    #[max_len(4)]
    pub default_borrow_duration: u32, // Default maximum lending duration in seconds
    #[max_len(32)]
    pub base_fee_recipient: Pubkey, // Base fee recipient account
    #[max_len(1)]
    pub default_fee_split: u8, // Default fee allocation ratio (0-100), example: 80: operator gets 80%, technology provider gets 20%
    #[max_len(32)]
    pub admin: Pubkey, // Super admin account
    #[max_len(1)]
    pub bump: u8, // PDA bump
}

// Trading configuration data
#[account]
#[derive(InitSpace)]
pub struct Params {
    #[max_len(2)]
    pub base_swap_fee: u16, // Default spot trading fee swap_fee
    #[max_len(2)]
    pub base_borrow_fee: u16, // Default margin trading fee borrow_fee
    #[max_len(4)]
    pub base_borrow_duration: u32, // Default maximum lending duration in seconds
    #[max_len(32)]
    pub base_fee_recipient: Pubkey, // Base fee recipient account
    #[max_len(32)]
    pub fee_recipient: Pubkey, // Partner fee recipient account
    #[max_len(1)]
    pub fee_split: u8, // Fee allocation ratio (0-100), example: 80: partner gets 80%, base fee recipient gets 20%
    #[max_len(1)]
    pub bump: u8, // PDA bump
}

// Define borrowing bonding curve account structure
#[account]
#[derive(InitSpace)]
pub struct BorrowingBondingCurve {
    #[max_len(8)]
    pub lp_token_reserve: u64, // Token amount in liquidity pool
    #[max_len(8)]
    pub lp_sol_reserve: u64, // SOL amount in liquidity pool
    #[max_len(16)]
    pub price: u128, // Current price
    #[max_len(8)]
    pub borrow_token_reserve: u64, // Token amount in virtual borrow pool
    #[max_len(8)]
    pub borrow_sol_reserve: u64, // SOL amount in virtual borrow pool
    #[max_len(2)]
    pub swap_fee: u16, // Spot trading fee (ratio multiplied by 10000, so 0.99 = 9900)
    #[max_len(2)]
    pub borrow_fee: u16, // Margin trading fee (ratio multiplied by 10000, so 0.9975 = 9975)
    #[max_len(1)]
    pub fee_discount_flag: u8, // Fee discount flag 0: original price 1: 50% off (10x) 2: 75% off (100x) 3: 87.5% off (1000x, terminal)
    #[max_len(32)]
    pub base_fee_recipient: Pubkey, // Technology provider base fee recipient account
    #[max_len(32)]
    pub fee_recipient: Pubkey, // Partner fee recipient account
    #[max_len(1)]
    pub fee_split: u8, // Fee allocation ratio (0-100), example: 80: partner gets 80%, technology provider gets 20%
    #[max_len(4)]
    pub borrow_duration: u32, // Loan duration (seconds)
    #[max_len(32)]
    pub mint: Pubkey, // Associated token mint account
    #[max_len(32)]
    pub up_orderbook: Pubkey, // Short order book address (new)
    #[max_len(32)]
    pub down_orderbook: Pubkey, // Long order book address (new)
    #[max_len(32)]
    pub creator: Pubkey, // Token creator address
    #[max_len(1)]
    pub bump: u8, // PDA bump

    // ============ New fields: Dynamic liquidity pool parameters ============
    /// Initial virtual SOL reserve (lamports, 9-digit precision)
    /// Used to calculate k value and price curve
    /// Default value: 30_000_000_000 (30 SOL)
    #[max_len(8)]
    pub initial_virtual_sol: u64,

    /// Initial virtual Token reserve (minimum unit, 9-digit precision)
    /// Used to calculate k value and price curve
    /// Default value: 1_073_000_000_000_000_000 (1.073 billion Token)
    #[max_len(8)]
    pub initial_virtual_token: u64,

    /// Borrow pool initial funding (initial token amount minted into borrow pool at creation)
    /// Unlike borrow_token_reserve which changes in real time with borrow/repay,
    /// this field never changes after creation; used by external parties to compute
    /// "borrowed out = initial funding - current available reserve".
    #[max_len(8)]
    pub initial_borrow_token_reserve: u64,

    // ============ New fields: Phase 2 - Advanced pool ============
    /// Pool type
    /// 0 = Basic version (uses default parameters)
    /// 1 = Advanced version (custom parameters)
    /// Note: BOTH pool types participate in fee halving milestones
    /// (dynamic thresholds at 10x/100x/1000x of the pool's own initial price)
    #[max_len(1)]
    pub pool_type: u8,

    /// Borrow pool token ratio (recorded only for information display)
    /// Actual value range: 2-8 (represents 2%-8%, deducted from total supply)
    /// Basic version fixed at 4
    #[max_len(1)]
    pub borrow_pool_ratio: u8,
}

// Trade cooldown PDA
// Used to prevent high-frequency trading and transfer bypass attacks
#[account]
#[derive(InitSpace)]
pub struct TradeCooldown {
    // Timestamp of the most recent trade (Unix timestamp, seconds) - 8 bytes
    #[max_len(8)]
    pub last_trade_time: i64,

    // Approved token amount (balance snapshot after last trade) - 8 bytes
    // Used to prevent users from bypassing cooldown through transfers
    #[max_len(8)]
    pub approval_token_amount: u64,

    // PDA bump - 1 byte
    #[max_len(1)]
    pub bump: u8,
}

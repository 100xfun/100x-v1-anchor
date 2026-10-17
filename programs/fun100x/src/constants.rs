// Order-related constants

// ==================== Borrow Duration Boundary Constants ====================
// Minimum borrow duration: 3 days (259200 seconds)
pub const MIN_BORROW_DURATION: u32 = 259_200;
// Maximum borrow duration: 6 months/180 days (15552000 seconds)
pub const MAX_BORROW_DURATION: u32 = 15_552_000;

// ==================== Fee Rate Boundary Constants ====================
// Minimum spot trading fee: 0.02% (20/100000)
pub const MIN_SWAP_FEE: u16 = 20;
// Minimum borrow fee: 0.024% (24/100000)
pub const MIN_BORROW_FEE: u16 = 24;

// Trade cooldown time in seconds, set to prevent sandwich bots or malicious liquidation attacks
pub const TRADE_COOLDOWN_SECONDS: i64 = 5; // seconds

// Minimum trade token amount to prevent trades that are too small
pub const MIN_TRADE_TOKEN_AMOUNT: u64 = 10_000_000; // 0.01 token (9 decimal places)

// Maximum SOL amount per single trade: 1000000 SOL
// Used to prevent: 1) excessive SOL input causing liquidity imbalance 2) price manipulation attacks 3) calculation overflow
pub const MAX_SINGLE_TRADE_SOL: u64 = 1_000_000_000_000_000; // 10000000 SOL (9 decimal places)

// Minimum equivalent SOL amount for long/short margin trading, margin trades cannot be too small and must also satisfy MIN_TRADE_TOKEN_AMOUNT
pub const MIN_MARGIN_SOL_AMOUNT: u64 = 2_000_000; // 0.002 sol

// Token amount difference check related constants
// Maximum allowed token amount difference: 20 for 6 decimals, 10000 for 9 decimals
pub const MAX_TOKEN_DIFFERENCE: u64 = 10000;

// Minimum stop loss percentage to avoid calculation issues from too small stop loss
pub const MIN_STOP_LOSS_PERCENT: u16 = 3; // 3%

// Fee retention probability denominator, determines trigger probability for fee reduction percentage
pub const FEE_RETENTION_PROBABILITY_DENOMINATOR: u64 = 20;

// Maximum number of insert indices when closing/opening positions
pub const MAX_CLOSE_INSERT_INDICES: usize = 41;

// Token Metadata field length limits (bytes)
pub const MAX_NAME_LENGTH: usize = 32;
pub const MAX_SYMBOL_LENGTH: usize = 10;
pub const MAX_URI_LENGTH: usize = 200;

// ==================== Price Milestone Constants ====================
// Fee halving milestones are computed DYNAMICALLY per pool:
//   threshold = CurveAMM::get_initial_price(initial_virtual_sol, initial_virtual_token)
//               * MILESTONE_MULTIPLIERS[fee_discount_flag]
// Both standard AND advanced pools participate in fee halving.
// (The old hardcoded PRICE_THRESHOLD_10X/100X/1000X constants were removed -
//  they were pre-computed against a single fixed initial price and could not
//  cover advanced pools with custom curve parameters.)

/// Milestone multipliers indexed by current fee_discount_flag:
/// flag 0 -> next milestone at 10x initial price
/// flag 1 -> next milestone at 100x initial price
/// flag 2 -> next milestone at 1000x initial price
pub const MILESTONE_MULTIPLIERS: [u128; 3] = [10, 100, 1_000];

/// Fee discount flag - initial state (no discount)
pub const FEE_DISCOUNT_FLAG_INITIAL: u8 = 0;

/// Fee discount flag - 10x price milestone reached (fees halved once)
pub const FEE_DISCOUNT_FLAG_10X: u8 = 1;

/// Fee discount flag - 100x price milestone reached (fees at 1/4)
pub const FEE_DISCOUNT_FLAG_100X: u8 = 2;

/// Fee discount flag - 1000x price milestone reached (fees at 1/8, terminal state)
pub const FEE_DISCOUNT_FLAG_1000X: u8 = 3;

// ==================== Custom Fee Constants ====================
/// Minimum custom fee rate: 1000 (1%)
pub const MIN_CUSTOM_FEE: u16 = 1_000;

/// Maximum custom fee rate: 5000 (5%)
pub const MAX_CUSTOM_FEE: u16 = 5_000;

/// Custom fee borrow fee multiplier: 1.2x (20% uplift)
pub const CUSTOM_FEE_BORROW_MULTIPLIER: u32 = 120; // 120/100 = 1.2

/// Denominator for borrow fee calculation
pub const BORROW_MULTIPLIER_DENOMINATOR: u32 = 100;

// ==================== Phase 2: Advanced Pool Constants ====================

// -------- Pool Type --------
/// Pool type: Standard
pub const POOL_TYPE_STANDARD: u8 = 0;
/// Pool type: Advanced
pub const POOL_TYPE_ADVANCED: u8 = 1;

// -------- Initial Virtual SOL Limits --------
/// Minimum initial virtual SOL (lamports, 9 decimal places): 3 SOL
pub const MIN_INITIAL_VIRTUAL_SOL: u64 = 3_000_000_000;
/// Maximum initial virtual SOL (lamports, 9 decimal places): 100,000 SOL
pub const MAX_INITIAL_VIRTUAL_SOL: u64 = 100_000_000_000_000;
/// Default initial virtual SOL (lamports, 9 decimal places): 30 SOL (used by standard version)
pub const DEFAULT_INITIAL_VIRTUAL_SOL: u64 = 30_000_000_000;

// -------- Initial Virtual Token Limits --------
/// Minimum initial virtual Token (smallest unit, 9 decimal places): 10 million Token
pub const MIN_INITIAL_VIRTUAL_TOKEN: u64 = 10_000_000_000_000_000;
/// Maximum initial virtual Token (smallest unit, 9 decimal places): 10 billion Token
pub const MAX_INITIAL_VIRTUAL_TOKEN: u64 = 10_000_000_000_000_000_000;
/// Default initial virtual Token (smallest unit, 9 decimal places): 1.03008 billion Token (used by standard version)
/// = STANDARD_TOTAL_SUPPLY - STANDARD_BORROW_TOKEN_RESERVE (borrow pool deducted from total supply)
pub const DEFAULT_INITIAL_VIRTUAL_TOKEN: u64 = 1_030_080_000_000_000_000;

// -------- Borrow Pool Token Ratio Limits --------
// Note: Borrow pool tokens are deducted from the total supply (not minted additionally),
// so on-chain total supply always equals the nominal supply.
/// Minimum borrow pool token ratio: 2%
pub const MIN_BORROW_POOL_RATIO: u8 = 2;
/// Maximum borrow pool token ratio: 8%
pub const MAX_BORROW_POOL_RATIO: u8 = 8;
/// Default borrow pool token ratio: 4% (used by standard version)
pub const DEFAULT_BORROW_POOL_RATIO: u8 = 4;

// -------- Borrow Duration Limits --------
/// Default borrow duration (seconds): 30 days (used by standard version)
pub const DEFAULT_BORROW_DURATION: u32 = 2_592_000; // 30 * 24 * 3600

// -------- Liquidity Pool Token Fixed Values (used by standard version) --------
// Borrow pool tokens are deducted from the total supply:
// STANDARD_TOTAL_SUPPLY = STANDARD_LP_TOKEN_RESERVE + STANDARD_BORROW_TOKEN_RESERVE
/// Standard version total supply (9 decimal places): 1.073 billion Token (on-chain mint total supply)
pub const STANDARD_TOTAL_SUPPLY: u64 = 1_073_000_000_000_000_000;
/// Standard version liquidity pool token amount (9 decimal places): 1.03008 billion Token (96%)
pub const STANDARD_LP_TOKEN_RESERVE: u64 = 1_030_080_000_000_000_000;
/// Standard version borrow pool token amount (9 decimal places): 42.92 million Token (4%)
pub const STANDARD_BORROW_TOKEN_RESERVE: u64 = 42_920_000_000_000_000;

// ==================== Pool SOL Account Safety ====================
/// Outflow floor for pool_sol_account is the LIVE rent-exempt minimum
/// (see utils::pool_sol_required_balance), never a hardcoded value, because
/// Solana rent changes over time (SIMD-0437 lowers lamports_per_byte in steps).
///
/// Extra lamports deposited at pool creation on top of the rent-exempt minimum.
/// This is a consumable rounding buffer, NOT part of the outflow floor, so the
/// last trader can always exit even with tiny rounding drift.
pub const POOL_SOL_INITIAL_ROUNDING_BUFFER: u64 = 10_000;

// ==================== Partial Close Adjustment ====================
/// Adjustment constant for partial close profit calculation
/// To prevent rounding errors during partial closes from allowing users to withdraw
/// slightly more funds than they should from the contract, a small amount is deducted
/// from the profit calculation on each partial close operation
pub const PARTIAL_CLOSE_ADJUSTMENT: u64 = 1;

// Deprecated code:

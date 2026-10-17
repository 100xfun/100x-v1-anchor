// Other Structures
use anchor_lang::prelude::*;

// ============================================================================
// OrderBook Account Data Structure
// ============================================================================

// OrderBook Header Structure
// Used for up_orderbook and down_orderbook
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct OrderBook {
    // Version number (for future upgrades, current version: 1) - 1 byte
    pub version: u8,

    // Order type (1=long/down, 2=short/up) - 1 byte
    pub order_type: u8,

    // PDA bump - 1 byte
    pub bump: u8,

    // Reserved field (align to 8-byte boundary) - 5 bytes
    pub _padding1: [u8; 5],

    // Protocol authority (has permission to expand/shrink) - 32 bytes
    pub authority: Pubkey,

    // Order ID counter (allocates unique order IDs, monotonically increasing) - 8 bytes
    pub order_id_counter: u64,

    // Orderbook creation timestamp (Unix timestamp, seconds) - 8 bytes
    pub created_at: i64,

    // Last modification timestamp (Unix timestamp, seconds) - 8 bytes
    pub last_modified: i64,

    // Total capacity (maximum number of slots) - 4 bytes
    pub total_capacity: u32,

    // Head index of linked list (first order) - 2 bytes
    pub head: u16,

    // Tail index of linked list (last order) - 2 bytes
    pub tail: u16,

    // Current total number of orders (also the index of next insertion) - 2 bytes
    pub total: u16,

    // Reserved field (align to 8-byte boundary) - 2 bytes
    pub _padding2: u16,

    // Reserved field (for future expansion, e.g. migration_pda) - 32 bytes
    pub reserved: [u8; 32],
    // Note: Dynamic order slot array adjusted via realloc
    // Actual storage: MarginOrder[total_capacity]
    // Accessed via memory offset, not declared here
}

impl OrderBook {
    // Current version number
    pub const CURRENT_VERSION: u8 = 1;

    // Header size: (1+1+1+5) + 32 + 8 + (8+8) + (4+4+4+4) + 32 = 112 bytes
    pub const HEADER_SIZE: usize = std::mem::size_of::<OrderBook>();

    // Maximum capacity (10MB / 192 bytes about 54,612, conservatively take 52,000)
    pub const MAX_CAPACITY: u32 = 52_000;

    // Calculate total account size
    pub fn account_size(capacity: u32) -> usize {
        8 + Self::HEADER_SIZE + (capacity as usize) * MarginOrder::SIZE
    }
}

// Margin order slot structure
// Use zero-copy and bytemuck for efficient memory operations
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MarginOrder {
    // ========== 32-byte aligned field (Pubkey) ==========
    // User who opened the position - 32 bytes
    pub user: Pubkey,

    // ========== 16-byte aligned field (u128) ==========
    // Locked liquidity pool interval start price (Q64.64 format) - 16 bytes
    pub lock_lp_start_price: u128,

    // Locked liquidity pool interval end price (Q64.64 format) - 16 bytes
    pub lock_lp_end_price: u128,

    // Opening price (Q64.64 format, set at opening, never changes) - 16 bytes
    pub open_price: u128,

    // ========== 8-byte aligned field (u64) ==========
    // Order unique identifier (globally incremented, allocated by OrderBook.order_id_counter) - 8 bytes
    pub order_id: u64,

    // Locked liquidity pool interval SOL amount (exact value, lamports) - 8 bytes
    pub lock_lp_sol_amount: u64,

    // Locked liquidity pool interval Token amount (exact value, minimum unit) - 8 bytes
    pub lock_lp_token_amount: u64,

    // To next node, liquidity pool interval SOL amount - 8 bytes
    pub next_lp_sol_amount: u64,

    // To next node, liquidity pool interval Token amount - 8 bytes
    pub next_lp_token_amount: u64,

    // Initial margin SOL amount (mainly for recording, not used in calculations) - 8 bytes
    pub margin_init_sol_amount: u64,

    // Margin SOL amount - 8 bytes
    pub margin_sol_amount: u64,

    // Loan amount: if long, SOL is borrowed; if short, Token is borrowed - 8 bytes
    pub borrow_amount: u64,

    // Current position asset amount (SOL for short, Token for long) - 8 bytes
    // Note: For long positions, this value is exactly equal to lock_lp_token_amount
    pub position_asset_amount: u64,

    // Realized SOL profit - 8 bytes
    pub realized_sol_amount: u64,

    // ========== 8-byte aligned field (i64 timestamp) ==========
    // Order start timestamp (Unix timestamp, seconds) - 8 bytes
    pub start_time: i64,

    // Loan expiration timestamp (Unix timestamp, seconds), can be closed by any user after expiration - 8 bytes
    pub end_time: i64,

    // ========== 4-byte aligned field (u32) ==========
    // Order version number (incremented on each update) - 4 bytes
    pub version: u32,

    // ========== 2-byte aligned field (u16) ==========
    // Slot index pointing to next order - 2 bytes
    pub next_order: u16,

    // Slot index pointing to previous order - 2 bytes
    pub prev_order: u16,

    // Margin trading fee (basis points, bps) - 2 bytes
    // Need to record because fee base may change (but cannot change for same order)
    // Example: 50 = 0.5%, 100 = 1%
    pub borrow_fee: u16,

    // ========== 1-byte aligned field (u8) ==========
    // Order type: 1=long(Down direction) 2=short(Up direction) - 1 byte
    pub order_type: u8,

    // Reserved field (align to struct 32-byte boundary, bytemuck::Pod requires no padding) - 5 bytes
    pub _padding: [u8; 5],
}

impl MarginOrder {
    // Slot size: automatically calculated using size_of
    pub const SIZE: usize = std::mem::size_of::<MarginOrder>();

    // // Check if order is expired (can be closed by any user)
    // pub fn is_expired(&self, current_timestamp: i64) -> bool {
    //     self.end_time > 0 && current_timestamp >= self.end_time
    // }

    // // Reset slot to free state
    // pub fn reset(&mut self) {
    //     *self = Self::zeroed();
    //     self.next_order = u16::MAX;
    //     self.prev_order = u16::MAX;
    // }

    // // Initialize as free slot
    // pub fn init_free() -> Self {
    //     let mut slot = Self::zeroed();
    //     slot.next_order = u16::MAX;
    //     slot.prev_order = u16::MAX;
    //     slot
    // }
}

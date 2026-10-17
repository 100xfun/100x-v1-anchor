#![allow(unused_imports, unused_variables, unused_mut, dead_code)]

use crate::error::ErrorCode;
use crate::instructions::structs::{MarginOrder, OrderBook};
use anchor_lang::prelude::*;

pub struct OrderBookManager;

impl OrderBookManager {
    #[allow(deprecated)] // TODO: Waiting for Anchor to provide correct signature for resize method
    fn resize_account<'a>(account: &AccountInfo<'a>, new_size: usize) -> Result<()> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    fn check_capacity_before_insert(current_total: u16, new_total: u16) -> Result<()> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    pub fn get_order(orderbook_data: &[u8], index: u16) -> Result<&MarginOrder> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    fn get_order_mut(orderbook_data: &mut [u8], index: u16) -> Result<&mut MarginOrder> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    pub fn load_orderbook_header(data: &[u8]) -> Result<&OrderBook> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    fn load_orderbook_header_mut(data: &mut [u8]) -> Result<&mut OrderBook> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    fn get_order_safe(data: &[u8], index: u16) -> Result<&MarginOrder> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    fn get_order_mut_safe(data: &mut [u8], index: u16) -> Result<&mut MarginOrder> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    pub fn insert_after<'a>(
        orderbook_account: &AccountInfo<'a>,
        after_index: u16,
        order_data: &MarginOrder,
        payer: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
    ) -> Result<(u16, u64)> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    pub fn insert_before<'a>(
        orderbook_account: &AccountInfo<'a>,
        before_index: u16,
        order_data: &MarginOrder,
        payer: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
    ) -> Result<(u16, u64)> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    pub fn traverse<F>(
        orderbook_data: &[u8],
        start: u16,
        limit: u32,
        mut callback: F,
    ) -> Result<TraversalResult>
    where
        F: FnMut(u16, &MarginOrder) -> Result<bool>,
    {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    pub fn update_order<'a>(
        orderbook_account: &AccountInfo<'a>,
        update_index: u16,
        order_id: u64,
        update_data: &MarginOrderUpdateData,
    ) -> Result<()> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    pub fn get_insert_neighbors(
        orderbook_data: &[u8],
        insert_pos: u16,
    ) -> Result<(Option<u16>, Option<u16>)> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    pub fn batch_remove_by_indices_unsafe<'a>(
        orderbook_account: &AccountInfo<'a>,
        indices: &[u16],
        payer: &AccountInfo<'a>,
        _system_program: &AccountInfo<'a>,
    ) -> Result<()> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    fn unlink_node_internal(
        data: &mut [u8],
        _remove_index: u16,
        removed_prev: u16,
        removed_next: u16,
        _old_head: u16,
        _old_tail: u16,
    ) -> Result<()> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    fn move_tail_to_index_internal(
        data: &mut [u8],
        tail_index: u16,
        target_index: u16,
    ) -> Result<()> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    fn batch_remove_all<'a>(
        orderbook_account: &AccountInfo<'a>,
        payer: &AccountInfo<'a>,
        old_total: u16,
    ) -> Result<()> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }

    fn final_resize_and_refund<'a>(
        orderbook_account: &AccountInfo<'a>,
        payer: &AccountInfo<'a>,
        old_total: u16,
        delete_count: u16,
    ) -> Result<()> {
        // Internal implementation is omitted from this public source.
        Err(crate::error::ErrorCode::CurveCalculationError.into())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TraversalResult {
    #[allow(dead_code)]
    pub processed: u32,

    #[allow(dead_code)]
    pub next: u16,

    #[allow(dead_code)]
    pub done: bool,
}

#[derive(Clone, Copy, Default)]
pub struct MarginOrderUpdateData {
    pub lock_lp_start_price: Option<u128>,
    pub lock_lp_end_price: Option<u128>,
    pub lock_lp_sol_amount: Option<u64>,
    pub lock_lp_token_amount: Option<u64>,
    pub next_lp_sol_amount: Option<u64>,
    pub next_lp_token_amount: Option<u64>,
    pub end_time: Option<i64>,
    pub margin_init_sol_amount: Option<u64>,
    pub margin_sol_amount: Option<u64>,
    pub borrow_amount: Option<u64>,
    pub position_asset_amount: Option<u64>,
    pub borrow_fee: Option<u16>,
    pub open_price: Option<u128>,
    pub realized_sol_amount: Option<u64>,
}

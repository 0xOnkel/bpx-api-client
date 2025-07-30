use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::margin::MarginFunction;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturePosition {
    pub break_even_price: Decimal,
    pub cumulative_funding_payment: Decimal,
    pub cumulative_interest: Decimal,
    pub entry_price: Decimal,
    pub est_liquidation_price: Decimal,
    pub imf: Decimal,
    pub imf_function: MarginFunction,
    pub mark_price: Decimal,
    pub mmf: Decimal,
    pub mmf_function: MarginFunction,
    pub net_cost: Decimal,
    pub net_exposure_notional: Decimal,
    pub net_exposure_quantity: Decimal,
    pub net_quantity: Decimal,
    pub pnl_realized: Decimal,
    pub pnl_unrealized: Decimal,
    pub position_id: String,
    pub subaccount_id: Option<u64>,
    pub symbol: String,
    pub user_id: u64,
}

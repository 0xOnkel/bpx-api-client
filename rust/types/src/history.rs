use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PnlHistory {
    pub pnl_realized: Decimal,
    pub symbol: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertDustHistory {
    pub id: u64,
    pub quantity: Decimal,
    pub symbol: String,
    pub usdc_received: Decimal,
    pub timestamp: u64,
}

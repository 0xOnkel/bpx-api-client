use bpx_api_types::history::PnlHistory;

use crate::{BpxClient, Result};

#[doc(hidden)]
pub const API_HISTORY_PNL: &str = "/wapi/v1/history/pnl";
pub const API_FUNDING_PAYMENTS_HISTORY: &str = "/wapi/v1/history/funding";

impl BpxClient {
    /// Fetches the account's settings.
    pub async fn get_pnl_history(&self) -> Result<Vec<PnlHistory>> {
        let url = format!("{}{}", self.base_url, API_HISTORY_PNL);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    pub async fn get_funding_payments_history(&self) -> Result<Vec<FundingPayment>> {
        let url = format!("{}{}", self.base_url, API_FUNDING_PAYMENTS_HISTORY);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}

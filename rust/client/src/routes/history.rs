use bpx_api_types::history::FundingPayment;

use crate::error::Result;
use crate::BpxClient;

#[doc(hidden)]
pub const API_FUNDING_PAYMENTS_HISTORY: &str = "/wapi/v1/history/funding";

impl BpxClient {
    pub async fn get_funding_payments_history(&self) -> Result<Vec<FundingPayment>> {
        let url = format!("{}{}", self.base_url, API_FUNDING_PAYMENTS_HISTORY);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}

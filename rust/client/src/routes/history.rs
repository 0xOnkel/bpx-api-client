use bpx_api_types::history::PnlHistory;

use crate::{BpxClient, Result};

#[doc(hidden)]
pub const API_HISTORY_PNL: &str = "/wapi/v1/history/pnl";

impl BpxClient {
    /// Fetches the account's settings.
    pub async fn get_pnl_history(&self) -> Result<Vec<PnlHistory>> {
        let url = format!("{}{}", self.base_url, API_HISTORY_PNL);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}

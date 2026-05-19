//! `iNumbering` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::inumbering::*;

/// Handle for the `iNumbering` resource — inventory, orders, port-ins.
#[derive(Debug)]
pub struct INumberingService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> INumberingService<'c> {
    /// `GET /v2.2/inventory` — search available TNs.
    pub async fn search_inventory(
        &self,
        q: &InventoryQuery,
    ) -> Result<InventorySearchData, ApiError> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(n) = q.npa {
            query.push(("npa", n.to_string()));
        }
        if let Some(n) = q.nxx {
            query.push(("nxx", n.to_string()));
        }
        if let Some(ref s) = q.state {
            query.push(("state", s.clone()));
        }
        if let Some(ref rc) = q.rate_center {
            query.push(("ratecenter", rc.clone()));
        }
        if let Some(ref c) = q.contains {
            query.push(("contains", c.clone()));
        }
        if let Some(ref e) = q.ends_with {
            query.push(("endswith", e.clone()));
        }
        if let Some(l) = q.limit {
            query.push(("limit", l.to_string()));
        }
        self.client
            .transport
            .request::<(), InventorySearchData>(Method::GET, "/v2.2/inventory", &query, None, true)
            .await
    }

    /// `GET /v2.2/inventory/coverage` — aggregated availability buckets.
    pub async fn coverage(&self, q: &CoverageQuery) -> Result<InventoryCoverageData, ApiError> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(ref s) = q.state {
            query.push(("state", s.clone()));
        }
        if let Some(ref rc) = q.rate_center {
            query.push(("ratecenter", rc.clone()));
        }
        self.client
            .transport
            .request::<(), InventoryCoverageData>(
                Method::GET,
                "/v2.2/inventory/coverage",
                &query,
                None,
                true,
            )
            .await
    }

    /// `POST /v2.2/orders` — purchase new TNs.
    pub async fn order(&self, body: &OrderCreateRequest) -> Result<OrderCreateData, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/orders", &[], Some(body), true)
            .await
    }

    /// `GET /v2.2/ports` — every port-in on the account.
    pub async fn ports(&self) -> Result<PortListData, ApiError> {
        self.client
            .transport
            .request::<(), PortListData>(Method::GET, "/v2.2/ports", &[], None, true)
            .await
    }

    /// `GET /v2.2/ports/{id}` — detail for one port-in.
    pub async fn port(&self, id: i64) -> Result<PortDetailData, ApiError> {
        let path = format!("/v2.2/ports/{}", id);
        self.client
            .transport
            .request::<(), PortDetailData>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `POST /v2.2/ports` — submit a port-in order.
    pub async fn submit_port(&self, body: &PortSubmitRequest) -> Result<PortSubmitData, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/ports", &[], Some(body), true)
            .await
    }

    /// `GET /v2.2/ports/availability/{number}` — check whether `number` is portable.
    pub async fn port_availability(&self, number: &str) -> Result<PortAvailabilityData, ApiError> {
        let path = format!("/v2.2/ports/availability/{}", number);
        self.client
            .transport
            .request::<(), PortAvailabilityData>(Method::GET, &path, &[], None, true)
            .await
    }
}

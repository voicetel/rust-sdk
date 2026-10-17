//! `iNumbering` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::inumbering::*;
use crate::transport::encode_path_segment;

/// Handle for the `iNumbering` resource — inventory, orders, port-ins.
#[derive(Debug)]
pub struct INumberingService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> INumberingService<'c> {
    /// `GET /v2.2/inventory` — search available TNs. Results are cached
    /// server-side for 10 minutes.
    pub async fn search_inventory(
        &self,
        q: &InventoryQuery,
    ) -> Result<InventorySearchData, ApiError> {
        let mut query: Vec<(&str, String)> = vec![("tnMask", q.tn_mask.clone())];
        if let Some(n) = q.quantity {
            query.push(("quantity", n.to_string()));
        }
        if let Some(ref s) = q.province {
            query.push(("province", s.clone()));
        }
        if let Some(ref rc) = q.rate_center {
            query.push(("rateCenter", rc.clone()));
        }
        if let Some(ref l) = q.lata {
            query.push(("lata", l.to_string()));
        }
        self.client
            .transport
            .request::<(), InventorySearchData>(Method::GET, "/v2.2/inventory", &query, None, true)
            .await
    }

    /// `GET /v2.2/inventory/coverage` — aggregated availability buckets.
    /// Results are cached server-side for 10 minutes.
    pub async fn coverage(&self, q: &CoverageQuery) -> Result<InventoryCoverageData, ApiError> {
        let mut query: Vec<(&str, String)> = vec![("countBy", q.count_by.as_str().to_string())];
        if let Some(ref s) = q.loc_state {
            query.push(("locState", s.clone()));
        }
        if let Some(ref c) = q.city {
            query.push(("city", c.clone()));
        }
        if let Some(ref rc) = q.rc_abbre {
            query.push(("rcAbbre", rc.clone()));
        }
        if let Some(ref n) = q.npa {
            query.push(("npa", n.clone()));
        }
        if let Some(ref l) = q.lata {
            query.push(("lata", l.to_string()));
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

    /// `GET /v2.2/ports/{id}` — detail for one port-in. `id` is the
    /// 5-character alphanumeric pid from the list response's `pid` field.
    pub async fn port(&self, id: &str) -> Result<PortDetailData, ApiError> {
        let path = format!("/v2.2/ports/{}", encode_path_segment(id));
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
        let path = format!("/v2.2/ports/availability/{}", encode_path_segment(number));
        self.client
            .transport
            .request::<(), PortAvailabilityData>(Method::GET, &path, &[], None, true)
            .await
    }
}

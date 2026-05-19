//! `Numbers` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::numbers::*;

/// Handle for the `Numbers` resource — the largest surface in the SDK.
#[derive(Debug)]
pub struct NumbersService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> NumbersService<'c> {
    /// `GET /v2.2/numbers` — every TN on the account.
    pub async fn list(&self) -> Result<NumbersListData, ApiError> {
        self.client
            .transport
            .request::<(), NumbersListData>(Method::GET, "/v2.2/numbers", &[], None, true)
            .await
    }

    /// `POST /v2.2/numbers` — attach a TN to the account.
    pub async fn add(&self, body: &NumberAddRequest) -> Result<NumberAddData, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/numbers", &[], Some(body), true)
            .await
    }

    /// `GET /v2.2/numbers/{number}` — fetch one TN.
    pub async fn get(&self, number: &str) -> Result<NumberDetail, ApiError> {
        let path = format!("/v2.2/numbers/{}", number);
        self.client
            .transport
            .request::<(), NumberDetail>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `DELETE /v2.2/numbers/{number}` — detach a TN. Returns `()` on 204.
    pub async fn remove(&self, number: &str) -> Result<(), ApiError> {
        let path = format!("/v2.2/numbers/{}", number);
        self.client
            .transport
            .request_no_content::<()>(Method::DELETE, &path, &[], None, true)
            .await
    }

    /// `PATCH /v2.2/numbers/{number}` — transfer a TN to another account.
    pub async fn move_to(
        &self,
        number: &str,
        body: &NumberMoveRequest,
    ) -> Result<NumberMoveData, ApiError> {
        let path = format!("/v2.2/numbers/{}", number);
        self.client
            .transport
            .request(Method::PATCH, &path, &[], Some(body), true)
            .await
    }

    /// `POST /v2.2/numbers/{number}/release` — return a TN to the network. 204.
    pub async fn release(&self, number: &str) -> Result<(), ApiError> {
        let path = format!("/v2.2/numbers/{}/release", number);
        self.client
            .transport
            .request_no_content::<()>(Method::POST, &path, &[], None, true)
            .await
    }

    /// `PUT /v2.2/numbers/{number}/route` — update outbound route.
    pub async fn set_route(
        &self,
        number: &str,
        body: &NumberRouteRequest,
    ) -> Result<NumberRouteData, ApiError> {
        let path = format!("/v2.2/numbers/{}/route", number);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `PUT /v2.2/numbers/{number}/translation` — update DNIS translation.
    pub async fn set_translation(
        &self,
        number: &str,
        body: &NumberTranslationRequest,
    ) -> Result<NumberTranslationData, ApiError> {
        let path = format!("/v2.2/numbers/{}/translation", number);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `PUT /v2.2/numbers/{number}/cnam` — toggle inbound CNAM.
    pub async fn set_cnam(
        &self,
        number: &str,
        body: &NumberCnamRequest,
    ) -> Result<NumberCnamData, ApiError> {
        let path = format!("/v2.2/numbers/{}/cnam", number);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `PUT /v2.2/numbers/{number}/lidb` — update outbound caller name (LIDB).
    pub async fn set_lidb(
        &self,
        number: &str,
        body: &NumberLidbRequest,
    ) -> Result<NumberLidbData, ApiError> {
        let path = format!("/v2.2/numbers/{}/lidb", number);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `GET /v2.2/numbers/{number}/fax` — read fax-to-email routing.
    pub async fn get_fax(&self, number: &str) -> Result<NumberFaxData, ApiError> {
        let path = format!("/v2.2/numbers/{}/fax", number);
        self.client
            .transport
            .request::<(), NumberFaxData>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `PUT /v2.2/numbers/{number}/fax` — enable fax-to-email.
    pub async fn set_fax(
        &self,
        number: &str,
        body: &NumberFaxRequest,
    ) -> Result<NumberFaxData, ApiError> {
        let path = format!("/v2.2/numbers/{}/fax", number);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `DELETE /v2.2/numbers/{number}/fax` — disable fax-to-email. 204.
    pub async fn remove_fax(&self, number: &str) -> Result<(), ApiError> {
        let path = format!("/v2.2/numbers/{}/fax", number);
        self.client
            .transport
            .request_no_content::<()>(Method::DELETE, &path, &[], None, true)
            .await
    }

    /// `PUT /v2.2/numbers/{number}/forward` — enable call forwarding.
    pub async fn set_forward(
        &self,
        number: &str,
        body: &NumberForwardRequest,
    ) -> Result<NumberForwardData, ApiError> {
        let path = format!("/v2.2/numbers/{}/forward", number);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `DELETE /v2.2/numbers/{number}/forward` — disable call forwarding. 204.
    pub async fn remove_forward(&self, number: &str) -> Result<(), ApiError> {
        let path = format!("/v2.2/numbers/{}/forward", number);
        self.client
            .transport
            .request_no_content::<()>(Method::DELETE, &path, &[], None, true)
            .await
    }

    /// `GET /v2.2/numbers/{number}/sms` — read SMS routing.
    pub async fn get_sms(&self, number: &str) -> Result<NumberSmsData, ApiError> {
        let path = format!("/v2.2/numbers/{}/sms", number);
        self.client
            .transport
            .request::<(), NumberSmsData>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `PUT /v2.2/numbers/{number}/sms` — configure SMS routing.
    pub async fn set_sms(
        &self,
        number: &str,
        body: &NumberSmsRequest,
    ) -> Result<NumberSmsData, ApiError> {
        let path = format!("/v2.2/numbers/{}/sms", number);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `DELETE /v2.2/numbers/{number}/sms` — clear SMS routing. 204.
    pub async fn remove_sms(&self, number: &str) -> Result<(), ApiError> {
        let path = format!("/v2.2/numbers/{}/sms", number);
        self.client
            .transport
            .request_no_content::<()>(Method::DELETE, &path, &[], None, true)
            .await
    }

    /// `GET /v2.2/numbers/{number}/messaging` — messaging state for one TN.
    pub async fn get_messaging(&self, number: &str) -> Result<NumberMessagingState, ApiError> {
        let path = format!("/v2.2/numbers/{}/messaging", number);
        self.client
            .transport
            .request::<(), NumberMessagingState>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `PATCH /v2.2/numbers/{number}/messaging` — update inbound/outbound routing.
    pub async fn patch_messaging(
        &self,
        number: &str,
        body: &NumberMessagingPatchRequest,
    ) -> Result<NumberMessagingPatchData, ApiError> {
        let path = format!("/v2.2/numbers/{}/messaging", number);
        self.client
            .transport
            .request(Method::PATCH, &path, &[], Some(body), true)
            .await
    }

    /// `PUT /v2.2/numbers/{number}/messaging-campaign` — bind a 10DLC campaign.
    pub async fn assign_campaign(
        &self,
        number: &str,
        body: &NumberCampaignAssignRequest,
    ) -> Result<NumberMessagingCampaignAssignData, ApiError> {
        let path = format!("/v2.2/numbers/{}/messaging-campaign", number);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `DELETE /v2.2/numbers/{number}/messaging-campaign` — unbind a campaign.
    /// Returns 200 with a body.
    pub async fn unassign_campaign(
        &self,
        number: &str,
    ) -> Result<NumberMessagingCampaignUnassignData, ApiError> {
        let path = format!("/v2.2/numbers/{}/messaging-campaign", number);
        self.client
            .transport
            .request::<(), NumberMessagingCampaignUnassignData>(
                Method::DELETE,
                &path,
                &[],
                None,
                true,
            )
            .await
    }

    /// `DELETE /v2.2/numbers/messaging-campaign` — bulk unbind. Returns 200 with a body.
    pub async fn bulk_unassign_campaign(
        &self,
        numbers: Vec<String>,
    ) -> Result<NumbersMessagingCampaignUnassignData, ApiError> {
        let body = BulkUnassignRequest { numbers };
        self.client
            .transport
            .request(
                Method::DELETE,
                "/v2.2/numbers/messaging-campaign",
                &[],
                Some(&body),
                true,
            )
            .await
    }

    /// `PATCH /v2.2/numbers/{number}/port-out-pin` — set the port-out PIN.
    pub async fn set_port_out_pin(
        &self,
        number: &str,
        body: &PortOutPinUpdateRequest,
    ) -> Result<PortOutPinUpdateData, ApiError> {
        let path = format!("/v2.2/numbers/{}/port-out-pin", number);
        self.client
            .transport
            .request(Method::PATCH, &path, &[], Some(body), true)
            .await
    }
}

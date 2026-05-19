//! `Support` resource.

use reqwest::Method;

use crate::client::Client;
use crate::error::ApiError;
use crate::models::support::*;

/// Handle for the `Support` resource — support tickets and replies.
#[derive(Debug)]
pub struct SupportService<'c> {
    pub(crate) client: &'c Client,
}

impl<'c> SupportService<'c> {
    /// `GET /v2.2/support/tickets` — every ticket on the account.
    pub async fn list(&self) -> Result<TicketsListData, ApiError> {
        self.client
            .transport
            .request::<(), TicketsListData>(Method::GET, "/v2.2/support/tickets", &[], None, true)
            .await
    }

    /// `POST /v2.2/support/tickets` — open a new ticket.
    pub async fn create(&self, body: &TicketCreateRequest) -> Result<TicketData, ApiError> {
        self.client
            .transport
            .request(Method::POST, "/v2.2/support/tickets", &[], Some(body), true)
            .await
    }

    /// `GET /v2.2/support/tickets/{id}` — fetch one ticket.
    pub async fn get(&self, id: i64) -> Result<TicketData, ApiError> {
        let path = format!("/v2.2/support/tickets/{}", id);
        self.client
            .transport
            .request::<(), TicketData>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `PUT /v2.2/support/tickets/{id}` — change a ticket's status.
    pub async fn update(
        &self,
        id: i64,
        body: &TicketUpdateRequest,
    ) -> Result<TicketUpdateData, ApiError> {
        let path = format!("/v2.2/support/tickets/{}", id);
        self.client
            .transport
            .request(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// `DELETE /v2.2/support/tickets/{id}` — admin-only deletion. 204.
    pub async fn delete(&self, id: i64) -> Result<(), ApiError> {
        let path = format!("/v2.2/support/tickets/{}", id);
        self.client
            .transport
            .request_no_content::<()>(Method::DELETE, &path, &[], None, true)
            .await
    }

    /// `GET /v2.2/support/tickets/{id}/messages` — every thread on a ticket.
    pub async fn messages(&self, id: i64) -> Result<TicketThreadsData, ApiError> {
        let path = format!("/v2.2/support/tickets/{}/messages", id);
        self.client
            .transport
            .request::<(), TicketThreadsData>(Method::GET, &path, &[], None, true)
            .await
    }

    /// `POST /v2.2/support/tickets/{id}/replies` — add a reply to a ticket.
    pub async fn reply(
        &self,
        id: i64,
        body: &TicketReplyRequest,
    ) -> Result<TicketReplyData, ApiError> {
        let path = format!("/v2.2/support/tickets/{}/replies", id);
        self.client
            .transport
            .request(Method::POST, &path, &[], Some(body), true)
            .await
    }
}

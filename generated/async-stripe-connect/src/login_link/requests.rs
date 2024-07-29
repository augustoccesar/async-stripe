use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateAccountLoginLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CreateAccountLoginLinkBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Creates a single-use login link for a connected account to access the Express Dashboard.
///
/// **You can only create login links for accounts that use the <a href="/connect/express-dashboard">Express Dashboard</a> and are connected to your platform**.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountLoginLink<'a> {
    inner: CreateAccountLoginLinkBuilder<'a>,
    account: &'a stripe_shared::AccountId,
}
impl<'a> CreateAccountLoginLink<'a> {
    /// Construct a new `CreateAccountLoginLink`.
    pub fn new(account: &'a stripe_shared::AccountId) -> Self {
        Self { account, inner: CreateAccountLoginLinkBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CreateAccountLoginLink<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateAccountLoginLink<'_> {
    type Output = stripe_connect::LoginLink;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(StripeMethod::Post, format!("/accounts/{account}/login_links"))
            .form(&self.inner)
    }
}
use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Permanently deletes a customer.
/// It cannot be undone.
/// Also immediately cancels any active subscriptions on the customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteCustomer<'a> {
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> DeleteCustomer<'a> {
    /// Construct a new `DeleteCustomer`.
    pub fn new(customer: &'a stripe_shared::CustomerId) -> Self {
        Self { customer }
    }
}
impl DeleteCustomer<'_> {
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

impl StripeRequest for DeleteCustomer<'_> {
    type Output = stripe_shared::DeletedCustomer;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(StripeMethod::Delete, format!("/customers/{customer}"))
    }
}
/// Removes the currently applied discount on a customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteDiscountCustomer<'a> {
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> DeleteDiscountCustomer<'a> {
    /// Construct a new `DeleteDiscountCustomer`.
    pub fn new(customer: &'a stripe_shared::CustomerId) -> Self {
        Self { customer }
    }
}
impl DeleteDiscountCustomer<'_> {
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

impl StripeRequest for DeleteDiscountCustomer<'_> {
    type Output = stripe_shared::DeletedDiscount;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(StripeMethod::Delete, format!("/customers/{customer}/discount"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListCustomerBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_clock: Option<&'a str>,
}
impl<'a> ListCustomerBuilder<'a> {
    fn new() -> Self {
        Self {
            created: None,
            email: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            test_clock: None,
        }
    }
}
/// Returns a list of your customers.
/// The customers are returned sorted by creation date, with the most recent customers appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCustomer<'a> {
    inner: ListCustomerBuilder<'a>,
}
impl<'a> ListCustomer<'a> {
    /// Construct a new `ListCustomer`.
    pub fn new() -> Self {
        Self { inner: ListCustomerBuilder::new() }
    }
    /// Only return customers that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// A case-sensitive filter on the list based on the customer's `email` field.
    /// The value must be a string.
    pub fn email(mut self, email: &'a str) -> Self {
        self.inner.email = Some(email);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Provides a list of customers that are associated with the specified test clock.
    /// The response will not include customers with test clocks if this parameter is not set.
    pub fn test_clock(mut self, test_clock: &'a str) -> Self {
        self.inner.test_clock = Some(test_clock);
        self
    }
}
impl<'a> Default for ListCustomer<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListCustomer<'_> {
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

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Customer>> {
        stripe_client_core::ListPaginator::new_list("/customers", self.inner)
    }
}

impl StripeRequest for ListCustomer<'_> {
    type Output = stripe_types::List<stripe_shared::Customer>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/customers").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveCustomerBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCustomerBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Customer object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCustomer<'a> {
    inner: RetrieveCustomerBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> RetrieveCustomer<'a> {
    /// Construct a new `RetrieveCustomer`.
    pub fn new(customer: &'a stripe_shared::CustomerId) -> Self {
        Self { customer, inner: RetrieveCustomerBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveCustomer<'_> {
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

impl StripeRequest for RetrieveCustomer<'_> {
    type Output = RetrieveCustomerReturned;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(StripeMethod::Get, format!("/customers/{customer}")).query(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum RetrieveCustomerReturned {
    Customer(stripe_shared::Customer),
    DeletedCustomer(stripe_shared::DeletedCustomer),
}

#[derive(Default)]
pub struct RetrieveCustomerReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<RetrieveCustomerReturned>,
        builder: RetrieveCustomerReturnedBuilder,
    }

    impl Deserialize for RetrieveCustomerReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<RetrieveCustomerReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for RetrieveCustomerReturnedBuilder {
        type Out = RetrieveCustomerReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                RetrieveCustomerReturned::DeletedCustomer(FromValueOpt::from_value(Value::Object(
                    o,
                ))?)
            } else {
                RetrieveCustomerReturned::Customer(FromValueOpt::from_value(Value::Object(o))?)
            })
        }
    }

    impl stripe_types::ObjectDeser for RetrieveCustomerReturned {
        type Builder = RetrieveCustomerReturnedBuilder;
    }
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct BalanceTransactionsCustomerBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> BalanceTransactionsCustomerBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of transactions that updated the customer’s [balances](https://stripe.com/docs/billing/customer/balance).
#[derive(Clone, Debug, serde::Serialize)]
pub struct BalanceTransactionsCustomer<'a> {
    inner: BalanceTransactionsCustomerBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> BalanceTransactionsCustomer<'a> {
    /// Construct a new `BalanceTransactionsCustomer`.
    pub fn new(customer: &'a stripe_shared::CustomerId) -> Self {
        Self { customer, inner: BalanceTransactionsCustomerBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl BalanceTransactionsCustomer<'_> {
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

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_shared::CustomerBalanceTransaction>,
    > {
        let customer = self.customer;

        stripe_client_core::ListPaginator::new_list(
            format!("/customers/{customer}/balance_transactions"),
            self.inner,
        )
    }
}

impl StripeRequest for BalanceTransactionsCustomer<'_> {
    type Output = stripe_types::List<stripe_shared::CustomerBalanceTransaction>;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/customers/{customer}/balance_transactions"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListPaymentMethodsCustomerBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_redisplay: Option<ListPaymentMethodsCustomerAllowRedisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<ListPaymentMethodsCustomerType>,
}
impl<'a> ListPaymentMethodsCustomerBuilder<'a> {
    fn new() -> Self {
        Self {
            allow_redisplay: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
        }
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListPaymentMethodsCustomerAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl ListPaymentMethodsCustomerAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use ListPaymentMethodsCustomerAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for ListPaymentMethodsCustomerAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListPaymentMethodsCustomerAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ListPaymentMethodsCustomerAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListPaymentMethodsCustomerAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListPaymentMethodsCustomerAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListPaymentMethodsCustomerAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListPaymentMethodsCustomerAllowRedisplay")
        })
    }
}
/// An optional filter on the list, based on the object `type` field.
/// Without the filter, the list includes all current and future payment method types.
/// If your integration expects only one type of payment method in the response, make sure to provide a type value in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListPaymentMethodsCustomerType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Mobilepay,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    Swish,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl ListPaymentMethodsCustomerType {
    pub fn as_str(self) -> &'static str {
        use ListPaymentMethodsCustomerType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Klarna => "klarna",
            Konbini => "konbini",
            Link => "link",
            Mobilepay => "mobilepay",
            Oxxo => "oxxo",
            P24 => "p24",
            Paynow => "paynow",
            Paypal => "paypal",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ListPaymentMethodsCustomerType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListPaymentMethodsCustomerType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "mobilepay" => Ok(Mobilepay),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for ListPaymentMethodsCustomerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListPaymentMethodsCustomerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListPaymentMethodsCustomerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListPaymentMethodsCustomerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Returns a list of PaymentMethods for a given Customer
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPaymentMethodsCustomer<'a> {
    inner: ListPaymentMethodsCustomerBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> ListPaymentMethodsCustomer<'a> {
    /// Construct a new `ListPaymentMethodsCustomer`.
    pub fn new(customer: &'a stripe_shared::CustomerId) -> Self {
        Self { customer, inner: ListPaymentMethodsCustomerBuilder::new() }
    }
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    pub fn allow_redisplay(
        mut self,
        allow_redisplay: ListPaymentMethodsCustomerAllowRedisplay,
    ) -> Self {
        self.inner.allow_redisplay = Some(allow_redisplay);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// An optional filter on the list, based on the object `type` field.
    /// Without the filter, the list includes all current and future payment method types.
    /// If your integration expects only one type of payment method in the response, make sure to provide a type value in the request.
    pub fn type_(mut self, type_: ListPaymentMethodsCustomerType) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
}
impl ListPaymentMethodsCustomer<'_> {
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

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::PaymentMethod>> {
        let customer = self.customer;

        stripe_client_core::ListPaginator::new_list(
            format!("/customers/{customer}/payment_methods"),
            self.inner,
        )
    }
}

impl StripeRequest for ListPaymentMethodsCustomer<'_> {
    type Output = stripe_types::List<stripe_shared::PaymentMethod>;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(StripeMethod::Get, format!("/customers/{customer}/payment_methods"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrievePaymentMethodCustomerBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentMethodCustomerBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a PaymentMethod object for a given Customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePaymentMethodCustomer<'a> {
    inner: RetrievePaymentMethodCustomerBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
    payment_method: &'a str,
}
impl<'a> RetrievePaymentMethodCustomer<'a> {
    /// Construct a new `RetrievePaymentMethodCustomer`.
    pub fn new(customer: &'a stripe_shared::CustomerId, payment_method: &'a str) -> Self {
        Self { customer, payment_method, inner: RetrievePaymentMethodCustomerBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrievePaymentMethodCustomer<'_> {
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

impl StripeRequest for RetrievePaymentMethodCustomer<'_> {
    type Output = stripe_shared::PaymentMethod;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        let payment_method = self.payment_method;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/customers/{customer}/payment_methods/{payment_method}"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct SearchCustomerBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<&'a str>,
    query: &'a str,
}
impl<'a> SearchCustomerBuilder<'a> {
    fn new(query: &'a str) -> Self {
        Self { expand: None, limit: None, page: None, query }
    }
}
/// Search for customers you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
/// Under normal operating.
/// conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up.
/// to an hour behind during outages. Search functionality is not available to merchants in India.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SearchCustomer<'a> {
    inner: SearchCustomerBuilder<'a>,
}
impl<'a> SearchCustomer<'a> {
    /// Construct a new `SearchCustomer`.
    pub fn new(query: &'a str) -> Self {
        Self { inner: SearchCustomerBuilder::new(query) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for pagination across multiple pages of results.
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    pub fn page(mut self, page: &'a str) -> Self {
        self.inner.page = Some(page);
        self
    }
}
impl SearchCustomer<'_> {
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

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::SearchList<stripe_shared::Customer>> {
        stripe_client_core::ListPaginator::new_search_list("/customers/search", self.inner)
    }
}

impl StripeRequest for SearchCustomer<'_> {
    type Output = stripe_types::SearchList<stripe_shared::Customer>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/customers/search").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateCustomerBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<OptionalFieldsAddress<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cash_balance: Option<CreateCustomerCashBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_prefix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_settings: Option<CreateCustomerInvoiceSettings<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_invoice_sequence: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locales: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promotion_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<CustomerShipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax: Option<CreateCustomerTax<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_exempt: Option<stripe_shared::CustomerTaxExempt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_id_data: Option<&'a [CreateCustomerTaxIdData<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_clock: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<bool>,
}
impl<'a> CreateCustomerBuilder<'a> {
    fn new() -> Self {
        Self {
            address: None,
            balance: None,
            cash_balance: None,
            coupon: None,
            description: None,
            email: None,
            expand: None,
            invoice_prefix: None,
            invoice_settings: None,
            metadata: None,
            name: None,
            next_invoice_sequence: None,
            payment_method: None,
            phone: None,
            preferred_locales: None,
            promotion_code: None,
            shipping: None,
            source: None,
            tax: None,
            tax_exempt: None,
            tax_id_data: None,
            test_clock: None,
            validate: None,
        }
    }
}
/// Balance information and default balance settings for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateCustomerCashBalanceSettings>,
}
impl CreateCustomerCashBalance {
    pub fn new() -> Self {
        Self { settings: None }
    }
}
impl Default for CreateCustomerCashBalance {
    fn default() -> Self {
        Self::new()
    }
}
/// Settings controlling the behavior of the customer's cash balance,
/// such as reconciliation of funds received.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    /// Valid options are `automatic`, `manual`, or `merchant_default`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<CreateCustomerCashBalanceSettingsReconciliationMode>,
}
impl CreateCustomerCashBalanceSettings {
    pub fn new() -> Self {
        Self { reconciliation_mode: None }
    }
}
impl Default for CreateCustomerCashBalanceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls how funds transferred by the customer are applied to payment intents and invoices.
/// Valid options are `automatic`, `manual`, or `merchant_default`.
/// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
    MerchantDefault,
}
impl CreateCustomerCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerCashBalanceSettingsReconciliationMode::*;
        match self {
            Automatic => "automatic",
            Manual => "manual",
            MerchantDefault => "merchant_default",
        }
    }
}

impl std::str::FromStr for CreateCustomerCashBalanceSettingsReconciliationMode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerCashBalanceSettingsReconciliationMode::*;
        match s {
            "automatic" => Ok(Automatic),
            "manual" => Ok(Manual),
            "merchant_default" => Ok(MerchantDefault),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCustomerCashBalanceSettingsReconciliationMode",
            )
        })
    }
}
/// Default invoice settings for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerInvoiceSettings<'a> {
    /// The list of up to 4 default custom fields to be displayed on invoices for this customer.
    /// When updating, pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CustomFieldParams<'a>]>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateCustomerInvoiceSettingsRenderingOptions>,
}
impl<'a> CreateCustomerInvoiceSettings<'a> {
    pub fn new() -> Self {
        Self {
            custom_fields: None,
            default_payment_method: None,
            footer: None,
            rendering_options: None,
        }
    }
}
impl<'a> Default for CreateCustomerInvoiceSettings<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerInvoiceSettingsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
}
impl CreateCustomerInvoiceSettingsRenderingOptions {
    pub fn new() -> Self {
        Self { amount_tax_display: None }
    }
}
impl Default for CreateCustomerInvoiceSettingsRenderingOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}
impl CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay",
            )
        })
    }
}
/// Tax details about the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerTax<'a> {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// A flag that indicates when Stripe should validate the customer tax location.
    /// Defaults to `deferred`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_location: Option<CreateCustomerTaxValidateLocation>,
}
impl<'a> CreateCustomerTax<'a> {
    pub fn new() -> Self {
        Self { ip_address: None, validate_location: None }
    }
}
impl<'a> Default for CreateCustomerTax<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// A flag that indicates when Stripe should validate the customer tax location.
/// Defaults to `deferred`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerTaxValidateLocation {
    Deferred,
    Immediately,
}
impl CreateCustomerTaxValidateLocation {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerTaxValidateLocation::*;
        match self {
            Deferred => "deferred",
            Immediately => "immediately",
        }
    }
}

impl std::str::FromStr for CreateCustomerTaxValidateLocation {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerTaxValidateLocation::*;
        match s {
            "deferred" => Ok(Deferred),
            "immediately" => Ok(Immediately),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCustomerTaxValidateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerTaxValidateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerTaxValidateLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCustomerTaxValidateLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateCustomerTaxValidateLocation")
        })
    }
}
/// The customer's tax IDs.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerTaxIdData<'a> {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: CreateCustomerTaxIdDataType,
    /// Value of the tax ID.
    pub value: &'a str,
}
impl<'a> CreateCustomerTaxIdData<'a> {
    pub fn new(type_: CreateCustomerTaxIdDataType, value: &'a str) -> Self {
        Self { type_, value }
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bh_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `kz_bin`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `ng_tin`, `no_vat`, `no_voec`, `nz_gst`, `om_vat`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateCustomerTaxIdDataType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BhVat,
    BoTin,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    KzBin,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NgTin,
    NoVat,
    NoVoec,
    NzGst,
    OmVat,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateCustomerTaxIdDataType {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerTaxIdDataType::*;
        match self {
            AdNrt => "ad_nrt",
            AeTrn => "ae_trn",
            ArCuit => "ar_cuit",
            AuAbn => "au_abn",
            AuArn => "au_arn",
            BgUic => "bg_uic",
            BhVat => "bh_vat",
            BoTin => "bo_tin",
            BrCnpj => "br_cnpj",
            BrCpf => "br_cpf",
            CaBn => "ca_bn",
            CaGstHst => "ca_gst_hst",
            CaPstBc => "ca_pst_bc",
            CaPstMb => "ca_pst_mb",
            CaPstSk => "ca_pst_sk",
            CaQst => "ca_qst",
            ChVat => "ch_vat",
            ClTin => "cl_tin",
            CnTin => "cn_tin",
            CoNit => "co_nit",
            CrTin => "cr_tin",
            DoRcn => "do_rcn",
            EcRuc => "ec_ruc",
            EgTin => "eg_tin",
            EsCif => "es_cif",
            EuOssVat => "eu_oss_vat",
            EuVat => "eu_vat",
            GbVat => "gb_vat",
            GeVat => "ge_vat",
            HkBr => "hk_br",
            HuTin => "hu_tin",
            IdNpwp => "id_npwp",
            IlVat => "il_vat",
            InGst => "in_gst",
            IsVat => "is_vat",
            JpCn => "jp_cn",
            JpRn => "jp_rn",
            JpTrn => "jp_trn",
            KePin => "ke_pin",
            KrBrn => "kr_brn",
            KzBin => "kz_bin",
            LiUid => "li_uid",
            MxRfc => "mx_rfc",
            MyFrp => "my_frp",
            MyItn => "my_itn",
            MySst => "my_sst",
            NgTin => "ng_tin",
            NoVat => "no_vat",
            NoVoec => "no_voec",
            NzGst => "nz_gst",
            OmVat => "om_vat",
            PeRuc => "pe_ruc",
            PhTin => "ph_tin",
            RoTin => "ro_tin",
            RsPib => "rs_pib",
            RuInn => "ru_inn",
            RuKpp => "ru_kpp",
            SaVat => "sa_vat",
            SgGst => "sg_gst",
            SgUen => "sg_uen",
            SiTin => "si_tin",
            SvNit => "sv_nit",
            ThVat => "th_vat",
            TrTin => "tr_tin",
            TwVat => "tw_vat",
            UaVat => "ua_vat",
            UsEin => "us_ein",
            UyRuc => "uy_ruc",
            VeRif => "ve_rif",
            VnTin => "vn_tin",
            ZaVat => "za_vat",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateCustomerTaxIdDataType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerTaxIdDataType::*;
        match s {
            "ad_nrt" => Ok(AdNrt),
            "ae_trn" => Ok(AeTrn),
            "ar_cuit" => Ok(ArCuit),
            "au_abn" => Ok(AuAbn),
            "au_arn" => Ok(AuArn),
            "bg_uic" => Ok(BgUic),
            "bh_vat" => Ok(BhVat),
            "bo_tin" => Ok(BoTin),
            "br_cnpj" => Ok(BrCnpj),
            "br_cpf" => Ok(BrCpf),
            "ca_bn" => Ok(CaBn),
            "ca_gst_hst" => Ok(CaGstHst),
            "ca_pst_bc" => Ok(CaPstBc),
            "ca_pst_mb" => Ok(CaPstMb),
            "ca_pst_sk" => Ok(CaPstSk),
            "ca_qst" => Ok(CaQst),
            "ch_vat" => Ok(ChVat),
            "cl_tin" => Ok(ClTin),
            "cn_tin" => Ok(CnTin),
            "co_nit" => Ok(CoNit),
            "cr_tin" => Ok(CrTin),
            "do_rcn" => Ok(DoRcn),
            "ec_ruc" => Ok(EcRuc),
            "eg_tin" => Ok(EgTin),
            "es_cif" => Ok(EsCif),
            "eu_oss_vat" => Ok(EuOssVat),
            "eu_vat" => Ok(EuVat),
            "gb_vat" => Ok(GbVat),
            "ge_vat" => Ok(GeVat),
            "hk_br" => Ok(HkBr),
            "hu_tin" => Ok(HuTin),
            "id_npwp" => Ok(IdNpwp),
            "il_vat" => Ok(IlVat),
            "in_gst" => Ok(InGst),
            "is_vat" => Ok(IsVat),
            "jp_cn" => Ok(JpCn),
            "jp_rn" => Ok(JpRn),
            "jp_trn" => Ok(JpTrn),
            "ke_pin" => Ok(KePin),
            "kr_brn" => Ok(KrBrn),
            "kz_bin" => Ok(KzBin),
            "li_uid" => Ok(LiUid),
            "mx_rfc" => Ok(MxRfc),
            "my_frp" => Ok(MyFrp),
            "my_itn" => Ok(MyItn),
            "my_sst" => Ok(MySst),
            "ng_tin" => Ok(NgTin),
            "no_vat" => Ok(NoVat),
            "no_voec" => Ok(NoVoec),
            "nz_gst" => Ok(NzGst),
            "om_vat" => Ok(OmVat),
            "pe_ruc" => Ok(PeRuc),
            "ph_tin" => Ok(PhTin),
            "ro_tin" => Ok(RoTin),
            "rs_pib" => Ok(RsPib),
            "ru_inn" => Ok(RuInn),
            "ru_kpp" => Ok(RuKpp),
            "sa_vat" => Ok(SaVat),
            "sg_gst" => Ok(SgGst),
            "sg_uen" => Ok(SgUen),
            "si_tin" => Ok(SiTin),
            "sv_nit" => Ok(SvNit),
            "th_vat" => Ok(ThVat),
            "tr_tin" => Ok(TrTin),
            "tw_vat" => Ok(TwVat),
            "ua_vat" => Ok(UaVat),
            "us_ein" => Ok(UsEin),
            "uy_ruc" => Ok(UyRuc),
            "ve_rif" => Ok(VeRif),
            "vn_tin" => Ok(VnTin),
            "za_vat" => Ok(ZaVat),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateCustomerTaxIdDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerTaxIdDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerTaxIdDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCustomerTaxIdDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Creates a new customer object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomer<'a> {
    inner: CreateCustomerBuilder<'a>,
}
impl<'a> CreateCustomer<'a> {
    /// Construct a new `CreateCustomer`.
    pub fn new() -> Self {
        Self { inner: CreateCustomerBuilder::new() }
    }
    /// The customer's address.
    pub fn address(mut self, address: OptionalFieldsAddress<'a>) -> Self {
        self.inner.address = Some(address);
        self
    }
    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    pub fn balance(mut self, balance: i64) -> Self {
        self.inner.balance = Some(balance);
        self
    }
    /// Balance information and default balance settings for this customer.
    pub fn cash_balance(mut self, cash_balance: CreateCustomerCashBalance) -> Self {
        self.inner.cash_balance = Some(cash_balance);
        self
    }
    pub fn coupon(mut self, coupon: &'a str) -> Self {
        self.inner.coupon = Some(coupon);
        self
    }
    /// An arbitrary string that you can attach to a customer object.
    /// It is displayed alongside the customer in the dashboard.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Customer's email address.
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    pub fn email(mut self, email: &'a str) -> Self {
        self.inner.email = Some(email);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The prefix for the customer used to generate unique invoice numbers.
    /// Must be 3–12 uppercase letters or numbers.
    pub fn invoice_prefix(mut self, invoice_prefix: &'a str) -> Self {
        self.inner.invoice_prefix = Some(invoice_prefix);
        self
    }
    /// Default invoice settings for this customer.
    pub fn invoice_settings(mut self, invoice_settings: CreateCustomerInvoiceSettings<'a>) -> Self {
        self.inner.invoice_settings = Some(invoice_settings);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The customer's full name or business name.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// The sequence to be used on the customer's next invoice. Defaults to 1.
    pub fn next_invoice_sequence(mut self, next_invoice_sequence: i64) -> Self {
        self.inner.next_invoice_sequence = Some(next_invoice_sequence);
        self
    }
    pub fn payment_method(mut self, payment_method: &'a str) -> Self {
        self.inner.payment_method = Some(payment_method);
        self
    }
    /// The customer's phone number.
    pub fn phone(mut self, phone: &'a str) -> Self {
        self.inner.phone = Some(phone);
        self
    }
    /// Customer's preferred languages, ordered by preference.
    pub fn preferred_locales(mut self, preferred_locales: &'a [&'a str]) -> Self {
        self.inner.preferred_locales = Some(preferred_locales);
        self
    }
    /// The ID of a promotion code to apply to the customer.
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    pub fn promotion_code(mut self, promotion_code: &'a str) -> Self {
        self.inner.promotion_code = Some(promotion_code);
        self
    }
    /// The customer's shipping information. Appears on invoices emailed to this customer.
    pub fn shipping(mut self, shipping: CustomerShipping<'a>) -> Self {
        self.inner.shipping = Some(shipping);
        self
    }
    pub fn source(mut self, source: &'a str) -> Self {
        self.inner.source = Some(source);
        self
    }
    /// Tax details about the customer.
    pub fn tax(mut self, tax: CreateCustomerTax<'a>) -> Self {
        self.inner.tax = Some(tax);
        self
    }
    /// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
    pub fn tax_exempt(mut self, tax_exempt: stripe_shared::CustomerTaxExempt) -> Self {
        self.inner.tax_exempt = Some(tax_exempt);
        self
    }
    /// The customer's tax IDs.
    pub fn tax_id_data(mut self, tax_id_data: &'a [CreateCustomerTaxIdData<'a>]) -> Self {
        self.inner.tax_id_data = Some(tax_id_data);
        self
    }
    /// ID of the test clock to attach to the customer.
    pub fn test_clock(mut self, test_clock: &'a str) -> Self {
        self.inner.test_clock = Some(test_clock);
        self
    }
    pub fn validate(mut self, validate: bool) -> Self {
        self.inner.validate = Some(validate);
        self
    }
}
impl<'a> Default for CreateCustomer<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateCustomer<'_> {
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

impl StripeRequest for CreateCustomer<'_> {
    type Output = stripe_shared::Customer;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/customers").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateCustomerBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<OptionalFieldsAddress<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cash_balance: Option<UpdateCustomerCashBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_prefix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_settings: Option<UpdateCustomerInvoiceSettings<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_invoice_sequence: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locales: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promotion_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<CustomerShipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax: Option<UpdateCustomerTax<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_exempt: Option<stripe_shared::CustomerTaxExempt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<bool>,
}
impl<'a> UpdateCustomerBuilder<'a> {
    fn new() -> Self {
        Self {
            address: None,
            balance: None,
            cash_balance: None,
            coupon: None,
            default_source: None,
            description: None,
            email: None,
            expand: None,
            invoice_prefix: None,
            invoice_settings: None,
            metadata: None,
            name: None,
            next_invoice_sequence: None,
            phone: None,
            preferred_locales: None,
            promotion_code: None,
            shipping: None,
            source: None,
            tax: None,
            tax_exempt: None,
            validate: None,
        }
    }
}
/// Balance information and default balance settings for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateCustomerCashBalanceSettings>,
}
impl UpdateCustomerCashBalance {
    pub fn new() -> Self {
        Self { settings: None }
    }
}
impl Default for UpdateCustomerCashBalance {
    fn default() -> Self {
        Self::new()
    }
}
/// Settings controlling the behavior of the customer's cash balance,
/// such as reconciliation of funds received.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    /// Valid options are `automatic`, `manual`, or `merchant_default`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<UpdateCustomerCashBalanceSettingsReconciliationMode>,
}
impl UpdateCustomerCashBalanceSettings {
    pub fn new() -> Self {
        Self { reconciliation_mode: None }
    }
}
impl Default for UpdateCustomerCashBalanceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls how funds transferred by the customer are applied to payment intents and invoices.
/// Valid options are `automatic`, `manual`, or `merchant_default`.
/// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCustomerCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
    MerchantDefault,
}
impl UpdateCustomerCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        use UpdateCustomerCashBalanceSettingsReconciliationMode::*;
        match self {
            Automatic => "automatic",
            Manual => "manual",
            MerchantDefault => "merchant_default",
        }
    }
}

impl std::str::FromStr for UpdateCustomerCashBalanceSettingsReconciliationMode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCustomerCashBalanceSettingsReconciliationMode::*;
        match s {
            "automatic" => Ok(Automatic),
            "manual" => Ok(Manual),
            "merchant_default" => Ok(MerchantDefault),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateCustomerCashBalanceSettingsReconciliationMode",
            )
        })
    }
}
/// Default invoice settings for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerInvoiceSettings<'a> {
    /// The list of up to 4 default custom fields to be displayed on invoices for this customer.
    /// When updating, pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CustomFieldParams<'a>]>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<UpdateCustomerInvoiceSettingsRenderingOptions>,
}
impl<'a> UpdateCustomerInvoiceSettings<'a> {
    pub fn new() -> Self {
        Self {
            custom_fields: None,
            default_payment_method: None,
            footer: None,
            rendering_options: None,
        }
    }
}
impl<'a> Default for UpdateCustomerInvoiceSettings<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerInvoiceSettingsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
}
impl UpdateCustomerInvoiceSettingsRenderingOptions {
    pub fn new() -> Self {
        Self { amount_tax_display: None }
    }
}
impl Default for UpdateCustomerInvoiceSettingsRenderingOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}
impl UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay",
            )
        })
    }
}
/// Tax details about the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCustomerTax<'a> {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// A flag that indicates when Stripe should validate the customer tax location.
    /// Defaults to `deferred`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_location: Option<UpdateCustomerTaxValidateLocation>,
}
impl<'a> UpdateCustomerTax<'a> {
    pub fn new() -> Self {
        Self { ip_address: None, validate_location: None }
    }
}
impl<'a> Default for UpdateCustomerTax<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// A flag that indicates when Stripe should validate the customer tax location.
/// Defaults to `deferred`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCustomerTaxValidateLocation {
    Deferred,
    Immediately,
}
impl UpdateCustomerTaxValidateLocation {
    pub fn as_str(self) -> &'static str {
        use UpdateCustomerTaxValidateLocation::*;
        match self {
            Deferred => "deferred",
            Immediately => "immediately",
        }
    }
}

impl std::str::FromStr for UpdateCustomerTaxValidateLocation {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCustomerTaxValidateLocation::*;
        match s {
            "deferred" => Ok(Deferred),
            "immediately" => Ok(Immediately),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateCustomerTaxValidateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCustomerTaxValidateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCustomerTaxValidateLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateCustomerTaxValidateLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateCustomerTaxValidateLocation")
        })
    }
}
/// Updates the specified customer by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
/// For example, if you pass the **source** parameter, that becomes the customer’s active source (e.g., a card) to be used for all charges in the future.
/// When you update a customer to a new valid card source by passing the **source** parameter: for each of the customer’s current subscriptions, if the subscription bills automatically and is in the `past_due` state, then the latest open invoice for the subscription with automatic collection enabled will be retried.
/// This retry will not count as an automatic retry, and will not affect the next regularly scheduled payment for the invoice.
/// Changing the **default_source** for a customer will not trigger this behavior.
///
/// This request accepts mostly the same arguments as the customer creation call.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCustomer<'a> {
    inner: UpdateCustomerBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> UpdateCustomer<'a> {
    /// Construct a new `UpdateCustomer`.
    pub fn new(customer: &'a stripe_shared::CustomerId) -> Self {
        Self { customer, inner: UpdateCustomerBuilder::new() }
    }
    /// The customer's address.
    pub fn address(mut self, address: OptionalFieldsAddress<'a>) -> Self {
        self.inner.address = Some(address);
        self
    }
    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    pub fn balance(mut self, balance: i64) -> Self {
        self.inner.balance = Some(balance);
        self
    }
    /// Balance information and default balance settings for this customer.
    pub fn cash_balance(mut self, cash_balance: UpdateCustomerCashBalance) -> Self {
        self.inner.cash_balance = Some(cash_balance);
        self
    }
    pub fn coupon(mut self, coupon: &'a str) -> Self {
        self.inner.coupon = Some(coupon);
        self
    }
    /// If you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) parameter.
    ///
    /// Provide the ID of a payment source already attached to this customer to make it this customer's default payment source.
    ///
    /// If you want to add a new payment source and make it the default, see the [source](https://stripe.com/docs/api/customers/update#update_customer-source) property.
    pub fn default_source(mut self, default_source: &'a str) -> Self {
        self.inner.default_source = Some(default_source);
        self
    }
    /// An arbitrary string that you can attach to a customer object.
    /// It is displayed alongside the customer in the dashboard.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Customer's email address.
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    pub fn email(mut self, email: &'a str) -> Self {
        self.inner.email = Some(email);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The prefix for the customer used to generate unique invoice numbers.
    /// Must be 3–12 uppercase letters or numbers.
    pub fn invoice_prefix(mut self, invoice_prefix: &'a str) -> Self {
        self.inner.invoice_prefix = Some(invoice_prefix);
        self
    }
    /// Default invoice settings for this customer.
    pub fn invoice_settings(mut self, invoice_settings: UpdateCustomerInvoiceSettings<'a>) -> Self {
        self.inner.invoice_settings = Some(invoice_settings);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The customer's full name or business name.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// The sequence to be used on the customer's next invoice. Defaults to 1.
    pub fn next_invoice_sequence(mut self, next_invoice_sequence: i64) -> Self {
        self.inner.next_invoice_sequence = Some(next_invoice_sequence);
        self
    }
    /// The customer's phone number.
    pub fn phone(mut self, phone: &'a str) -> Self {
        self.inner.phone = Some(phone);
        self
    }
    /// Customer's preferred languages, ordered by preference.
    pub fn preferred_locales(mut self, preferred_locales: &'a [&'a str]) -> Self {
        self.inner.preferred_locales = Some(preferred_locales);
        self
    }
    /// The ID of a promotion code to apply to the customer.
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    pub fn promotion_code(mut self, promotion_code: &'a str) -> Self {
        self.inner.promotion_code = Some(promotion_code);
        self
    }
    /// The customer's shipping information. Appears on invoices emailed to this customer.
    pub fn shipping(mut self, shipping: CustomerShipping<'a>) -> Self {
        self.inner.shipping = Some(shipping);
        self
    }
    pub fn source(mut self, source: &'a str) -> Self {
        self.inner.source = Some(source);
        self
    }
    /// Tax details about the customer.
    pub fn tax(mut self, tax: UpdateCustomerTax<'a>) -> Self {
        self.inner.tax = Some(tax);
        self
    }
    /// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
    pub fn tax_exempt(mut self, tax_exempt: stripe_shared::CustomerTaxExempt) -> Self {
        self.inner.tax_exempt = Some(tax_exempt);
        self
    }
    pub fn validate(mut self, validate: bool) -> Self {
        self.inner.validate = Some(validate);
        self
    }
}
impl UpdateCustomer<'_> {
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

impl StripeRequest for UpdateCustomer<'_> {
    type Output = stripe_shared::Customer;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(StripeMethod::Post, format!("/customers/{customer}")).form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateFundingInstructionsCustomerBuilder<'a> {
    bank_transfer: CreateFundingInstructionsCustomerBankTransfer<'a>,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    funding_type: CreateFundingInstructionsCustomerFundingType,
}
impl<'a> CreateFundingInstructionsCustomerBuilder<'a> {
    fn new(
        bank_transfer: CreateFundingInstructionsCustomerBankTransfer<'a>,
        currency: stripe_types::Currency,
        funding_type: CreateFundingInstructionsCustomerFundingType,
    ) -> Self {
        Self { bank_transfer, currency, expand: None, funding_type }
    }
}
/// Additional parameters for `bank_transfer` funding types
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFundingInstructionsCustomerBankTransfer<'a> {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<CreateFundingInstructionsCustomerBankTransferEuBankTransfer<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    /// If not specified, all valid types will be returned.
    ///
    /// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types:
        Option<&'a [CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes]>,
    /// The type of the `bank_transfer`
    #[serde(rename = "type")]
    pub type_: CreateFundingInstructionsCustomerBankTransferType,
}
impl<'a> CreateFundingInstructionsCustomerBankTransfer<'a> {
    pub fn new(type_: CreateFundingInstructionsCustomerBankTransferType) -> Self {
        Self { eu_bank_transfer: None, requested_address_types: None, type_ }
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFundingInstructionsCustomerBankTransferEuBankTransfer<'a> {
    /// The desired country code of the bank account information.
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> CreateFundingInstructionsCustomerBankTransferEuBankTransfer<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// List of address types that should be returned in the financial_addresses response.
/// If not specified, all valid types will be returned.
///
/// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    Iban,
    SortCode,
    Spei,
    Zengin,
}
impl CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        use CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes::*;
        match self {
            Iban => "iban",
            SortCode => "sort_code",
            Spei => "spei",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes::*;
        match s {
            "iban" => Ok(Iban),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "zengin" => Ok(Zengin),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes"))
    }
}
/// The type of the `bank_transfer`
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateFundingInstructionsCustomerBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}
impl CreateFundingInstructionsCustomerBankTransferType {
    pub fn as_str(self) -> &'static str {
        use CreateFundingInstructionsCustomerBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl std::str::FromStr for CreateFundingInstructionsCustomerBankTransferType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFundingInstructionsCustomerBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            "us_bank_transfer" => Ok(UsBankTransfer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateFundingInstructionsCustomerBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateFundingInstructionsCustomerBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateFundingInstructionsCustomerBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateFundingInstructionsCustomerBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateFundingInstructionsCustomerBankTransferType",
            )
        })
    }
}
/// The `funding_type` to get the instructions for.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateFundingInstructionsCustomerFundingType {
    BankTransfer,
}
impl CreateFundingInstructionsCustomerFundingType {
    pub fn as_str(self) -> &'static str {
        use CreateFundingInstructionsCustomerFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for CreateFundingInstructionsCustomerFundingType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFundingInstructionsCustomerFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateFundingInstructionsCustomerFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateFundingInstructionsCustomerFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateFundingInstructionsCustomerFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateFundingInstructionsCustomerFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateFundingInstructionsCustomerFundingType",
            )
        })
    }
}
/// Retrieve funding instructions for a customer cash balance.
/// If funding instructions do not yet exist for the customer, new.
/// funding instructions will be created.
/// If funding instructions have already been created for a given customer, the same.
/// funding instructions will be retrieved.
/// In other words, we will return the same funding instructions each time.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateFundingInstructionsCustomer<'a> {
    inner: CreateFundingInstructionsCustomerBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
}
impl<'a> CreateFundingInstructionsCustomer<'a> {
    /// Construct a new `CreateFundingInstructionsCustomer`.
    pub fn new(
        customer: &'a stripe_shared::CustomerId,
        bank_transfer: CreateFundingInstructionsCustomerBankTransfer<'a>,
        currency: stripe_types::Currency,
        funding_type: CreateFundingInstructionsCustomerFundingType,
    ) -> Self {
        Self {
            customer,
            inner: CreateFundingInstructionsCustomerBuilder::new(
                bank_transfer,
                currency,
                funding_type,
            ),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CreateFundingInstructionsCustomer<'_> {
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

impl StripeRequest for CreateFundingInstructionsCustomer<'_> {
    type Output = stripe_shared::FundingInstructions;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/customers/{customer}/funding_instructions"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct FundCashBalanceCustomerBuilder<'a> {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference: Option<&'a str>,
}
impl<'a> FundCashBalanceCustomerBuilder<'a> {
    fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, expand: None, reference: None }
    }
}
/// Create an incoming testmode bank transfer
#[derive(Clone, Debug, serde::Serialize)]
pub struct FundCashBalanceCustomer<'a> {
    inner: FundCashBalanceCustomerBuilder<'a>,
    customer: &'a str,
}
impl<'a> FundCashBalanceCustomer<'a> {
    /// Construct a new `FundCashBalanceCustomer`.
    pub fn new(customer: &'a str, amount: i64, currency: stripe_types::Currency) -> Self {
        Self { customer, inner: FundCashBalanceCustomerBuilder::new(amount, currency) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A description of the test funding.
    /// This simulates free-text references supplied by customers when making bank transfers to their cash balance.
    /// You can use this to test how Stripe's [reconciliation algorithm](https://stripe.com/docs/payments/customer-balance/reconciliation) applies to different user inputs.
    pub fn reference(mut self, reference: &'a str) -> Self {
        self.inner.reference = Some(reference);
        self
    }
}
impl FundCashBalanceCustomer<'_> {
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

impl StripeRequest for FundCashBalanceCustomer<'_> {
    type Output = stripe_shared::CustomerCashBalanceTransaction;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/customers/{customer}/fund_cash_balance"),
        )
        .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct OptionalFieldsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> OptionalFieldsAddress<'a> {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl<'a> Default for OptionalFieldsAddress<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomFieldParams<'a> {
    /// The name of the custom field. This may be up to 40 characters.
    pub name: &'a str,
    /// The value of the custom field. This may be up to 140 characters.
    pub value: &'a str,
}
impl<'a> CustomFieldParams<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomerShipping<'a> {
    /// Customer shipping address.
    pub address: OptionalFieldsAddress<'a>,
    /// Customer name.
    pub name: &'a str,
    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CustomerShipping<'a> {
    pub fn new(address: OptionalFieldsAddress<'a>, name: &'a str) -> Self {
        Self { address, name, phone: None }
    }
}
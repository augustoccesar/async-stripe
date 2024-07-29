#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupAttemptPaymentMethodDetails {
    pub acss_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAcssDebit>,
    pub amazon_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBancontact>,
    pub boleto: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBoleto>,
    pub card: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCard>,
    pub card_present: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCardPresent>,
    pub cashapp: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCashapp>,
    pub ideal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsIdeal>,
    pub klarna: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKlarna>,
    pub link: Option<stripe_shared::SetupAttemptPaymentMethodDetailsLink>,
    pub paypal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPaypal>,
    pub revolut_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsRevolutPay>,
    pub sepa_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSepaDebit>,
    pub sofort: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSofort>,
    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub us_bank_account: Option<stripe_shared::SetupAttemptPaymentMethodDetailsUsBankAccount>,
}
#[doc(hidden)]
pub struct SetupAttemptPaymentMethodDetailsBuilder {
    acss_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsAcssDebit>>,
    amazon_pay: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsAmazonPay>>,
    au_becs_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsBacsDebit>>,
    bancontact: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsBancontact>>,
    boleto: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsBoleto>>,
    card: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsCard>>,
    card_present: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsCardPresent>>,
    cashapp: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsCashapp>>,
    ideal: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsIdeal>>,
    klarna: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsKlarna>>,
    link: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsLink>>,
    paypal: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsPaypal>>,
    revolut_pay: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsRevolutPay>>,
    sepa_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsSepaDebit>>,
    sofort: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsSofort>>,
    type_: Option<String>,
    us_bank_account: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsUsBankAccount>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupAttemptPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupAttemptPaymentMethodDetails>,
        builder: SetupAttemptPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<SetupAttemptPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupAttemptPaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupAttemptPaymentMethodDetailsBuilder {
        type Out = SetupAttemptPaymentMethodDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.bancontact),
                "boleto" => Deserialize::begin(&mut self.boleto),
                "card" => Deserialize::begin(&mut self.card),
                "card_present" => Deserialize::begin(&mut self.card_present),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "ideal" => Deserialize::begin(&mut self.ideal),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "link" => Deserialize::begin(&mut self.link),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
                "type" => Deserialize::begin(&mut self.type_),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                amazon_pay: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                cashapp: Deserialize::default(),
                ideal: Deserialize::default(),
                klarna: Deserialize::default(),
                link: Deserialize::default(),
                paypal: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                acss_debit: self.acss_debit?,
                amazon_pay: self.amazon_pay?,
                au_becs_debit: self.au_becs_debit?,
                bacs_debit: self.bacs_debit?,
                bancontact: self.bancontact.take()?,
                boleto: self.boleto?,
                card: self.card.take()?,
                card_present: self.card_present.take()?,
                cashapp: self.cashapp?,
                ideal: self.ideal.take()?,
                klarna: self.klarna?,
                link: self.link?,
                paypal: self.paypal?,
                revolut_pay: self.revolut_pay?,
                sepa_debit: self.sepa_debit?,
                sofort: self.sofort.take()?,
                type_: self.type_.take()?,
                us_bank_account: self.us_bank_account?,
            })
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

    impl ObjectDeser for SetupAttemptPaymentMethodDetails {
        type Builder = SetupAttemptPaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for SetupAttemptPaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupAttemptPaymentMethodDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit" => b.acss_debit = Some(FromValueOpt::from_value(v)?),
                    "amazon_pay" => b.amazon_pay = Some(FromValueOpt::from_value(v)?),
                    "au_becs_debit" => b.au_becs_debit = Some(FromValueOpt::from_value(v)?),
                    "bacs_debit" => b.bacs_debit = Some(FromValueOpt::from_value(v)?),
                    "bancontact" => b.bancontact = Some(FromValueOpt::from_value(v)?),
                    "boleto" => b.boleto = Some(FromValueOpt::from_value(v)?),
                    "card" => b.card = Some(FromValueOpt::from_value(v)?),
                    "card_present" => b.card_present = Some(FromValueOpt::from_value(v)?),
                    "cashapp" => b.cashapp = Some(FromValueOpt::from_value(v)?),
                    "ideal" => b.ideal = Some(FromValueOpt::from_value(v)?),
                    "klarna" => b.klarna = Some(FromValueOpt::from_value(v)?),
                    "link" => b.link = Some(FromValueOpt::from_value(v)?),
                    "paypal" => b.paypal = Some(FromValueOpt::from_value(v)?),
                    "revolut_pay" => b.revolut_pay = Some(FromValueOpt::from_value(v)?),
                    "sepa_debit" => b.sepa_debit = Some(FromValueOpt::from_value(v)?),
                    "sofort" => b.sofort = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "us_bank_account" => b.us_bank_account = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
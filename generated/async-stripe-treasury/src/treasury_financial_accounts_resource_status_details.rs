#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceStatusDetails {
    /// Details related to the closure of this FinancialAccount
    pub closed: Option<stripe_treasury::TreasuryFinancialAccountsResourceClosedStatusDetails>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceStatusDetailsBuilder {
    closed: Option<Option<stripe_treasury::TreasuryFinancialAccountsResourceClosedStatusDetails>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryFinancialAccountsResourceStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceStatusDetails>,
        builder: TreasuryFinancialAccountsResourceStatusDetailsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceStatusDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceStatusDetailsBuilder {
        type Out = TreasuryFinancialAccountsResourceStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "closed" => Deserialize::begin(&mut self.closed),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { closed: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { closed: self.closed.take()? })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceStatusDetails {
        type Builder = TreasuryFinancialAccountsResourceStatusDetailsBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryFinancialAccountsResourceStatusDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "closed" => b.closed = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
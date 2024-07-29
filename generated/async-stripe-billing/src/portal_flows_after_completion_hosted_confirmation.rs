#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsAfterCompletionHostedConfirmation {
    /// A custom message to display to the customer after the flow is completed.
    pub custom_message: Option<String>,
}
#[doc(hidden)]
pub struct PortalFlowsAfterCompletionHostedConfirmationBuilder {
    custom_message: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsAfterCompletionHostedConfirmation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsAfterCompletionHostedConfirmation>,
        builder: PortalFlowsAfterCompletionHostedConfirmationBuilder,
    }

    impl Visitor for Place<PortalFlowsAfterCompletionHostedConfirmation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsAfterCompletionHostedConfirmationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsAfterCompletionHostedConfirmationBuilder {
        type Out = PortalFlowsAfterCompletionHostedConfirmation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom_message" => Deserialize::begin(&mut self.custom_message),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { custom_message: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { custom_message: self.custom_message.take()? })
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

    impl ObjectDeser for PortalFlowsAfterCompletionHostedConfirmation {
        type Builder = PortalFlowsAfterCompletionHostedConfirmationBuilder;
    }

    impl FromValueOpt for PortalFlowsAfterCompletionHostedConfirmation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsAfterCompletionHostedConfirmationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "custom_message" => b.custom_message = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
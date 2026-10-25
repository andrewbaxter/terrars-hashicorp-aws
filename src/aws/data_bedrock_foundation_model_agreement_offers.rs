#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBedrockFoundationModelAgreementOffersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    model_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offer_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBedrockFoundationModelAgreementOffers_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBedrockFoundationModelAgreementOffersData_>,
}

#[derive(Clone)]
pub struct DataBedrockFoundationModelAgreementOffers(std::rc::Rc<DataBedrockFoundationModelAgreementOffers_>);

impl DataBedrockFoundationModelAgreementOffers {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `offer_type`.\n"]
    pub fn set_offer_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().offer_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offer_type` after provisioning.\n"]
    pub fn offer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offers` after provisioning.\n"]
    pub fn offers(&self) -> terrars::ListRef<DataBedrockFoundationModelAgreementOffers_OffersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.offers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataBedrockFoundationModelAgreementOffers {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBedrockFoundationModelAgreementOffers { }

impl terrars::ToListMappable for DataBedrockFoundationModelAgreementOffers {
    type O = terrars::ListRef<DataBedrockFoundationModelAgreementOffersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBedrockFoundationModelAgreementOffers_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_bedrock_foundation_model_agreement_offers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBedrockFoundationModelAgreementOffers {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildDataBedrockFoundationModelAgreementOffers {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBedrockFoundationModelAgreementOffers {
        let out =
            DataBedrockFoundationModelAgreementOffers(std::rc::Rc::new(DataBedrockFoundationModelAgreementOffers_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataBedrockFoundationModelAgreementOffersData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    model_id: self.model_id,
                    offer_type: core::default::Default::default(),
                    region: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBedrockFoundationModelAgreementOffersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModelAgreementOffersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBedrockFoundationModelAgreementOffersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offer_type` after provisioning.\n"]
    pub fn offer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offers` after provisioning.\n"]
    pub fn offers(&self) -> terrars::ListRef<DataBedrockFoundationModelAgreementOffers_OffersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.offers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm {
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm {
    type O = terrars::BlockAssignable<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm {}

impl BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm {
    pub fn build(self) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm {
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTermRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTermRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTermRef {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTermRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTermRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refund_policy_description: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm {
    #[doc = "Set the field `refund_policy_description`.\n"]
    pub fn set_refund_policy_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.refund_policy_description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm {
    type O = terrars::BlockAssignable<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm {}

impl BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm {
    pub fn build(self) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm {
            refund_policy_description: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTermRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTermRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTermRef {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTermRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTermRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `refund_policy_description` after provisioning.\n"]
    pub fn refund_policy_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refund_policy_description", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price`.\n"]
    pub fn set_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard {
    type O =
        terrars
        ::BlockAssignable<
            DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard {}

impl BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard {
    pub fn build(
        self,
    ) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard {
            description: core::default::Default::default(),
            dimension: core::default::Default::default(),
            price: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCardRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCardRef {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `price` after provisioning.\n"]
    pub fn price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_card: std
    ::option
    ::Option<
        terrars
        ::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard>,
    >,
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm {
    #[doc = "Set the field `rate_card`.\n"]
    pub fn set_rate_card(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCard,
                        >,
                    >,
    ) -> Self {
        self.rate_card = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm {
    type O =
        terrars::BlockAssignable<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm {}

impl BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm {
    pub fn build(self) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm {
            rate_card: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTermRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTermRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTermRef {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTermRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTermRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rate_card` after provisioning.\n"]
    pub fn rate_card(
        &self,
    ) -> terrars
    ::ListRef<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm_RateCardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_card", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agreement_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm {
    #[doc = "Set the field `agreement_duration`.\n"]
    pub fn set_agreement_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.agreement_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm {
    type O = terrars::BlockAssignable<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm {}

impl BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm {
    pub fn build(self) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm {
            agreement_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTermRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTermRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTermRef {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTermRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTermRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agreement_duration` after provisioning.\n"]
    pub fn agreement_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agreement_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    legal_term: std
    ::option
    ::Option<terrars::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_term: std
    ::option
    ::Option<terrars::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_based_pricing_term: std
    ::option
    ::Option<terrars::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validity_term: std
    ::option
    ::Option<terrars::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm>>,
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetails {
    #[doc = "Set the field `legal_term`.\n"]
    pub fn set_legal_term(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTerm>,
                    >,
    ) -> Self {
        self.legal_term = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `support_term`.\n"]
    pub fn set_support_term(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTerm>,
                    >,
    ) -> Self {
        self.support_term = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usage_based_pricing_term`.\n"]
    pub fn set_usage_based_pricing_term(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTerm,
                        >,
                    >,
    ) -> Self {
        self.usage_based_pricing_term = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validity_term`.\n"]
    pub fn set_validity_term(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTerm>,
                    >,
    ) -> Self {
        self.validity_term = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockFoundationModelAgreementOffers_Offers_TermDetails {
    type O = terrars::BlockAssignable<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails {}

impl BuildDataBedrockFoundationModelAgreementOffers_Offers_TermDetails {
    pub fn build(self) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetails {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetails {
            legal_term: core::default::Default::default(),
            support_term: core::default::Default::default(),
            usage_based_pricing_term: core::default::Default::default(),
            validity_term: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers_TermDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModelAgreementOffers_Offers_TermDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockFoundationModelAgreementOffers_Offers_TermDetailsRef {
        DataBedrockFoundationModelAgreementOffers_Offers_TermDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockFoundationModelAgreementOffers_Offers_TermDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `legal_term` after provisioning.\n"]
    pub fn legal_term(
        &self,
    ) -> terrars::ListRef<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_LegalTermRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.legal_term", self.base))
    }

    #[doc = "Get a reference to the value of field `support_term` after provisioning.\n"]
    pub fn support_term(
        &self,
    ) -> terrars::ListRef<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_SupportTermRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.support_term", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_based_pricing_term` after provisioning.\n"]
    pub fn usage_based_pricing_term(
        &self,
    ) -> terrars::ListRef<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_UsageBasedPricingTermRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usage_based_pricing_term", self.base))
    }

    #[doc = "Get a reference to the value of field `validity_term` after provisioning.\n"]
    pub fn validity_term(
        &self,
    ) -> terrars::ListRef<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails_ValidityTermRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validity_term", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBedrockFoundationModelAgreementOffers_Offers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offer_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offer_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    term_details: std::option::Option<terrars::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails>>,
}

impl DataBedrockFoundationModelAgreementOffers_Offers {
    #[doc = "Set the field `offer_id`.\n"]
    pub fn set_offer_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.offer_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offer_token`.\n"]
    pub fn set_offer_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.offer_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `term_details`.\n"]
    pub fn set_term_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBedrockFoundationModelAgreementOffers_Offers_TermDetails>>,
    ) -> Self {
        self.term_details = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBedrockFoundationModelAgreementOffers_Offers {
    type O = terrars::BlockAssignable<DataBedrockFoundationModelAgreementOffers_Offers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBedrockFoundationModelAgreementOffers_Offers {}

impl BuildDataBedrockFoundationModelAgreementOffers_Offers {
    pub fn build(self) -> DataBedrockFoundationModelAgreementOffers_Offers {
        DataBedrockFoundationModelAgreementOffers_Offers {
            offer_id: core::default::Default::default(),
            offer_token: core::default::Default::default(),
            term_details: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBedrockFoundationModelAgreementOffers_OffersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBedrockFoundationModelAgreementOffers_OffersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBedrockFoundationModelAgreementOffers_OffersRef {
        DataBedrockFoundationModelAgreementOffers_OffersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBedrockFoundationModelAgreementOffers_OffersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `offer_id` after provisioning.\n"]
    pub fn offer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `offer_token` after provisioning.\n"]
    pub fn offer_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offer_token", self.base))
    }

    #[doc = "Get a reference to the value of field `term_details` after provisioning.\n"]
    pub fn term_details(&self) -> terrars::ListRef<DataBedrockFoundationModelAgreementOffers_Offers_TermDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.term_details", self.base))
    }
}

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLicensemanagerReceivedLicenseData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    license_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLicensemanagerReceivedLicense_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLicensemanagerReceivedLicenseData_>,
}

#[derive(Clone)]
pub struct DataLicensemanagerReceivedLicense(std::rc::Rc<DataLicensemanagerReceivedLicense_>);

impl DataLicensemanagerReceivedLicense {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `beneficiary` after provisioning.\n"]
    pub fn beneficiary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.beneficiary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumption_configuration` after provisioning.\n"]
    pub fn consumption_configuration(
        &self,
    ) -> terrars::ListRef<DataLicensemanagerReceivedLicense_ConsumptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entitlements` after provisioning.\n"]
    pub fn entitlements(&self) -> terrars::SetRef<DataLicensemanagerReceivedLicense_EntitlementsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.entitlements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_region` after provisioning.\n"]
    pub fn home_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::ListRef<DataLicensemanagerReceivedLicense_IssuerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_arn` after provisioning.\n"]
    pub fn license_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_metadata` after provisioning.\n"]
    pub fn license_metadata(&self) -> terrars::SetRef<DataLicensemanagerReceivedLicense_LicenseMetadataRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.license_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_name` after provisioning.\n"]
    pub fn license_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_name` after provisioning.\n"]
    pub fn product_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_sku` after provisioning.\n"]
    pub fn product_sku(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_sku", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `received_metadata` after provisioning.\n"]
    pub fn received_metadata(&self) -> terrars::ListRef<DataLicensemanagerReceivedLicense_ReceivedMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.received_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> terrars::ListRef<DataLicensemanagerReceivedLicense_ValidityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for DataLicensemanagerReceivedLicense {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLicensemanagerReceivedLicense { }

impl terrars::ToListMappable for DataLicensemanagerReceivedLicense {
    type O = terrars::ListRef<DataLicensemanagerReceivedLicenseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLicensemanagerReceivedLicense_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_licensemanager_received_license".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLicensemanagerReceivedLicense {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub license_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataLicensemanagerReceivedLicense {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLicensemanagerReceivedLicense {
        let out = DataLicensemanagerReceivedLicense(std::rc::Rc::new(DataLicensemanagerReceivedLicense_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLicensemanagerReceivedLicenseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                license_arn: self.license_arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLicensemanagerReceivedLicenseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLicensemanagerReceivedLicenseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLicensemanagerReceivedLicenseRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `beneficiary` after provisioning.\n"]
    pub fn beneficiary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.beneficiary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumption_configuration` after provisioning.\n"]
    pub fn consumption_configuration(
        &self,
    ) -> terrars::ListRef<DataLicensemanagerReceivedLicense_ConsumptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entitlements` after provisioning.\n"]
    pub fn entitlements(&self) -> terrars::SetRef<DataLicensemanagerReceivedLicense_EntitlementsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.entitlements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `home_region` after provisioning.\n"]
    pub fn home_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.home_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::ListRef<DataLicensemanagerReceivedLicense_IssuerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issuer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_arn` after provisioning.\n"]
    pub fn license_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_metadata` after provisioning.\n"]
    pub fn license_metadata(&self) -> terrars::SetRef<DataLicensemanagerReceivedLicense_LicenseMetadataRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.license_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_name` after provisioning.\n"]
    pub fn license_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_name` after provisioning.\n"]
    pub fn product_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_sku` after provisioning.\n"]
    pub fn product_sku(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_sku", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `received_metadata` after provisioning.\n"]
    pub fn received_metadata(&self) -> terrars::ListRef<DataLicensemanagerReceivedLicense_ReceivedMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.received_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> terrars::ListRef<DataLicensemanagerReceivedLicense_ValidityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_early_check_in: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_time_to_live_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration {
    #[doc = "Set the field `allow_early_check_in`.\n"]
    pub fn set_allow_early_check_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_early_check_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_time_to_live_in_minutes`.\n"]
    pub fn set_max_time_to_live_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_time_to_live_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration {
    type O =
        terrars::BlockAssignable<DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration {}

impl BuildDataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration {
    pub fn build(self) -> DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration {
        DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration {
            allow_early_check_in: core::default::Default::default(),
            max_time_to_live_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfigurationRef {
        DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_early_check_in` after provisioning.\n"]
    pub fn allow_early_check_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_early_check_in", self.base))
    }

    #[doc = "Get a reference to the value of field `max_time_to_live_in_minutes` after provisioning.\n"]
    pub fn max_time_to_live_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_time_to_live_in_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_time_to_live_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration {
    #[doc = "Set the field `max_time_to_live_in_minutes`.\n"]
    pub fn set_max_time_to_live_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_time_to_live_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration {
    type O =
        terrars
        ::BlockAssignable<DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration {}

impl BuildDataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration {
    pub fn build(self) -> DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration {
        DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration {
            max_time_to_live_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfigurationRef {
        DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_time_to_live_in_minutes` after provisioning.\n"]
    pub fn max_time_to_live_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_time_to_live_in_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicense_ConsumptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    borrow_configuration: std
    ::option
    ::Option<terrars::ListField<DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisional_configuration: std
    ::option
    ::Option<terrars::ListField<DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    renew_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLicensemanagerReceivedLicense_ConsumptionConfiguration {
    #[doc = "Set the field `borrow_configuration`.\n"]
    pub fn set_borrow_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfiguration>,
                    >,
    ) -> Self {
        self.borrow_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisional_configuration`.\n"]
    pub fn set_provisional_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfiguration,
                        >,
                    >,
    ) -> Self {
        self.provisional_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `renew_type`.\n"]
    pub fn set_renew_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.renew_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLicensemanagerReceivedLicense_ConsumptionConfiguration {
    type O = terrars::BlockAssignable<DataLicensemanagerReceivedLicense_ConsumptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicense_ConsumptionConfiguration {}

impl BuildDataLicensemanagerReceivedLicense_ConsumptionConfiguration {
    pub fn build(self) -> DataLicensemanagerReceivedLicense_ConsumptionConfiguration {
        DataLicensemanagerReceivedLicense_ConsumptionConfiguration {
            borrow_configuration: core::default::Default::default(),
            provisional_configuration: core::default::Default::default(),
            renew_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLicensemanagerReceivedLicense_ConsumptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLicensemanagerReceivedLicense_ConsumptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLicensemanagerReceivedLicense_ConsumptionConfigurationRef {
        DataLicensemanagerReceivedLicense_ConsumptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicense_ConsumptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `borrow_configuration` after provisioning.\n"]
    pub fn borrow_configuration(
        &self,
    ) -> terrars::ListRef<DataLicensemanagerReceivedLicense_ConsumptionConfiguration_BorrowConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.borrow_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `provisional_configuration` after provisioning.\n"]
    pub fn provisional_configuration(
        &self,
    ) -> terrars::ListRef<DataLicensemanagerReceivedLicense_ConsumptionConfiguration_ProvisionalConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisional_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `renew_type` after provisioning.\n"]
    pub fn renew_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.renew_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicense_Entitlements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_check_in: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overage: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLicensemanagerReceivedLicense_Entitlements {
    #[doc = "Set the field `allow_check_in`.\n"]
    pub fn set_allow_check_in(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_check_in = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_count`.\n"]
    pub fn set_max_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overage`.\n"]
    pub fn set_overage(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.overage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLicensemanagerReceivedLicense_Entitlements {
    type O = terrars::BlockAssignable<DataLicensemanagerReceivedLicense_Entitlements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicense_Entitlements {}

impl BuildDataLicensemanagerReceivedLicense_Entitlements {
    pub fn build(self) -> DataLicensemanagerReceivedLicense_Entitlements {
        DataLicensemanagerReceivedLicense_Entitlements {
            allow_check_in: core::default::Default::default(),
            max_count: core::default::Default::default(),
            name: core::default::Default::default(),
            overage: core::default::Default::default(),
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLicensemanagerReceivedLicense_EntitlementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLicensemanagerReceivedLicense_EntitlementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLicensemanagerReceivedLicense_EntitlementsRef {
        DataLicensemanagerReceivedLicense_EntitlementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicense_EntitlementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_check_in` after provisioning.\n"]
    pub fn allow_check_in(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_check_in", self.base))
    }

    #[doc = "Get a reference to the value of field `max_count` after provisioning.\n"]
    pub fn max_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_count", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `overage` after provisioning.\n"]
    pub fn overage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overage", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicense_Issuer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLicensemanagerReceivedLicense_Issuer {
    #[doc = "Set the field `key_fingerprint`.\n"]
    pub fn set_key_fingerprint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_fingerprint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sign_key`.\n"]
    pub fn set_sign_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sign_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLicensemanagerReceivedLicense_Issuer {
    type O = terrars::BlockAssignable<DataLicensemanagerReceivedLicense_Issuer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicense_Issuer {}

impl BuildDataLicensemanagerReceivedLicense_Issuer {
    pub fn build(self) -> DataLicensemanagerReceivedLicense_Issuer {
        DataLicensemanagerReceivedLicense_Issuer {
            key_fingerprint: core::default::Default::default(),
            name: core::default::Default::default(),
            sign_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLicensemanagerReceivedLicense_IssuerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLicensemanagerReceivedLicense_IssuerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLicensemanagerReceivedLicense_IssuerRef {
        DataLicensemanagerReceivedLicense_IssuerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicense_IssuerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_fingerprint` after provisioning.\n"]
    pub fn key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `sign_key` after provisioning.\n"]
    pub fn sign_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sign_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicense_LicenseMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLicensemanagerReceivedLicense_LicenseMetadata {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLicensemanagerReceivedLicense_LicenseMetadata {
    type O = terrars::BlockAssignable<DataLicensemanagerReceivedLicense_LicenseMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicense_LicenseMetadata {}

impl BuildDataLicensemanagerReceivedLicense_LicenseMetadata {
    pub fn build(self) -> DataLicensemanagerReceivedLicense_LicenseMetadata {
        DataLicensemanagerReceivedLicense_LicenseMetadata {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLicensemanagerReceivedLicense_LicenseMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLicensemanagerReceivedLicense_LicenseMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLicensemanagerReceivedLicense_LicenseMetadataRef {
        DataLicensemanagerReceivedLicense_LicenseMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicense_LicenseMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicense_ReceivedMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_operations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    received_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    received_status_reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLicensemanagerReceivedLicense_ReceivedMetadata {
    #[doc = "Set the field `allowed_operations`.\n"]
    pub fn set_allowed_operations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_operations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `received_status`.\n"]
    pub fn set_received_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.received_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `received_status_reason`.\n"]
    pub fn set_received_status_reason(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.received_status_reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLicensemanagerReceivedLicense_ReceivedMetadata {
    type O = terrars::BlockAssignable<DataLicensemanagerReceivedLicense_ReceivedMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicense_ReceivedMetadata {}

impl BuildDataLicensemanagerReceivedLicense_ReceivedMetadata {
    pub fn build(self) -> DataLicensemanagerReceivedLicense_ReceivedMetadata {
        DataLicensemanagerReceivedLicense_ReceivedMetadata {
            allowed_operations: core::default::Default::default(),
            received_status: core::default::Default::default(),
            received_status_reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLicensemanagerReceivedLicense_ReceivedMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLicensemanagerReceivedLicense_ReceivedMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLicensemanagerReceivedLicense_ReceivedMetadataRef {
        DataLicensemanagerReceivedLicense_ReceivedMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicense_ReceivedMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_operations` after provisioning.\n"]
    pub fn allowed_operations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_operations", self.base))
    }

    #[doc = "Get a reference to the value of field `received_status` after provisioning.\n"]
    pub fn received_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.received_status", self.base))
    }

    #[doc = "Get a reference to the value of field `received_status_reason` after provisioning.\n"]
    pub fn received_status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.received_status_reason", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLicensemanagerReceivedLicense_Validity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    begin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLicensemanagerReceivedLicense_Validity {
    #[doc = "Set the field `begin`.\n"]
    pub fn set_begin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.begin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLicensemanagerReceivedLicense_Validity {
    type O = terrars::BlockAssignable<DataLicensemanagerReceivedLicense_Validity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLicensemanagerReceivedLicense_Validity {}

impl BuildDataLicensemanagerReceivedLicense_Validity {
    pub fn build(self) -> DataLicensemanagerReceivedLicense_Validity {
        DataLicensemanagerReceivedLicense_Validity {
            begin: core::default::Default::default(),
            end: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLicensemanagerReceivedLicense_ValidityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLicensemanagerReceivedLicense_ValidityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLicensemanagerReceivedLicense_ValidityRef {
        DataLicensemanagerReceivedLicense_ValidityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLicensemanagerReceivedLicense_ValidityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `begin` after provisioning.\n"]
    pub fn begin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.begin", self.base))
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
}

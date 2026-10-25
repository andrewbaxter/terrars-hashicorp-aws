#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsoadminApplicationProvidersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSsoadminApplicationProviders_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSsoadminApplicationProvidersData_>,
}

#[derive(Clone)]
pub struct DataSsoadminApplicationProviders(std::rc::Rc<DataSsoadminApplicationProviders_>);

impl DataSsoadminApplicationProviders {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_providers` after provisioning.\n"]
    pub fn application_providers(&self) -> terrars::ListRef<DataSsoadminApplicationProviders_ApplicationProvidersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataSsoadminApplicationProviders {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSsoadminApplicationProviders { }

impl terrars::ToListMappable for DataSsoadminApplicationProviders {
    type O = terrars::ListRef<DataSsoadminApplicationProvidersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSsoadminApplicationProviders_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ssoadmin_application_providers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsoadminApplicationProviders {
    pub tf_id: std::string::String,
}

impl BuildDataSsoadminApplicationProviders {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSsoadminApplicationProviders {
        let out = DataSsoadminApplicationProviders(std::rc::Rc::new(DataSsoadminApplicationProviders_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSsoadminApplicationProvidersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsoadminApplicationProvidersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsoadminApplicationProvidersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSsoadminApplicationProvidersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `application_providers` after provisioning.\n"]
    pub fn application_providers(&self) -> terrars::ListRef<DataSsoadminApplicationProviders_ApplicationProvidersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSsoadminApplicationProviders_ApplicationProviders_DisplayData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icon_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsoadminApplicationProviders_ApplicationProviders_DisplayData {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icon_url`.\n"]
    pub fn set_icon_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.icon_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsoadminApplicationProviders_ApplicationProviders_DisplayData {
    type O = terrars::BlockAssignable<DataSsoadminApplicationProviders_ApplicationProviders_DisplayData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminApplicationProviders_ApplicationProviders_DisplayData {}

impl BuildDataSsoadminApplicationProviders_ApplicationProviders_DisplayData {
    pub fn build(self) -> DataSsoadminApplicationProviders_ApplicationProviders_DisplayData {
        DataSsoadminApplicationProviders_ApplicationProviders_DisplayData {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            icon_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsoadminApplicationProviders_ApplicationProviders_DisplayDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsoadminApplicationProviders_ApplicationProviders_DisplayDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsoadminApplicationProviders_ApplicationProviders_DisplayDataRef {
        DataSsoadminApplicationProviders_ApplicationProviders_DisplayDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminApplicationProviders_ApplicationProviders_DisplayDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `icon_url` after provisioning.\n"]
    pub fn icon_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icon_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsoadminApplicationProviders_ApplicationProviders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_provider_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_data: std
    ::option
    ::Option<terrars::ListField<DataSsoadminApplicationProviders_ApplicationProviders_DisplayData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    federation_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsoadminApplicationProviders_ApplicationProviders {
    #[doc = "Set the field `application_provider_arn`.\n"]
    pub fn set_application_provider_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.application_provider_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_data`.\n"]
    pub fn set_display_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSsoadminApplicationProviders_ApplicationProviders_DisplayData>>,
    ) -> Self {
        self.display_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `federation_protocol`.\n"]
    pub fn set_federation_protocol(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.federation_protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsoadminApplicationProviders_ApplicationProviders {
    type O = terrars::BlockAssignable<DataSsoadminApplicationProviders_ApplicationProviders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminApplicationProviders_ApplicationProviders {}

impl BuildDataSsoadminApplicationProviders_ApplicationProviders {
    pub fn build(self) -> DataSsoadminApplicationProviders_ApplicationProviders {
        DataSsoadminApplicationProviders_ApplicationProviders {
            application_provider_arn: core::default::Default::default(),
            display_data: core::default::Default::default(),
            federation_protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsoadminApplicationProviders_ApplicationProvidersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsoadminApplicationProviders_ApplicationProvidersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsoadminApplicationProviders_ApplicationProvidersRef {
        DataSsoadminApplicationProviders_ApplicationProvidersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminApplicationProviders_ApplicationProvidersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_provider_arn` after provisioning.\n"]
    pub fn application_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `display_data` after provisioning.\n"]
    pub fn display_data(&self) -> terrars::ListRef<DataSsoadminApplicationProviders_ApplicationProviders_DisplayDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.display_data", self.base))
    }

    #[doc = "Get a reference to the value of field `federation_protocol` after provisioning.\n"]
    pub fn federation_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.federation_protocol", self.base))
    }
}

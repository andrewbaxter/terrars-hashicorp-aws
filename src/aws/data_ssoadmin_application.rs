#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsoadminApplicationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    application_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSsoadminApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSsoadminApplicationData_>,
}

#[derive(Clone)]
pub struct DataSsoadminApplication(std::rc::Rc<DataSsoadminApplication_>);

impl DataSsoadminApplication {
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

    #[doc = "Get a reference to the value of field `application_account` after provisioning.\n"]
    pub fn application_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_provider_arn` after provisioning.\n"]
    pub fn application_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_provider_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `portal_options` after provisioning.\n"]
    pub fn portal_options(&self) -> terrars::ListRef<DataSsoadminApplication_PortalOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.portal_options", self.extract_ref()))
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
}

impl terrars::Referable for DataSsoadminApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSsoadminApplication { }

impl terrars::ToListMappable for DataSsoadminApplication {
    type O = terrars::ListRef<DataSsoadminApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSsoadminApplication_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ssoadmin_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsoadminApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub application_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataSsoadminApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSsoadminApplication {
        let out = DataSsoadminApplication(std::rc::Rc::new(DataSsoadminApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSsoadminApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                application_arn: self.application_arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsoadminApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsoadminApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSsoadminApplicationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `application_account` after provisioning.\n"]
    pub fn application_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_provider_arn` after provisioning.\n"]
    pub fn application_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_provider_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `portal_options` after provisioning.\n"]
    pub fn portal_options(&self) -> terrars::ListRef<DataSsoadminApplication_PortalOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.portal_options", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataSsoadminApplication_PortalOptions_SignInOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsoadminApplication_PortalOptions_SignInOptions {
    #[doc = "Set the field `application_url`.\n"]
    pub fn set_application_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsoadminApplication_PortalOptions_SignInOptions {
    type O = terrars::BlockAssignable<DataSsoadminApplication_PortalOptions_SignInOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminApplication_PortalOptions_SignInOptions {}

impl BuildDataSsoadminApplication_PortalOptions_SignInOptions {
    pub fn build(self) -> DataSsoadminApplication_PortalOptions_SignInOptions {
        DataSsoadminApplication_PortalOptions_SignInOptions {
            application_url: core::default::Default::default(),
            origin: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsoadminApplication_PortalOptions_SignInOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsoadminApplication_PortalOptions_SignInOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsoadminApplication_PortalOptions_SignInOptionsRef {
        DataSsoadminApplication_PortalOptions_SignInOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminApplication_PortalOptions_SignInOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_url` after provisioning.\n"]
    pub fn application_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_url", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsoadminApplication_PortalOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_in_options: std::option::Option<terrars::ListField<DataSsoadminApplication_PortalOptions_SignInOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsoadminApplication_PortalOptions {
    #[doc = "Set the field `sign_in_options`.\n"]
    pub fn set_sign_in_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsoadminApplication_PortalOptions_SignInOptions>>,
    ) -> Self {
        self.sign_in_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.visibility = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsoadminApplication_PortalOptions {
    type O = terrars::BlockAssignable<DataSsoadminApplication_PortalOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminApplication_PortalOptions {}

impl BuildDataSsoadminApplication_PortalOptions {
    pub fn build(self) -> DataSsoadminApplication_PortalOptions {
        DataSsoadminApplication_PortalOptions {
            sign_in_options: core::default::Default::default(),
            visibility: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsoadminApplication_PortalOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsoadminApplication_PortalOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsoadminApplication_PortalOptionsRef {
        DataSsoadminApplication_PortalOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminApplication_PortalOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sign_in_options` after provisioning.\n"]
    pub fn sign_in_options(&self) -> terrars::ListRef<DataSsoadminApplication_PortalOptions_SignInOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_in_options", self.base))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}

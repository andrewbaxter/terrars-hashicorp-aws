#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAccountaccessApplicationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center_instance_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAccountaccessApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountaccessApplicationData_>,
}

#[derive(Clone)]
pub struct DataAccountaccessApplication(std::rc::Rc<DataAccountaccessApplication_>);

impl DataAccountaccessApplication {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_center_instance_arn`.\n"]
    pub fn set_identity_center_instance_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().identity_center_instance_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_instance_arn` after provisioning.\n"]
    pub fn identity_center_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_center_instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_source` after provisioning.\n"]
    pub fn identity_source(&self) -> terrars::ListRef<DataAccountaccessApplication_IdentitySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_source", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountaccessApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountaccessApplication { }

impl terrars::ToListMappable for DataAccountaccessApplication {
    type O = terrars::ListRef<DataAccountaccessApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountaccessApplication_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_accountaccess_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountaccessApplication {
    pub tf_id: std::string::String,
}

impl BuildDataAccountaccessApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountaccessApplication {
        let out = DataAccountaccessApplication(std::rc::Rc::new(DataAccountaccessApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountaccessApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                identity_center_instance_arn: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountaccessApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountaccessApplicationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_instance_arn` after provisioning.\n"]
    pub fn identity_center_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_center_instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_source` after provisioning.\n"]
    pub fn identity_source(&self) -> terrars::ListRef<DataAccountaccessApplication_IdentitySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_source", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountaccessApplication_IdentitySource_IdentityCenter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccountaccessApplication_IdentitySource_IdentityCenter {
    #[doc = "Set the field `application_arn`.\n"]
    pub fn set_application_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_arn`.\n"]
    pub fn set_instance_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccountaccessApplication_IdentitySource_IdentityCenter {
    type O = terrars::BlockAssignable<DataAccountaccessApplication_IdentitySource_IdentityCenter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessApplication_IdentitySource_IdentityCenter {}

impl BuildDataAccountaccessApplication_IdentitySource_IdentityCenter {
    pub fn build(self) -> DataAccountaccessApplication_IdentitySource_IdentityCenter {
        DataAccountaccessApplication_IdentitySource_IdentityCenter {
            application_arn: core::default::Default::default(),
            instance_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessApplication_IdentitySource_IdentityCenterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessApplication_IdentitySource_IdentityCenterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountaccessApplication_IdentitySource_IdentityCenterRef {
        DataAccountaccessApplication_IdentitySource_IdentityCenterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessApplication_IdentitySource_IdentityCenterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAccountaccessApplication_IdentitySource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center: std::option::Option<terrars::ListField<DataAccountaccessApplication_IdentitySource_IdentityCenter>>,
}

impl DataAccountaccessApplication_IdentitySource {
    #[doc = "Set the field `identity_center`.\n"]
    pub fn set_identity_center(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAccountaccessApplication_IdentitySource_IdentityCenter>>,
    ) -> Self {
        self.identity_center = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccountaccessApplication_IdentitySource {
    type O = terrars::BlockAssignable<DataAccountaccessApplication_IdentitySource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountaccessApplication_IdentitySource {}

impl BuildDataAccountaccessApplication_IdentitySource {
    pub fn build(self) -> DataAccountaccessApplication_IdentitySource {
        DataAccountaccessApplication_IdentitySource { identity_center: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAccountaccessApplication_IdentitySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountaccessApplication_IdentitySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccountaccessApplication_IdentitySourceRef {
        DataAccountaccessApplication_IdentitySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountaccessApplication_IdentitySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_center` after provisioning.\n"]
    pub fn identity_center(&self) -> terrars::ListRef<DataAccountaccessApplication_IdentitySource_IdentityCenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center", self.base))
    }
}

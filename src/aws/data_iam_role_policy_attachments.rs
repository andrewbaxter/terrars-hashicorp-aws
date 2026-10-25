#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIamRolePolicyAttachmentsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    role_name: terrars::PrimField<std::string::String>,
}

struct DataIamRolePolicyAttachments_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamRolePolicyAttachmentsData_>,
}

#[derive(Clone)]
pub struct DataIamRolePolicyAttachments(std::rc::Rc<DataIamRolePolicyAttachments_>);

impl DataIamRolePolicyAttachments {
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

    #[doc = "Set the field `path_prefix`.\n"]
    pub fn set_path_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().path_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `attached_policies` after provisioning.\n"]
    pub fn attached_policies(&self) -> terrars::ListRef<DataIamRolePolicyAttachments_AttachedPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_prefix` after provisioning.\n"]
    pub fn path_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataIamRolePolicyAttachments {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamRolePolicyAttachments { }

impl terrars::ToListMappable for DataIamRolePolicyAttachments {
    type O = terrars::ListRef<DataIamRolePolicyAttachmentsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamRolePolicyAttachments_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_iam_role_policy_attachments".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamRolePolicyAttachments {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub role_name: terrars::PrimField<std::string::String>,
}

impl BuildDataIamRolePolicyAttachments {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamRolePolicyAttachments {
        let out = DataIamRolePolicyAttachments(std::rc::Rc::new(DataIamRolePolicyAttachments_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamRolePolicyAttachmentsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                path_prefix: core::default::Default::default(),
                role_name: self.role_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamRolePolicyAttachmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamRolePolicyAttachmentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamRolePolicyAttachmentsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `attached_policies` after provisioning.\n"]
    pub fn attached_policies(&self) -> terrars::ListRef<DataIamRolePolicyAttachments_AttachedPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_prefix` after provisioning.\n"]
    pub fn path_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIamRolePolicyAttachments_AttachedPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamRolePolicyAttachments_AttachedPolicies {
    #[doc = "Set the field `policy_arn`.\n"]
    pub fn set_policy_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_name`.\n"]
    pub fn set_policy_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamRolePolicyAttachments_AttachedPolicies {
    type O = terrars::BlockAssignable<DataIamRolePolicyAttachments_AttachedPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamRolePolicyAttachments_AttachedPolicies {}

impl BuildDataIamRolePolicyAttachments_AttachedPolicies {
    pub fn build(self) -> DataIamRolePolicyAttachments_AttachedPolicies {
        DataIamRolePolicyAttachments_AttachedPolicies {
            policy_arn: core::default::Default::default(),
            policy_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamRolePolicyAttachments_AttachedPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamRolePolicyAttachments_AttachedPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamRolePolicyAttachments_AttachedPoliciesRef {
        DataIamRolePolicyAttachments_AttachedPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamRolePolicyAttachments_AttachedPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy_arn` after provisioning.\n"]
    pub fn policy_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_name` after provisioning.\n"]
    pub fn policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_name", self.base))
    }
}

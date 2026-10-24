use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsoadminAccountAssignmentData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_arn: terrars::PrimField<std::string::String>,
    permission_set_arn: terrars::PrimField<std::string::String>,
    principal_id: terrars::PrimField<std::string::String>,
    principal_type: terrars::PrimField<std::string::String>,
    target_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct SsoadminAccountAssignment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsoadminAccountAssignmentData>,
}

#[derive(Clone)]
pub struct SsoadminAccountAssignment(std::rc::Rc<SsoadminAccountAssignment_>);

impl SsoadminAccountAssignment {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `target_type`.\n"]
    pub fn set_target_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().target_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_set_arn` after provisioning.\n"]
    pub fn permission_set_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_set_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_id` after provisioning.\n"]
    pub fn principal_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }
}

impl terrars::Referable for SsoadminAccountAssignment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsoadminAccountAssignment { }

impl terrars::ToListMappable for SsoadminAccountAssignment {
    type O = terrars::ListRef<SsoadminAccountAssignmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsoadminAccountAssignment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssoadmin_account_assignment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminAccountAssignment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub permission_set_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub principal_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub principal_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_id: terrars::PrimField<std::string::String>,
}

impl BuildSsoadminAccountAssignment {
    pub fn build(self, stack: &mut terrars::Stack) -> SsoadminAccountAssignment {
        let out = SsoadminAccountAssignment(std::rc::Rc::new(SsoadminAccountAssignment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsoadminAccountAssignmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_arn: self.instance_arn,
                permission_set_arn: self.permission_set_arn,
                principal_id: self.principal_id,
                principal_type: self.principal_type,
                target_id: self.target_id,
                target_type: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminAccountAssignmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminAccountAssignmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsoadminAccountAssignmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_set_arn` after provisioning.\n"]
    pub fn permission_set_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_set_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_id` after provisioning.\n"]
    pub fn principal_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }
}

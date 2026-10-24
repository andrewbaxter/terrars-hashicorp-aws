use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3controlMultiRegionAccessPointPolicyData {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<std::vec::Vec<S3controlMultiRegionAccessPointPolicyDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<S3controlMultiRegionAccessPointPolicyTimeoutsEl>,
    dynamic: S3controlMultiRegionAccessPointPolicyDynamic,
}

struct S3controlMultiRegionAccessPointPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3controlMultiRegionAccessPointPolicyData>,
}

#[derive(Clone)]
pub struct S3controlMultiRegionAccessPointPolicy(std::rc::Rc<S3controlMultiRegionAccessPointPolicy_>);

impl S3controlMultiRegionAccessPointPolicy {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3controlMultiRegionAccessPointPolicyDetailsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<S3controlMultiRegionAccessPointPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `established` after provisioning.\n"]
    pub fn established(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.established", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proposed` after provisioning.\n"]
    pub fn proposed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proposed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<S3controlMultiRegionAccessPointPolicyDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
        S3controlMultiRegionAccessPointPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for S3controlMultiRegionAccessPointPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3controlMultiRegionAccessPointPolicy { }

impl terrars::ToListMappable for S3controlMultiRegionAccessPointPolicy {
    type O = terrars::ListRef<S3controlMultiRegionAccessPointPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3controlMultiRegionAccessPointPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3control_multi_region_access_point_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlMultiRegionAccessPointPolicy {
    pub tf_id: std::string::String,
}

impl BuildS3controlMultiRegionAccessPointPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> S3controlMultiRegionAccessPointPolicy {
        let out = S3controlMultiRegionAccessPointPolicy(std::rc::Rc::new(S3controlMultiRegionAccessPointPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3controlMultiRegionAccessPointPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlMultiRegionAccessPointPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlMultiRegionAccessPointPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3controlMultiRegionAccessPointPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `established` after provisioning.\n"]
    pub fn established(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.established", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proposed` after provisioning.\n"]
    pub fn proposed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proposed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<S3controlMultiRegionAccessPointPolicyDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
        S3controlMultiRegionAccessPointPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointPolicyDetailsEl {
    name: terrars::PrimField<std::string::String>,
    policy: terrars::PrimField<std::string::String>,
}

impl S3controlMultiRegionAccessPointPolicyDetailsEl { }

impl terrars::ToListMappable for S3controlMultiRegionAccessPointPolicyDetailsEl {
    type O = terrars::BlockAssignable<S3controlMultiRegionAccessPointPolicyDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlMultiRegionAccessPointPolicyDetailsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub policy: terrars::PrimField<std::string::String>,
}

impl BuildS3controlMultiRegionAccessPointPolicyDetailsEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointPolicyDetailsEl {
        S3controlMultiRegionAccessPointPolicyDetailsEl {
            name: self.name,
            policy: self.policy,
        }
    }
}

pub struct S3controlMultiRegionAccessPointPolicyDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlMultiRegionAccessPointPolicyDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlMultiRegionAccessPointPolicyDetailsElRef {
        S3controlMultiRegionAccessPointPolicyDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlMultiRegionAccessPointPolicyDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlMultiRegionAccessPointPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3controlMultiRegionAccessPointPolicyTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3controlMultiRegionAccessPointPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<S3controlMultiRegionAccessPointPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlMultiRegionAccessPointPolicyTimeoutsEl {}

impl BuildS3controlMultiRegionAccessPointPolicyTimeoutsEl {
    pub fn build(self) -> S3controlMultiRegionAccessPointPolicyTimeoutsEl {
        S3controlMultiRegionAccessPointPolicyTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
        S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlMultiRegionAccessPointPolicyTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlMultiRegionAccessPointPolicyDynamic {
    details: std::option::Option<terrars::DynamicBlock<S3controlMultiRegionAccessPointPolicyDetailsEl>>,
}

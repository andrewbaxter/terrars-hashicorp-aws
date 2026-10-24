use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsoadminCustomerManagedPolicyAttachmentData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_policy_reference: std
    ::option
    ::Option<std::vec::Vec<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl>>,
    dynamic: SsoadminCustomerManagedPolicyAttachmentDynamic,
}

struct SsoadminCustomerManagedPolicyAttachment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsoadminCustomerManagedPolicyAttachmentData>,
}

#[derive(Clone)]
pub struct SsoadminCustomerManagedPolicyAttachment(std::rc::Rc<SsoadminCustomerManagedPolicyAttachment_>);

impl SsoadminCustomerManagedPolicyAttachment {
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

    #[doc = "Set the field `customer_managed_policy_reference`.\n"]
    pub fn set_customer_managed_policy_reference(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().customer_managed_policy_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.customer_managed_policy_reference = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `customer_managed_policy_reference` after provisioning.\n"]
    pub fn customer_managed_policy_reference(
        &self,
    ) -> terrars::ListRef<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_policy_reference", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SsoadminCustomerManagedPolicyAttachment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsoadminCustomerManagedPolicyAttachment { }

impl terrars::ToListMappable for SsoadminCustomerManagedPolicyAttachment {
    type O = terrars::ListRef<SsoadminCustomerManagedPolicyAttachmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsoadminCustomerManagedPolicyAttachment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssoadmin_customer_managed_policy_attachment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminCustomerManagedPolicyAttachment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub permission_set_arn: terrars::PrimField<std::string::String>,
}

impl BuildSsoadminCustomerManagedPolicyAttachment {
    pub fn build(self, stack: &mut terrars::Stack) -> SsoadminCustomerManagedPolicyAttachment {
        let out = SsoadminCustomerManagedPolicyAttachment(std::rc::Rc::new(SsoadminCustomerManagedPolicyAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsoadminCustomerManagedPolicyAttachmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_arn: self.instance_arn,
                permission_set_arn: self.permission_set_arn,
                customer_managed_policy_reference: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminCustomerManagedPolicyAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminCustomerManagedPolicyAttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsoadminCustomerManagedPolicyAttachmentRef {
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

    #[doc = "Get a reference to the value of field `customer_managed_policy_reference` after provisioning.\n"]
    pub fn customer_managed_policy_reference(
        &self,
    ) -> terrars::ListRef<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_policy_reference", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
    type O = terrars::BlockAssignable<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
    pub fn build(self) -> SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
        SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
            name: self.name,
            path: core::default::Default::default(),
        }
    }
}

pub struct SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef {
        SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminCustomerManagedPolicyAttachmentDynamic {
    customer_managed_policy_reference: std
    ::option
    ::Option<terrars::DynamicBlock<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl>>,
}

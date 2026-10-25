#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsoadminCustomerManagedPolicyAttachmentsExclusiveData_ {
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
    instance_arn: terrars::PrimField<std::string::String>,
    permission_set_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_policy_reference: std
    ::option
    ::Option<std::vec::Vec<SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts>,
    dynamic: SsoadminCustomerManagedPolicyAttachmentsExclusiveDynamic,
}

struct SsoadminCustomerManagedPolicyAttachmentsExclusive_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsoadminCustomerManagedPolicyAttachmentsExclusiveData_>,
}

#[derive(Clone)]
pub struct SsoadminCustomerManagedPolicyAttachmentsExclusive(
    std::rc::Rc<SsoadminCustomerManagedPolicyAttachmentsExclusive_>,
);

impl SsoadminCustomerManagedPolicyAttachmentsExclusive {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<
                            SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference,
                        >,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl std::convert::Into<SsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_set_arn` after provisioning.\n"]
    pub fn permission_set_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_set_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_policy_reference` after provisioning.\n"]
    pub fn customer_managed_policy_reference(
        &self,
    ) -> terrars::ListRef<SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReferenceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_policy_reference", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SsoadminCustomerManagedPolicyAttachmentsExclusive_TimeoutsRef {
        SsoadminCustomerManagedPolicyAttachmentsExclusive_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SsoadminCustomerManagedPolicyAttachmentsExclusive {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsoadminCustomerManagedPolicyAttachmentsExclusive { }

impl terrars::ToListMappable for SsoadminCustomerManagedPolicyAttachmentsExclusive {
    type O = terrars::ListRef<SsoadminCustomerManagedPolicyAttachmentsExclusiveRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsoadminCustomerManagedPolicyAttachmentsExclusive_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssoadmin_customer_managed_policy_attachments_exclusive".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminCustomerManagedPolicyAttachmentsExclusive {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub permission_set_arn: terrars::PrimField<std::string::String>,
}

impl BuildSsoadminCustomerManagedPolicyAttachmentsExclusive {
    pub fn build(self, stack: &mut terrars::Stack) -> SsoadminCustomerManagedPolicyAttachmentsExclusive {
        let out =
            SsoadminCustomerManagedPolicyAttachmentsExclusive(
                std::rc::Rc::new(SsoadminCustomerManagedPolicyAttachmentsExclusive_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(SsoadminCustomerManagedPolicyAttachmentsExclusiveData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        instance_arn: self.instance_arn,
                        permission_set_arn: self.permission_set_arn,
                        region: core::default::Default::default(),
                        customer_managed_policy_reference: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminCustomerManagedPolicyAttachmentsExclusiveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminCustomerManagedPolicyAttachmentsExclusiveRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsoadminCustomerManagedPolicyAttachmentsExclusiveRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_set_arn` after provisioning.\n"]
    pub fn permission_set_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_set_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_managed_policy_reference` after provisioning.\n"]
    pub fn customer_managed_policy_reference(
        &self,
    ) -> terrars::ListRef<SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReferenceRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_managed_policy_reference", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SsoadminCustomerManagedPolicyAttachmentsExclusive_TimeoutsRef {
        SsoadminCustomerManagedPolicyAttachmentsExclusive_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct SsoadminCustomerManagedPolicyAttachmentsExclusiveDynamic {
    customer_managed_policy_reference: std
    ::option
    ::Option<
        terrars::DynamicBlock<SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference>,
    >,
}

#[derive(Serialize)]
pub struct SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference {
    type O =
        terrars::BlockAssignable<SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference {
    pub fn build(self) -> SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference {
        SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReference {
            name: self.name,
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReferenceRef {
        SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminCustomerManagedPolicyAttachmentsExclusive_CustomerManagedPolicyReferenceRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct SsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts {
    type O = terrars::BlockAssignable<SsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts {}

impl BuildSsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts {
    pub fn build(self) -> SsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts {
        SsoadminCustomerManagedPolicyAttachmentsExclusive_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SsoadminCustomerManagedPolicyAttachmentsExclusive_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminCustomerManagedPolicyAttachmentsExclusive_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsoadminCustomerManagedPolicyAttachmentsExclusive_TimeoutsRef {
        SsoadminCustomerManagedPolicyAttachmentsExclusive_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminCustomerManagedPolicyAttachmentsExclusive_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

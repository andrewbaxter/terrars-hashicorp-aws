use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsoadminCustomerManagedPolicyAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_arn: PrimField<String>,
    permission_set_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_managed_policy_reference: Option<
        Vec<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl>,
    >,
    dynamic: SsoadminCustomerManagedPolicyAttachmentDynamic,
}

struct SsoadminCustomerManagedPolicyAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SsoadminCustomerManagedPolicyAttachmentData>,
}

#[derive(Clone)]
pub struct SsoadminCustomerManagedPolicyAttachment(Rc<SsoadminCustomerManagedPolicyAttachment_>);

impl SsoadminCustomerManagedPolicyAttachment {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `customer_managed_policy_reference`.\n"]
    pub fn set_customer_managed_policy_reference(
        self,
        v: impl Into<BlockAssignable<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().customer_managed_policy_reference = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.customer_managed_policy_reference = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `permission_set_arn` after provisioning.\n"]
    pub fn permission_set_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission_set_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_managed_policy_reference` after provisioning.\n"]
    pub fn customer_managed_policy_reference(
        &self,
    ) -> ListRef<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.customer_managed_policy_reference", self.extract_ref()))
    }
}

impl Resource for SsoadminCustomerManagedPolicyAttachment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for SsoadminCustomerManagedPolicyAttachment {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for SsoadminCustomerManagedPolicyAttachment {
    type O = ListRef<SsoadminCustomerManagedPolicyAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for SsoadminCustomerManagedPolicyAttachment_ {
    fn extract_resource_type(&self) -> String {
        "aws_ssoadmin_customer_managed_policy_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminCustomerManagedPolicyAttachment {
    pub tf_id: String,
    #[doc= ""]
    pub instance_arn: PrimField<String>,
    #[doc= ""]
    pub permission_set_arn: PrimField<String>,
}

impl BuildSsoadminCustomerManagedPolicyAttachment {
    pub fn build(self, stack: &mut Stack) -> SsoadminCustomerManagedPolicyAttachment {
        let out = SsoadminCustomerManagedPolicyAttachment(Rc::new(SsoadminCustomerManagedPolicyAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SsoadminCustomerManagedPolicyAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                instance_arn: self.instance_arn,
                permission_set_arn: self.permission_set_arn,
                customer_managed_policy_reference: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminCustomerManagedPolicyAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminCustomerManagedPolicyAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsoadminCustomerManagedPolicyAttachmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `permission_set_arn` after provisioning.\n"]
    pub fn permission_set_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission_set_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_managed_policy_reference` after provisioning.\n"]
    pub fn customer_managed_policy_reference(
        &self,
    ) -> ListRef<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.customer_managed_policy_reference", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}

impl SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}

impl ToListMappable for SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
    type O = BlockAssignable<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl {
    #[doc= ""]
    pub name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef {
        SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminCustomerManagedPolicyAttachmentDynamic {
    customer_managed_policy_reference: Option<
        DynamicBlock<SsoadminCustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceEl>,
    >,
}

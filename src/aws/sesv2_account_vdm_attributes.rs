#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Sesv2AccountVdmAttributesData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    vdm_enabled: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dashboard_attributes: std::option::Option<std::vec::Vec<Sesv2AccountVdmAttributes_DashboardAttributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardian_attributes: std::option::Option<std::vec::Vec<Sesv2AccountVdmAttributes_GuardianAttributes>>,
    dynamic: Sesv2AccountVdmAttributesDynamic,
}

struct Sesv2AccountVdmAttributes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Sesv2AccountVdmAttributesData_>,
}

#[derive(Clone)]
pub struct Sesv2AccountVdmAttributes(std::rc::Rc<Sesv2AccountVdmAttributes_>);

impl Sesv2AccountVdmAttributes {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dashboard_attributes`.\n"]
    pub fn set_dashboard_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2AccountVdmAttributes_DashboardAttributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dashboard_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dashboard_attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `guardian_attributes`.\n"]
    pub fn set_guardian_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2AccountVdmAttributes_GuardianAttributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().guardian_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.guardian_attributes = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `vdm_enabled` after provisioning.\n"]
    pub fn vdm_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vdm_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_attributes` after provisioning.\n"]
    pub fn dashboard_attributes(&self) -> terrars::ListRef<Sesv2AccountVdmAttributes_DashboardAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dashboard_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardian_attributes` after provisioning.\n"]
    pub fn guardian_attributes(&self) -> terrars::ListRef<Sesv2AccountVdmAttributes_GuardianAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardian_attributes", self.extract_ref()))
    }
}

impl terrars::Referable for Sesv2AccountVdmAttributes {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Sesv2AccountVdmAttributes { }

impl terrars::ToListMappable for Sesv2AccountVdmAttributes {
    type O = terrars::ListRef<Sesv2AccountVdmAttributesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Sesv2AccountVdmAttributes_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sesv2_account_vdm_attributes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesv2AccountVdmAttributes {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub vdm_enabled: terrars::PrimField<std::string::String>,
}

impl BuildSesv2AccountVdmAttributes {
    pub fn build(self, stack: &mut terrars::Stack) -> Sesv2AccountVdmAttributes {
        let out = Sesv2AccountVdmAttributes(std::rc::Rc::new(Sesv2AccountVdmAttributes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Sesv2AccountVdmAttributesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                vdm_enabled: self.vdm_enabled,
                dashboard_attributes: core::default::Default::default(),
                guardian_attributes: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Sesv2AccountVdmAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2AccountVdmAttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Sesv2AccountVdmAttributesRef {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vdm_enabled` after provisioning.\n"]
    pub fn vdm_enabled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vdm_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_attributes` after provisioning.\n"]
    pub fn dashboard_attributes(&self) -> terrars::ListRef<Sesv2AccountVdmAttributes_DashboardAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dashboard_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guardian_attributes` after provisioning.\n"]
    pub fn guardian_attributes(&self) -> terrars::ListRef<Sesv2AccountVdmAttributes_GuardianAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardian_attributes", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Sesv2AccountVdmAttributesDynamic {
    dashboard_attributes: std::option::Option<terrars::DynamicBlock<Sesv2AccountVdmAttributes_DashboardAttributes>>,
    guardian_attributes: std::option::Option<terrars::DynamicBlock<Sesv2AccountVdmAttributes_GuardianAttributes>>,
}

#[derive(Serialize)]
pub struct Sesv2AccountVdmAttributes_DashboardAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engagement_metrics: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Sesv2AccountVdmAttributes_DashboardAttributes {
    #[doc = "Set the field `engagement_metrics`.\n"]
    pub fn set_engagement_metrics(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.engagement_metrics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Sesv2AccountVdmAttributes_DashboardAttributes {
    type O = terrars::BlockAssignable<Sesv2AccountVdmAttributes_DashboardAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2AccountVdmAttributes_DashboardAttributes {}

impl BuildSesv2AccountVdmAttributes_DashboardAttributes {
    pub fn build(self) -> Sesv2AccountVdmAttributes_DashboardAttributes {
        Sesv2AccountVdmAttributes_DashboardAttributes { engagement_metrics: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Sesv2AccountVdmAttributes_DashboardAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2AccountVdmAttributes_DashboardAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Sesv2AccountVdmAttributes_DashboardAttributesRef {
        Sesv2AccountVdmAttributes_DashboardAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2AccountVdmAttributes_DashboardAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `engagement_metrics` after provisioning.\n"]
    pub fn engagement_metrics(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engagement_metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2AccountVdmAttributes_GuardianAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optimized_shared_delivery: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Sesv2AccountVdmAttributes_GuardianAttributes {
    #[doc = "Set the field `optimized_shared_delivery`.\n"]
    pub fn set_optimized_shared_delivery(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.optimized_shared_delivery = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Sesv2AccountVdmAttributes_GuardianAttributes {
    type O = terrars::BlockAssignable<Sesv2AccountVdmAttributes_GuardianAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2AccountVdmAttributes_GuardianAttributes {}

impl BuildSesv2AccountVdmAttributes_GuardianAttributes {
    pub fn build(self) -> Sesv2AccountVdmAttributes_GuardianAttributes {
        Sesv2AccountVdmAttributes_GuardianAttributes { optimized_shared_delivery: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Sesv2AccountVdmAttributes_GuardianAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2AccountVdmAttributes_GuardianAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Sesv2AccountVdmAttributes_GuardianAttributesRef {
        Sesv2AccountVdmAttributes_GuardianAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2AccountVdmAttributes_GuardianAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `optimized_shared_delivery` after provisioning.\n"]
    pub fn optimized_shared_delivery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optimized_shared_delivery", self.base))
    }
}

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ResiliencehubResiliencyPolicyData_ {
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
    data_location_constraint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    tier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<std::vec::Vec<ResiliencehubResiliencyPolicy_Policy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ResiliencehubResiliencyPolicy_Timeouts>,
    dynamic: ResiliencehubResiliencyPolicyDynamic,
}

struct ResiliencehubResiliencyPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ResiliencehubResiliencyPolicyData_>,
}

#[derive(Clone)]
pub struct ResiliencehubResiliencyPolicy(std::rc::Rc<ResiliencehubResiliencyPolicy_>);

impl ResiliencehubResiliencyPolicy {
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
        "Set the field `data_location_constraint`.\nSpecifies a high-level geographical location constraint for where resilience policy data can be stored."]
    pub fn set_data_location_constraint(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().data_location_constraint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description for the policy."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ResiliencehubResiliencyPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_location_constraint` after provisioning.\nSpecifies a high-level geographical location constraint for where resilience policy data can be stored."]
    pub fn data_location_constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_location_constraint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description for the policy."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `estimated_cost_tier` after provisioning.\nSpecifies the estimated cost tier of the resiliency policy."]
    pub fn estimated_cost_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_cost_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe tier for the resiliency policy, ranging from the highest severity (MissionCritical) to lowest (NonCritical)."]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::ListRef<ResiliencehubResiliencyPolicy_PolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ResiliencehubResiliencyPolicy_TimeoutsRef {
        ResiliencehubResiliencyPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ResiliencehubResiliencyPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ResiliencehubResiliencyPolicy { }

impl terrars::ToListMappable for ResiliencehubResiliencyPolicy {
    type O = terrars::ListRef<ResiliencehubResiliencyPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ResiliencehubResiliencyPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_resiliencehub_resiliency_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildResiliencehubResiliencyPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the policy."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The tier for the resiliency policy, ranging from the highest severity (MissionCritical) to lowest (NonCritical)."]
    pub tier: terrars::PrimField<std::string::String>,
}

impl BuildResiliencehubResiliencyPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ResiliencehubResiliencyPolicy {
        let out = ResiliencehubResiliencyPolicy(std::rc::Rc::new(ResiliencehubResiliencyPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ResiliencehubResiliencyPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_location_constraint: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tier: self.tier,
                policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ResiliencehubResiliencyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ResiliencehubResiliencyPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ResiliencehubResiliencyPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_location_constraint` after provisioning.\nSpecifies a high-level geographical location constraint for where resilience policy data can be stored."]
    pub fn data_location_constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_location_constraint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description for the policy."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `estimated_cost_tier` after provisioning.\nSpecifies the estimated cost tier of the resiliency policy."]
    pub fn estimated_cost_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_cost_tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe tier for the resiliency policy, ranging from the highest severity (MissionCritical) to lowest (NonCritical)."]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::ListRef<ResiliencehubResiliencyPolicy_PolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ResiliencehubResiliencyPolicy_TimeoutsRef {
        ResiliencehubResiliencyPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ResiliencehubResiliencyPolicyDynamic {
    policy: std::option::Option<terrars::DynamicBlock<ResiliencehubResiliencyPolicy_Policy>>,
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicy_Policy_Az {
    rpo: terrars::PrimField<std::string::String>,
    rto: terrars::PrimField<std::string::String>,
}

impl ResiliencehubResiliencyPolicy_Policy_Az { }

impl terrars::ToListMappable for ResiliencehubResiliencyPolicy_Policy_Az {
    type O = terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy_Az>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicy_Policy_Az {
    #[doc = "Recovery Point Objective (RPO) as a Go duration."]
    pub rpo: terrars::PrimField<std::string::String>,
    #[doc = "Recovery Time Objective (RTO) as a Go duration."]
    pub rto: terrars::PrimField<std::string::String>,
}

impl BuildResiliencehubResiliencyPolicy_Policy_Az {
    pub fn build(self) -> ResiliencehubResiliencyPolicy_Policy_Az {
        ResiliencehubResiliencyPolicy_Policy_Az {
            rpo: self.rpo,
            rto: self.rto,
        }
    }
}

#[allow(dead_code)]
pub struct ResiliencehubResiliencyPolicy_Policy_AzRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ResiliencehubResiliencyPolicy_Policy_AzRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ResiliencehubResiliencyPolicy_Policy_AzRef {
        ResiliencehubResiliencyPolicy_Policy_AzRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicy_Policy_AzRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rpo` after provisioning.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn rpo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rto` after provisioning.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn rto(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rto", self.base))
    }
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicy_Policy_Hardware {
    rpo: terrars::PrimField<std::string::String>,
    rto: terrars::PrimField<std::string::String>,
}

impl ResiliencehubResiliencyPolicy_Policy_Hardware { }

impl terrars::ToListMappable for ResiliencehubResiliencyPolicy_Policy_Hardware {
    type O = terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy_Hardware>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicy_Policy_Hardware {
    #[doc = "Recovery Point Objective (RPO) as a Go duration."]
    pub rpo: terrars::PrimField<std::string::String>,
    #[doc = "Recovery Time Objective (RTO) as a Go duration."]
    pub rto: terrars::PrimField<std::string::String>,
}

impl BuildResiliencehubResiliencyPolicy_Policy_Hardware {
    pub fn build(self) -> ResiliencehubResiliencyPolicy_Policy_Hardware {
        ResiliencehubResiliencyPolicy_Policy_Hardware {
            rpo: self.rpo,
            rto: self.rto,
        }
    }
}

#[allow(dead_code)]
pub struct ResiliencehubResiliencyPolicy_Policy_HardwareRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ResiliencehubResiliencyPolicy_Policy_HardwareRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ResiliencehubResiliencyPolicy_Policy_HardwareRef {
        ResiliencehubResiliencyPolicy_Policy_HardwareRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicy_Policy_HardwareRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rpo` after provisioning.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn rpo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rto` after provisioning.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn rto(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rto", self.base))
    }
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicy_Policy_Region {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rpo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rto: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ResiliencehubResiliencyPolicy_Policy_Region {
    #[doc = "Set the field `rpo`.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn set_rpo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rpo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rto`.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn set_rto(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rto = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ResiliencehubResiliencyPolicy_Policy_Region {
    type O = terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy_Region>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicy_Policy_Region {}

impl BuildResiliencehubResiliencyPolicy_Policy_Region {
    pub fn build(self) -> ResiliencehubResiliencyPolicy_Policy_Region {
        ResiliencehubResiliencyPolicy_Policy_Region {
            rpo: core::default::Default::default(),
            rto: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ResiliencehubResiliencyPolicy_Policy_RegionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ResiliencehubResiliencyPolicy_Policy_RegionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ResiliencehubResiliencyPolicy_Policy_RegionRef {
        ResiliencehubResiliencyPolicy_Policy_RegionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicy_Policy_RegionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rpo` after provisioning.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn rpo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rto` after provisioning.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn rto(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rto", self.base))
    }
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicy_Policy_Software {
    rpo: terrars::PrimField<std::string::String>,
    rto: terrars::PrimField<std::string::String>,
}

impl ResiliencehubResiliencyPolicy_Policy_Software { }

impl terrars::ToListMappable for ResiliencehubResiliencyPolicy_Policy_Software {
    type O = terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy_Software>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicy_Policy_Software {
    #[doc = "Recovery Point Objective (RPO) as a Go duration."]
    pub rpo: terrars::PrimField<std::string::String>,
    #[doc = "Recovery Time Objective (RTO) as a Go duration."]
    pub rto: terrars::PrimField<std::string::String>,
}

impl BuildResiliencehubResiliencyPolicy_Policy_Software {
    pub fn build(self) -> ResiliencehubResiliencyPolicy_Policy_Software {
        ResiliencehubResiliencyPolicy_Policy_Software {
            rpo: self.rpo,
            rto: self.rto,
        }
    }
}

#[allow(dead_code)]
pub struct ResiliencehubResiliencyPolicy_Policy_SoftwareRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ResiliencehubResiliencyPolicy_Policy_SoftwareRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ResiliencehubResiliencyPolicy_Policy_SoftwareRef {
        ResiliencehubResiliencyPolicy_Policy_SoftwareRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicy_Policy_SoftwareRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `rpo` after provisioning.\nRecovery Point Objective (RPO) as a Go duration."]
    pub fn rpo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rto` after provisioning.\nRecovery Time Objective (RTO) as a Go duration."]
    pub fn rto(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rto", self.base))
    }
}

#[derive(Serialize, Default)]
struct ResiliencehubResiliencyPolicy_PolicyDynamic {
    az: std::option::Option<terrars::DynamicBlock<ResiliencehubResiliencyPolicy_Policy_Az>>,
    hardware: std::option::Option<terrars::DynamicBlock<ResiliencehubResiliencyPolicy_Policy_Hardware>>,
    region: std::option::Option<terrars::DynamicBlock<ResiliencehubResiliencyPolicy_Policy_Region>>,
    software: std::option::Option<terrars::DynamicBlock<ResiliencehubResiliencyPolicy_Policy_Software>>,
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicy_Policy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    az: std::option::Option<std::vec::Vec<ResiliencehubResiliencyPolicy_Policy_Az>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hardware: std::option::Option<std::vec::Vec<ResiliencehubResiliencyPolicy_Policy_Hardware>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<std::vec::Vec<ResiliencehubResiliencyPolicy_Policy_Region>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software: std::option::Option<std::vec::Vec<ResiliencehubResiliencyPolicy_Policy_Software>>,
    dynamic: ResiliencehubResiliencyPolicy_PolicyDynamic,
}

impl ResiliencehubResiliencyPolicy_Policy {
    #[doc = "Set the field `az`.\n"]
    pub fn set_az(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy_Az>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.az = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.az = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hardware`.\n"]
    pub fn set_hardware(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy_Hardware>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hardware = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hardware = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy_Region>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.region = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.region = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `software`.\n"]
    pub fn set_software(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy_Software>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.software = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.software = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ResiliencehubResiliencyPolicy_Policy {
    type O = terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Policy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicy_Policy {}

impl BuildResiliencehubResiliencyPolicy_Policy {
    pub fn build(self) -> ResiliencehubResiliencyPolicy_Policy {
        ResiliencehubResiliencyPolicy_Policy {
            az: core::default::Default::default(),
            hardware: core::default::Default::default(),
            region: core::default::Default::default(),
            software: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ResiliencehubResiliencyPolicy_PolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ResiliencehubResiliencyPolicy_PolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ResiliencehubResiliencyPolicy_PolicyRef {
        ResiliencehubResiliencyPolicy_PolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicy_PolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `az` after provisioning.\n"]
    pub fn az(&self) -> terrars::ListRef<ResiliencehubResiliencyPolicy_Policy_AzRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.az", self.base))
    }

    #[doc = "Get a reference to the value of field `hardware` after provisioning.\n"]
    pub fn hardware(&self) -> terrars::ListRef<ResiliencehubResiliencyPolicy_Policy_HardwareRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hardware", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::ListRef<ResiliencehubResiliencyPolicy_Policy_RegionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `software` after provisioning.\n"]
    pub fn software(&self) -> terrars::ListRef<ResiliencehubResiliencyPolicy_Policy_SoftwareRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software", self.base))
    }
}

#[derive(Serialize)]
pub struct ResiliencehubResiliencyPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ResiliencehubResiliencyPolicy_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ResiliencehubResiliencyPolicy_Timeouts {
    type O = terrars::BlockAssignable<ResiliencehubResiliencyPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResiliencehubResiliencyPolicy_Timeouts {}

impl BuildResiliencehubResiliencyPolicy_Timeouts {
    pub fn build(self) -> ResiliencehubResiliencyPolicy_Timeouts {
        ResiliencehubResiliencyPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ResiliencehubResiliencyPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ResiliencehubResiliencyPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ResiliencehubResiliencyPolicy_TimeoutsRef {
        ResiliencehubResiliencyPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ResiliencehubResiliencyPolicy_TimeoutsRef {
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
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

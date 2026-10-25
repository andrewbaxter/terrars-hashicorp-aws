#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrBlockPublicAccessConfigurationData_ {
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
    block_public_security_group_rules: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permitted_public_security_group_rule_range: std
    ::option
    ::Option<std::vec::Vec<EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange>>,
    dynamic: EmrBlockPublicAccessConfigurationDynamic,
}

struct EmrBlockPublicAccessConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmrBlockPublicAccessConfigurationData_>,
}

#[derive(Clone)]
pub struct EmrBlockPublicAccessConfiguration(std::rc::Rc<EmrBlockPublicAccessConfiguration_>);

impl EmrBlockPublicAccessConfiguration {
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

    #[doc = "Set the field `permitted_public_security_group_rule_range`.\n"]
    pub fn set_permitted_public_security_group_rule_range(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permitted_public_security_group_rule_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permitted_public_security_group_rule_range =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `block_public_security_group_rules` after provisioning.\n"]
    pub fn block_public_security_group_rules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_public_security_group_rules", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `permitted_public_security_group_rule_range` after provisioning.\n"]
    pub fn permitted_public_security_group_rule_range(
        &self,
    ) -> terrars::ListRef<EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRangeRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_public_security_group_rule_range", self.extract_ref()),
        )
    }
}

impl terrars::Referable for EmrBlockPublicAccessConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmrBlockPublicAccessConfiguration { }

impl terrars::ToListMappable for EmrBlockPublicAccessConfiguration {
    type O = terrars::ListRef<EmrBlockPublicAccessConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmrBlockPublicAccessConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_emr_block_public_access_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrBlockPublicAccessConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub block_public_security_group_rules: terrars::PrimField<bool>,
}

impl BuildEmrBlockPublicAccessConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> EmrBlockPublicAccessConfiguration {
        let out = EmrBlockPublicAccessConfiguration(std::rc::Rc::new(EmrBlockPublicAccessConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmrBlockPublicAccessConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                block_public_security_group_rules: self.block_public_security_group_rules,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                permitted_public_security_group_rule_range: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrBlockPublicAccessConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrBlockPublicAccessConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmrBlockPublicAccessConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_public_security_group_rules` after provisioning.\n"]
    pub fn block_public_security_group_rules(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.block_public_security_group_rules", self.extract_ref()),
        )
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

    #[doc = "Get a reference to the value of field `permitted_public_security_group_rule_range` after provisioning.\n"]
    pub fn permitted_public_security_group_rule_range(
        &self,
    ) -> terrars::ListRef<EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRangeRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.permitted_public_security_group_rule_range", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct EmrBlockPublicAccessConfigurationDynamic {
    permitted_public_security_group_rule_range: std
    ::option
    ::Option<terrars::DynamicBlock<EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange>>,
}

#[derive(Serialize)]
pub struct EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange {
    max_range: terrars::PrimField<f64>,
    min_range: terrars::PrimField<f64>,
}

impl EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange { }

impl terrars::ToListMappable for EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange {
    type O = terrars::BlockAssignable<EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange {
    #[doc = ""]
    pub max_range: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_range: terrars::PrimField<f64>,
}

impl BuildEmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange {
    pub fn build(self) -> EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange {
        EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRange {
            max_range: self.max_range,
            min_range: self.min_range,
        }
    }
}

#[allow(dead_code)]
pub struct EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRangeRef {
        EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrBlockPublicAccessConfiguration_PermittedPublicSecurityGroupRuleRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_range` after provisioning.\n"]
    pub fn max_range(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_range", self.base))
    }

    #[doc = "Get a reference to the value of field `min_range` after provisioning.\n"]
    pub fn min_range(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_range", self.base))
    }
}

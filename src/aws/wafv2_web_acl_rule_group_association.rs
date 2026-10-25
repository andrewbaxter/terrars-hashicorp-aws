#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Wafv2WebAclRuleGroupAssociationData_ {
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
    override_action: std::option::Option<terrars::PrimField<std::string::String>>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_name: terrars::PrimField<std::string::String>,
    web_acl_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_rule_group: std::option::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_group_reference: std::option::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_RuleGroupReference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Wafv2WebAclRuleGroupAssociation_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility_config: std::option::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_VisibilityConfig>>,
    dynamic: Wafv2WebAclRuleGroupAssociationDynamic,
}

struct Wafv2WebAclRuleGroupAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Wafv2WebAclRuleGroupAssociationData_>,
}

#[derive(Clone)]
pub struct Wafv2WebAclRuleGroupAssociation(std::rc::Rc<Wafv2WebAclRuleGroupAssociation_>);

impl Wafv2WebAclRuleGroupAssociation {
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
        "Set the field `override_action`.\nOverride action for the rule group. Valid values are 'none' and 'count'. Defaults to 'none'."]
    pub fn set_override_action(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().override_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_rule_group`.\n"]
    pub fn set_managed_rule_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_rule_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_rule_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_group_reference`.\n"]
    pub fn set_rule_group_reference(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_RuleGroupReference>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_group_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_group_reference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Wafv2WebAclRuleGroupAssociation_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visibility_config`.\n"]
    pub fn set_visibility_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_VisibilityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().visibility_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.visibility_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `override_action` after provisioning.\nOverride action for the rule group. Valid values are 'none' and 'count'. Defaults to 'none'."]
    pub fn override_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nPriority of the rule within the Web ACL."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_name` after provisioning.\nName of the rule to create in the Web ACL that references the rule group."]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_acl_arn` after provisioning.\nARN of the Web ACL to associate the Rule Group with."]
    pub fn web_acl_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_acl_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_rule_group` after provisioning.\n"]
    pub fn managed_rule_group(&self) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_rule_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_group_reference` after provisioning.\n"]
    pub fn rule_group_reference(&self) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_RuleGroupReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_group_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Wafv2WebAclRuleGroupAssociation_TimeoutsRef {
        Wafv2WebAclRuleGroupAssociation_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `visibility_config` after provisioning.\n"]
    pub fn visibility_config(&self) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_VisibilityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.visibility_config", self.extract_ref()))
    }
}

impl terrars::Referable for Wafv2WebAclRuleGroupAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Wafv2WebAclRuleGroupAssociation { }

impl terrars::ToListMappable for Wafv2WebAclRuleGroupAssociation {
    type O = terrars::ListRef<Wafv2WebAclRuleGroupAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Wafv2WebAclRuleGroupAssociation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_wafv2_web_acl_rule_group_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation {
    pub tf_id: std::string::String,
    #[doc = "Priority of the rule within the Web ACL."]
    pub priority: terrars::PrimField<f64>,
    #[doc = "Name of the rule to create in the Web ACL that references the rule group."]
    pub rule_name: terrars::PrimField<std::string::String>,
    #[doc = "ARN of the Web ACL to associate the Rule Group with."]
    pub web_acl_arn: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> Wafv2WebAclRuleGroupAssociation {
        let out = Wafv2WebAclRuleGroupAssociation(std::rc::Rc::new(Wafv2WebAclRuleGroupAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Wafv2WebAclRuleGroupAssociationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                override_action: core::default::Default::default(),
                priority: self.priority,
                region: core::default::Default::default(),
                rule_name: self.rule_name,
                web_acl_arn: self.web_acl_arn,
                managed_rule_group: core::default::Default::default(),
                rule_group_reference: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                visibility_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Wafv2WebAclRuleGroupAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Wafv2WebAclRuleGroupAssociationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `override_action` after provisioning.\nOverride action for the rule group. Valid values are 'none' and 'count'. Defaults to 'none'."]
    pub fn override_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nPriority of the rule within the Web ACL."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_name` after provisioning.\nName of the rule to create in the Web ACL that references the rule group."]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `web_acl_arn` after provisioning.\nARN of the Web ACL to associate the Rule Group with."]
    pub fn web_acl_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_acl_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_rule_group` after provisioning.\n"]
    pub fn managed_rule_group(&self) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_rule_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_group_reference` after provisioning.\n"]
    pub fn rule_group_reference(&self) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_RuleGroupReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_group_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Wafv2WebAclRuleGroupAssociation_TimeoutsRef {
        Wafv2WebAclRuleGroupAssociation_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `visibility_config` after provisioning.\n"]
    pub fn visibility_config(&self) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_VisibilityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.visibility_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociationDynamic {
    managed_rule_group: std::option::Option<terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup>>,
    rule_group_reference: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_RuleGroupReference>>,
    visibility_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_VisibilityConfig>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
    identifiers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
    #[doc = "Identifiers of the address fields"]
    pub identifiers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
            identifiers: self.identifiers,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identifiers` after provisioning.\nIdentifiers of the address fields"]
    pub fn identifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifiers", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
    identifier: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
    #[doc = "Identifier of the email field"]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
            identifier: self.identifier,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\nIdentifier of the email field"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField {
    identifier: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField {
    #[doc = "Identifier of the password field"]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField {
            identifier: self.identifier,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordFieldRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordFieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\nIdentifier of the password field"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
    identifiers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
    #[doc = "Identifiers of the phone number fields"]
    pub identifiers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
            identifiers: self.identifiers,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identifiers` after provisioning.\nIdentifiers of the phone number fields"]
    pub fn identifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifiers", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField {
    identifier: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField {
    #[doc = "Identifier of the username field"]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField {
            identifier: self.identifier,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameFieldRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameFieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\nIdentifier of the username field"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionDynamic {
    address_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields,
        >,
    >,
    email_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField,
        >,
    >,
    password_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField,
        >,
    >,
    phone_number_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields,
        >,
    >,
    username_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    payload_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_fields: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_field: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_field: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number_fields: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_field: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    #[doc = "Set the field `address_fields`.\n"]
    pub fn set_address_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.address_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.address_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `email_field`.\n"]
    pub fn set_email_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.email_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.email_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `password_field`.\n"]
    pub fn set_password_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `phone_number_fields`.\n"]
    pub fn set_phone_number_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.phone_number_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.phone_number_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `username_field`.\n"]
    pub fn set_username_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.username_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.username_field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    #[doc = "Payload type for inspection, either JSON or FORM_ENCODED."]
    pub payload_type: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
            payload_type: self.payload_type,
            address_fields: core::default::Default::default(),
            email_field: core::default::Default::default(),
            password_field: core::default::Default::default(),
            phone_number_fields: core::default::Default::default(),
            username_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `payload_type` after provisioning.\nPayload type for inspection, either JSON or FORM_ENCODED."]
    pub fn payload_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_type", self.base))
    }

    #[doc = "Get a reference to the value of field `address_fields` after provisioning.\n"]
    pub fn address_fields(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `email_field` after provisioning.\n"]
    pub fn email_field(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_field", self.base))
    }

    #[doc = "Get a reference to the value of field `password_field` after provisioning.\n"]
    pub fn password_field(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordFieldRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_field", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number_fields` after provisioning.\n"]
    pub fn phone_number_fields(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_number_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `username_field` after provisioning.\n"]
    pub fn username_field(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameFieldRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_field", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
    failure_strings: terrars::SetField<terrars::PrimField<std::string::String>>,
    success_strings: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
    #[doc = "Strings that indicate a failed login or account creation attempt"]
    pub failure_strings: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "Strings that indicate a successful login or account creation attempt"]
    pub success_strings: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
            failure_strings: self.failure_strings,
            success_strings: self.success_strings,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_strings` after provisioning.\nStrings that indicate a failed login or account creation attempt"]
    pub fn failure_strings(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.failure_strings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_strings` after provisioning.\nStrings that indicate a successful login or account creation attempt"]
    pub fn success_strings(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.success_strings", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
    failure_values: terrars::SetField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    success_values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
    #[doc = "Strings that indicate a failed login or account creation attempt"]
    pub failure_values: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "Name of the HTTP header to inspect"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Strings that indicate a successful login or account creation attempt"]
    pub success_values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
            failure_values: self.failure_values,
            name: self.name,
            success_values: self.success_values,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_values` after provisioning.\nStrings that indicate a failed login or account creation attempt"]
    pub fn failure_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.failure_values", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the HTTP header to inspect"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_values` after provisioning.\nStrings that indicate a successful login or account creation attempt"]
    pub fn success_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.success_values", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
    failure_values: terrars::SetField<terrars::PrimField<std::string::String>>,
    identifier: terrars::PrimField<std::string::String>,
    success_values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
    #[doc = "Strings that indicate a failed login or account creation attempt"]
    pub failure_values: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "Identifier of the JSON field to inspect"]
    pub identifier: terrars::PrimField<std::string::String>,
    #[doc = "Strings that indicate a successful login or account creation attempt"]
    pub success_values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
            failure_values: self.failure_values,
            identifier: self.identifier,
            success_values: self.success_values,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_values` after provisioning.\nStrings that indicate a failed login or account creation attempt"]
    pub fn failure_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.failure_values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identifier` after provisioning.\nIdentifier of the JSON field to inspect"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_values` after provisioning.\nStrings that indicate a successful login or account creation attempt"]
    pub fn success_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.success_values", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
    failure_codes: terrars::SetField<terrars::PrimField<f64>>,
    success_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
    #[doc = "Status codes that indicate a failed login or account creation attempt"]
    pub failure_codes: terrars::SetField<terrars::PrimField<f64>>,
    #[doc = "Status codes that indicate a successful login or account creation attempt"]
    pub success_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
            failure_codes: self.failure_codes,
            success_codes: self.success_codes,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_codes` after provisioning.\nStatus codes that indicate a failed login or account creation attempt"]
    pub fn failure_codes(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.failure_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_codes` after provisioning.\nStatus codes that indicate a successful login or account creation attempt"]
    pub fn success_codes(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.success_codes", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionDynamic {
    body_contains: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains,
        >,
    >,
    header: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header,
        >,
    >,
    json: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json,
        >,
    >,
    status_code: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body_contains: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
    #[doc = "Set the field `body_contains`.\n"]
    pub fn set_body_contains(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.body_contains = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.body_contains = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.status_code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.status_code = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
            body_contains: core::default::Default::default(),
            header: core::default::Default::default(),
            json: core::default::Default::default(),
            status_code: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `body_contains` after provisioning.\n"]
    pub fn body_contains(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body_contains", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetDynamic {
    request_inspection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection,
        >,
    >,
    response_inspection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    creation_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_regex_in_path: std::option::Option<terrars::PrimField<bool>>,
    registration_page_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_inspection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_inspection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    #[doc = "Set the field `enable_regex_in_path`.\n"]
    pub fn set_enable_regex_in_path(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_regex_in_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_inspection`.\n"]
    pub fn set_request_inspection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_inspection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_inspection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_inspection`.\n"]
    pub fn set_response_inspection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_inspection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_inspection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    #[doc = "Path to the account creation endpoint on the protected website"]
    pub creation_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub registration_page_path: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
            creation_path: self.creation_path,
            enable_regex_in_path: core::default::Default::default(),
            registration_page_path: self.registration_page_path,
            request_inspection: core::default::Default::default(),
            response_inspection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_path` after provisioning.\nPath to the account creation endpoint on the protected website"]
    pub fn creation_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_path", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_regex_in_path` after provisioning.\n"]
    pub fn enable_regex_in_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_regex_in_path", self.base))
    }

    #[doc = "Get a reference to the value of field `registration_page_path` after provisioning.\n"]
    pub fn registration_page_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registration_page_path", self.base))
    }

    #[doc = "Get a reference to the value of field `request_inspection` after provisioning.\n"]
    pub fn request_inspection(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_inspection", self.base))
    }

    #[doc = "Get a reference to the value of field `response_inspection` after provisioning.\n"]
    pub fn response_inspection(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_inspection", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_string: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
    #[doc = "Set the field `regex_string`.\n"]
    pub fn set_regex_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex_string = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
            regex_string: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex_string` after provisioning.\n"]
    pub fn regex_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex_string", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeDynamic {
    exempt_uri_regular_expression: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity: std::option::Option<terrars::PrimField<std::string::String>>,
    usage_of_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exempt_uri_regular_expression: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    #[doc = "Set the field `sensitivity`.\n"]
    pub fn set_sensitivity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exempt_uri_regular_expression`.\n"]
    pub fn set_exempt_uri_regular_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exempt_uri_regular_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exempt_uri_regular_expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    #[doc = ""]
    pub usage_of_action: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
            sensitivity: core::default::Default::default(),
            usage_of_action: self.usage_of_action,
            exempt_uri_regular_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sensitivity` after provisioning.\n"]
    pub fn sensitivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_of_action` after provisioning.\n"]
    pub fn usage_of_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_of_action", self.base))
    }

    #[doc = "Get a reference to the value of field `exempt_uri_regular_expression` after provisioning.\n"]
    pub fn exempt_uri_regular_expression(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exempt_uri_regular_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigDynamic {
    challenge: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
    #[doc = "Set the field `challenge`.\n"]
    pub fn set_challenge(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.challenge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
            challenge: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `challenge` after provisioning.\n"]
    pub fn challenge(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetDynamic {
    client_side_action_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_to_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_side_action_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
    #[doc = "Set the field `sensitivity_to_block`.\n"]
    pub fn set_sensitivity_to_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sensitivity_to_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_side_action_config`.\n"]
    pub fn set_client_side_action_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_side_action_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_side_action_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
            sensitivity_to_block: core::default::Default::default(),
            client_side_action_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sensitivity_to_block` after provisioning.\n"]
    pub fn sensitivity_to_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_to_block", self.base))
    }

    #[doc = "Get a reference to the value of field `client_side_action_config` after provisioning.\n"]
    pub fn client_side_action_config(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_side_action_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionDynamic {
    password_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField,
        >,
    >,
    username_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    payload_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_field: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_field: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    #[doc = "Set the field `password_field`.\n"]
    pub fn set_password_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `username_field`.\n"]
    pub fn set_username_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.username_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.username_field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    #[doc = "Payload type for inspection, either JSON or FORM_ENCODED."]
    pub payload_type: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
            payload_type: self.payload_type,
            password_field: core::default::Default::default(),
            username_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `payload_type` after provisioning.\nPayload type for inspection, either JSON or FORM_ENCODED."]
    pub fn payload_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_type", self.base))
    }

    #[doc = "Get a reference to the value of field `password_field` after provisioning.\n"]
    pub fn password_field(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_PasswordFieldRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_field", self.base))
    }

    #[doc = "Get a reference to the value of field `username_field` after provisioning.\n"]
    pub fn username_field(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection_UsernameFieldRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetDynamic {
    request_inspection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection,
        >,
    >,
    response_inspection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_regex_in_path: std::option::Option<terrars::PrimField<bool>>,
    login_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_inspection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_inspection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    #[doc = "Set the field `enable_regex_in_path`.\n"]
    pub fn set_enable_regex_in_path(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_regex_in_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_inspection`.\n"]
    pub fn set_request_inspection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_inspection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_inspection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_inspection`.\n"]
    pub fn set_response_inspection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_inspection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_inspection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    #[doc = ""]
    pub login_path: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
            enable_regex_in_path: core::default::Default::default(),
            login_path: self.login_path,
            request_inspection: core::default::Default::default(),
            response_inspection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_regex_in_path` after provisioning.\n"]
    pub fn enable_regex_in_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_regex_in_path", self.base))
    }

    #[doc = "Get a reference to the value of field `login_path` after provisioning.\n"]
    pub fn login_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_path", self.base))
    }

    #[doc = "Get a reference to the value of field `request_inspection` after provisioning.\n"]
    pub fn request_inspection(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_inspection", self.base))
    }

    #[doc = "Get a reference to the value of field `response_inspection` after provisioning.\n"]
    pub fn response_inspection(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_inspection", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_machine_learning: std::option::Option<terrars::PrimField<bool>>,
    inspection_level: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    #[doc = "Set the field `enable_machine_learning`.\n"]
    pub fn set_enable_machine_learning(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_machine_learning = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    #[doc = ""]
    pub inspection_level: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
            enable_machine_learning: core::default::Default::default(),
            inspection_level: self.inspection_level,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_machine_learning` after provisioning.\n"]
    pub fn enable_machine_learning(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_machine_learning", self.base))
    }

    #[doc = "Get a reference to the value of field `inspection_level` after provisioning.\n"]
    pub fn inspection_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspection_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigsDynamic {
    aws_managed_rules_acfp_rule_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet,
        >,
    >,
    aws_managed_rules_anti_ddos_rule_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet,
        >,
    >,
    aws_managed_rules_atp_rule_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet,
        >,
    >,
    aws_managed_rules_bot_control_rule_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_managed_rules_acfp_rule_set: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_managed_rules_anti_ddos_rule_set: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_managed_rules_atp_rule_set: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_managed_rules_bot_control_rule_set: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigsDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs {
    #[doc = "Set the field `aws_managed_rules_acfp_rule_set`.\n"]
    pub fn set_aws_managed_rules_acfp_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_managed_rules_acfp_rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_managed_rules_acfp_rule_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_managed_rules_anti_ddos_rule_set`.\n"]
    pub fn set_aws_managed_rules_anti_ddos_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_managed_rules_anti_ddos_rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_managed_rules_anti_ddos_rule_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_managed_rules_atp_rule_set`.\n"]
    pub fn set_aws_managed_rules_atp_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_managed_rules_atp_rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_managed_rules_atp_rule_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_managed_rules_bot_control_rule_set`.\n"]
    pub fn set_aws_managed_rules_bot_control_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_managed_rules_bot_control_rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_managed_rules_bot_control_rule_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs {
    type O = terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs {
            aws_managed_rules_acfp_rule_set: core::default::Default::default(),
            aws_managed_rules_anti_ddos_rule_set: core::default::Default::default(),
            aws_managed_rules_atp_rule_set: core::default::Default::default(),
            aws_managed_rules_bot_control_rule_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigsRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_managed_rules_acfp_rule_set` after provisioning.\n"]
    pub fn aws_managed_rules_acfp_rule_set(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_managed_rules_acfp_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_managed_rules_anti_ddos_rule_set` after provisioning.\n"]
    pub fn aws_managed_rules_anti_ddos_rule_set(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_managed_rules_anti_ddos_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_managed_rules_atp_rule_set` after provisioning.\n"]
    pub fn aws_managed_rules_atp_rule_set(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_managed_rules_atp_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_managed_rules_bot_control_rule_set` after provisioning.\n"]
    pub fn aws_managed_rules_bot_control_rule_set(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_managed_rules_bot_control_rule_set", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeaderRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingDynamic {
    insert_header: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insert_header: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling {
    #[doc = "Set the field `insert_header`.\n"]
    pub fn set_insert_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeader,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.insert_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.insert_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling {
            insert_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insert_header` after provisioning.\n"]
    pub fn insert_header(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling_InsertHeaderRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.insert_header", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_AllowDynamic {
    custom_request_handling: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_AllowDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_request_handling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow {
            custom_request_handling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_AllowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_AllowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_AllowRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_AllowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_AllowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader {

}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeaderRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponseDynamic {
    response_header: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_response_body_key: std::option::Option<terrars::PrimField<std::string::String>>,
    response_code: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponseDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse {
    #[doc = "Set the field `custom_response_body_key`.\n"]
    pub fn set_custom_response_body_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_response_body_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_header`.\n"]
    pub fn set_response_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeader,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse {
    #[doc = ""]
    pub response_code: terrars::PrimField<f64>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse {
    pub fn build(
        self,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse {
            custom_response_body_key: core::default::Default::default(),
            response_code: self.response_code,
            response_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponseRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_response_body_key` after provisioning.\n"]
    pub fn custom_response_body_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_response_body_key", self.base))
    }

    #[doc = "Get a reference to the value of field `response_code` after provisioning.\n"]
    pub fn response_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_code", self.base))
    }

    #[doc = "Get a reference to the value of field `response_header` after provisioning.\n"]
    pub fn response_header(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse_ResponseHeaderRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_header", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_BlockDynamic {
    custom_response: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_response: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse>,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_BlockDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block {
    #[doc = "Set the field `custom_response`.\n"]
    pub fn set_custom_response(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block {
            custom_response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_BlockRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_BlockRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_BlockRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_BlockRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_BlockRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_response` after provisioning.\n"]
    pub fn custom_response(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block_CustomResponseRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_response", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CaptchaDynamic {
    custom_request_handling: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CaptchaDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_request_handling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha {
            custom_request_handling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CaptchaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CaptchaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CaptchaRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CaptchaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CaptchaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_ChallengeDynamic {
    custom_request_handling: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_ChallengeDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_request_handling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge {
            custom_request_handling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_ChallengeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_ChallengeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_ChallengeRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_ChallengeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_ChallengeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CountDynamic {
    custom_request_handling: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
        >,
    >,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CountDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandling,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_request_handling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count {
            custom_request_handling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CountRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow_CustomRequestHandlingRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUseDynamic {
    allow: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow>,
    >,
    block: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block>,
    >,
    captcha: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha>,
    >,
    challenge: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge>,
    >,
    count: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    captcha: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count>>,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUseDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Allow,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `block`.\n"]
    pub fn set_block(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Block,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.block = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.block = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `captcha`.\n"]
    pub fn set_captcha(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Captcha,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.captcha = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.captcha = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `challenge`.\n"]
    pub fn set_challenge(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Challenge,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.challenge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `count`.\n"]
    pub fn set_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_Count,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.count = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse {
    type O =
        terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse {}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse {
            allow: core::default::Default::default(),
            block: core::default::Default::default(),
            captcha: core::default::Default::default(),
            challenge: core::default::Default::default(),
            count: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUseRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_AllowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `block` after provisioning.\n"]
    pub fn block(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_BlockRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block", self.base))
    }

    #[doc = "Get a reference to the value of field `captcha` after provisioning.\n"]
    pub fn captcha(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CaptchaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.captcha", self.base))
    }

    #[doc = "Get a reference to the value of field `challenge` after provisioning.\n"]
    pub fn challenge(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_ChallengeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge", self.base))
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse_CountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverrideDynamic {
    action_to_use: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_to_use: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse>>,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverrideDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride {
    #[doc = "Set the field `action_to_use`.\n"]
    pub fn set_action_to_use(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action_to_use = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action_to_use = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride {
    type O = terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride {
    #[doc = "Name of the rule to override."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride {
            name: self.name,
            action_to_use: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverrideRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the rule to override."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `action_to_use` after provisioning.\n"]
    pub fn action_to_use(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride_ActionToUseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_to_use", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroupDynamic {
    managed_rule_group_configs: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs>>,
    rule_action_override: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup {
    name: terrars::PrimField<std::string::String>,
    vendor_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_rule_group_configs: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action_override: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride>>,
    dynamic: Wafv2WebAclRuleGroupAssociation_ManagedRuleGroupDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup {
    #[doc = "Set the field `version`.\nVersion of the managed rule group. Omit this to use the default version."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_rule_group_configs`.\n"]
    pub fn set_managed_rule_group_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_rule_group_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_rule_group_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_action_override`.\n"]
    pub fn set_rule_action_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_action_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_action_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup {
    type O = terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup {
    #[doc = "Name of the managed rule group."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Name of the managed rule group vendor."]
    pub vendor_name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_ManagedRuleGroup {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup {
            name: self.name,
            vendor_name: self.vendor_name,
            version: core::default::Default::default(),
            managed_rule_group_configs: core::default::Default::default(),
            rule_action_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_ManagedRuleGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_ManagedRuleGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_ManagedRuleGroupRef {
        Wafv2WebAclRuleGroupAssociation_ManagedRuleGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_ManagedRuleGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the managed rule group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vendor_name` after provisioning.\nName of the managed rule group vendor."]
    pub fn vendor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vendor_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the managed rule group. Omit this to use the default version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_rule_group_configs` after provisioning.\n"]
    pub fn managed_rule_group_configs(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_ManagedRuleGroupConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_rule_group_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action_override` after provisioning.\n"]
    pub fn rule_action_override(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_action_override", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclRuleGroupAssociation_RuleGroupReferenceDynamic {
    rule_action_override: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_RuleGroupReference {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action_override: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride>>,
    dynamic: Wafv2WebAclRuleGroupAssociation_RuleGroupReferenceDynamic,
}

impl Wafv2WebAclRuleGroupAssociation_RuleGroupReference {
    #[doc = "Set the field `rule_action_override`.\n"]
    pub fn set_rule_action_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverride>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_action_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_action_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAclRuleGroupAssociation_RuleGroupReference {
    type O = terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_RuleGroupReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_RuleGroupReference {
    #[doc = "ARN of the Rule Group to associate with the Web ACL."]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclRuleGroupAssociation_RuleGroupReference {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_RuleGroupReference {
        Wafv2WebAclRuleGroupAssociation_RuleGroupReference {
            arn: self.arn,
            rule_action_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_RuleGroupReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_RuleGroupReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_RuleGroupReferenceRef {
        Wafv2WebAclRuleGroupAssociation_RuleGroupReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_RuleGroupReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `arn` after provisioning.\nARN of the Rule Group to associate with the Web ACL."]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action_override` after provisioning.\n"]
    pub fn rule_action_override(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclRuleGroupAssociation_ManagedRuleGroup_RuleActionOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_action_override", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAclRuleGroupAssociation_Timeouts {
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

impl terrars::ToListMappable for Wafv2WebAclRuleGroupAssociation_Timeouts {
    type O = terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_Timeouts {}

impl BuildWafv2WebAclRuleGroupAssociation_Timeouts {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_Timeouts {
        Wafv2WebAclRuleGroupAssociation_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAclRuleGroupAssociation_TimeoutsRef {
        Wafv2WebAclRuleGroupAssociation_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_TimeoutsRef {
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

#[derive(Serialize)]
pub struct Wafv2WebAclRuleGroupAssociation_VisibilityConfig {
    cloudwatch_metrics_enabled: terrars::PrimField<bool>,
    metric_name: terrars::PrimField<std::string::String>,
    sampled_requests_enabled: terrars::PrimField<bool>,
}

impl Wafv2WebAclRuleGroupAssociation_VisibilityConfig { }

impl terrars::ToListMappable for Wafv2WebAclRuleGroupAssociation_VisibilityConfig {
    type O = terrars::BlockAssignable<Wafv2WebAclRuleGroupAssociation_VisibilityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclRuleGroupAssociation_VisibilityConfig {
    #[doc = "Indicates whether the rule is available for use in the metrics for the web ACL."]
    pub cloudwatch_metrics_enabled: terrars::PrimField<bool>,
    #[doc = "A name for the metrics for this rule."]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = "Indicates whether to store a sampling of the web requests that match the rule."]
    pub sampled_requests_enabled: terrars::PrimField<bool>,
}

impl BuildWafv2WebAclRuleGroupAssociation_VisibilityConfig {
    pub fn build(self) -> Wafv2WebAclRuleGroupAssociation_VisibilityConfig {
        Wafv2WebAclRuleGroupAssociation_VisibilityConfig {
            cloudwatch_metrics_enabled: self.cloudwatch_metrics_enabled,
            metric_name: self.metric_name,
            sampled_requests_enabled: self.sampled_requests_enabled,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclRuleGroupAssociation_VisibilityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRuleGroupAssociation_VisibilityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclRuleGroupAssociation_VisibilityConfigRef {
        Wafv2WebAclRuleGroupAssociation_VisibilityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclRuleGroupAssociation_VisibilityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloudwatch_metrics_enabled` after provisioning.\nIndicates whether the rule is available for use in the metrics for the web ACL."]
    pub fn cloudwatch_metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_metrics_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metric_name` after provisioning.\nA name for the metrics for this rule."]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sampled_requests_enabled` after provisioning.\nIndicates whether to store a sampling of the web requests that match the rule."]
    pub fn sampled_requests_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampled_requests_enabled", self.base))
    }
}

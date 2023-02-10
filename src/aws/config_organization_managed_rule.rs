use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConfigOrganizationManagedRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_accounts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_parameters: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_execution_frequency: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_types_scope: Option<SetField<PrimField<String>>>,
    rule_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_key_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_value_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ConfigOrganizationManagedRuleTimeoutsEl>,
}

struct ConfigOrganizationManagedRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConfigOrganizationManagedRuleData>,
}

#[derive(Clone)]
pub struct ConfigOrganizationManagedRule(Rc<ConfigOrganizationManagedRule_>);

impl ConfigOrganizationManagedRule {
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

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `excluded_accounts`.\n"]
    pub fn set_excluded_accounts(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().excluded_accounts = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `input_parameters`.\n"]
    pub fn set_input_parameters(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().input_parameters = Some(v.into());
        self
    }

    #[doc= "Set the field `maximum_execution_frequency`.\n"]
    pub fn set_maximum_execution_frequency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maximum_execution_frequency = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_id_scope`.\n"]
    pub fn set_resource_id_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_id_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_types_scope`.\n"]
    pub fn set_resource_types_scope(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_types_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_key_scope`.\n"]
    pub fn set_tag_key_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tag_key_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `tag_value_scope`.\n"]
    pub fn set_tag_value_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tag_value_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ConfigOrganizationManagedRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `excluded_accounts` after provisioning.\n"]
    pub fn excluded_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.excluded_accounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `input_parameters` after provisioning.\n"]
    pub fn input_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_parameters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_execution_frequency` after provisioning.\n"]
    pub fn maximum_execution_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_execution_frequency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_id_scope` after provisioning.\n"]
    pub fn resource_id_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_types_scope` after provisioning.\n"]
    pub fn resource_types_scope(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_types_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_identifier` after provisioning.\n"]
    pub fn rule_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_key_scope` after provisioning.\n"]
    pub fn tag_key_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_key_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_value_scope` after provisioning.\n"]
    pub fn tag_value_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_value_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ConfigOrganizationManagedRuleTimeoutsElRef {
        ConfigOrganizationManagedRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Resource for ConfigOrganizationManagedRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for ConfigOrganizationManagedRule {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for ConfigOrganizationManagedRule {
    type O = ListRef<ConfigOrganizationManagedRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for ConfigOrganizationManagedRule_ {
    fn extract_resource_type(&self) -> String {
        "aws_config_organization_managed_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConfigOrganizationManagedRule {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub rule_identifier: PrimField<String>,
}

impl BuildConfigOrganizationManagedRule {
    pub fn build(self, stack: &mut Stack) -> ConfigOrganizationManagedRule {
        let out = ConfigOrganizationManagedRule(Rc::new(ConfigOrganizationManagedRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConfigOrganizationManagedRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                excluded_accounts: core::default::Default::default(),
                id: core::default::Default::default(),
                input_parameters: core::default::Default::default(),
                maximum_execution_frequency: core::default::Default::default(),
                name: self.name,
                resource_id_scope: core::default::Default::default(),
                resource_types_scope: core::default::Default::default(),
                rule_identifier: self.rule_identifier,
                tag_key_scope: core::default::Default::default(),
                tag_value_scope: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConfigOrganizationManagedRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConfigOrganizationManagedRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConfigOrganizationManagedRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `excluded_accounts` after provisioning.\n"]
    pub fn excluded_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.excluded_accounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `input_parameters` after provisioning.\n"]
    pub fn input_parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_parameters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_execution_frequency` after provisioning.\n"]
    pub fn maximum_execution_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_execution_frequency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_id_scope` after provisioning.\n"]
    pub fn resource_id_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_types_scope` after provisioning.\n"]
    pub fn resource_types_scope(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_types_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_identifier` after provisioning.\n"]
    pub fn rule_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_key_scope` after provisioning.\n"]
    pub fn tag_key_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_key_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag_value_scope` after provisioning.\n"]
    pub fn tag_value_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_value_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ConfigOrganizationManagedRuleTimeoutsElRef {
        ConfigOrganizationManagedRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ConfigOrganizationManagedRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ConfigOrganizationManagedRuleTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ConfigOrganizationManagedRuleTimeoutsEl {
    type O = BlockAssignable<ConfigOrganizationManagedRuleTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigOrganizationManagedRuleTimeoutsEl {}

impl BuildConfigOrganizationManagedRuleTimeoutsEl {
    pub fn build(self) -> ConfigOrganizationManagedRuleTimeoutsEl {
        ConfigOrganizationManagedRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ConfigOrganizationManagedRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConfigOrganizationManagedRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ConfigOrganizationManagedRuleTimeoutsElRef {
        ConfigOrganizationManagedRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigOrganizationManagedRuleTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

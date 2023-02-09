use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FmsPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_all_policy_resources: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_unused_fm_managed_resources: Option<PrimField<bool>>,
    exclude_resource_tags: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remediation_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_map: Option<Vec<FmsPolicyExcludeMapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_map: Option<Vec<FmsPolicyIncludeMapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_service_policy_data: Option<Vec<FmsPolicySecurityServicePolicyDataEl>>,
    dynamic: FmsPolicyDynamic,
}

struct FmsPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FmsPolicyData>,
}

#[derive(Clone)]
pub struct FmsPolicy(Rc<FmsPolicy_>);

impl FmsPolicy {
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

    #[doc= "Set the field `delete_all_policy_resources`.\n"]
    pub fn set_delete_all_policy_resources(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_all_policy_resources = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_unused_fm_managed_resources`.\n"]
    pub fn set_delete_unused_fm_managed_resources(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_unused_fm_managed_resources = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `remediation_enabled`.\n"]
    pub fn set_remediation_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remediation_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_tags = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_type`.\n"]
    pub fn set_resource_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().resource_type = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_type_list`.\n"]
    pub fn set_resource_type_list(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_type_list = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `exclude_map`.\n"]
    pub fn set_exclude_map(self, v: impl Into<BlockAssignable<FmsPolicyExcludeMapEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_map = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_map = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `include_map`.\n"]
    pub fn set_include_map(self, v: impl Into<BlockAssignable<FmsPolicyIncludeMapEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_map = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include_map = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `security_service_policy_data`.\n"]
    pub fn set_security_service_policy_data(
        self,
        v: impl Into<BlockAssignable<FmsPolicySecurityServicePolicyDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_service_policy_data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_service_policy_data = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_all_policy_resources` after provisioning.\n"]
    pub fn delete_all_policy_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_all_policy_resources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_unused_fm_managed_resources` after provisioning.\n"]
    pub fn delete_unused_fm_managed_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_unused_fm_managed_resources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclude_resource_tags` after provisioning.\n"]
    pub fn exclude_resource_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_resource_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_update_token` after provisioning.\n"]
    pub fn policy_update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_update_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remediation_enabled` after provisioning.\n"]
    pub fn remediation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remediation_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_type_list` after provisioning.\n"]
    pub fn resource_type_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_type_list", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclude_map` after provisioning.\n"]
    pub fn exclude_map(&self) -> ListRef<FmsPolicyExcludeMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclude_map", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_map` after provisioning.\n"]
    pub fn include_map(&self) -> ListRef<FmsPolicyIncludeMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.include_map", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_service_policy_data` after provisioning.\n"]
    pub fn security_service_policy_data(&self) -> ListRef<FmsPolicySecurityServicePolicyDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_service_policy_data", self.extract_ref()))
    }
}

impl Resource for FmsPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for FmsPolicy {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for FmsPolicy {
    type O = ListRef<FmsPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FmsPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_fms_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFmsPolicy {
    pub tf_id: String,
    #[doc= ""]
    pub exclude_resource_tags: PrimField<bool>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildFmsPolicy {
    pub fn build(self, stack: &mut Stack) -> FmsPolicy {
        let out = FmsPolicy(Rc::new(FmsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FmsPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                delete_all_policy_resources: core::default::Default::default(),
                delete_unused_fm_managed_resources: core::default::Default::default(),
                exclude_resource_tags: self.exclude_resource_tags,
                id: core::default::Default::default(),
                name: self.name,
                remediation_enabled: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
                resource_type: core::default::Default::default(),
                resource_type_list: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                exclude_map: core::default::Default::default(),
                include_map: core::default::Default::default(),
                security_service_policy_data: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FmsPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FmsPolicyRef {
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

    #[doc= "Get a reference to the value of field `delete_all_policy_resources` after provisioning.\n"]
    pub fn delete_all_policy_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_all_policy_resources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_unused_fm_managed_resources` after provisioning.\n"]
    pub fn delete_unused_fm_managed_resources(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_unused_fm_managed_resources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclude_resource_tags` after provisioning.\n"]
    pub fn exclude_resource_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_resource_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_update_token` after provisioning.\n"]
    pub fn policy_update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_update_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remediation_enabled` after provisioning.\n"]
    pub fn remediation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.remediation_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_type_list` after provisioning.\n"]
    pub fn resource_type_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resource_type_list", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `exclude_map` after provisioning.\n"]
    pub fn exclude_map(&self) -> ListRef<FmsPolicyExcludeMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exclude_map", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `include_map` after provisioning.\n"]
    pub fn include_map(&self) -> ListRef<FmsPolicyIncludeMapElRef> {
        ListRef::new(self.shared().clone(), format!("{}.include_map", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_service_policy_data` after provisioning.\n"]
    pub fn security_service_policy_data(&self) -> ListRef<FmsPolicySecurityServicePolicyDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_service_policy_data", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FmsPolicyExcludeMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orgunit: Option<SetField<PrimField<String>>>,
}

impl FmsPolicyExcludeMapEl {
    #[doc= "Set the field `account`.\n"]
    pub fn set_account(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.account = Some(v.into());
        self
    }

    #[doc= "Set the field `orgunit`.\n"]
    pub fn set_orgunit(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.orgunit = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicyExcludeMapEl {
    type O = BlockAssignable<FmsPolicyExcludeMapEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicyExcludeMapEl {}

impl BuildFmsPolicyExcludeMapEl {
    pub fn build(self) -> FmsPolicyExcludeMapEl {
        FmsPolicyExcludeMapEl {
            account: core::default::Default::default(),
            orgunit: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicyExcludeMapElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicyExcludeMapElRef {
    fn new(shared: StackShared, base: String) -> FmsPolicyExcludeMapElRef {
        FmsPolicyExcludeMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicyExcludeMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc= "Get a reference to the value of field `orgunit` after provisioning.\n"]
    pub fn orgunit(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.orgunit", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicyIncludeMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orgunit: Option<SetField<PrimField<String>>>,
}

impl FmsPolicyIncludeMapEl {
    #[doc= "Set the field `account`.\n"]
    pub fn set_account(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.account = Some(v.into());
        self
    }

    #[doc= "Set the field `orgunit`.\n"]
    pub fn set_orgunit(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.orgunit = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicyIncludeMapEl {
    type O = BlockAssignable<FmsPolicyIncludeMapEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicyIncludeMapEl {}

impl BuildFmsPolicyIncludeMapEl {
    pub fn build(self) -> FmsPolicyIncludeMapEl {
        FmsPolicyIncludeMapEl {
            account: core::default::Default::default(),
            orgunit: core::default::Default::default(),
        }
    }
}

pub struct FmsPolicyIncludeMapElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicyIncludeMapElRef {
    fn new(shared: StackShared, base: String) -> FmsPolicyIncludeMapElRef {
        FmsPolicyIncludeMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicyIncludeMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc= "Get a reference to the value of field `orgunit` after provisioning.\n"]
    pub fn orgunit(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.orgunit", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicySecurityServicePolicyDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_service_data: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl FmsPolicySecurityServicePolicyDataEl {
    #[doc= "Set the field `managed_service_data`.\n"]
    pub fn set_managed_service_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.managed_service_data = Some(v.into());
        self
    }
}

impl ToListMappable for FmsPolicySecurityServicePolicyDataEl {
    type O = BlockAssignable<FmsPolicySecurityServicePolicyDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicySecurityServicePolicyDataEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildFmsPolicySecurityServicePolicyDataEl {
    pub fn build(self) -> FmsPolicySecurityServicePolicyDataEl {
        FmsPolicySecurityServicePolicyDataEl {
            managed_service_data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct FmsPolicySecurityServicePolicyDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FmsPolicySecurityServicePolicyDataElRef {
    fn new(shared: StackShared, base: String) -> FmsPolicySecurityServicePolicyDataElRef {
        FmsPolicySecurityServicePolicyDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicySecurityServicePolicyDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `managed_service_data` after provisioning.\n"]
    pub fn managed_service_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_service_data", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicyDynamic {
    exclude_map: Option<DynamicBlock<FmsPolicyExcludeMapEl>>,
    include_map: Option<DynamicBlock<FmsPolicyIncludeMapEl>>,
    security_service_policy_data: Option<DynamicBlock<FmsPolicySecurityServicePolicyDataEl>>,
}

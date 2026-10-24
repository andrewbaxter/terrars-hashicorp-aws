use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LexBotAliasData {
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
    bot_name: terrars::PrimField<std::string::String>,
    bot_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_logs: std::option::Option<std::vec::Vec<LexBotAliasConversationLogsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LexBotAliasTimeoutsEl>,
    dynamic: LexBotAliasDynamic,
}

struct LexBotAlias_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LexBotAliasData>,
}

#[derive(Clone)]
pub struct LexBotAlias(std::rc::Rc<LexBotAlias_>);

impl LexBotAlias {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_logs`.\n"]
    pub fn set_conversation_logs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexBotAliasConversationLogsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conversation_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conversation_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LexBotAliasTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bot_name` after provisioning.\n"]
    pub fn bot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bot_version` after provisioning.\n"]
    pub fn bot_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `checksum` after provisioning.\n"]
    pub fn checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checksum", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conversation_logs` after provisioning.\n"]
    pub fn conversation_logs(&self) -> terrars::ListRef<LexBotAliasConversationLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexBotAliasTimeoutsElRef {
        LexBotAliasTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LexBotAlias {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LexBotAlias { }

impl terrars::ToListMappable for LexBotAlias {
    type O = terrars::ListRef<LexBotAliasRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LexBotAlias_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lex_bot_alias".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLexBotAlias {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bot_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bot_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLexBotAlias {
    pub fn build(self, stack: &mut terrars::Stack) -> LexBotAlias {
        let out = LexBotAlias(std::rc::Rc::new(LexBotAlias_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LexBotAliasData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bot_name: self.bot_name,
                bot_version: self.bot_version,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                conversation_logs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LexBotAliasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBotAliasRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LexBotAliasRef {
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

    #[doc = "Get a reference to the value of field `bot_name` after provisioning.\n"]
    pub fn bot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bot_version` after provisioning.\n"]
    pub fn bot_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bot_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `checksum` after provisioning.\n"]
    pub fn checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checksum", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conversation_logs` after provisioning.\n"]
    pub fn conversation_logs(&self) -> terrars::ListRef<LexBotAliasConversationLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LexBotAliasTimeoutsElRef {
        LexBotAliasTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LexBotAliasConversationLogsElLogSettingsEl {
    destination: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    log_type: terrars::PrimField<std::string::String>,
    resource_arn: terrars::PrimField<std::string::String>,
}

impl LexBotAliasConversationLogsElLogSettingsEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexBotAliasConversationLogsElLogSettingsEl {
    type O = terrars::BlockAssignable<LexBotAliasConversationLogsElLogSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBotAliasConversationLogsElLogSettingsEl {
    #[doc = ""]
    pub destination: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildLexBotAliasConversationLogsElLogSettingsEl {
    pub fn build(self) -> LexBotAliasConversationLogsElLogSettingsEl {
        LexBotAliasConversationLogsElLogSettingsEl {
            destination: self.destination,
            kms_key_arn: core::default::Default::default(),
            log_type: self.log_type,
            resource_arn: self.resource_arn,
        }
    }
}

pub struct LexBotAliasConversationLogsElLogSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBotAliasConversationLogsElLogSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexBotAliasConversationLogsElLogSettingsElRef {
        LexBotAliasConversationLogsElLogSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBotAliasConversationLogsElLogSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_prefix` after provisioning.\n"]
    pub fn resource_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexBotAliasConversationLogsElDynamic {
    log_settings: std::option::Option<terrars::DynamicBlock<LexBotAliasConversationLogsElLogSettingsEl>>,
}

#[derive(Serialize)]
pub struct LexBotAliasConversationLogsEl {
    iam_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_settings: std::option::Option<std::vec::Vec<LexBotAliasConversationLogsElLogSettingsEl>>,
    dynamic: LexBotAliasConversationLogsElDynamic,
}

impl LexBotAliasConversationLogsEl {
    #[doc = "Set the field `log_settings`.\n"]
    pub fn set_log_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LexBotAliasConversationLogsElLogSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LexBotAliasConversationLogsEl {
    type O = terrars::BlockAssignable<LexBotAliasConversationLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBotAliasConversationLogsEl {
    #[doc = ""]
    pub iam_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildLexBotAliasConversationLogsEl {
    pub fn build(self) -> LexBotAliasConversationLogsEl {
        LexBotAliasConversationLogsEl {
            iam_role_arn: self.iam_role_arn,
            log_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LexBotAliasConversationLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBotAliasConversationLogsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexBotAliasConversationLogsElRef {
        LexBotAliasConversationLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBotAliasConversationLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct LexBotAliasTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LexBotAliasTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LexBotAliasTimeoutsEl {
    type O = terrars::BlockAssignable<LexBotAliasTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLexBotAliasTimeoutsEl {}

impl BuildLexBotAliasTimeoutsEl {
    pub fn build(self) -> LexBotAliasTimeoutsEl {
        LexBotAliasTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct LexBotAliasTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LexBotAliasTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LexBotAliasTimeoutsElRef {
        LexBotAliasTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LexBotAliasTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct LexBotAliasDynamic {
    conversation_logs: std::option::Option<terrars::DynamicBlock<LexBotAliasConversationLogsEl>>,
}

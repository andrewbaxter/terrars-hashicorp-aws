use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AthenaWorkgroupData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<AthenaWorkgroupConfigurationEl>>,
    dynamic: AthenaWorkgroupDynamic,
}

struct AthenaWorkgroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AthenaWorkgroupData>,
}

#[derive(Clone)]
pub struct AthenaWorkgroup(std::rc::Rc<AthenaWorkgroup_>);

impl AthenaWorkgroup {
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

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AthenaWorkgroupConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<AthenaWorkgroupConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl terrars::Referable for AthenaWorkgroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AthenaWorkgroup { }

impl terrars::ToListMappable for AthenaWorkgroup {
    type O = terrars::ListRef<AthenaWorkgroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AthenaWorkgroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_athena_workgroup".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAthenaWorkgroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAthenaWorkgroup {
    pub fn build(self, stack: &mut terrars::Stack) -> AthenaWorkgroup {
        let out = AthenaWorkgroup(std::rc::Rc::new(AthenaWorkgroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AthenaWorkgroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                state: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AthenaWorkgroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AthenaWorkgroupRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<AthenaWorkgroupConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationElEngineVersionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroupConfigurationElEngineVersionEl {
    #[doc = "Set the field `selected_engine_version`.\n"]
    pub fn set_selected_engine_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.selected_engine_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroupConfigurationElEngineVersionEl {
    type O = terrars::BlockAssignable<AthenaWorkgroupConfigurationElEngineVersionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationElEngineVersionEl {}

impl BuildAthenaWorkgroupConfigurationElEngineVersionEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationElEngineVersionEl {
        AthenaWorkgroupConfigurationElEngineVersionEl { selected_engine_version: core::default::Default::default() }
    }
}

pub struct AthenaWorkgroupConfigurationElEngineVersionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroupConfigurationElEngineVersionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroupConfigurationElEngineVersionElRef {
        AthenaWorkgroupConfigurationElEngineVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElEngineVersionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effective_engine_version` after provisioning.\n"]
    pub fn effective_engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_engine_version", self.base))
    }

    #[doc = "Get a reference to the value of field `selected_engine_version` after provisioning.\n"]
    pub fn selected_engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selected_engine_version", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
    s3_acl_option: terrars::PrimField<std::string::String>,
}

impl AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl { }

impl terrars::ToListMappable for AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
    type O = terrars::BlockAssignable<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
    #[doc = ""]
    pub s3_acl_option: terrars::PrimField<std::string::String>,
}

impl BuildAthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl {
        AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl { s3_acl_option: self.s3_acl_option }
    }
}

pub struct AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
        AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_acl_option` after provisioning.\n"]
    pub fn s3_acl_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_acl_option", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
    #[doc = "Set the field `encryption_option`.\n"]
    pub fn set_encryption_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {}

impl BuildAthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
        AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl {
            encryption_option: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

pub struct AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
        AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_option", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroupConfigurationElResultConfigurationElDynamic {
    acl_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl>>,
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl>>,
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationElResultConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl>>,
    dynamic: AthenaWorkgroupConfigurationElResultConfigurationElDynamic,
}

impl AthenaWorkgroupConfigurationElResultConfigurationEl {
    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.expected_bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_location`.\n"]
    pub fn set_output_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `acl_configuration`.\n"]
    pub fn set_acl_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.acl_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.acl_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroupConfigurationElResultConfigurationEl {
    type O = terrars::BlockAssignable<AthenaWorkgroupConfigurationElResultConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationElResultConfigurationEl {}

impl BuildAthenaWorkgroupConfigurationElResultConfigurationEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationElResultConfigurationEl {
        AthenaWorkgroupConfigurationElResultConfigurationEl {
            expected_bucket_owner: core::default::Default::default(),
            output_location: core::default::Default::default(),
            acl_configuration: core::default::Default::default(),
            encryption_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AthenaWorkgroupConfigurationElResultConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroupConfigurationElResultConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroupConfigurationElResultConfigurationElRef {
        AthenaWorkgroupConfigurationElResultConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElResultConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.base))
    }

    #[doc = "Get a reference to the value of field `output_location` after provisioning.\n"]
    pub fn output_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_location", self.base))
    }

    #[doc = "Get a reference to the value of field `acl_configuration` after provisioning.\n"]
    pub fn acl_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroupConfigurationElResultConfigurationElAclConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroupConfigurationElResultConfigurationElEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroupConfigurationElDynamic {
    engine_version: std::option::Option<terrars::DynamicBlock<AthenaWorkgroupConfigurationElEngineVersionEl>>,
    result_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroupConfigurationElResultConfigurationEl>>,
}

#[derive(Serialize)]
pub struct AthenaWorkgroupConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bytes_scanned_cutoff_per_query: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_workgroup_configuration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_cloudwatch_metrics_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester_pays_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_version: std::option::Option<std::vec::Vec<AthenaWorkgroupConfigurationElEngineVersionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result_configuration: std::option::Option<std::vec::Vec<AthenaWorkgroupConfigurationElResultConfigurationEl>>,
    dynamic: AthenaWorkgroupConfigurationElDynamic,
}

impl AthenaWorkgroupConfigurationEl {
    #[doc = "Set the field `bytes_scanned_cutoff_per_query`.\n"]
    pub fn set_bytes_scanned_cutoff_per_query(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bytes_scanned_cutoff_per_query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_workgroup_configuration`.\n"]
    pub fn set_enforce_workgroup_configuration(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce_workgroup_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role`.\n"]
    pub fn set_execution_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `publish_cloudwatch_metrics_enabled`.\n"]
    pub fn set_publish_cloudwatch_metrics_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.publish_cloudwatch_metrics_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requester_pays_enabled`.\n"]
    pub fn set_requester_pays_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.requester_pays_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AthenaWorkgroupConfigurationElEngineVersionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.engine_version = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.engine_version = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `result_configuration`.\n"]
    pub fn set_result_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AthenaWorkgroupConfigurationElResultConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.result_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.result_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroupConfigurationEl {
    type O = terrars::BlockAssignable<AthenaWorkgroupConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroupConfigurationEl {}

impl BuildAthenaWorkgroupConfigurationEl {
    pub fn build(self) -> AthenaWorkgroupConfigurationEl {
        AthenaWorkgroupConfigurationEl {
            bytes_scanned_cutoff_per_query: core::default::Default::default(),
            enforce_workgroup_configuration: core::default::Default::default(),
            execution_role: core::default::Default::default(),
            publish_cloudwatch_metrics_enabled: core::default::Default::default(),
            requester_pays_enabled: core::default::Default::default(),
            engine_version: core::default::Default::default(),
            result_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AthenaWorkgroupConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroupConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AthenaWorkgroupConfigurationElRef {
        AthenaWorkgroupConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroupConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bytes_scanned_cutoff_per_query` after provisioning.\n"]
    pub fn bytes_scanned_cutoff_per_query(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bytes_scanned_cutoff_per_query", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_workgroup_configuration` after provisioning.\n"]
    pub fn enforce_workgroup_configuration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_workgroup_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_cloudwatch_metrics_enabled` after provisioning.\n"]
    pub fn publish_cloudwatch_metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_cloudwatch_metrics_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `requester_pays_enabled` after provisioning.\n"]
    pub fn requester_pays_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_pays_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::ListRef<AthenaWorkgroupConfigurationElEngineVersionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.engine_version", self.base))
    }

    #[doc = "Get a reference to the value of field `result_configuration` after provisioning.\n"]
    pub fn result_configuration(&self) -> terrars::ListRef<AthenaWorkgroupConfigurationElResultConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroupDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<AthenaWorkgroupConfigurationEl>>,
}

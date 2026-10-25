#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AthenaWorkgroupData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<AthenaWorkgroup_Configuration>>,
    dynamic: AthenaWorkgroupDynamic,
}

struct AthenaWorkgroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AthenaWorkgroupData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<AthenaWorkgroup_Configuration>>,
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn configuration(&self) -> terrars::ListRef<AthenaWorkgroup_ConfigurationRef> {
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
            data: std::cell::RefCell::new(AthenaWorkgroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn configuration(&self) -> terrars::ListRef<AthenaWorkgroup_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroupDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration>>,
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration {}

impl BuildAthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration {
        AthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration {
            kms_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_CustomerContentEncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_CustomerContentEncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_CustomerContentEncryptionConfigurationRef {
        AthenaWorkgroup_Configuration_CustomerContentEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_CustomerContentEncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_EngineVersion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroup_Configuration_EngineVersion {
    #[doc = "Set the field `selected_engine_version`.\n"]
    pub fn set_selected_engine_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.selected_engine_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_EngineVersion {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_EngineVersion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_EngineVersion {}

impl BuildAthenaWorkgroup_Configuration_EngineVersion {
    pub fn build(self) -> AthenaWorkgroup_Configuration_EngineVersion {
        AthenaWorkgroup_Configuration_EngineVersion { selected_engine_version: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_EngineVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_EngineVersionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_EngineVersionRef {
        AthenaWorkgroup_Configuration_EngineVersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_EngineVersionRef {
    #[allow(dead_code)]
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
pub struct AthenaWorkgroup_Configuration_IdentityCenterConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_identity_center: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center_instance_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroup_Configuration_IdentityCenterConfiguration {
    #[doc = "Set the field `enable_identity_center`.\n"]
    pub fn set_enable_identity_center(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_identity_center = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_center_instance_arn`.\n"]
    pub fn set_identity_center_instance_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_center_instance_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_IdentityCenterConfiguration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_IdentityCenterConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_IdentityCenterConfiguration {}

impl BuildAthenaWorkgroup_Configuration_IdentityCenterConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_IdentityCenterConfiguration {
        AthenaWorkgroup_Configuration_IdentityCenterConfiguration {
            enable_identity_center: core::default::Default::default(),
            identity_center_instance_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_IdentityCenterConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_IdentityCenterConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_IdentityCenterConfigurationRef {
        AthenaWorkgroup_Configuration_IdentityCenterConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_IdentityCenterConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_identity_center` after provisioning.\n"]
    pub fn enable_identity_center(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_identity_center", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_center_instance_arn` after provisioning.\n"]
    pub fn identity_center_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_center_instance_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration {
    type O =
        terrars
        ::BlockAssignable<AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration {}

impl BuildAthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration {
        AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration {
            kms_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfigurationRef {
        AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroup_Configuration_ManagedQueryResultsConfigurationDynamic {
    encryption_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration>>,
    dynamic: AthenaWorkgroup_Configuration_ManagedQueryResultsConfigurationDynamic,
}

impl AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
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
                            AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfiguration,
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

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration {}

impl BuildAthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration {
        AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration {
            enabled: core::default::Default::default(),
            encryption_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_ManagedQueryResultsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_ManagedQueryResultsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_ManagedQueryResultsConfigurationRef {
        AthenaWorkgroup_Configuration_ManagedQueryResultsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_ManagedQueryResultsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType {
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType { }

impl terrars
::ToListMappable for AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType {
    type O =
        terrars
        ::BlockAssignable<
            AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType {
    pub fn build(
        self,
    ) -> AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType {
        AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogTypeRef {
        AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfigurationDynamic {
    log_type: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType>,
    >,
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_type: std
    ::option
    ::Option<
        std::vec::Vec<AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType>,
    >,
    dynamic: AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfigurationDynamic,
}

impl AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_stream_name_prefix`.\n"]
    pub fn set_log_stream_name_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.log_stream_name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_type`.\n"]
    pub fn set_log_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration_LogType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration {
    type O =
        terrars
        ::BlockAssignable<AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildAthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration {
        AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
            log_stream_name_prefix: core::default::Default::default(),
            log_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfigurationRef {
        AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_name_prefix` after provisioning.\n"]
    pub fn log_stream_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration {
    type O =
        terrars::BlockAssignable<AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildAthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration {
        AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration {
            enabled: self.enabled,
            kms_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfigurationRef {
        AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_location`.\n"]
    pub fn set_log_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildAthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration {
        AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration {
            enabled: self.enabled,
            kms_key: core::default::Default::default(),
            log_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfigurationRef {
        AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc = "Get a reference to the value of field `log_location` after provisioning.\n"]
    pub fn log_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroup_Configuration_MonitoringConfigurationDynamic {
    cloud_watch_logging_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration>,
    >,
    managed_logging_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration>,
    >,
    s3_logging_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration>>,
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_watch_logging_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_logging_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_logging_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration>>,
    dynamic: AthenaWorkgroup_Configuration_MonitoringConfigurationDynamic,
}

impl AthenaWorkgroup_Configuration_MonitoringConfiguration {
    #[doc = "Set the field `cloud_watch_logging_configuration`.\n"]
    pub fn set_cloud_watch_logging_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_watch_logging_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_watch_logging_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_logging_configuration`.\n"]
    pub fn set_managed_logging_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_logging_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_logging_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_logging_configuration`.\n"]
    pub fn set_s3_logging_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_logging_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_logging_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_MonitoringConfiguration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_MonitoringConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_MonitoringConfiguration {}

impl BuildAthenaWorkgroup_Configuration_MonitoringConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_MonitoringConfiguration {
        AthenaWorkgroup_Configuration_MonitoringConfiguration {
            cloud_watch_logging_configuration: core::default::Default::default(),
            managed_logging_configuration: core::default::Default::default(),
            s3_logging_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_MonitoringConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_MonitoringConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_MonitoringConfigurationRef {
        AthenaWorkgroup_Configuration_MonitoringConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_MonitoringConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_logging_configuration` after provisioning.\n"]
    pub fn cloud_watch_logging_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_MonitoringConfiguration_CloudWatchLoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_watch_logging_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_logging_configuration` after provisioning.\n"]
    pub fn managed_logging_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_MonitoringConfiguration_ManagedLoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_logging_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_logging_configuration` after provisioning.\n"]
    pub fn s3_logging_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_MonitoringConfiguration_S3LoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_logging_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration {
    authentication_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_user_level_prefix: std::option::Option<terrars::PrimField<bool>>,
    enable_s3_access_grants: terrars::PrimField<bool>,
}

impl AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration {
    #[doc = "Set the field `create_user_level_prefix`.\n"]
    pub fn set_create_user_level_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.create_user_level_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration {
    #[doc = ""]
    pub authentication_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub enable_s3_access_grants: terrars::PrimField<bool>,
}

impl BuildAthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration {
        AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration {
            authentication_type: self.authentication_type,
            create_user_level_prefix: core::default::Default::default(),
            enable_s3_access_grants: self.enable_s3_access_grants,
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfigurationRef {
        AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc = "Get a reference to the value of field `create_user_level_prefix` after provisioning.\n"]
    pub fn create_user_level_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_user_level_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_s3_access_grants` after provisioning.\n"]
    pub fn enable_s3_access_grants(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_s3_access_grants", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration {
    s3_acl_option: terrars::PrimField<std::string::String>,
}

impl AthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration { }

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration {
    #[doc = ""]
    pub s3_acl_option: terrars::PrimField<std::string::String>,
}

impl BuildAthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration {
        AthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration { s3_acl_option: self.s3_acl_option }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_ResultConfiguration_AclConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_ResultConfiguration_AclConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_ResultConfiguration_AclConfigurationRef {
        AthenaWorkgroup_Configuration_ResultConfiguration_AclConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_ResultConfiguration_AclConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_acl_option` after provisioning.\n"]
    pub fn s3_acl_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_acl_option", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration {
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

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration {}

impl BuildAthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration {
        AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration {
            encryption_option: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfigurationRef {
        AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfigurationRef {
    #[allow(dead_code)]
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
struct AthenaWorkgroup_Configuration_ResultConfigurationDynamic {
    acl_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration>>,
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration>>,
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration_ResultConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration>>,
    dynamic: AthenaWorkgroup_Configuration_ResultConfigurationDynamic,
}

impl AthenaWorkgroup_Configuration_ResultConfiguration {
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
                        terrars::BlockAssignable<AthenaWorkgroup_Configuration_ResultConfiguration_AclConfiguration>,
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
                        ::BlockAssignable<AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfiguration>,
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

impl terrars::ToListMappable for AthenaWorkgroup_Configuration_ResultConfiguration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration_ResultConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration_ResultConfiguration {}

impl BuildAthenaWorkgroup_Configuration_ResultConfiguration {
    pub fn build(self) -> AthenaWorkgroup_Configuration_ResultConfiguration {
        AthenaWorkgroup_Configuration_ResultConfiguration {
            expected_bucket_owner: core::default::Default::default(),
            output_location: core::default::Default::default(),
            acl_configuration: core::default::Default::default(),
            encryption_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_Configuration_ResultConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_Configuration_ResultConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AthenaWorkgroup_Configuration_ResultConfigurationRef {
        AthenaWorkgroup_Configuration_ResultConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_Configuration_ResultConfigurationRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_ResultConfiguration_AclConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_ResultConfiguration_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct AthenaWorkgroup_ConfigurationDynamic {
    customer_content_encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration>>,
    engine_version: std::option::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_EngineVersion>>,
    identity_center_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_IdentityCenterConfiguration>>,
    managed_query_results_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration>>,
    monitoring_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_MonitoringConfiguration>>,
    query_results_s3_access_grants_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration>>,
    result_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AthenaWorkgroup_Configuration_ResultConfiguration>>,
}

#[derive(Serialize)]
pub struct AthenaWorkgroup_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bytes_scanned_cutoff_per_query: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_minimum_encryption_configuration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_workgroup_configuration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_cloudwatch_metrics_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester_pays_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_content_encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_version: std::option::Option<std::vec::Vec<AthenaWorkgroup_Configuration_EngineVersion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_IdentityCenterConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_query_results_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring_configuration: std::option::Option<std::vec::Vec<AthenaWorkgroup_Configuration_MonitoringConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_results_s3_access_grants_configuration: std
    ::option
    ::Option<std::vec::Vec<AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result_configuration: std::option::Option<std::vec::Vec<AthenaWorkgroup_Configuration_ResultConfiguration>>,
    dynamic: AthenaWorkgroup_ConfigurationDynamic,
}

impl AthenaWorkgroup_Configuration {
    #[doc = "Set the field `bytes_scanned_cutoff_per_query`.\n"]
    pub fn set_bytes_scanned_cutoff_per_query(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bytes_scanned_cutoff_per_query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_minimum_encryption_configuration`.\n"]
    pub fn set_enable_minimum_encryption_configuration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_minimum_encryption_configuration = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `customer_content_encryption_configuration`.\n"]
    pub fn set_customer_content_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AthenaWorkgroup_Configuration_CustomerContentEncryptionConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customer_content_encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_content_encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AthenaWorkgroup_Configuration_EngineVersion>>,
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

    #[doc = "Set the field `identity_center_configuration`.\n"]
    pub fn set_identity_center_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AthenaWorkgroup_Configuration_IdentityCenterConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity_center_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_center_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_query_results_configuration`.\n"]
    pub fn set_managed_query_results_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AthenaWorkgroup_Configuration_ManagedQueryResultsConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_query_results_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_query_results_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring_configuration`.\n"]
    pub fn set_monitoring_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AthenaWorkgroup_Configuration_MonitoringConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_results_s3_access_grants_configuration`.\n"]
    pub fn set_query_results_s3_access_grants_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_results_s3_access_grants_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_results_s3_access_grants_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `result_configuration`.\n"]
    pub fn set_result_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AthenaWorkgroup_Configuration_ResultConfiguration>>,
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

impl terrars::ToListMappable for AthenaWorkgroup_Configuration {
    type O = terrars::BlockAssignable<AthenaWorkgroup_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaWorkgroup_Configuration {}

impl BuildAthenaWorkgroup_Configuration {
    pub fn build(self) -> AthenaWorkgroup_Configuration {
        AthenaWorkgroup_Configuration {
            bytes_scanned_cutoff_per_query: core::default::Default::default(),
            enable_minimum_encryption_configuration: core::default::Default::default(),
            enforce_workgroup_configuration: core::default::Default::default(),
            execution_role: core::default::Default::default(),
            publish_cloudwatch_metrics_enabled: core::default::Default::default(),
            requester_pays_enabled: core::default::Default::default(),
            customer_content_encryption_configuration: core::default::Default::default(),
            engine_version: core::default::Default::default(),
            identity_center_configuration: core::default::Default::default(),
            managed_query_results_configuration: core::default::Default::default(),
            monitoring_configuration: core::default::Default::default(),
            query_results_s3_access_grants_configuration: core::default::Default::default(),
            result_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AthenaWorkgroup_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaWorkgroup_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AthenaWorkgroup_ConfigurationRef {
        AthenaWorkgroup_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaWorkgroup_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bytes_scanned_cutoff_per_query` after provisioning.\n"]
    pub fn bytes_scanned_cutoff_per_query(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bytes_scanned_cutoff_per_query", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_minimum_encryption_configuration` after provisioning.\n"]
    pub fn enable_minimum_encryption_configuration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_minimum_encryption_configuration", self.base),
        )
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

    #[doc = "Get a reference to the value of field `customer_content_encryption_configuration` after provisioning.\n"]
    pub fn customer_content_encryption_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_CustomerContentEncryptionConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.customer_content_encryption_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::ListRef<AthenaWorkgroup_Configuration_EngineVersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.engine_version", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_center_configuration` after provisioning.\n"]
    pub fn identity_center_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_IdentityCenterConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_query_results_configuration` after provisioning.\n"]
    pub fn managed_query_results_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_ManagedQueryResultsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_query_results_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring_configuration` after provisioning.\n"]
    pub fn monitoring_configuration(&self) -> terrars::ListRef<AthenaWorkgroup_Configuration_MonitoringConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_results_s3_access_grants_configuration` after provisioning.\n"]
    pub fn query_results_s3_access_grants_configuration(
        &self,
    ) -> terrars::ListRef<AthenaWorkgroup_Configuration_QueryResultsS3AccessGrantsConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.query_results_s3_access_grants_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `result_configuration` after provisioning.\n"]
    pub fn result_configuration(&self) -> terrars::ListRef<AthenaWorkgroup_Configuration_ResultConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result_configuration", self.base))
    }
}

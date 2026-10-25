#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConfigConfigurationRecorderData_ {
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
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recording_group: std::option::Option<std::vec::Vec<ConfigConfigurationRecorder_RecordingGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recording_mode: std::option::Option<std::vec::Vec<ConfigConfigurationRecorder_RecordingMode>>,
    dynamic: ConfigConfigurationRecorderDynamic,
}

struct ConfigConfigurationRecorder_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConfigConfigurationRecorderData_>,
}

#[derive(Clone)]
pub struct ConfigConfigurationRecorder(std::rc::Rc<ConfigConfigurationRecorder_>);

impl ConfigConfigurationRecorder {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recording_group`.\n"]
    pub fn set_recording_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recording_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recording_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recording_mode`.\n"]
    pub fn set_recording_mode(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingMode>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recording_mode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recording_mode = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording_group` after provisioning.\n"]
    pub fn recording_group(&self) -> terrars::ListRef<ConfigConfigurationRecorder_RecordingGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recording_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording_mode` after provisioning.\n"]
    pub fn recording_mode(&self) -> terrars::ListRef<ConfigConfigurationRecorder_RecordingModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recording_mode", self.extract_ref()))
    }
}

impl terrars::Referable for ConfigConfigurationRecorder {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConfigConfigurationRecorder { }

impl terrars::ToListMappable for ConfigConfigurationRecorder {
    type O = terrars::ListRef<ConfigConfigurationRecorderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConfigConfigurationRecorder_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_config_configuration_recorder".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConfigConfigurationRecorder {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildConfigConfigurationRecorder {
    pub fn build(self, stack: &mut terrars::Stack) -> ConfigConfigurationRecorder {
        let out = ConfigConfigurationRecorder(std::rc::Rc::new(ConfigConfigurationRecorder_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConfigConfigurationRecorderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                recording_group: core::default::Default::default(),
                recording_mode: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConfigConfigurationRecorderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigurationRecorderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConfigConfigurationRecorderRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording_group` after provisioning.\n"]
    pub fn recording_group(&self) -> terrars::ListRef<ConfigConfigurationRecorder_RecordingGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recording_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording_mode` after provisioning.\n"]
    pub fn recording_mode(&self) -> terrars::ListRef<ConfigConfigurationRecorder_RecordingModeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recording_mode", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ConfigConfigurationRecorderDynamic {
    recording_group: std::option::Option<terrars::DynamicBlock<ConfigConfigurationRecorder_RecordingGroup>>,
    recording_mode: std::option::Option<terrars::DynamicBlock<ConfigConfigurationRecorder_RecordingMode>>,
}

#[derive(Serialize)]
pub struct ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes {
    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes {
    type O = terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes {}

impl BuildConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes {
    pub fn build(self) -> ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes {
        ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes {
            resource_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypesRef {
        ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }
}

#[derive(Serialize)]
pub struct ConfigConfigurationRecorder_RecordingGroup_RecordingStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_only: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ConfigConfigurationRecorder_RecordingGroup_RecordingStrategy {
    #[doc = "Set the field `use_only`.\n"]
    pub fn set_use_only(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.use_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigConfigurationRecorder_RecordingGroup_RecordingStrategy {
    type O = terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingGroup_RecordingStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigurationRecorder_RecordingGroup_RecordingStrategy {}

impl BuildConfigConfigurationRecorder_RecordingGroup_RecordingStrategy {
    pub fn build(self) -> ConfigConfigurationRecorder_RecordingGroup_RecordingStrategy {
        ConfigConfigurationRecorder_RecordingGroup_RecordingStrategy { use_only: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ConfigConfigurationRecorder_RecordingGroup_RecordingStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigurationRecorder_RecordingGroup_RecordingStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConfigConfigurationRecorder_RecordingGroup_RecordingStrategyRef {
        ConfigConfigurationRecorder_RecordingGroup_RecordingStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigurationRecorder_RecordingGroup_RecordingStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `use_only` after provisioning.\n"]
    pub fn use_only(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_only", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConfigConfigurationRecorder_RecordingGroupDynamic {
    exclusion_by_resource_types: std
    ::option
    ::Option<terrars::DynamicBlock<ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes>>,
    recording_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<ConfigConfigurationRecorder_RecordingGroup_RecordingStrategy>>,
}

#[derive(Serialize)]
pub struct ConfigConfigurationRecorder_RecordingGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_supported: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_global_resource_types: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_by_resource_types: std
    ::option
    ::Option<std::vec::Vec<ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recording_strategy: std::option::Option<std::vec::Vec<ConfigConfigurationRecorder_RecordingGroup_RecordingStrategy>>,
    dynamic: ConfigConfigurationRecorder_RecordingGroupDynamic,
}

impl ConfigConfigurationRecorder_RecordingGroup {
    #[doc = "Set the field `all_supported`.\n"]
    pub fn set_all_supported(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_supported = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_global_resource_types`.\n"]
    pub fn set_include_global_resource_types(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_global_resource_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclusion_by_resource_types`.\n"]
    pub fn set_exclusion_by_resource_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypes>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_by_resource_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_by_resource_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recording_strategy`.\n"]
    pub fn set_recording_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingGroup_RecordingStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recording_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recording_strategy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ConfigConfigurationRecorder_RecordingGroup {
    type O = terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigurationRecorder_RecordingGroup {}

impl BuildConfigConfigurationRecorder_RecordingGroup {
    pub fn build(self) -> ConfigConfigurationRecorder_RecordingGroup {
        ConfigConfigurationRecorder_RecordingGroup {
            all_supported: core::default::Default::default(),
            include_global_resource_types: core::default::Default::default(),
            resource_types: core::default::Default::default(),
            exclusion_by_resource_types: core::default::Default::default(),
            recording_strategy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConfigConfigurationRecorder_RecordingGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigurationRecorder_RecordingGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConfigConfigurationRecorder_RecordingGroupRef {
        ConfigConfigurationRecorder_RecordingGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigurationRecorder_RecordingGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_supported` after provisioning.\n"]
    pub fn all_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_supported", self.base))
    }

    #[doc = "Get a reference to the value of field `include_global_resource_types` after provisioning.\n"]
    pub fn include_global_resource_types(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_global_resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusion_by_resource_types` after provisioning.\n"]
    pub fn exclusion_by_resource_types(
        &self,
    ) -> terrars::ListRef<ConfigConfigurationRecorder_RecordingGroup_ExclusionByResourceTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_by_resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `recording_strategy` after provisioning.\n"]
    pub fn recording_strategy(
        &self,
    ) -> terrars::ListRef<ConfigConfigurationRecorder_RecordingGroup_RecordingStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recording_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct ConfigConfigurationRecorder_RecordingMode_RecordingModeOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    recording_frequency: terrars::PrimField<std::string::String>,
    resource_types: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ConfigConfigurationRecorder_RecordingMode_RecordingModeOverride {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigConfigurationRecorder_RecordingMode_RecordingModeOverride {
    type O = terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingMode_RecordingModeOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigurationRecorder_RecordingMode_RecordingModeOverride {
    #[doc = ""]
    pub recording_frequency: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_types: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildConfigConfigurationRecorder_RecordingMode_RecordingModeOverride {
    pub fn build(self) -> ConfigConfigurationRecorder_RecordingMode_RecordingModeOverride {
        ConfigConfigurationRecorder_RecordingMode_RecordingModeOverride {
            description: core::default::Default::default(),
            recording_frequency: self.recording_frequency,
            resource_types: self.resource_types,
        }
    }
}

#[allow(dead_code)]
pub struct ConfigConfigurationRecorder_RecordingMode_RecordingModeOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigurationRecorder_RecordingMode_RecordingModeOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConfigConfigurationRecorder_RecordingMode_RecordingModeOverrideRef {
        ConfigConfigurationRecorder_RecordingMode_RecordingModeOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigurationRecorder_RecordingMode_RecordingModeOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `recording_frequency` after provisioning.\n"]
    pub fn recording_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recording_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConfigConfigurationRecorder_RecordingModeDynamic {
    recording_mode_override: std
    ::option
    ::Option<terrars::DynamicBlock<ConfigConfigurationRecorder_RecordingMode_RecordingModeOverride>>,
}

#[derive(Serialize)]
pub struct ConfigConfigurationRecorder_RecordingMode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recording_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recording_mode_override: std
    ::option
    ::Option<std::vec::Vec<ConfigConfigurationRecorder_RecordingMode_RecordingModeOverride>>,
    dynamic: ConfigConfigurationRecorder_RecordingModeDynamic,
}

impl ConfigConfigurationRecorder_RecordingMode {
    #[doc = "Set the field `recording_frequency`.\n"]
    pub fn set_recording_frequency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.recording_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recording_mode_override`.\n"]
    pub fn set_recording_mode_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingMode_RecordingModeOverride>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recording_mode_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recording_mode_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ConfigConfigurationRecorder_RecordingMode {
    type O = terrars::BlockAssignable<ConfigConfigurationRecorder_RecordingMode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigurationRecorder_RecordingMode {}

impl BuildConfigConfigurationRecorder_RecordingMode {
    pub fn build(self) -> ConfigConfigurationRecorder_RecordingMode {
        ConfigConfigurationRecorder_RecordingMode {
            recording_frequency: core::default::Default::default(),
            recording_mode_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ConfigConfigurationRecorder_RecordingModeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigurationRecorder_RecordingModeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConfigConfigurationRecorder_RecordingModeRef {
        ConfigConfigurationRecorder_RecordingModeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigurationRecorder_RecordingModeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recording_frequency` after provisioning.\n"]
    pub fn recording_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recording_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `recording_mode_override` after provisioning.\n"]
    pub fn recording_mode_override(
        &self,
    ) -> terrars::ListRef<ConfigConfigurationRecorder_RecordingMode_RecordingModeOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recording_mode_override", self.base))
    }
}

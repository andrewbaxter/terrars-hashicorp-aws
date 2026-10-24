use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksStackData {
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
    agent_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    berkshelf_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_manager_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_manager_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    default_instance_profile_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_os: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_root_device_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ssh_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_theme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manage_berkshelf: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
    service_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_custom_cookbooks: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_opsworks_security_groups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_cookbooks_source: std::option::Option<std::vec::Vec<OpsworksStackCustomCookbooksSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OpsworksStackTimeoutsEl>,
    dynamic: OpsworksStackDynamic,
}

struct OpsworksStack_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpsworksStackData>,
}

#[derive(Clone)]
pub struct OpsworksStack(std::rc::Rc<OpsworksStack_>);

impl OpsworksStack {
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

    #[doc = "Set the field `agent_version`.\n"]
    pub fn set_agent_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().agent_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `berkshelf_version`.\n"]
    pub fn set_berkshelf_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().berkshelf_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `color`.\n"]
    pub fn set_color(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration_manager_name`.\n"]
    pub fn set_configuration_manager_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().configuration_manager_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration_manager_version`.\n"]
    pub fn set_configuration_manager_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().configuration_manager_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_json`.\n"]
    pub fn set_custom_json(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_availability_zone`.\n"]
    pub fn set_default_availability_zone(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_os`.\n"]
    pub fn set_default_os(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_os = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_root_device_type`.\n"]
    pub fn set_default_root_device_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_root_device_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_ssh_key_name`.\n"]
    pub fn set_default_ssh_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_ssh_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_subnet_id`.\n"]
    pub fn set_default_subnet_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_subnet_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname_theme`.\n"]
    pub fn set_hostname_theme(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hostname_theme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manage_berkshelf`.\n"]
    pub fn set_manage_berkshelf(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().manage_berkshelf = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `use_custom_cookbooks`.\n"]
    pub fn set_use_custom_cookbooks(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_custom_cookbooks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_opsworks_security_groups`.\n"]
    pub fn set_use_opsworks_security_groups(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_opsworks_security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_cookbooks_source`.\n"]
    pub fn set_custom_cookbooks_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksStackCustomCookbooksSourceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_cookbooks_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_cookbooks_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OpsworksStackTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `berkshelf_version` after provisioning.\n"]
    pub fn berkshelf_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.berkshelf_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_manager_name` after provisioning.\n"]
    pub fn configuration_manager_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_manager_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_manager_version` after provisioning.\n"]
    pub fn configuration_manager_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_manager_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_json` after provisioning.\n"]
    pub fn custom_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_availability_zone` after provisioning.\n"]
    pub fn default_availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_instance_profile_arn` after provisioning.\n"]
    pub fn default_instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_instance_profile_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_os` after provisioning.\n"]
    pub fn default_os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_os", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_root_device_type` after provisioning.\n"]
    pub fn default_root_device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_root_device_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_ssh_key_name` after provisioning.\n"]
    pub fn default_ssh_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ssh_key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_subnet_id` after provisioning.\n"]
    pub fn default_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname_theme` after provisioning.\n"]
    pub fn hostname_theme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_theme", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manage_berkshelf` after provisioning.\n"]
    pub fn manage_berkshelf(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manage_berkshelf", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_endpoint` after provisioning.\n"]
    pub fn stack_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_custom_cookbooks` after provisioning.\n"]
    pub fn use_custom_cookbooks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_custom_cookbooks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_opsworks_security_groups` after provisioning.\n"]
    pub fn use_opsworks_security_groups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_opsworks_security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_cookbooks_source` after provisioning.\n"]
    pub fn custom_cookbooks_source(&self) -> terrars::ListRef<OpsworksStackCustomCookbooksSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_cookbooks_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpsworksStackTimeoutsElRef {
        OpsworksStackTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for OpsworksStack {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OpsworksStack { }

impl terrars::ToListMappable for OpsworksStack {
    type O = terrars::ListRef<OpsworksStackRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OpsworksStack_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_opsworks_stack".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksStack {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub default_instance_profile_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksStack {
    pub fn build(self, stack: &mut terrars::Stack) -> OpsworksStack {
        let out = OpsworksStack(std::rc::Rc::new(OpsworksStack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OpsworksStackData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_version: core::default::Default::default(),
                berkshelf_version: core::default::Default::default(),
                color: core::default::Default::default(),
                configuration_manager_name: core::default::Default::default(),
                configuration_manager_version: core::default::Default::default(),
                custom_json: core::default::Default::default(),
                default_availability_zone: core::default::Default::default(),
                default_instance_profile_arn: self.default_instance_profile_arn,
                default_os: core::default::Default::default(),
                default_root_device_type: core::default::Default::default(),
                default_ssh_key_name: core::default::Default::default(),
                default_subnet_id: core::default::Default::default(),
                hostname_theme: core::default::Default::default(),
                id: core::default::Default::default(),
                manage_berkshelf: core::default::Default::default(),
                name: self.name,
                region: self.region,
                service_role_arn: self.service_role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                use_custom_cookbooks: core::default::Default::default(),
                use_opsworks_security_groups: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                custom_cookbooks_source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksStackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksStackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksStackRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `berkshelf_version` after provisioning.\n"]
    pub fn berkshelf_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.berkshelf_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.color", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_manager_name` after provisioning.\n"]
    pub fn configuration_manager_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_manager_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_manager_version` after provisioning.\n"]
    pub fn configuration_manager_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_manager_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `custom_json` after provisioning.\n"]
    pub fn custom_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_availability_zone` after provisioning.\n"]
    pub fn default_availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_instance_profile_arn` after provisioning.\n"]
    pub fn default_instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_instance_profile_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_os` after provisioning.\n"]
    pub fn default_os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_os", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_root_device_type` after provisioning.\n"]
    pub fn default_root_device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_root_device_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_ssh_key_name` after provisioning.\n"]
    pub fn default_ssh_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ssh_key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_subnet_id` after provisioning.\n"]
    pub fn default_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname_theme` after provisioning.\n"]
    pub fn hostname_theme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_theme", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manage_berkshelf` after provisioning.\n"]
    pub fn manage_berkshelf(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manage_berkshelf", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_endpoint` after provisioning.\n"]
    pub fn stack_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_custom_cookbooks` after provisioning.\n"]
    pub fn use_custom_cookbooks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_custom_cookbooks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_opsworks_security_groups` after provisioning.\n"]
    pub fn use_opsworks_security_groups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_opsworks_security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_cookbooks_source` after provisioning.\n"]
    pub fn custom_cookbooks_source(&self) -> terrars::ListRef<OpsworksStackCustomCookbooksSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_cookbooks_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpsworksStackTimeoutsElRef {
        OpsworksStackTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpsworksStackCustomCookbooksSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpsworksStackCustomCookbooksSourceEl {
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_key`.\n"]
    pub fn set_ssh_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssh_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksStackCustomCookbooksSourceEl {
    type O = terrars::BlockAssignable<OpsworksStackCustomCookbooksSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksStackCustomCookbooksSourceEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksStackCustomCookbooksSourceEl {
    pub fn build(self) -> OpsworksStackCustomCookbooksSourceEl {
        OpsworksStackCustomCookbooksSourceEl {
            password: core::default::Default::default(),
            revision: core::default::Default::default(),
            ssh_key: core::default::Default::default(),
            type_: self.type_,
            url: self.url,
            username: core::default::Default::default(),
        }
    }
}

pub struct OpsworksStackCustomCookbooksSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksStackCustomCookbooksSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksStackCustomCookbooksSourceElRef {
        OpsworksStackCustomCookbooksSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksStackCustomCookbooksSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_key` after provisioning.\n"]
    pub fn ssh_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssh_key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksStackTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpsworksStackTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksStackTimeoutsEl {
    type O = terrars::BlockAssignable<OpsworksStackTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksStackTimeoutsEl {}

impl BuildOpsworksStackTimeoutsEl {
    pub fn build(self) -> OpsworksStackTimeoutsEl {
        OpsworksStackTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct OpsworksStackTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksStackTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksStackTimeoutsElRef {
        OpsworksStackTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksStackTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksStackDynamic {
    custom_cookbooks_source: std::option::Option<terrars::DynamicBlock<OpsworksStackCustomCookbooksSourceEl>>,
}

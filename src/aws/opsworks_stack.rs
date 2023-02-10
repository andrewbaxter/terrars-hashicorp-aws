use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksStackData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    berkshelf_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_manager_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_manager_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_availability_zone: Option<PrimField<String>>,
    default_instance_profile_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_os: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_root_device_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ssh_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname_theme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manage_berkshelf: Option<PrimField<bool>>,
    name: PrimField<String>,
    region: PrimField<String>,
    service_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_custom_cookbooks: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_opsworks_security_groups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_cookbooks_source: Option<Vec<OpsworksStackCustomCookbooksSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OpsworksStackTimeoutsEl>,
    dynamic: OpsworksStackDynamic,
}

struct OpsworksStack_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpsworksStackData>,
}

#[derive(Clone)]
pub struct OpsworksStack(Rc<OpsworksStack_>);

impl OpsworksStack {
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

    #[doc= "Set the field `agent_version`.\n"]
    pub fn set_agent_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().agent_version = Some(v.into());
        self
    }

    #[doc= "Set the field `berkshelf_version`.\n"]
    pub fn set_berkshelf_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().berkshelf_version = Some(v.into());
        self
    }

    #[doc= "Set the field `color`.\n"]
    pub fn set_color(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().color = Some(v.into());
        self
    }

    #[doc= "Set the field `configuration_manager_name`.\n"]
    pub fn set_configuration_manager_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().configuration_manager_name = Some(v.into());
        self
    }

    #[doc= "Set the field `configuration_manager_version`.\n"]
    pub fn set_configuration_manager_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().configuration_manager_version = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_json`.\n"]
    pub fn set_custom_json(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_json = Some(v.into());
        self
    }

    #[doc= "Set the field `default_availability_zone`.\n"]
    pub fn set_default_availability_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_availability_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `default_os`.\n"]
    pub fn set_default_os(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_os = Some(v.into());
        self
    }

    #[doc= "Set the field `default_root_device_type`.\n"]
    pub fn set_default_root_device_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_root_device_type = Some(v.into());
        self
    }

    #[doc= "Set the field `default_ssh_key_name`.\n"]
    pub fn set_default_ssh_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_ssh_key_name = Some(v.into());
        self
    }

    #[doc= "Set the field `default_subnet_id`.\n"]
    pub fn set_default_subnet_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_subnet_id = Some(v.into());
        self
    }

    #[doc= "Set the field `hostname_theme`.\n"]
    pub fn set_hostname_theme(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().hostname_theme = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `manage_berkshelf`.\n"]
    pub fn set_manage_berkshelf(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().manage_berkshelf = Some(v.into());
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

    #[doc= "Set the field `use_custom_cookbooks`.\n"]
    pub fn set_use_custom_cookbooks(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_custom_cookbooks = Some(v.into());
        self
    }

    #[doc= "Set the field `use_opsworks_security_groups`.\n"]
    pub fn set_use_opsworks_security_groups(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_opsworks_security_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_cookbooks_source`.\n"]
    pub fn set_custom_cookbooks_source(
        self,
        v: impl Into<BlockAssignable<OpsworksStackCustomCookbooksSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_cookbooks_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_cookbooks_source = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OpsworksStackTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `berkshelf_version` after provisioning.\n"]
    pub fn berkshelf_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.berkshelf_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration_manager_name` after provisioning.\n"]
    pub fn configuration_manager_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_manager_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration_manager_version` after provisioning.\n"]
    pub fn configuration_manager_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_manager_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_json` after provisioning.\n"]
    pub fn custom_json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_json", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_availability_zone` after provisioning.\n"]
    pub fn default_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_availability_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_instance_profile_arn` after provisioning.\n"]
    pub fn default_instance_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_instance_profile_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_os` after provisioning.\n"]
    pub fn default_os(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_os", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_root_device_type` after provisioning.\n"]
    pub fn default_root_device_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_root_device_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_ssh_key_name` after provisioning.\n"]
    pub fn default_ssh_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ssh_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_subnet_id` after provisioning.\n"]
    pub fn default_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_subnet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname_theme` after provisioning.\n"]
    pub fn hostname_theme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname_theme", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `manage_berkshelf` after provisioning.\n"]
    pub fn manage_berkshelf(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.manage_berkshelf", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_endpoint` after provisioning.\n"]
    pub fn stack_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_custom_cookbooks` after provisioning.\n"]
    pub fn use_custom_cookbooks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_custom_cookbooks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_opsworks_security_groups` after provisioning.\n"]
    pub fn use_opsworks_security_groups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_opsworks_security_groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_cookbooks_source` after provisioning.\n"]
    pub fn custom_cookbooks_source(&self) -> ListRef<OpsworksStackCustomCookbooksSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_cookbooks_source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpsworksStackTimeoutsElRef {
        OpsworksStackTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for OpsworksStack {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for OpsworksStack {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for OpsworksStack {
    type O = ListRef<OpsworksStackRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for OpsworksStack_ {
    fn extract_resource_type(&self) -> String {
        "aws_opsworks_stack".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksStack {
    pub tf_id: String,
    #[doc= ""]
    pub default_instance_profile_arn: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub region: PrimField<String>,
    #[doc= ""]
    pub service_role_arn: PrimField<String>,
}

impl BuildOpsworksStack {
    pub fn build(self, stack: &mut Stack) -> OpsworksStack {
        let out = OpsworksStack(Rc::new(OpsworksStack_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpsworksStackData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksStackRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksStackRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksStackRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `berkshelf_version` after provisioning.\n"]
    pub fn berkshelf_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.berkshelf_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `color` after provisioning.\n"]
    pub fn color(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.color", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration_manager_name` after provisioning.\n"]
    pub fn configuration_manager_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_manager_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration_manager_version` after provisioning.\n"]
    pub fn configuration_manager_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_manager_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_json` after provisioning.\n"]
    pub fn custom_json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_json", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_availability_zone` after provisioning.\n"]
    pub fn default_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_availability_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_instance_profile_arn` after provisioning.\n"]
    pub fn default_instance_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_instance_profile_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_os` after provisioning.\n"]
    pub fn default_os(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_os", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_root_device_type` after provisioning.\n"]
    pub fn default_root_device_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_root_device_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_ssh_key_name` after provisioning.\n"]
    pub fn default_ssh_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ssh_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_subnet_id` after provisioning.\n"]
    pub fn default_subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_subnet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname_theme` after provisioning.\n"]
    pub fn hostname_theme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname_theme", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `manage_berkshelf` after provisioning.\n"]
    pub fn manage_berkshelf(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.manage_berkshelf", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_endpoint` after provisioning.\n"]
    pub fn stack_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_custom_cookbooks` after provisioning.\n"]
    pub fn use_custom_cookbooks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_custom_cookbooks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_opsworks_security_groups` after provisioning.\n"]
    pub fn use_opsworks_security_groups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_opsworks_security_groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_cookbooks_source` after provisioning.\n"]
    pub fn custom_cookbooks_source(&self) -> ListRef<OpsworksStackCustomCookbooksSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_cookbooks_source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpsworksStackTimeoutsElRef {
        OpsworksStackTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpsworksStackCustomCookbooksSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_key: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}

impl OpsworksStackCustomCookbooksSourceEl {
    #[doc= "Set the field `password`.\n"]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }

    #[doc= "Set the field `revision`.\n"]
    pub fn set_revision(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.revision = Some(v.into());
        self
    }

    #[doc= "Set the field `ssh_key`.\n"]
    pub fn set_ssh_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssh_key = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksStackCustomCookbooksSourceEl {
    type O = BlockAssignable<OpsworksStackCustomCookbooksSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksStackCustomCookbooksSourceEl {
    #[doc= ""]
    pub type_: PrimField<String>,
    #[doc= ""]
    pub url: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksStackCustomCookbooksSourceElRef {
    fn new(shared: StackShared, base: String) -> OpsworksStackCustomCookbooksSourceElRef {
        OpsworksStackCustomCookbooksSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksStackCustomCookbooksSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc= "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }

    #[doc= "Get a reference to the value of field `ssh_key` after provisioning.\n"]
    pub fn ssh_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssh_key", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksStackTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl OpsworksStackTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksStackTimeoutsEl {
    type O = BlockAssignable<OpsworksStackTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksStackTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OpsworksStackTimeoutsElRef {
        OpsworksStackTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksStackTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksStackDynamic {
    custom_cookbooks_source: Option<DynamicBlock<OpsworksStackCustomCookbooksSourceEl>>,
}

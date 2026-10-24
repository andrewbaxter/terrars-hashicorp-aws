use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DirectoryServiceDirectoryData {
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
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_number_of_domain_controllers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_sso: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    short_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect_settings: std::option::Option<std::vec::Vec<DirectoryServiceDirectoryConnectSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DirectoryServiceDirectoryTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_settings: std::option::Option<std::vec::Vec<DirectoryServiceDirectoryVpcSettingsEl>>,
    dynamic: DirectoryServiceDirectoryDynamic,
}

struct DirectoryServiceDirectory_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DirectoryServiceDirectoryData>,
}

#[derive(Clone)]
pub struct DirectoryServiceDirectory(std::rc::Rc<DirectoryServiceDirectory_>);

impl DirectoryServiceDirectory {
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

    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_number_of_domain_controllers`.\n"]
    pub fn set_desired_number_of_domain_controllers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().desired_number_of_domain_controllers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edition`.\n"]
    pub fn set_edition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_sso`.\n"]
    pub fn set_enable_sso(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_sso = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `short_name`.\n"]
    pub fn set_short_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().short_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().size = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connect_settings`.\n"]
    pub fn set_connect_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DirectoryServiceDirectoryConnectSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connect_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connect_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DirectoryServiceDirectoryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_settings`.\n"]
    pub fn set_vpc_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DirectoryServiceDirectoryVpcSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_url` after provisioning.\n"]
    pub fn access_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_number_of_domain_controllers` after provisioning.\n"]
    pub fn desired_number_of_domain_controllers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_number_of_domain_controllers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_sso` after provisioning.\n"]
    pub fn enable_sso(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sso", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_settings` after provisioning.\n"]
    pub fn connect_settings(&self) -> terrars::ListRef<DirectoryServiceDirectoryConnectSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connect_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceDirectoryTimeoutsElRef {
        DirectoryServiceDirectoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> terrars::ListRef<DirectoryServiceDirectoryVpcSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_settings", self.extract_ref()))
    }
}

impl terrars::Referable for DirectoryServiceDirectory {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DirectoryServiceDirectory { }

impl terrars::ToListMappable for DirectoryServiceDirectory {
    type O = terrars::ListRef<DirectoryServiceDirectoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DirectoryServiceDirectory_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_directory_service_directory".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDirectoryServiceDirectory {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
}

impl BuildDirectoryServiceDirectory {
    pub fn build(self, stack: &mut terrars::Stack) -> DirectoryServiceDirectory {
        let out = DirectoryServiceDirectory(std::rc::Rc::new(DirectoryServiceDirectory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DirectoryServiceDirectoryData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alias: core::default::Default::default(),
                description: core::default::Default::default(),
                desired_number_of_domain_controllers: core::default::Default::default(),
                edition: core::default::Default::default(),
                enable_sso: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                password: self.password,
                short_name: core::default::Default::default(),
                size: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                connect_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DirectoryServiceDirectoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DirectoryServiceDirectoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DirectoryServiceDirectoryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_url` after provisioning.\n"]
    pub fn access_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_number_of_domain_controllers` after provisioning.\n"]
    pub fn desired_number_of_domain_controllers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_number_of_domain_controllers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_sso` after provisioning.\n"]
    pub fn enable_sso(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sso", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_settings` after provisioning.\n"]
    pub fn connect_settings(&self) -> terrars::ListRef<DirectoryServiceDirectoryConnectSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connect_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DirectoryServiceDirectoryTimeoutsElRef {
        DirectoryServiceDirectoryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> terrars::ListRef<DirectoryServiceDirectoryVpcSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DirectoryServiceDirectoryConnectSettingsEl {
    customer_dns_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
    customer_username: terrars::PrimField<std::string::String>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
}

impl DirectoryServiceDirectoryConnectSettingsEl { }

impl terrars::ToListMappable for DirectoryServiceDirectoryConnectSettingsEl {
    type O = terrars::BlockAssignable<DirectoryServiceDirectoryConnectSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDirectoryServiceDirectoryConnectSettingsEl {
    #[doc = ""]
    pub customer_dns_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub customer_username: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildDirectoryServiceDirectoryConnectSettingsEl {
    pub fn build(self) -> DirectoryServiceDirectoryConnectSettingsEl {
        DirectoryServiceDirectoryConnectSettingsEl {
            customer_dns_ips: self.customer_dns_ips,
            customer_username: self.customer_username,
            subnet_ids: self.subnet_ids,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct DirectoryServiceDirectoryConnectSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DirectoryServiceDirectoryConnectSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DirectoryServiceDirectoryConnectSettingsElRef {
        DirectoryServiceDirectoryConnectSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceDirectoryConnectSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `connect_ips` after provisioning.\n"]
    pub fn connect_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.connect_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_dns_ips` after provisioning.\n"]
    pub fn customer_dns_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.customer_dns_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_username` after provisioning.\n"]
    pub fn customer_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_username", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DirectoryServiceDirectoryTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DirectoryServiceDirectoryTimeoutsEl {
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

impl terrars::ToListMappable for DirectoryServiceDirectoryTimeoutsEl {
    type O = terrars::BlockAssignable<DirectoryServiceDirectoryTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDirectoryServiceDirectoryTimeoutsEl {}

impl BuildDirectoryServiceDirectoryTimeoutsEl {
    pub fn build(self) -> DirectoryServiceDirectoryTimeoutsEl {
        DirectoryServiceDirectoryTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DirectoryServiceDirectoryTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DirectoryServiceDirectoryTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DirectoryServiceDirectoryTimeoutsElRef {
        DirectoryServiceDirectoryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceDirectoryTimeoutsElRef {
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

#[derive(Serialize)]
pub struct DirectoryServiceDirectoryVpcSettingsEl {
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
}

impl DirectoryServiceDirectoryVpcSettingsEl { }

impl terrars::ToListMappable for DirectoryServiceDirectoryVpcSettingsEl {
    type O = terrars::BlockAssignable<DirectoryServiceDirectoryVpcSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDirectoryServiceDirectoryVpcSettingsEl {
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildDirectoryServiceDirectoryVpcSettingsEl {
    pub fn build(self) -> DirectoryServiceDirectoryVpcSettingsEl {
        DirectoryServiceDirectoryVpcSettingsEl {
            subnet_ids: self.subnet_ids,
            vpc_id: self.vpc_id,
        }
    }
}

pub struct DirectoryServiceDirectoryVpcSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DirectoryServiceDirectoryVpcSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DirectoryServiceDirectoryVpcSettingsElRef {
        DirectoryServiceDirectoryVpcSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DirectoryServiceDirectoryVpcSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DirectoryServiceDirectoryDynamic {
    connect_settings: std::option::Option<terrars::DynamicBlock<DirectoryServiceDirectoryConnectSettingsEl>>,
    vpc_settings: std::option::Option<terrars::DynamicBlock<DirectoryServiceDirectoryVpcSettingsEl>>,
}

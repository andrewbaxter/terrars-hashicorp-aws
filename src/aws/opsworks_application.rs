use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksApplicationData {
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
    auto_bundle_on_deploy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_flow_ruby_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_root: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rails_env: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    short_name: std::option::Option<terrars::PrimField<std::string::String>>,
    stack_id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_source: std::option::Option<std::vec::Vec<OpsworksApplicationAppSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<OpsworksApplicationEnvironmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_configuration: std::option::Option<std::vec::Vec<OpsworksApplicationSslConfigurationEl>>,
    dynamic: OpsworksApplicationDynamic,
}

struct OpsworksApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpsworksApplicationData>,
}

#[derive(Clone)]
pub struct OpsworksApplication(std::rc::Rc<OpsworksApplication_>);

impl OpsworksApplication {
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

    #[doc = "Set the field `auto_bundle_on_deploy`.\n"]
    pub fn set_auto_bundle_on_deploy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().auto_bundle_on_deploy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_flow_ruby_settings`.\n"]
    pub fn set_aws_flow_ruby_settings(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().aws_flow_ruby_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_arn`.\n"]
    pub fn set_data_source_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_source_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_database_name`.\n"]
    pub fn set_data_source_database_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().data_source_database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_type`.\n"]
    pub fn set_data_source_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_source_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document_root`.\n"]
    pub fn set_document_root(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().document_root = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domains`.\n"]
    pub fn set_domains(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ssl`.\n"]
    pub fn set_enable_ssl(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rails_env`.\n"]
    pub fn set_rails_env(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rails_env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `short_name`.\n"]
    pub fn set_short_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().short_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_source`.\n"]
    pub fn set_app_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksApplicationAppSourceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().app_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.app_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksApplicationEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssl_configuration`.\n"]
    pub fn set_ssl_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksApplicationSslConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `auto_bundle_on_deploy` after provisioning.\n"]
    pub fn auto_bundle_on_deploy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_bundle_on_deploy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_flow_ruby_settings` after provisioning.\n"]
    pub fn aws_flow_ruby_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_flow_ruby_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_database_name` after provisioning.\n"]
    pub fn data_source_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_root` after provisioning.\n"]
    pub fn document_root(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_root", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_ssl` after provisioning.\n"]
    pub fn enable_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rails_env` after provisioning.\n"]
    pub fn rails_env(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rails_env", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_source` after provisioning.\n"]
    pub fn app_source(&self) -> terrars::ListRef<OpsworksApplicationAppSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_configuration` after provisioning.\n"]
    pub fn ssl_configuration(&self) -> terrars::ListRef<OpsworksApplicationSslConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for OpsworksApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OpsworksApplication { }

impl terrars::ToListMappable for OpsworksApplication {
    type O = terrars::ListRef<OpsworksApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OpsworksApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_opsworks_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stack_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> OpsworksApplication {
        let out = OpsworksApplication(std::rc::Rc::new(OpsworksApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OpsworksApplicationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_bundle_on_deploy: core::default::Default::default(),
                aws_flow_ruby_settings: core::default::Default::default(),
                data_source_arn: core::default::Default::default(),
                data_source_database_name: core::default::Default::default(),
                data_source_type: core::default::Default::default(),
                description: core::default::Default::default(),
                document_root: core::default::Default::default(),
                domains: core::default::Default::default(),
                enable_ssl: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                rails_env: core::default::Default::default(),
                short_name: core::default::Default::default(),
                stack_id: self.stack_id,
                type_: self.type_,
                app_source: core::default::Default::default(),
                environment: core::default::Default::default(),
                ssl_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_bundle_on_deploy` after provisioning.\n"]
    pub fn auto_bundle_on_deploy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_bundle_on_deploy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_flow_ruby_settings` after provisioning.\n"]
    pub fn aws_flow_ruby_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_flow_ruby_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_database_name` after provisioning.\n"]
    pub fn data_source_database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_root` after provisioning.\n"]
    pub fn document_root(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_root", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_ssl` after provisioning.\n"]
    pub fn enable_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rails_env` after provisioning.\n"]
    pub fn rails_env(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rails_env", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_source` after provisioning.\n"]
    pub fn app_source(&self) -> terrars::ListRef<OpsworksApplicationAppSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_configuration` after provisioning.\n"]
    pub fn ssl_configuration(&self) -> terrars::ListRef<OpsworksApplicationSslConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpsworksApplicationAppSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpsworksApplicationAppSourceEl {
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

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksApplicationAppSourceEl {
    type O = terrars::BlockAssignable<OpsworksApplicationAppSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksApplicationAppSourceEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksApplicationAppSourceEl {
    pub fn build(self) -> OpsworksApplicationAppSourceEl {
        OpsworksApplicationAppSourceEl {
            password: core::default::Default::default(),
            revision: core::default::Default::default(),
            ssh_key: core::default::Default::default(),
            type_: self.type_,
            url: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

pub struct OpsworksApplicationAppSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksApplicationAppSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksApplicationAppSourceElRef {
        OpsworksApplicationAppSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksApplicationAppSourceElRef {
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
pub struct OpsworksApplicationEnvironmentEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secure: std::option::Option<terrars::PrimField<bool>>,
    value: terrars::PrimField<std::string::String>,
}

impl OpsworksApplicationEnvironmentEl {
    #[doc = "Set the field `secure`.\n"]
    pub fn set_secure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.secure = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksApplicationEnvironmentEl {
    type O = terrars::BlockAssignable<OpsworksApplicationEnvironmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksApplicationEnvironmentEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksApplicationEnvironmentEl {
    pub fn build(self) -> OpsworksApplicationEnvironmentEl {
        OpsworksApplicationEnvironmentEl {
            key: self.key,
            secure: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct OpsworksApplicationEnvironmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksApplicationEnvironmentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksApplicationEnvironmentElRef {
        OpsworksApplicationEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksApplicationEnvironmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `secure` after provisioning.\n"]
    pub fn secure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secure", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksApplicationSslConfigurationEl {
    certificate: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chain: std::option::Option<terrars::PrimField<std::string::String>>,
    private_key: terrars::PrimField<std::string::String>,
}

impl OpsworksApplicationSslConfigurationEl {
    #[doc = "Set the field `chain`.\n"]
    pub fn set_chain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.chain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksApplicationSslConfigurationEl {
    type O = terrars::BlockAssignable<OpsworksApplicationSslConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksApplicationSslConfigurationEl {
    #[doc = ""]
    pub certificate: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub private_key: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksApplicationSslConfigurationEl {
    pub fn build(self) -> OpsworksApplicationSslConfigurationEl {
        OpsworksApplicationSslConfigurationEl {
            certificate: self.certificate,
            chain: core::default::Default::default(),
            private_key: self.private_key,
        }
    }
}

pub struct OpsworksApplicationSslConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksApplicationSslConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksApplicationSslConfigurationElRef {
        OpsworksApplicationSslConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksApplicationSslConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `chain` after provisioning.\n"]
    pub fn chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksApplicationDynamic {
    app_source: std::option::Option<terrars::DynamicBlock<OpsworksApplicationAppSourceEl>>,
    environment: std::option::Option<terrars::DynamicBlock<OpsworksApplicationEnvironmentEl>>,
    ssl_configuration: std::option::Option<terrars::DynamicBlock<OpsworksApplicationSslConfigurationEl>>,
}

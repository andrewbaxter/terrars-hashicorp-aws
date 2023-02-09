use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_bundle_on_deploy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_flow_ruby_settings: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_database_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_root: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_ssl: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rails_env: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<PrimField<String>>,
    stack_id: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_source: Option<Vec<OpsworksApplicationAppSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<OpsworksApplicationEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_configuration: Option<Vec<OpsworksApplicationSslConfigurationEl>>,
    dynamic: OpsworksApplicationDynamic,
}

struct OpsworksApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpsworksApplicationData>,
}

#[derive(Clone)]
pub struct OpsworksApplication(Rc<OpsworksApplication_>);

impl OpsworksApplication {
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

    #[doc= "Set the field `auto_bundle_on_deploy`.\n"]
    pub fn set_auto_bundle_on_deploy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_bundle_on_deploy = Some(v.into());
        self
    }

    #[doc= "Set the field `aws_flow_ruby_settings`.\n"]
    pub fn set_aws_flow_ruby_settings(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_flow_ruby_settings = Some(v.into());
        self
    }

    #[doc= "Set the field `data_source_arn`.\n"]
    pub fn set_data_source_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_source_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `data_source_database_name`.\n"]
    pub fn set_data_source_database_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_source_database_name = Some(v.into());
        self
    }

    #[doc= "Set the field `data_source_type`.\n"]
    pub fn set_data_source_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_source_type = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `document_root`.\n"]
    pub fn set_document_root(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().document_root = Some(v.into());
        self
    }

    #[doc= "Set the field `domains`.\n"]
    pub fn set_domains(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().domains = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_ssl`.\n"]
    pub fn set_enable_ssl(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `rails_env`.\n"]
    pub fn set_rails_env(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().rails_env = Some(v.into());
        self
    }

    #[doc= "Set the field `short_name`.\n"]
    pub fn set_short_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().short_name = Some(v.into());
        self
    }

    #[doc= "Set the field `app_source`.\n"]
    pub fn set_app_source(self, v: impl Into<BlockAssignable<OpsworksApplicationAppSourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().app_source = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.app_source = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `environment`.\n"]
    pub fn set_environment(self, v: impl Into<BlockAssignable<OpsworksApplicationEnvironmentEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ssl_configuration`.\n"]
    pub fn set_ssl_configuration(self, v: impl Into<BlockAssignable<OpsworksApplicationSslConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `auto_bundle_on_deploy` after provisioning.\n"]
    pub fn auto_bundle_on_deploy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_bundle_on_deploy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `aws_flow_ruby_settings` after provisioning.\n"]
    pub fn aws_flow_ruby_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_flow_ruby_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source_database_name` after provisioning.\n"]
    pub fn data_source_database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_database_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `document_root` after provisioning.\n"]
    pub fn document_root(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_root", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ssl` after provisioning.\n"]
    pub fn enable_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rails_env` after provisioning.\n"]
    pub fn rails_env(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rails_env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app_source` after provisioning.\n"]
    pub fn app_source(&self) -> ListRef<OpsworksApplicationAppSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.app_source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_configuration` after provisioning.\n"]
    pub fn ssl_configuration(&self) -> ListRef<OpsworksApplicationSslConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl_configuration", self.extract_ref()))
    }
}

impl Resource for OpsworksApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for OpsworksApplication {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for OpsworksApplication {
    type O = ListRef<OpsworksApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OpsworksApplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_opsworks_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksApplication {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub stack_id: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildOpsworksApplication {
    pub fn build(self, stack: &mut Stack) -> OpsworksApplication {
        let out = OpsworksApplication(Rc::new(OpsworksApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpsworksApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_bundle_on_deploy` after provisioning.\n"]
    pub fn auto_bundle_on_deploy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_bundle_on_deploy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `aws_flow_ruby_settings` after provisioning.\n"]
    pub fn aws_flow_ruby_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_flow_ruby_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source_database_name` after provisioning.\n"]
    pub fn data_source_database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_database_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `document_root` after provisioning.\n"]
    pub fn document_root(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.document_root", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_ssl` after provisioning.\n"]
    pub fn enable_ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_ssl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rails_env` after provisioning.\n"]
    pub fn rails_env(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rails_env", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app_source` after provisioning.\n"]
    pub fn app_source(&self) -> ListRef<OpsworksApplicationAppSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.app_source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl_configuration` after provisioning.\n"]
    pub fn ssl_configuration(&self) -> ListRef<OpsworksApplicationSslConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpsworksApplicationAppSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_key: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}

impl OpsworksApplicationAppSourceEl {
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

    #[doc= "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksApplicationAppSourceEl {
    type O = BlockAssignable<OpsworksApplicationAppSourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksApplicationAppSourceEl {
    #[doc= ""]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksApplicationAppSourceElRef {
    fn new(shared: StackShared, base: String) -> OpsworksApplicationAppSourceElRef {
        OpsworksApplicationAppSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksApplicationAppSourceElRef {
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
pub struct OpsworksApplicationEnvironmentEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure: Option<PrimField<bool>>,
    value: PrimField<String>,
}

impl OpsworksApplicationEnvironmentEl {
    #[doc= "Set the field `secure`.\n"]
    pub fn set_secure(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.secure = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksApplicationEnvironmentEl {
    type O = BlockAssignable<OpsworksApplicationEnvironmentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksApplicationEnvironmentEl {
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksApplicationEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> OpsworksApplicationEnvironmentElRef {
        OpsworksApplicationEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksApplicationEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `secure` after provisioning.\n"]
    pub fn secure(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.secure", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksApplicationSslConfigurationEl {
    certificate: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chain: Option<PrimField<String>>,
    private_key: PrimField<String>,
}

impl OpsworksApplicationSslConfigurationEl {
    #[doc= "Set the field `chain`.\n"]
    pub fn set_chain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.chain = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksApplicationSslConfigurationEl {
    type O = BlockAssignable<OpsworksApplicationSslConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksApplicationSslConfigurationEl {
    #[doc= ""]
    pub certificate: PrimField<String>,
    #[doc= ""]
    pub private_key: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksApplicationSslConfigurationElRef {
    fn new(shared: StackShared, base: String) -> OpsworksApplicationSslConfigurationElRef {
        OpsworksApplicationSslConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksApplicationSslConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc= "Get a reference to the value of field `chain` after provisioning.\n"]
    pub fn chain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.chain", self.base))
    }

    #[doc= "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksApplicationDynamic {
    app_source: Option<DynamicBlock<OpsworksApplicationAppSourceEl>>,
    environment: Option<DynamicBlock<OpsworksApplicationEnvironmentEl>>,
    ssl_configuration: Option<DynamicBlock<OpsworksApplicationSslConfigurationEl>>,
}

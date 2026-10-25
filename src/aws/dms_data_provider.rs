#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DmsDataProviderData_ {
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
    engine: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "virtual", skip_serializing_if = "std::option::Option::is_none")]
    virtual_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings>>,
    dynamic: DmsDataProviderDynamic,
}

struct DmsDataProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DmsDataProviderData_>,
}

#[derive(Clone)]
pub struct DmsDataProvider(std::rc::Rc<DmsDataProvider_>);

impl DmsDataProvider {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_`.\n"]
    pub fn set_virtual(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().virtual_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_` after provisioning.\n"]
    pub fn virtual_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DmsDataProvider_SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl terrars::Referable for DmsDataProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DmsDataProvider { }

impl terrars::ToListMappable for DmsDataProvider {
    type O = terrars::ListRef<DmsDataProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DmsDataProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dms_data_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDmsDataProvider {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub engine: terrars::PrimField<std::string::String>,
}

impl BuildDmsDataProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> DmsDataProvider {
        let out = DmsDataProvider(std::rc::Rc::new(DmsDataProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DmsDataProviderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                engine: self.engine,
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                virtual_: core::default::Default::default(),
                settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DmsDataProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DmsDataProviderRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_` after provisioning.\n"]
    pub fn virtual_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DmsDataProvider_SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DmsDataProviderDynamic {
    settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings>>,
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_DocDbSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_DocDbSettings {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_DocDbSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_DocDbSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_DocDbSettings {}

impl BuildDmsDataProvider_Settings_DocDbSettings {
    pub fn build(self) -> DmsDataProvider_Settings_DocDbSettings {
        DmsDataProvider_Settings_DocDbSettings {
            certificate_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            port: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_DocDbSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_DocDbSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_Settings_DocDbSettingsRef {
        DmsDataProvider_Settings_DocDbSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_DocDbSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_IbmDb2LuwSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_algorithm: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_mechanism: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_IbmDb2LuwSettings {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_algorithm`.\n"]
    pub fn set_encryption_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.encryption_algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access_role_arn`.\n"]
    pub fn set_s3_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_path`.\n"]
    pub fn set_s3_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_mechanism`.\n"]
    pub fn set_security_mechanism(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.security_mechanism = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_IbmDb2LuwSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_IbmDb2LuwSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_IbmDb2LuwSettings {}

impl BuildDmsDataProvider_Settings_IbmDb2LuwSettings {
    pub fn build(self) -> DmsDataProvider_Settings_IbmDb2LuwSettings {
        DmsDataProvider_Settings_IbmDb2LuwSettings {
            certificate_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            encryption_algorithm: core::default::Default::default(),
            port: core::default::Default::default(),
            s3_access_role_arn: core::default::Default::default(),
            s3_path: core::default::Default::default(),
            security_mechanism: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_IbmDb2LuwSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_IbmDb2LuwSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_Settings_IbmDb2LuwSettingsRef {
        DmsDataProvider_Settings_IbmDb2LuwSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_IbmDb2LuwSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_algorithm` after provisioning.\n"]
    pub fn encryption_algorithm(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access_role_arn` after provisioning.\n"]
    pub fn s3_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_path` after provisioning.\n"]
    pub fn s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_path", self.base))
    }

    #[doc = "Get a reference to the value of field `security_mechanism` after provisioning.\n"]
    pub fn security_mechanism(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_mechanism", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_IbmDb2ZosSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_IbmDb2ZosSettings {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access_role_arn`.\n"]
    pub fn set_s3_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_path`.\n"]
    pub fn set_s3_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_IbmDb2ZosSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_IbmDb2ZosSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_IbmDb2ZosSettings {}

impl BuildDmsDataProvider_Settings_IbmDb2ZosSettings {
    pub fn build(self) -> DmsDataProvider_Settings_IbmDb2ZosSettings {
        DmsDataProvider_Settings_IbmDb2ZosSettings {
            certificate_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            port: core::default::Default::default(),
            s3_access_role_arn: core::default::Default::default(),
            s3_path: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_IbmDb2ZosSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_IbmDb2ZosSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_Settings_IbmDb2ZosSettingsRef {
        DmsDataProvider_Settings_IbmDb2ZosSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_IbmDb2ZosSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access_role_arn` after provisioning.\n"]
    pub fn s3_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_path` after provisioning.\n"]
    pub fn s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_path", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_MariaDbSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_MariaDbSettings {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access_role_arn`.\n"]
    pub fn set_s3_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_path`.\n"]
    pub fn set_s3_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_MariaDbSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_MariaDbSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_MariaDbSettings {}

impl BuildDmsDataProvider_Settings_MariaDbSettings {
    pub fn build(self) -> DmsDataProvider_Settings_MariaDbSettings {
        DmsDataProvider_Settings_MariaDbSettings {
            certificate_arn: core::default::Default::default(),
            port: core::default::Default::default(),
            s3_access_role_arn: core::default::Default::default(),
            s3_path: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_MariaDbSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_MariaDbSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_Settings_MariaDbSettingsRef {
        DmsDataProvider_Settings_MariaDbSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_MariaDbSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access_role_arn` after provisioning.\n"]
    pub fn s3_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_path` after provisioning.\n"]
    pub fn s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_path", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_MicrosoftSqlServerSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_MicrosoftSqlServerSettings {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access_role_arn`.\n"]
    pub fn set_s3_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_path`.\n"]
    pub fn set_s3_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_MicrosoftSqlServerSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_MicrosoftSqlServerSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_MicrosoftSqlServerSettings {}

impl BuildDmsDataProvider_Settings_MicrosoftSqlServerSettings {
    pub fn build(self) -> DmsDataProvider_Settings_MicrosoftSqlServerSettings {
        DmsDataProvider_Settings_MicrosoftSqlServerSettings {
            certificate_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            port: core::default::Default::default(),
            s3_access_role_arn: core::default::Default::default(),
            s3_path: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_MicrosoftSqlServerSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_MicrosoftSqlServerSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DmsDataProvider_Settings_MicrosoftSqlServerSettingsRef {
        DmsDataProvider_Settings_MicrosoftSqlServerSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_MicrosoftSqlServerSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access_role_arn` after provisioning.\n"]
    pub fn s3_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_path` after provisioning.\n"]
    pub fn s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_path", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_MongoDbSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_mechanism: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_MongoDbSettings {
    #[doc = "Set the field `auth_mechanism`.\n"]
    pub fn set_auth_mechanism(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_mechanism = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_source`.\n"]
    pub fn set_auth_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_type`.\n"]
    pub fn set_auth_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_MongoDbSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_MongoDbSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_MongoDbSettings {}

impl BuildDmsDataProvider_Settings_MongoDbSettings {
    pub fn build(self) -> DmsDataProvider_Settings_MongoDbSettings {
        DmsDataProvider_Settings_MongoDbSettings {
            auth_mechanism: core::default::Default::default(),
            auth_source: core::default::Default::default(),
            auth_type: core::default::Default::default(),
            certificate_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            port: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_MongoDbSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_MongoDbSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_Settings_MongoDbSettingsRef {
        DmsDataProvider_Settings_MongoDbSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_MongoDbSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_mechanism` after provisioning.\n"]
    pub fn auth_mechanism(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_mechanism", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_source` after provisioning.\n"]
    pub fn auth_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_source", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_MysqlSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_MysqlSettings {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access_role_arn`.\n"]
    pub fn set_s3_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_path`.\n"]
    pub fn set_s3_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_MysqlSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_MysqlSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_MysqlSettings {}

impl BuildDmsDataProvider_Settings_MysqlSettings {
    pub fn build(self) -> DmsDataProvider_Settings_MysqlSettings {
        DmsDataProvider_Settings_MysqlSettings {
            certificate_arn: core::default::Default::default(),
            port: core::default::Default::default(),
            s3_access_role_arn: core::default::Default::default(),
            s3_path: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_MysqlSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_MysqlSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_Settings_MysqlSettingsRef {
        DmsDataProvider_Settings_MysqlSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_MysqlSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access_role_arn` after provisioning.\n"]
    pub fn s3_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_path` after provisioning.\n"]
    pub fn s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_path", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_OracleSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asm_server: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_oracle_asm_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_oracle_asm_secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_security_db_encryption_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_security_db_encryption_secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_OracleSettings {
    #[doc = "Set the field `asm_server`.\n"]
    pub fn set_asm_server(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asm_server = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access_role_arn`.\n"]
    pub fn set_s3_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_path`.\n"]
    pub fn set_s3_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_oracle_asm_access_role_arn`.\n"]
    pub fn set_secrets_manager_oracle_asm_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_oracle_asm_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_oracle_asm_secret_id`.\n"]
    pub fn set_secrets_manager_oracle_asm_secret_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_oracle_asm_secret_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_security_db_encryption_access_role_arn`.\n"]
    pub fn set_secrets_manager_security_db_encryption_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_security_db_encryption_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_security_db_encryption_secret_id`.\n"]
    pub fn set_secrets_manager_security_db_encryption_secret_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_security_db_encryption_secret_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_OracleSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_OracleSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_OracleSettings {}

impl BuildDmsDataProvider_Settings_OracleSettings {
    pub fn build(self) -> DmsDataProvider_Settings_OracleSettings {
        DmsDataProvider_Settings_OracleSettings {
            asm_server: core::default::Default::default(),
            certificate_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            port: core::default::Default::default(),
            s3_access_role_arn: core::default::Default::default(),
            s3_path: core::default::Default::default(),
            secrets_manager_oracle_asm_access_role_arn: core::default::Default::default(),
            secrets_manager_oracle_asm_secret_id: core::default::Default::default(),
            secrets_manager_security_db_encryption_access_role_arn: core::default::Default::default(),
            secrets_manager_security_db_encryption_secret_id: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_OracleSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_OracleSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_Settings_OracleSettingsRef {
        DmsDataProvider_Settings_OracleSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_OracleSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asm_server` after provisioning.\n"]
    pub fn asm_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asm_server", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access_role_arn` after provisioning.\n"]
    pub fn s3_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_path` after provisioning.\n"]
    pub fn s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_path", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_oracle_asm_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_oracle_asm_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_oracle_asm_access_role_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `secrets_manager_oracle_asm_secret_id` after provisioning.\n"]
    pub fn secrets_manager_oracle_asm_secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_oracle_asm_secret_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secrets_manager_security_db_encryption_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_security_db_encryption_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_security_db_encryption_access_role_arn", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `secrets_manager_security_db_encryption_secret_id` after provisioning.\n"]
    pub fn secrets_manager_security_db_encryption_secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_security_db_encryption_secret_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_PostgresqlSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_PostgresqlSettings {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access_role_arn`.\n"]
    pub fn set_s3_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_path`.\n"]
    pub fn set_s3_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_PostgresqlSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_PostgresqlSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_PostgresqlSettings {}

impl BuildDmsDataProvider_Settings_PostgresqlSettings {
    pub fn build(self) -> DmsDataProvider_Settings_PostgresqlSettings {
        DmsDataProvider_Settings_PostgresqlSettings {
            certificate_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            port: core::default::Default::default(),
            s3_access_role_arn: core::default::Default::default(),
            s3_path: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_PostgresqlSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_PostgresqlSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DmsDataProvider_Settings_PostgresqlSettingsRef {
        DmsDataProvider_Settings_PostgresqlSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_PostgresqlSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access_role_arn` after provisioning.\n"]
    pub fn s3_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_path` after provisioning.\n"]
    pub fn s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_path", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_RedshiftSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_RedshiftSettings {
    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_access_role_arn`.\n"]
    pub fn set_s3_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_path`.\n"]
    pub fn set_s3_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_RedshiftSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_RedshiftSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_RedshiftSettings {}

impl BuildDmsDataProvider_Settings_RedshiftSettings {
    pub fn build(self) -> DmsDataProvider_Settings_RedshiftSettings {
        DmsDataProvider_Settings_RedshiftSettings {
            database_name: core::default::Default::default(),
            port: core::default::Default::default(),
            s3_access_role_arn: core::default::Default::default(),
            s3_path: core::default::Default::default(),
            server_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_RedshiftSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_RedshiftSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_Settings_RedshiftSettingsRef {
        DmsDataProvider_Settings_RedshiftSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_RedshiftSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_access_role_arn` after provisioning.\n"]
    pub fn s3_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_path` after provisioning.\n"]
    pub fn s3_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_path", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings_SybaseAseSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypt_password: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsDataProvider_Settings_SybaseAseSettings {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypt_password`.\n"]
    pub fn set_encrypt_password(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypt_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_name`.\n"]
    pub fn set_server_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings_SybaseAseSettings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings_SybaseAseSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings_SybaseAseSettings {}

impl BuildDmsDataProvider_Settings_SybaseAseSettings {
    pub fn build(self) -> DmsDataProvider_Settings_SybaseAseSettings {
        DmsDataProvider_Settings_SybaseAseSettings {
            certificate_arn: core::default::Default::default(),
            database_name: core::default::Default::default(),
            encrypt_password: core::default::Default::default(),
            port: core::default::Default::default(),
            server_name: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_Settings_SybaseAseSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_Settings_SybaseAseSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_Settings_SybaseAseSettingsRef {
        DmsDataProvider_Settings_SybaseAseSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_Settings_SybaseAseSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypt_password` after provisioning.\n"]
    pub fn encrypt_password(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypt_password", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `server_name` after provisioning.\n"]
    pub fn server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct DmsDataProvider_SettingsDynamic {
    doc_db_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_DocDbSettings>>,
    ibm_db2_luw_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_IbmDb2LuwSettings>>,
    ibm_db2_zos_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_IbmDb2ZosSettings>>,
    maria_db_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_MariaDbSettings>>,
    microsoft_sql_server_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DmsDataProvider_Settings_MicrosoftSqlServerSettings>>,
    mongo_db_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_MongoDbSettings>>,
    mysql_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_MysqlSettings>>,
    oracle_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_OracleSettings>>,
    postgresql_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_PostgresqlSettings>>,
    redshift_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_RedshiftSettings>>,
    sybase_ase_settings: std::option::Option<terrars::DynamicBlock<DmsDataProvider_Settings_SybaseAseSettings>>,
}

#[derive(Serialize)]
pub struct DmsDataProvider_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    doc_db_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_DocDbSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ibm_db2_luw_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_IbmDb2LuwSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ibm_db2_zos_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_IbmDb2ZosSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maria_db_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_MariaDbSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microsoft_sql_server_settings: std
    ::option
    ::Option<std::vec::Vec<DmsDataProvider_Settings_MicrosoftSqlServerSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongo_db_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_MongoDbSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_MysqlSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_OracleSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_PostgresqlSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_RedshiftSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sybase_ase_settings: std::option::Option<std::vec::Vec<DmsDataProvider_Settings_SybaseAseSettings>>,
    dynamic: DmsDataProvider_SettingsDynamic,
}

impl DmsDataProvider_Settings {
    #[doc = "Set the field `doc_db_settings`.\n"]
    pub fn set_doc_db_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_DocDbSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.doc_db_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.doc_db_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ibm_db2_luw_settings`.\n"]
    pub fn set_ibm_db2_luw_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_IbmDb2LuwSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ibm_db2_luw_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ibm_db2_luw_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ibm_db2_zos_settings`.\n"]
    pub fn set_ibm_db2_zos_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_IbmDb2ZosSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ibm_db2_zos_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ibm_db2_zos_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maria_db_settings`.\n"]
    pub fn set_maria_db_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_MariaDbSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maria_db_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maria_db_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `microsoft_sql_server_settings`.\n"]
    pub fn set_microsoft_sql_server_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_MicrosoftSqlServerSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.microsoft_sql_server_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.microsoft_sql_server_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mongo_db_settings`.\n"]
    pub fn set_mongo_db_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_MongoDbSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mongo_db_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mongo_db_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mysql_settings`.\n"]
    pub fn set_mysql_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_MysqlSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle_settings`.\n"]
    pub fn set_oracle_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_OracleSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgresql_settings`.\n"]
    pub fn set_postgresql_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_PostgresqlSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift_settings`.\n"]
    pub fn set_redshift_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_RedshiftSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sybase_ase_settings`.\n"]
    pub fn set_sybase_ase_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsDataProvider_Settings_SybaseAseSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sybase_ase_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sybase_ase_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DmsDataProvider_Settings {
    type O = terrars::BlockAssignable<DmsDataProvider_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsDataProvider_Settings {}

impl BuildDmsDataProvider_Settings {
    pub fn build(self) -> DmsDataProvider_Settings {
        DmsDataProvider_Settings {
            doc_db_settings: core::default::Default::default(),
            ibm_db2_luw_settings: core::default::Default::default(),
            ibm_db2_zos_settings: core::default::Default::default(),
            maria_db_settings: core::default::Default::default(),
            microsoft_sql_server_settings: core::default::Default::default(),
            mongo_db_settings: core::default::Default::default(),
            mysql_settings: core::default::Default::default(),
            oracle_settings: core::default::Default::default(),
            postgresql_settings: core::default::Default::default(),
            redshift_settings: core::default::Default::default(),
            sybase_ase_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsDataProvider_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsDataProvider_SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsDataProvider_SettingsRef {
        DmsDataProvider_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsDataProvider_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `doc_db_settings` after provisioning.\n"]
    pub fn doc_db_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_DocDbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.doc_db_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ibm_db2_luw_settings` after provisioning.\n"]
    pub fn ibm_db2_luw_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_IbmDb2LuwSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ibm_db2_luw_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `ibm_db2_zos_settings` after provisioning.\n"]
    pub fn ibm_db2_zos_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_IbmDb2ZosSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ibm_db2_zos_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `maria_db_settings` after provisioning.\n"]
    pub fn maria_db_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_MariaDbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maria_db_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `microsoft_sql_server_settings` after provisioning.\n"]
    pub fn microsoft_sql_server_settings(
        &self,
    ) -> terrars::ListRef<DmsDataProvider_Settings_MicrosoftSqlServerSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.microsoft_sql_server_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `mongo_db_settings` after provisioning.\n"]
    pub fn mongo_db_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_MongoDbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongo_db_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_settings` after provisioning.\n"]
    pub fn mysql_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_MysqlSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_settings` after provisioning.\n"]
    pub fn oracle_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_OracleSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_settings` after provisioning.\n"]
    pub fn postgresql_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_PostgresqlSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift_settings` after provisioning.\n"]
    pub fn redshift_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_RedshiftSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sybase_ase_settings` after provisioning.\n"]
    pub fn sybase_ase_settings(&self) -> terrars::ListRef<DmsDataProvider_Settings_SybaseAseSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sybase_ase_settings", self.base))
    }
}

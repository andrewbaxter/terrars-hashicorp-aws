use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatasyncLocationHdfsData {
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
    agent_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_keytab: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_krb5_conf: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_principal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_provider_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    simple_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subdirectory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_node: std::option::Option<std::vec::Vec<DatasyncLocationHdfsNameNodeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qop_configuration: std::option::Option<std::vec::Vec<DatasyncLocationHdfsQopConfigurationEl>>,
    dynamic: DatasyncLocationHdfsDynamic,
}

struct DatasyncLocationHdfs_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatasyncLocationHdfsData>,
}

#[derive(Clone)]
pub struct DatasyncLocationHdfs(std::rc::Rc<DatasyncLocationHdfs_>);

impl DatasyncLocationHdfs {
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

    #[doc = "Set the field `authentication_type`.\n"]
    pub fn set_authentication_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authentication_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_size`.\n"]
    pub fn set_block_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().block_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_keytab`.\n"]
    pub fn set_kerberos_keytab(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kerberos_keytab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_krb5_conf`.\n"]
    pub fn set_kerberos_krb5_conf(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kerberos_krb5_conf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_principal`.\n"]
    pub fn set_kerberos_principal(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kerberos_principal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_provider_uri`.\n"]
    pub fn set_kms_key_provider_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_provider_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_factor`.\n"]
    pub fn set_replication_factor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replication_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `simple_user`.\n"]
    pub fn set_simple_user(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().simple_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subdirectory`.\n"]
    pub fn set_subdirectory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subdirectory = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `name_node`.\n"]
    pub fn set_name_node(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatasyncLocationHdfsNameNodeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().name_node = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.name_node = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `qop_configuration`.\n"]
    pub fn set_qop_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatasyncLocationHdfsQopConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().qop_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.qop_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.agent_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_size` after provisioning.\n"]
    pub fn block_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_keytab` after provisioning.\n"]
    pub fn kerberos_keytab(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos_keytab", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_krb5_conf` after provisioning.\n"]
    pub fn kerberos_krb5_conf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos_krb5_conf", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_principal` after provisioning.\n"]
    pub fn kerberos_principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos_principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_provider_uri` after provisioning.\n"]
    pub fn kms_key_provider_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_provider_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_factor` after provisioning.\n"]
    pub fn replication_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_factor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `simple_user` after provisioning.\n"]
    pub fn simple_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.simple_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qop_configuration` after provisioning.\n"]
    pub fn qop_configuration(&self) -> terrars::ListRef<DatasyncLocationHdfsQopConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qop_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for DatasyncLocationHdfs {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatasyncLocationHdfs { }

impl terrars::ToListMappable for DatasyncLocationHdfs {
    type O = terrars::ListRef<DatasyncLocationHdfsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatasyncLocationHdfs_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_datasync_location_hdfs".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncLocationHdfs {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub agent_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDatasyncLocationHdfs {
    pub fn build(self, stack: &mut terrars::Stack) -> DatasyncLocationHdfs {
        let out = DatasyncLocationHdfs(std::rc::Rc::new(DatasyncLocationHdfs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatasyncLocationHdfsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_arns: self.agent_arns,
                authentication_type: core::default::Default::default(),
                block_size: core::default::Default::default(),
                id: core::default::Default::default(),
                kerberos_keytab: core::default::Default::default(),
                kerberos_krb5_conf: core::default::Default::default(),
                kerberos_principal: core::default::Default::default(),
                kms_key_provider_uri: core::default::Default::default(),
                replication_factor: core::default::Default::default(),
                simple_user: core::default::Default::default(),
                subdirectory: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                name_node: core::default::Default::default(),
                qop_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncLocationHdfsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncLocationHdfsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatasyncLocationHdfsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.agent_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_size` after provisioning.\n"]
    pub fn block_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_keytab` after provisioning.\n"]
    pub fn kerberos_keytab(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos_keytab", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_krb5_conf` after provisioning.\n"]
    pub fn kerberos_krb5_conf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos_krb5_conf", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_principal` after provisioning.\n"]
    pub fn kerberos_principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kerberos_principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_provider_uri` after provisioning.\n"]
    pub fn kms_key_provider_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_provider_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_factor` after provisioning.\n"]
    pub fn replication_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_factor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `simple_user` after provisioning.\n"]
    pub fn simple_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.simple_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qop_configuration` after provisioning.\n"]
    pub fn qop_configuration(&self) -> terrars::ListRef<DatasyncLocationHdfsQopConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qop_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationHdfsNameNodeEl {
    hostname: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl DatasyncLocationHdfsNameNodeEl { }

impl terrars::ToListMappable for DatasyncLocationHdfsNameNodeEl {
    type O = terrars::BlockAssignable<DatasyncLocationHdfsNameNodeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationHdfsNameNodeEl {
    #[doc = ""]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildDatasyncLocationHdfsNameNodeEl {
    pub fn build(self) -> DatasyncLocationHdfsNameNodeEl {
        DatasyncLocationHdfsNameNodeEl {
            hostname: self.hostname,
            port: self.port,
        }
    }
}

pub struct DatasyncLocationHdfsNameNodeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncLocationHdfsNameNodeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatasyncLocationHdfsNameNodeElRef {
        DatasyncLocationHdfsNameNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationHdfsNameNodeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationHdfsQopConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_transfer_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rpc_protection: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatasyncLocationHdfsQopConfigurationEl {
    #[doc = "Set the field `data_transfer_protection`.\n"]
    pub fn set_data_transfer_protection(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_transfer_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rpc_protection`.\n"]
    pub fn set_rpc_protection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rpc_protection = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatasyncLocationHdfsQopConfigurationEl {
    type O = terrars::BlockAssignable<DatasyncLocationHdfsQopConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationHdfsQopConfigurationEl {}

impl BuildDatasyncLocationHdfsQopConfigurationEl {
    pub fn build(self) -> DatasyncLocationHdfsQopConfigurationEl {
        DatasyncLocationHdfsQopConfigurationEl {
            data_transfer_protection: core::default::Default::default(),
            rpc_protection: core::default::Default::default(),
        }
    }
}

pub struct DatasyncLocationHdfsQopConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncLocationHdfsQopConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatasyncLocationHdfsQopConfigurationElRef {
        DatasyncLocationHdfsQopConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationHdfsQopConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_transfer_protection` after provisioning.\n"]
    pub fn data_transfer_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_transfer_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `rpc_protection` after provisioning.\n"]
    pub fn rpc_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpc_protection", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationHdfsDynamic {
    name_node: std::option::Option<terrars::DynamicBlock<DatasyncLocationHdfsNameNodeEl>>,
    qop_configuration: std::option::Option<terrars::DynamicBlock<DatasyncLocationHdfsQopConfigurationEl>>,
}

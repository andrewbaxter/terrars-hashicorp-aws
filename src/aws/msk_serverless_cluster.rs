#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MskServerlessClusterData_ {
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
    cluster_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_authentication: std::option::Option<std::vec::Vec<MskServerlessCluster_ClientAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MskServerlessCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<MskServerlessCluster_VpcConfig>>,
    dynamic: MskServerlessClusterDynamic,
}

struct MskServerlessCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MskServerlessClusterData_>,
}

#[derive(Clone)]
pub struct MskServerlessCluster(std::rc::Rc<MskServerlessCluster_>);

impl MskServerlessCluster {
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_authentication`.\n"]
    pub fn set_client_authentication(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskServerlessCluster_ClientAuthentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MskServerlessCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskServerlessCluster_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> terrars::ListRef<MskServerlessCluster_ClientAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskServerlessCluster_TimeoutsRef {
        MskServerlessCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<MskServerlessCluster_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for MskServerlessCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MskServerlessCluster { }

impl terrars::ToListMappable for MskServerlessCluster {
    type O = terrars::ListRef<MskServerlessClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MskServerlessCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_msk_serverless_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMskServerlessCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildMskServerlessCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> MskServerlessCluster {
        let out = MskServerlessCluster(std::rc::Rc::new(MskServerlessCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MskServerlessClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                client_authentication: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MskServerlessClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MskServerlessClusterRef {
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

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_uuid` after provisioning.\n"]
    pub fn cluster_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `client_authentication` after provisioning.\n"]
    pub fn client_authentication(&self) -> terrars::ListRef<MskServerlessCluster_ClientAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskServerlessCluster_TimeoutsRef {
        MskServerlessCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<MskServerlessCluster_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MskServerlessClusterDynamic {
    client_authentication: std::option::Option<terrars::DynamicBlock<MskServerlessCluster_ClientAuthentication>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<MskServerlessCluster_VpcConfig>>,
}

#[derive(Serialize)]
pub struct MskServerlessCluster_ClientAuthentication_Sasl_Iam {
    enabled: terrars::PrimField<bool>,
}

impl MskServerlessCluster_ClientAuthentication_Sasl_Iam { }

impl terrars::ToListMappable for MskServerlessCluster_ClientAuthentication_Sasl_Iam {
    type O = terrars::BlockAssignable<MskServerlessCluster_ClientAuthentication_Sasl_Iam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessCluster_ClientAuthentication_Sasl_Iam {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskServerlessCluster_ClientAuthentication_Sasl_Iam {
    pub fn build(self) -> MskServerlessCluster_ClientAuthentication_Sasl_Iam {
        MskServerlessCluster_ClientAuthentication_Sasl_Iam { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct MskServerlessCluster_ClientAuthentication_Sasl_IamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessCluster_ClientAuthentication_Sasl_IamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskServerlessCluster_ClientAuthentication_Sasl_IamRef {
        MskServerlessCluster_ClientAuthentication_Sasl_IamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessCluster_ClientAuthentication_Sasl_IamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskServerlessCluster_ClientAuthentication_SaslDynamic {
    iam: std::option::Option<terrars::DynamicBlock<MskServerlessCluster_ClientAuthentication_Sasl_Iam>>,
}

#[derive(Serialize)]
pub struct MskServerlessCluster_ClientAuthentication_Sasl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam: std::option::Option<std::vec::Vec<MskServerlessCluster_ClientAuthentication_Sasl_Iam>>,
    dynamic: MskServerlessCluster_ClientAuthentication_SaslDynamic,
}

impl MskServerlessCluster_ClientAuthentication_Sasl {
    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskServerlessCluster_ClientAuthentication_Sasl_Iam>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iam = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iam = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskServerlessCluster_ClientAuthentication_Sasl {
    type O = terrars::BlockAssignable<MskServerlessCluster_ClientAuthentication_Sasl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessCluster_ClientAuthentication_Sasl {}

impl BuildMskServerlessCluster_ClientAuthentication_Sasl {
    pub fn build(self) -> MskServerlessCluster_ClientAuthentication_Sasl {
        MskServerlessCluster_ClientAuthentication_Sasl {
            iam: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskServerlessCluster_ClientAuthentication_SaslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessCluster_ClientAuthentication_SaslRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskServerlessCluster_ClientAuthentication_SaslRef {
        MskServerlessCluster_ClientAuthentication_SaslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessCluster_ClientAuthentication_SaslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> terrars::ListRef<MskServerlessCluster_ClientAuthentication_Sasl_IamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskServerlessCluster_ClientAuthenticationDynamic {
    sasl: std::option::Option<terrars::DynamicBlock<MskServerlessCluster_ClientAuthentication_Sasl>>,
}

#[derive(Serialize)]
pub struct MskServerlessCluster_ClientAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl: std::option::Option<std::vec::Vec<MskServerlessCluster_ClientAuthentication_Sasl>>,
    dynamic: MskServerlessCluster_ClientAuthenticationDynamic,
}

impl MskServerlessCluster_ClientAuthentication {
    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskServerlessCluster_ClientAuthentication_Sasl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sasl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sasl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskServerlessCluster_ClientAuthentication {
    type O = terrars::BlockAssignable<MskServerlessCluster_ClientAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessCluster_ClientAuthentication {}

impl BuildMskServerlessCluster_ClientAuthentication {
    pub fn build(self) -> MskServerlessCluster_ClientAuthentication {
        MskServerlessCluster_ClientAuthentication {
            sasl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskServerlessCluster_ClientAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessCluster_ClientAuthenticationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskServerlessCluster_ClientAuthenticationRef {
        MskServerlessCluster_ClientAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessCluster_ClientAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(&self) -> terrars::ListRef<MskServerlessCluster_ClientAuthentication_SaslRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }
}

#[derive(Serialize)]
pub struct MskServerlessCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskServerlessCluster_Timeouts {
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
}

impl terrars::ToListMappable for MskServerlessCluster_Timeouts {
    type O = terrars::BlockAssignable<MskServerlessCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessCluster_Timeouts {}

impl BuildMskServerlessCluster_Timeouts {
    pub fn build(self) -> MskServerlessCluster_Timeouts {
        MskServerlessCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MskServerlessCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskServerlessCluster_TimeoutsRef {
        MskServerlessCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessCluster_TimeoutsRef {
    #[allow(dead_code)]
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
}

#[derive(Serialize)]
pub struct MskServerlessCluster_VpcConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl MskServerlessCluster_VpcConfig {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskServerlessCluster_VpcConfig {
    type O = terrars::BlockAssignable<MskServerlessCluster_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessCluster_VpcConfig {
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMskServerlessCluster_VpcConfig {
    pub fn build(self) -> MskServerlessCluster_VpcConfig {
        MskServerlessCluster_VpcConfig {
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct MskServerlessCluster_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessCluster_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskServerlessCluster_VpcConfigRef {
        MskServerlessCluster_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessCluster_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

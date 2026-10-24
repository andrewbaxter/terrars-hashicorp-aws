use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MskServerlessClusterData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_authentication: std::option::Option<std::vec::Vec<MskServerlessClusterClientAuthenticationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MskServerlessClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<MskServerlessClusterVpcConfigEl>>,
    dynamic: MskServerlessClusterDynamic,
}

struct MskServerlessCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MskServerlessClusterData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<MskServerlessClusterClientAuthenticationEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<MskServerlessClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskServerlessClusterVpcConfigEl>>,
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

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn client_authentication(&self) -> terrars::ListRef<MskServerlessClusterClientAuthenticationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskServerlessClusterTimeoutsElRef {
        MskServerlessClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<MskServerlessClusterVpcConfigElRef> {
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
            data: std::cell::RefCell::new(MskServerlessClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
    pub fn client_authentication(&self) -> terrars::ListRef<MskServerlessClusterClientAuthenticationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskServerlessClusterTimeoutsElRef {
        MskServerlessClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<MskServerlessClusterVpcConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MskServerlessClusterClientAuthenticationElSaslElIamEl {
    enabled: terrars::PrimField<bool>,
}

impl MskServerlessClusterClientAuthenticationElSaslElIamEl { }

impl terrars::ToListMappable for MskServerlessClusterClientAuthenticationElSaslElIamEl {
    type O = terrars::BlockAssignable<MskServerlessClusterClientAuthenticationElSaslElIamEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessClusterClientAuthenticationElSaslElIamEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMskServerlessClusterClientAuthenticationElSaslElIamEl {
    pub fn build(self) -> MskServerlessClusterClientAuthenticationElSaslElIamEl {
        MskServerlessClusterClientAuthenticationElSaslElIamEl { enabled: self.enabled }
    }
}

pub struct MskServerlessClusterClientAuthenticationElSaslElIamElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessClusterClientAuthenticationElSaslElIamElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskServerlessClusterClientAuthenticationElSaslElIamElRef {
        MskServerlessClusterClientAuthenticationElSaslElIamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessClusterClientAuthenticationElSaslElIamElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskServerlessClusterClientAuthenticationElSaslElDynamic {
    iam: std::option::Option<terrars::DynamicBlock<MskServerlessClusterClientAuthenticationElSaslElIamEl>>,
}

#[derive(Serialize)]
pub struct MskServerlessClusterClientAuthenticationElSaslEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam: std::option::Option<std::vec::Vec<MskServerlessClusterClientAuthenticationElSaslElIamEl>>,
    dynamic: MskServerlessClusterClientAuthenticationElSaslElDynamic,
}

impl MskServerlessClusterClientAuthenticationElSaslEl {
    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskServerlessClusterClientAuthenticationElSaslElIamEl>>,
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

impl terrars::ToListMappable for MskServerlessClusterClientAuthenticationElSaslEl {
    type O = terrars::BlockAssignable<MskServerlessClusterClientAuthenticationElSaslEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessClusterClientAuthenticationElSaslEl {}

impl BuildMskServerlessClusterClientAuthenticationElSaslEl {
    pub fn build(self) -> MskServerlessClusterClientAuthenticationElSaslEl {
        MskServerlessClusterClientAuthenticationElSaslEl {
            iam: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskServerlessClusterClientAuthenticationElSaslElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessClusterClientAuthenticationElSaslElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MskServerlessClusterClientAuthenticationElSaslElRef {
        MskServerlessClusterClientAuthenticationElSaslElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessClusterClientAuthenticationElSaslElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> terrars::ListRef<MskServerlessClusterClientAuthenticationElSaslElIamElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskServerlessClusterClientAuthenticationElDynamic {
    sasl: std::option::Option<terrars::DynamicBlock<MskServerlessClusterClientAuthenticationElSaslEl>>,
}

#[derive(Serialize)]
pub struct MskServerlessClusterClientAuthenticationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl: std::option::Option<std::vec::Vec<MskServerlessClusterClientAuthenticationElSaslEl>>,
    dynamic: MskServerlessClusterClientAuthenticationElDynamic,
}

impl MskServerlessClusterClientAuthenticationEl {
    #[doc = "Set the field `sasl`.\n"]
    pub fn set_sasl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskServerlessClusterClientAuthenticationElSaslEl>>,
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

impl terrars::ToListMappable for MskServerlessClusterClientAuthenticationEl {
    type O = terrars::BlockAssignable<MskServerlessClusterClientAuthenticationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessClusterClientAuthenticationEl {}

impl BuildMskServerlessClusterClientAuthenticationEl {
    pub fn build(self) -> MskServerlessClusterClientAuthenticationEl {
        MskServerlessClusterClientAuthenticationEl {
            sasl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskServerlessClusterClientAuthenticationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessClusterClientAuthenticationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskServerlessClusterClientAuthenticationElRef {
        MskServerlessClusterClientAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessClusterClientAuthenticationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sasl` after provisioning.\n"]
    pub fn sasl(&self) -> terrars::ListRef<MskServerlessClusterClientAuthenticationElSaslElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sasl", self.base))
    }
}

#[derive(Serialize)]
pub struct MskServerlessClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskServerlessClusterTimeoutsEl {
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

impl terrars::ToListMappable for MskServerlessClusterTimeoutsEl {
    type O = terrars::BlockAssignable<MskServerlessClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessClusterTimeoutsEl {}

impl BuildMskServerlessClusterTimeoutsEl {
    pub fn build(self) -> MskServerlessClusterTimeoutsEl {
        MskServerlessClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct MskServerlessClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskServerlessClusterTimeoutsElRef {
        MskServerlessClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessClusterTimeoutsElRef {
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
pub struct MskServerlessClusterVpcConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl MskServerlessClusterVpcConfigEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskServerlessClusterVpcConfigEl {
    type O = terrars::BlockAssignable<MskServerlessClusterVpcConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskServerlessClusterVpcConfigEl {
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildMskServerlessClusterVpcConfigEl {
    pub fn build(self) -> MskServerlessClusterVpcConfigEl {
        MskServerlessClusterVpcConfigEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct MskServerlessClusterVpcConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskServerlessClusterVpcConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskServerlessClusterVpcConfigElRef {
        MskServerlessClusterVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskServerlessClusterVpcConfigElRef {
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

#[derive(Serialize, Default)]
struct MskServerlessClusterDynamic {
    client_authentication: std::option::Option<terrars::DynamicBlock<MskServerlessClusterClientAuthenticationEl>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<MskServerlessClusterVpcConfigEl>>,
}

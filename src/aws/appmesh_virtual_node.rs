#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshVirtualNodeData_ {
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
    mesh_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec>>,
    dynamic: AppmeshVirtualNodeDynamic,
}

struct AppmeshVirtualNode_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppmeshVirtualNodeData_>,
}

#[derive(Clone)]
pub struct AppmeshVirtualNode(std::rc::Rc<AppmeshVirtualNode_>);

impl AppmeshVirtualNode {
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

    #[doc = "Set the field `mesh_owner`.\n"]
    pub fn set_mesh_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mesh_owner = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualNode_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

impl terrars::Referable for AppmeshVirtualNode {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppmeshVirtualNode { }

impl terrars::ToListMappable for AppmeshVirtualNode {
    type O = terrars::ListRef<AppmeshVirtualNodeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppmeshVirtualNode_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appmesh_virtual_node".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppmeshVirtualNode {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode {
    pub fn build(self, stack: &mut terrars::Stack) -> AppmeshVirtualNode {
        let out = AppmeshVirtualNode(std::rc::Rc::new(AppmeshVirtualNode_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppmeshVirtualNodeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                spec: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppmeshVirtualNodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppmeshVirtualNodeRef {
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

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualNode_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File {
    certificate_chain: terrars::PrimField<std::string::String>,
    private_key: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls_Certificate_File {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub private_key: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls_Certificate_File {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File {
        AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Certificate_FileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Tls_Certificate_FileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Tls_Certificate_FileRef {
        AppmeshVirtualNode_Spec_Listener_Tls_Certificate_FileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Certificate_FileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds {
        AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds { secret_name: self.secret_name }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Certificate_SdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Tls_Certificate_SdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Tls_Certificate_SdsRef {
        AppmeshVirtualNode_Spec_Listener_Tls_Certificate_SdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Certificate_SdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_CertificateDynamic {
    file: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File>>,
    sds: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
    dynamic: AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_CertificateDynamic,
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {}

impl BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
    pub fn build(self) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_SdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
    exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
    #[doc = ""]
    pub exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
        AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match { exact: self.exact }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
        AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNamesDynamic {
    match_: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNamesDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames {}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
        AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
        AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
    certificate_authority_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
    #[doc = ""]
    pub certificate_authority_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
    pub fn build(self) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
            certificate_authority_arns: self.certificate_authority_arns,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File {
    certificate_chain: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File {
        AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File { certificate_chain: self.certificate_chain }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_FileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_FileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_FileRef {
        AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_FileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_FileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustDynamic {
    acm: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm>>,
    file: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File>>,
    sds: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
    dynamic: AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustDynamic,
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.acm = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.acm = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {}

impl BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
    pub fn build(self) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_SdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_ValidationDynamic {
    subject_alternative_names: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames>>,
    trust: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust>>,
    dynamic: AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_ValidationDynamic,
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.subject_alternative_names = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alternative_names = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trust = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trust = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation {}

impl BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
    pub fn build(self) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_TlsDynamic {
    certificate: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate>>,
    validation: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation>>,
    dynamic: AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_TlsDynamic,
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls {
    #[doc = "Set the field `enforce`.\n"]
    pub fn set_enforce(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(mut self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Certificate>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_Validation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls {}

impl BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls {
    pub fn build(self) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls {
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_TlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_TlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_TlsRef {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_TlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_TlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforce` after provisioning.\n"]
    pub fn enforce(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicyDynamic {
    tls: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls>>,
    dynamic: AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicyDynamic,
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_Tls>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy {}

impl BuildAppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy {
    pub fn build(self) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy {
            tls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicyRef {
        AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy_TlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Backend_VirtualServiceDynamic {
    client_policy: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Backend_VirtualService {
    virtual_service_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_policy: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy>>,
    dynamic: AppmeshVirtualNode_Spec_Backend_VirtualServiceDynamic,
}

impl AppmeshVirtualNode_Spec_Backend_VirtualService {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Backend_VirtualService {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Backend_VirtualService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Backend_VirtualService {
    #[doc = ""]
    pub virtual_service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_Backend_VirtualService {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Backend_VirtualService {
        AppmeshVirtualNode_Spec_Backend_VirtualService {
            virtual_service_name: self.virtual_service_name,
            client_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Backend_VirtualServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Backend_VirtualServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Backend_VirtualServiceRef {
        AppmeshVirtualNode_Spec_Backend_VirtualServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Backend_VirtualServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_BackendDynamic {
    virtual_service: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Backend_VirtualService>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Backend {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_service: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Backend_VirtualService>>,
    dynamic: AppmeshVirtualNode_Spec_BackendDynamic,
}

impl AppmeshVirtualNode_Spec_Backend {
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Backend_VirtualService>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.virtual_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_service = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Backend {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Backend>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Backend {}

impl BuildAppmeshVirtualNode_Spec_Backend {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Backend {
        AppmeshVirtualNode_Spec_Backend {
            virtual_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_BackendRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_BackendRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNode_Spec_BackendRef {
        AppmeshVirtualNode_Spec_BackendRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_BackendRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Backend_VirtualServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_BackendDefaultsDynamic {
    client_policy: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_BackendDefaults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_policy: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy>>,
    dynamic: AppmeshVirtualNode_Spec_BackendDefaultsDynamic,
}

impl AppmeshVirtualNode_Spec_BackendDefaults {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_BackendDefaults {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_BackendDefaults {}

impl BuildAppmeshVirtualNode_Spec_BackendDefaults {
    pub fn build(self) -> AppmeshVirtualNode_Spec_BackendDefaults {
        AppmeshVirtualNode_Spec_BackendDefaults {
            client_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_BackendDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_BackendDefaultsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNode_Spec_BackendDefaultsRef {
        AppmeshVirtualNode_Spec_BackendDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_BackendDefaultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_BackendDefaults_ClientPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc {
    max_requests: terrars::PrimField<f64>,
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc {
    #[doc = ""]
    pub max_requests: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc {
        AppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc { max_requests: self.max_requests }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPool_GrpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_ConnectionPool_GrpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_ConnectionPool_GrpcRef {
        AppmeshVirtualNode_Spec_Listener_ConnectionPool_GrpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPool_GrpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http {
    max_connections: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_requests: std::option::Option<terrars::PrimField<f64>>,
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http {
    #[doc = "Set the field `max_pending_requests`.\n"]
    pub fn set_max_pending_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pending_requests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool_Http {
    #[doc = ""]
    pub max_connections: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool_Http {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http {
        AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http {
            max_connections: self.max_connections,
            max_pending_requests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPool_HttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_ConnectionPool_HttpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_ConnectionPool_HttpRef {
        AppmeshVirtualNode_Spec_Listener_ConnectionPool_HttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPool_HttpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `max_pending_requests` after provisioning.\n"]
    pub fn max_pending_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pending_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2 {
    max_requests: terrars::PrimField<f64>,
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2 { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2 {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2 {
    #[doc = ""]
    pub max_requests: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2 {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2 {
        AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2 { max_requests: self.max_requests }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2Ref {
        AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp {
    max_connections: terrars::PrimField<f64>,
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp {
    #[doc = ""]
    pub max_connections: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp {
        AppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp { max_connections: self.max_connections }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPool_TcpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_ConnectionPool_TcpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_ConnectionPool_TcpRef {
        AppmeshVirtualNode_Spec_Listener_ConnectionPool_TcpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPool_TcpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_ConnectionPoolDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc>>,
    http: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http>>,
    http2: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2>>,
    tcp: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_ConnectionPoolDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPool {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Grpc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http2 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http2 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp`.\n"]
    pub fn set_tcp(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Tcp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tcp = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_ConnectionPool {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool {}

impl BuildAppmeshVirtualNode_Spec_Listener_ConnectionPool {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_ConnectionPool {
        AppmeshVirtualNode_Spec_Listener_ConnectionPool {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            tcp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_ConnectionPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_ConnectionPoolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_ConnectionPoolRef {
        AppmeshVirtualNode_Spec_Listener_ConnectionPoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_ConnectionPoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_ConnectionPool_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_ConnectionPool_HttpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_ConnectionPool_Http2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp` after provisioning.\n"]
    pub fn tcp(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_ConnectionPool_TcpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_HealthCheck {
    healthy_threshold: terrars::PrimField<f64>,
    interval_millis: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    protocol: terrars::PrimField<std::string::String>,
    timeout_millis: terrars::PrimField<f64>,
    unhealthy_threshold: terrars::PrimField<f64>,
}

impl AppmeshVirtualNode_Spec_Listener_HealthCheck {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_HealthCheck {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_HealthCheck {
    #[doc = ""]
    pub healthy_threshold: terrars::PrimField<f64>,
    #[doc = ""]
    pub interval_millis: terrars::PrimField<f64>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub timeout_millis: terrars::PrimField<f64>,
    #[doc = ""]
    pub unhealthy_threshold: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_HealthCheck {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_HealthCheck {
        AppmeshVirtualNode_Spec_Listener_HealthCheck {
            healthy_threshold: self.healthy_threshold,
            interval_millis: self.interval_millis,
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: self.protocol,
            timeout_millis: self.timeout_millis,
            unhealthy_threshold: self.unhealthy_threshold,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_HealthCheckRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_HealthCheckRef {
        AppmeshVirtualNode_Spec_Listener_HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_HealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_millis` after provisioning.\n"]
    pub fn interval_millis(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_millis", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_millis` after provisioning.\n"]
    pub fn timeout_millis(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_millis", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration {
        AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDurationRef {
        AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval {
        AppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_OutlierDetection_IntervalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_OutlierDetection_IntervalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_OutlierDetection_IntervalRef {
        AppmeshVirtualNode_Spec_Listener_OutlierDetection_IntervalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_OutlierDetection_IntervalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_OutlierDetectionDynamic {
    base_ejection_duration: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration>>,
    interval: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_OutlierDetection {
    max_ejection_percent: terrars::PrimField<f64>,
    max_server_errors: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_ejection_duration: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_OutlierDetectionDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_OutlierDetection {
    #[doc = "Set the field `base_ejection_duration`.\n"]
    pub fn set_base_ejection_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.base_ejection_duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.base_ejection_duration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_OutlierDetection_Interval>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.interval = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.interval = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_OutlierDetection {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_OutlierDetection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_OutlierDetection {
    #[doc = ""]
    pub max_ejection_percent: terrars::PrimField<f64>,
    #[doc = ""]
    pub max_server_errors: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_OutlierDetection {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_OutlierDetection {
        AppmeshVirtualNode_Spec_Listener_OutlierDetection {
            max_ejection_percent: self.max_ejection_percent,
            max_server_errors: self.max_server_errors,
            base_ejection_duration: core::default::Default::default(),
            interval: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_OutlierDetectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_OutlierDetectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_OutlierDetectionRef {
        AppmeshVirtualNode_Spec_Listener_OutlierDetectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_OutlierDetectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_ejection_percent` after provisioning.\n"]
    pub fn max_ejection_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ejection_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `max_server_errors` after provisioning.\n"]
    pub fn max_server_errors(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_server_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `base_ejection_duration` after provisioning.\n"]
    pub fn base_ejection_duration(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_OutlierDetection_BaseEjectionDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_ejection_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_OutlierDetection_IntervalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interval", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_PortMapping {
    port: terrars::PrimField<f64>,
    protocol: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNode_Spec_Listener_PortMapping { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_PortMapping {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_PortMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_PortMapping {
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_PortMapping {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_PortMapping {
        AppmeshVirtualNode_Spec_Listener_PortMapping {
            port: self.port,
            protocol: self.protocol,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_PortMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_PortMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_PortMappingRef {
        AppmeshVirtualNode_Spec_Listener_PortMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_PortMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle {
        AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_IdleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_IdleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_IdleRef {
        AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_IdleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_IdleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest {
        AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequestRef {
        AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_Timeout_GrpcDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    per_request: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_Grpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_Timeout_GrpcDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_Grpc {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.per_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Timeout_Grpc {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Timeout_Grpc {}

impl BuildAppmeshVirtualNode_Spec_Listener_Timeout_Grpc {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Timeout_Grpc {
        AppmeshVirtualNode_Spec_Listener_Timeout_Grpc {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_GrpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Timeout_GrpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Timeout_GrpcRef {
        AppmeshVirtualNode_Spec_Listener_Timeout_GrpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_GrpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_IdleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_Timeout_HttpDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    per_request: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_Http {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_Timeout_HttpDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_Http {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.per_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Timeout_Http {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Http>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Timeout_Http {}

impl BuildAppmeshVirtualNode_Spec_Listener_Timeout_Http {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Timeout_Http {
        AppmeshVirtualNode_Spec_Listener_Timeout_Http {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_HttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Timeout_HttpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Timeout_HttpRef {
        AppmeshVirtualNode_Spec_Listener_Timeout_HttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_HttpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_IdleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_Timeout_Http2Dynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    per_request: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_Http2 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_Timeout_Http2Dynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_Http2 {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequest>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.per_request = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.per_request = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Timeout_Http2 {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Http2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Timeout_Http2 {}

impl BuildAppmeshVirtualNode_Spec_Listener_Timeout_Http2 {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Timeout_Http2 {
        AppmeshVirtualNode_Spec_Listener_Timeout_Http2 {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_Http2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Timeout_Http2Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Timeout_Http2Ref {
        AppmeshVirtualNode_Spec_Listener_Timeout_Http2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_Http2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_IdleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc_PerRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_Timeout_TcpDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_Tcp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_Timeout_TcpDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_Tcp {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_Idle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idle = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Timeout_Tcp {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Timeout_Tcp {}

impl BuildAppmeshVirtualNode_Spec_Listener_Timeout_Tcp {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Timeout_Tcp {
        AppmeshVirtualNode_Spec_Listener_Timeout_Tcp {
            idle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout_TcpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Timeout_TcpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Timeout_TcpRef {
        AppmeshVirtualNode_Spec_Listener_Timeout_TcpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Timeout_TcpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp_IdleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_TimeoutDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc>>,
    http: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Http>>,
    http2: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Http2>>,
    tcp: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Http>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Http2>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_TimeoutDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Timeout {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Grpc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Http>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Http2>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http2 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http2 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp`.\n"]
    pub fn set_tcp(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout_Tcp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tcp = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Timeout {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Timeout {}

impl BuildAppmeshVirtualNode_Spec_Listener_Timeout {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Timeout {
        AppmeshVirtualNode_Spec_Listener_Timeout {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            tcp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_TimeoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNode_Spec_Listener_TimeoutRef {
        AppmeshVirtualNode_Spec_Listener_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_HttpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_Http2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp` after provisioning.\n"]
    pub fn tcp(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Timeout_TcpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm {
    certificate_arn: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm {
    #[doc = ""]
    pub certificate_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm {
        AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm { certificate_arn: self.certificate_arn }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Certificate_AcmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Tls_Certificate_AcmRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Tls_Certificate_AcmRef {
        AppmeshVirtualNode_Spec_Listener_Tls_Certificate_AcmRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Certificate_AcmRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_Tls_CertificateDynamic {
    acm: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm>>,
    file: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File>>,
    sds: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Certificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_Tls_CertificateDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Certificate {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Acm>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.acm = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.acm = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_File>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls_Certificate {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls_Certificate {}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls_Certificate {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls_Certificate {
        AppmeshVirtualNode_Spec_Listener_Tls_Certificate {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Tls_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Tls_CertificateRef {
        AppmeshVirtualNode_Spec_Listener_Tls_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Tls_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_AcmRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_SdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_TrustDynamic {
    file: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File>>,
    sds: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_Tls_Validation_TrustDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_File>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_Sds>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust {}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust {
        AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Validation_TrustRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Tls_Validation_TrustRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Tls_Validation_TrustRef {
        AppmeshVirtualNode_Spec_Listener_Tls_Validation_TrustRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Validation_TrustRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Certificate_SdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_Tls_ValidationDynamic {
    subject_alternative_names: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames>>,
    trust: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_Validation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_Tls_ValidationDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Tls_Validation {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNames>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.subject_alternative_names = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.subject_alternative_names = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation_Trust>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trust = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trust = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls_Validation {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation {}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls_Validation {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls_Validation {
        AppmeshVirtualNode_Spec_Listener_Tls_Validation {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls_ValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_Tls_ValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Listener_Tls_ValidationRef {
        AppmeshVirtualNode_Spec_Listener_Tls_ValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_Tls_ValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_Validation_TrustRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Listener_TlsDynamic {
    certificate: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Certificate>>,
    validation: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls_Validation>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener_Tls {
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Certificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls_Validation>>,
    dynamic: AppmeshVirtualNode_Spec_Listener_TlsDynamic,
}

impl AppmeshVirtualNode_Spec_Listener_Tls {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Certificate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.certificate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls_Validation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener_Tls {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener_Tls {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_Listener_Tls {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener_Tls {
        AppmeshVirtualNode_Spec_Listener_Tls {
            mode: self.mode,
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Listener_TlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Listener_TlsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNode_Spec_Listener_TlsRef {
        AppmeshVirtualNode_Spec_Listener_TlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Listener_TlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_Tls_ValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_ListenerDynamic {
    connection_pool: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_ConnectionPool>>,
    health_check: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_HealthCheck>>,
    outlier_detection: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_OutlierDetection>>,
    port_mapping: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_PortMapping>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Timeout>>,
    tls: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener_Tls>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Listener {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pool: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_ConnectionPool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_HealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outlier_detection: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_OutlierDetection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_mapping: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_PortMapping>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener_Tls>>,
    dynamic: AppmeshVirtualNode_Spec_ListenerDynamic,
}

impl AppmeshVirtualNode_Spec_Listener {
    #[doc = "Set the field `connection_pool`.\n"]
    pub fn set_connection_pool(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_ConnectionPool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connection_pool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_pool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_HealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outlier_detection`.\n"]
    pub fn set_outlier_detection(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_OutlierDetection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.outlier_detection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.outlier_detection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_mapping`.\n"]
    pub fn set_port_mapping(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_PortMapping>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Timeout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener_Tls>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tls = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Listener {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Listener {}

impl BuildAppmeshVirtualNode_Spec_Listener {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Listener {
        AppmeshVirtualNode_Spec_Listener {
            connection_pool: core::default::Default::default(),
            health_check: core::default::Default::default(),
            outlier_detection: core::default::Default::default(),
            port_mapping: core::default::Default::default(),
            timeout: core::default::Default::default(),
            tls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_ListenerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_ListenerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNode_Spec_ListenerRef {
        AppmeshVirtualNode_Spec_ListenerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_ListenerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_pool` after provisioning.\n"]
    pub fn connection_pool(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_ConnectionPoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_HealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_OutlierDetectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outlier_detection", self.base))
    }

    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_PortMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Listener_TlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json { }

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json {
        AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_JsonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_JsonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_JsonRef {
        AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_JsonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_JsonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Logging_AccessLog_File_FormatDynamic {
    json: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json>>,
    dynamic: AppmeshVirtualNode_Spec_Logging_AccessLog_File_FormatDynamic,
}

impl AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format {
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_Json>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Logging_AccessLog_File_Format {}

impl BuildAppmeshVirtualNode_Spec_Logging_AccessLog_File_Format {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format {
        AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format {
            text: core::default::Default::default(),
            json: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Logging_AccessLog_File_FormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Logging_AccessLog_File_FormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Logging_AccessLog_File_FormatRef {
        AppmeshVirtualNode_Spec_Logging_AccessLog_File_FormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Logging_AccessLog_File_FormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format_JsonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Logging_AccessLog_FileDynamic {
    format: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Logging_AccessLog_File {
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format>>,
    dynamic: AppmeshVirtualNode_Spec_Logging_AccessLog_FileDynamic,
}

impl AppmeshVirtualNode_Spec_Logging_AccessLog_File {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging_AccessLog_File_Format>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Logging_AccessLog_File {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging_AccessLog_File>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Logging_AccessLog_File {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_Logging_AccessLog_File {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Logging_AccessLog_File {
        AppmeshVirtualNode_Spec_Logging_AccessLog_File {
            path: self.path,
            format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Logging_AccessLog_FileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Logging_AccessLog_FileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_Logging_AccessLog_FileRef {
        AppmeshVirtualNode_Spec_Logging_AccessLog_FileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Logging_AccessLog_FileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Logging_AccessLog_File_FormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.format", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_Logging_AccessLogDynamic {
    file: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Logging_AccessLog_File>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Logging_AccessLog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Logging_AccessLog_File>>,
    dynamic: AppmeshVirtualNode_Spec_Logging_AccessLogDynamic,
}

impl AppmeshVirtualNode_Spec_Logging_AccessLog {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging_AccessLog_File>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Logging_AccessLog {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging_AccessLog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Logging_AccessLog {}

impl BuildAppmeshVirtualNode_Spec_Logging_AccessLog {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Logging_AccessLog {
        AppmeshVirtualNode_Spec_Logging_AccessLog {
            file: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_Logging_AccessLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_Logging_AccessLogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNode_Spec_Logging_AccessLogRef {
        AppmeshVirtualNode_Spec_Logging_AccessLogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_Logging_AccessLogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Logging_AccessLog_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_LoggingDynamic {
    access_log: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Logging_AccessLog>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_Logging {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_log: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Logging_AccessLog>>,
    dynamic: AppmeshVirtualNode_Spec_LoggingDynamic,
}

impl AppmeshVirtualNode_Spec_Logging {
    #[doc = "Set the field `access_log`.\n"]
    pub fn set_access_log(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging_AccessLog>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_log = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_log = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_Logging {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_Logging {}

impl BuildAppmeshVirtualNode_Spec_Logging {
    pub fn build(self) -> AppmeshVirtualNode_Spec_Logging {
        AppmeshVirtualNode_Spec_Logging {
            access_log: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNode_Spec_LoggingRef {
        AppmeshVirtualNode_Spec_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_LoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_log` after provisioning.\n"]
    pub fn access_log(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_Logging_AccessLogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_log", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    namespace_name: terrars::PrimField<std::string::String>,
    service_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap {
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attributes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap {
    #[doc = ""]
    pub namespace_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap {
    pub fn build(self) -> AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap {
        AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap {
            attributes: core::default::Default::default(),
            namespace_name: self.namespace_name,
            service_name: self.service_name,
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMapRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMapRef {
        AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace_name` after provisioning.\n"]
    pub fn namespace_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_ServiceDiscovery_Dns {
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualNode_Spec_ServiceDiscovery_Dns {
    #[doc = "Set the field `ip_preference`.\n"]
    pub fn set_ip_preference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_type`.\n"]
    pub fn set_response_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_ServiceDiscovery_Dns {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_ServiceDiscovery_Dns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_ServiceDiscovery_Dns {
    #[doc = ""]
    pub hostname: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNode_Spec_ServiceDiscovery_Dns {
    pub fn build(self) -> AppmeshVirtualNode_Spec_ServiceDiscovery_Dns {
        AppmeshVirtualNode_Spec_ServiceDiscovery_Dns {
            hostname: self.hostname,
            ip_preference: core::default::Default::default(),
            response_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_ServiceDiscovery_DnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_ServiceDiscovery_DnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNode_Spec_ServiceDiscovery_DnsRef {
        AppmeshVirtualNode_Spec_ServiceDiscovery_DnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_ServiceDiscovery_DnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_preference` after provisioning.\n"]
    pub fn ip_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `response_type` after provisioning.\n"]
    pub fn response_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_Spec_ServiceDiscoveryDynamic {
    aws_cloud_map: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap>>,
    dns: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_ServiceDiscovery_Dns>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec_ServiceDiscovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_cloud_map: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_ServiceDiscovery_Dns>>,
    dynamic: AppmeshVirtualNode_Spec_ServiceDiscoveryDynamic,
}

impl AppmeshVirtualNode_Spec_ServiceDiscovery {
    #[doc = "Set the field `aws_cloud_map`.\n"]
    pub fn set_aws_cloud_map(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMap>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_cloud_map = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_cloud_map = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dns`.\n"]
    pub fn set_dns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_ServiceDiscovery_Dns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec_ServiceDiscovery {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec_ServiceDiscovery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec_ServiceDiscovery {}

impl BuildAppmeshVirtualNode_Spec_ServiceDiscovery {
    pub fn build(self) -> AppmeshVirtualNode_Spec_ServiceDiscovery {
        AppmeshVirtualNode_Spec_ServiceDiscovery {
            aws_cloud_map: core::default::Default::default(),
            dns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_Spec_ServiceDiscoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_Spec_ServiceDiscoveryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNode_Spec_ServiceDiscoveryRef {
        AppmeshVirtualNode_Spec_ServiceDiscoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_Spec_ServiceDiscoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_cloud_map` after provisioning.\n"]
    pub fn aws_cloud_map(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_ServiceDiscovery_AwsCloudMapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_cloud_map", self.base))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_ServiceDiscovery_DnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNode_SpecDynamic {
    backend: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Backend>>,
    backend_defaults: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_BackendDefaults>>,
    listener: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Listener>>,
    logging: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_Logging>>,
    service_discovery: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNode_Spec_ServiceDiscovery>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNode_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Backend>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_defaults: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_BackendDefaults>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    listener: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Listener>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_Logging>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_discovery: std::option::Option<std::vec::Vec<AppmeshVirtualNode_Spec_ServiceDiscovery>>,
    dynamic: AppmeshVirtualNode_SpecDynamic,
}

impl AppmeshVirtualNode_Spec {
    #[doc = "Set the field `backend`.\n"]
    pub fn set_backend(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Backend>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.backend = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.backend = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `backend_defaults`.\n"]
    pub fn set_backend_defaults(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_BackendDefaults>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.backend_defaults = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.backend_defaults = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Listener>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.listener = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.listener = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_Logging>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_discovery`.\n"]
    pub fn set_service_discovery(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNode_Spec_ServiceDiscovery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_discovery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_discovery = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNode_Spec {
    type O = terrars::BlockAssignable<AppmeshVirtualNode_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNode_Spec {}

impl BuildAppmeshVirtualNode_Spec {
    pub fn build(self) -> AppmeshVirtualNode_Spec {
        AppmeshVirtualNode_Spec {
            backend: core::default::Default::default(),
            backend_defaults: core::default::Default::default(),
            listener: core::default::Default::default(),
            logging: core::default::Default::default(),
            service_discovery: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppmeshVirtualNode_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNode_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNode_SpecRef {
        AppmeshVirtualNode_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNode_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backend_defaults` after provisioning.\n"]
    pub fn backend_defaults(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_BackendDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backend_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_ListenerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> terrars::ListRef<AppmeshVirtualNode_Spec_ServiceDiscoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_discovery", self.base))
    }
}

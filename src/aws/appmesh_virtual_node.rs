use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshVirtualNodeData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecEl>>,
    dynamic: AppmeshVirtualNodeDynamic,
}

struct AppmeshVirtualNode_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppmeshVirtualNodeData>,
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
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecEl>>) -> Self {
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
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElRef> {
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
            data: std::cell::RefCell::new(AppmeshVirtualNodeData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
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
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
    private_key: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl { }

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub private_key: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef {
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
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl { }

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElDynamic {
    file: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl>,
    >,
    sds: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileEl,
                        >,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsEl,
                        >,
                    >,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    type O =
        terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {

}

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        terrars
        ::BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = ""]
    pub exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: self.exact,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic {
    match_: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
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
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
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

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    type O =
        terrars
        ::BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> terrars
    ::ListRef<
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    certificate_authority_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl { }

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    type O =
        terrars
        ::BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[doc = ""]
    pub certificate_authority_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl {
            certificate_authority_arns: self.certificate_authority_arns,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl { }

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    type O =
        terrars
        ::BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl { }

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    type O =
        terrars
        ::BlockAssignable<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElDynamic {
    acm: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl>,
    >,
    file: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl>,
    >,
    sds: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm: std
    ::option
    ::Option<
        std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<
        std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std
    ::option
    ::Option<
        std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl>,
    >,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
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
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmEl,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileEl,
                        >,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsEl,
                        >,
                    >,
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

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> terrars
    ::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElAcmElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars
    ::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> terrars
    ::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElDynamic {
    subject_alternative_names: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    trust: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
                        >,
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
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustEl,
                        >,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    type O =
        terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> terrars
    ::ListRef<
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElTrustElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElDynamic {
    certificate: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl>,
    >,
    validation: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
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
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateEl,
                        >,
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
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationEl,
                        >,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl {
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef {
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
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElCertificateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElValidationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElDynamic {
    tls: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsEl>,
                    >,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl {
            tls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElTlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElDynamic {
    client_policy: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    virtual_service_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_policy: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElVirtualServiceElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    #[doc = ""]
    pub virtual_service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceEl {
            virtual_service_name: self.virtual_service_name,
            client_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
        AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElClientPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendElDynamic {
    virtual_service: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendElVirtualServiceEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_service: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendElVirtualServiceEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendEl {
    #[doc = "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendElVirtualServiceEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendEl {}

impl BuildAppmeshVirtualNodeSpecElBackendEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendEl {
        AppmeshVirtualNodeSpecElBackendEl {
            virtual_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNodeSpecElBackendElRef {
        AppmeshVirtualNodeSpecElBackendElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendElVirtualServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
    private_key: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    type O =
        terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub private_key: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
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
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    type O =
        terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElDynamic {
    file: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>>,
    sds: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl,
                        >,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl,
                        >,
                    >,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl { }

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        terrars
        ::BlockAssignable<
            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = ""]
    pub exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: self.exact,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic {
    match_: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
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
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
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

impl terrars
::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    type O =
        terrars
        ::BlockAssignable<
            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> terrars
    ::ListRef<
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    certificate_authority_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[doc = ""]
    pub certificate_authority_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
            certificate_authority_arns: self.certificate_authority_arns,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElDynamic {
    acm: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>,
    >,
    file: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>,
    >,
    sds: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
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
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl,
                        >,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl,
                        >,
                    >,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    type O =
        terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElDynamic {
    subject_alternative_names: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    trust: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
                        >,
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
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl,
                        >,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> terrars
    ::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElDynamic {
    certificate: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    validation: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
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
                        terrars
                        ::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>,
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
                        terrars
                        ::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationEl>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl {
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef {
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
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElValidationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElDynamic {
    tls: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl {
            tls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElTlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElBackendDefaultsElDynamic {
    client_policy: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElBackendDefaultsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_policy: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>>,
    dynamic: AppmeshVirtualNodeSpecElBackendDefaultsElDynamic,
}

impl AppmeshVirtualNodeSpecElBackendDefaultsEl {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElBackendDefaultsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElBackendDefaultsEl {}

impl BuildAppmeshVirtualNodeSpecElBackendDefaultsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElBackendDefaultsEl {
        AppmeshVirtualNodeSpecElBackendDefaultsEl {
            client_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElBackendDefaultsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElBackendDefaultsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNodeSpecElBackendDefaultsElRef {
        AppmeshVirtualNodeSpecElBackendDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElBackendDefaultsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElClientPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    max_requests: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    #[doc = ""]
    pub max_requests: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl { max_requests: self.max_requests }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    max_connections: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_requests: std::option::Option<terrars::PrimField<f64>>,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    #[doc = "Set the field `max_pending_requests`.\n"]
    pub fn set_max_pending_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pending_requests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    #[doc = ""]
    pub max_connections: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl {
            max_connections: self.max_connections,
            max_pending_requests: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef {
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
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    max_requests: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    #[doc = ""]
    pub max_requests: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El { max_requests: self.max_requests }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    max_connections: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    #[doc = ""]
    pub max_connections: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl { max_connections: self.max_connections }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>>,
    http: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>>,
    http2: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>>,
    tcp: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElConnectionPoolElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2El>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolEl {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            tcp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
        AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElGrpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElHttp2ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp` after provisioning.\n"]
    pub fn tcp(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElTcpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
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

impl AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
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

impl BuildAppmeshVirtualNodeSpecElListenerElHealthCheckEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
        AppmeshVirtualNodeSpecElListenerElHealthCheckEl {
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

pub struct AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
        AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElHealthCheckElRef {
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
pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef {
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
pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef {
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
struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElDynamic {
    base_ejection_duration: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>>,
    interval: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    max_ejection_percent: terrars::PrimField<f64>,
    max_server_errors: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_ejection_duration: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElOutlierDetectionElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    #[doc = "Set the field `base_ejection_duration`.\n"]
    pub fn set_base_ejection_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationEl>,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    #[doc = ""]
    pub max_ejection_percent: terrars::PrimField<f64>,
    #[doc = ""]
    pub max_server_errors: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl {
            max_ejection_percent: self.max_ejection_percent,
            max_server_errors: self.max_server_errors,
            base_ejection_duration: core::default::Default::default(),
            interval: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
        AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef {
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
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElBaseEjectionDurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_ejection_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElIntervalElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interval", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElPortMappingEl {
    port: terrars::PrimField<f64>,
    protocol: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElListenerElPortMappingEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElPortMappingEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElPortMappingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElPortMappingEl {
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElPortMappingEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElPortMappingEl {
        AppmeshVirtualNodeSpecElListenerElPortMappingEl {
            port: self.port,
            protocol: self.protocol,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
        AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElPortMappingElRef {
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
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef {
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
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef {
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
struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>>,
    per_request: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleEl>>,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElIdleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElPerRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef {
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
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef {
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
struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>>,
    per_request: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleEl>>,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElIdleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElPerRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef {
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
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef {
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
struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>>,
    per_request: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleEl>>,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElIdleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElPerRequestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef {
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
struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElDynamic {
    idle: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl {
            idle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElIdleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTimeoutElDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>>,
    http: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>>,
    http2: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>>,
    tcp: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTimeoutElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutEl {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2El>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTimeoutEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTimeoutEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTimeoutEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTimeoutEl {
        AppmeshVirtualNodeSpecElListenerElTimeoutEl {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            tcp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
        AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElGrpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElHttp2ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp` after provisioning.\n"]
    pub fn tcp(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElTcpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    certificate_arn: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    #[doc = ""]
    pub certificate_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl { certificate_arn: self.certificate_arn }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
    private_key: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub private_key: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef {
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
pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl { secret_name: self.secret_name }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElDynamic {
    acm: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>>,
    file: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>>,
    sds: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElCertificateElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmEl>>,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElAcmElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = ""]
    pub exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl { exact: self.exact }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElDynamic {
    match_: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
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
                            AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl { secret_name: self.secret_name }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElDynamic {
    file: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>>,
    sds: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileEl>,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElDynamic {
    subject_alternative_names: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>>,
    trust: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElValidationElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesEl,
                        >,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElTrustElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElTlsElDynamic {
    certificate: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>>,
    validation: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsElValidationEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerElTlsEl {
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsElValidationEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElTlsElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerElTlsEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElCertificateEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsElValidationEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerElTlsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerElTlsEl {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElListenerElTlsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerElTlsEl {
        AppmeshVirtualNodeSpecElListenerElTlsEl {
            mode: self.mode,
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElListenerElTlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElTlsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNodeSpecElListenerElTlsElRef {
        AppmeshVirtualNodeSpecElListenerElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElTlsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElCertificateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElValidationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElListenerElDynamic {
    connection_pool: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElConnectionPoolEl>>,
    health_check: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElHealthCheckEl>>,
    outlier_detection: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>>,
    port_mapping: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElPortMappingEl>>,
    timeout: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTimeoutEl>>,
    tls: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerElTlsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElListenerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pool: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElConnectionPoolEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElHealthCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outlier_detection: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_mapping: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElPortMappingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTimeoutEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerElTlsEl>>,
    dynamic: AppmeshVirtualNodeSpecElListenerElDynamic,
}

impl AppmeshVirtualNodeSpecElListenerEl {
    #[doc = "Set the field `connection_pool`.\n"]
    pub fn set_connection_pool(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElConnectionPoolEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElHealthCheckEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElOutlierDetectionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElPortMappingEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTimeoutEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerElTlsEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElListenerEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElListenerEl {}

impl BuildAppmeshVirtualNodeSpecElListenerEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElListenerEl {
        AppmeshVirtualNodeSpecElListenerEl {
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

pub struct AppmeshVirtualNodeSpecElListenerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElListenerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNodeSpecElListenerElRef {
        AppmeshVirtualNodeSpecElListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElListenerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_pool` after provisioning.\n"]
    pub fn connection_pool(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElConnectionPoolElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `outlier_detection` after provisioning.\n"]
    pub fn outlier_detection(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElOutlierDetectionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outlier_detection", self.base))
    }

    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElPortMappingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElTlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    path: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl { path: self.path }
    }
}

pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElLoggingElAccessLogElDynamic {
    file: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>>,
    dynamic: AppmeshVirtualNodeSpecElLoggingElAccessLogElDynamic,
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElLoggingElAccessLogEl {}

impl BuildAppmeshVirtualNodeSpecElLoggingElAccessLogEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
        AppmeshVirtualNodeSpecElLoggingElAccessLogEl {
            file: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
        AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElLoggingElAccessLogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElLoggingElAccessLogElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElLoggingElDynamic {
    access_log: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElLoggingElAccessLogEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElLoggingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_log: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElLoggingElAccessLogEl>>,
    dynamic: AppmeshVirtualNodeSpecElLoggingElDynamic,
}

impl AppmeshVirtualNodeSpecElLoggingEl {
    #[doc = "Set the field `access_log`.\n"]
    pub fn set_access_log(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElLoggingElAccessLogEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElLoggingEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElLoggingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElLoggingEl {}

impl BuildAppmeshVirtualNodeSpecElLoggingEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElLoggingEl {
        AppmeshVirtualNodeSpecElLoggingEl {
            access_log: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElLoggingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElLoggingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNodeSpecElLoggingElRef {
        AppmeshVirtualNodeSpecElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElLoggingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_log` after provisioning.\n"]
    pub fn access_log(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElLoggingElAccessLogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_log", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    namespace_name: terrars::PrimField<std::string::String>,
    service_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attributes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    #[doc = ""]
    pub namespace_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
        AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl {
            attributes: core::default::Default::default(),
            namespace_name: self.namespace_name,
            service_name: self.service_name,
        }
    }
}

pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
        AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef {
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
pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    hostname: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl { }

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    #[doc = ""]
    pub hostname: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl {
        AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl { hostname: self.hostname }
    }
}

pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
        AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElServiceDiscoveryElDynamic {
    aws_cloud_map: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>>,
    dns: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecElServiceDiscoveryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_cloud_map: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>>,
    dynamic: AppmeshVirtualNodeSpecElServiceDiscoveryElDynamic,
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryEl {
    #[doc = "Set the field `aws_cloud_map`.\n"]
    pub fn set_aws_cloud_map(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecElServiceDiscoveryEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecElServiceDiscoveryEl {}

impl BuildAppmeshVirtualNodeSpecElServiceDiscoveryEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecElServiceDiscoveryEl {
        AppmeshVirtualNodeSpecElServiceDiscoveryEl {
            aws_cloud_map: core::default::Default::default(),
            dns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
        AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElServiceDiscoveryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_cloud_map` after provisioning.\n"]
    pub fn aws_cloud_map(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElServiceDiscoveryElAwsCloudMapElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_cloud_map", self.base))
    }

    #[doc = "Get a reference to the value of field `dns` after provisioning.\n"]
    pub fn dns(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElServiceDiscoveryElDnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeSpecElDynamic {
    backend: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendEl>>,
    backend_defaults: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElBackendDefaultsEl>>,
    listener: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElListenerEl>>,
    logging: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElLoggingEl>>,
    service_discovery: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecElServiceDiscoveryEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualNodeSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_defaults: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElBackendDefaultsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    listener: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElListenerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElLoggingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_discovery: std::option::Option<std::vec::Vec<AppmeshVirtualNodeSpecElServiceDiscoveryEl>>,
    dynamic: AppmeshVirtualNodeSpecElDynamic,
}

impl AppmeshVirtualNodeSpecEl {
    #[doc = "Set the field `backend`.\n"]
    pub fn set_backend(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElBackendDefaultsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElListenerEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElLoggingEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualNodeSpecElServiceDiscoveryEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualNodeSpecEl {
    type O = terrars::BlockAssignable<AppmeshVirtualNodeSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualNodeSpecEl {}

impl BuildAppmeshVirtualNodeSpecEl {
    pub fn build(self) -> AppmeshVirtualNodeSpecEl {
        AppmeshVirtualNodeSpecEl {
            backend: core::default::Default::default(),
            backend_defaults: core::default::Default::default(),
            listener: core::default::Default::default(),
            logging: core::default::Default::default(),
            service_discovery: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualNodeSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualNodeSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualNodeSpecElRef {
        AppmeshVirtualNodeSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualNodeSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backend_defaults` after provisioning.\n"]
    pub fn backend_defaults(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElBackendDefaultsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backend_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElListenerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }

    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> terrars::ListRef<AppmeshVirtualNodeSpecElServiceDiscoveryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_discovery", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualNodeDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppmeshVirtualNodeSpecEl>>,
}

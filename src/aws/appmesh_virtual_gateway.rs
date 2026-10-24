use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshVirtualGatewayData {
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
    spec: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecEl>>,
    dynamic: AppmeshVirtualGatewayDynamic,
}

struct AppmeshVirtualGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppmeshVirtualGatewayData>,
}

#[derive(Clone)]
pub struct AppmeshVirtualGateway(std::rc::Rc<AppmeshVirtualGateway_>);

impl AppmeshVirtualGateway {
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
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecEl>>) -> Self {
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
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

impl terrars::Referable for AppmeshVirtualGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppmeshVirtualGateway { }

impl terrars::ToListMappable for AppmeshVirtualGateway {
    type O = terrars::ListRef<AppmeshVirtualGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppmeshVirtualGateway_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appmesh_virtual_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppmeshVirtualGateway {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> AppmeshVirtualGateway {
        let out = AppmeshVirtualGateway(std::rc::Rc::new(AppmeshVirtualGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppmeshVirtualGatewayData {
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

pub struct AppmeshVirtualGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppmeshVirtualGatewayRef {
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
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
    private_key: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    type O =
        terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub private_key: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef {
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
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    type O =
        terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElDynamic {
    file: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>,
    >,
    sds: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElDynamic,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
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
                            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileEl,
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
                            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsEl,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl { }

impl terrars
::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        terrars
        ::BlockAssignable<
            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = ""]
    pub exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl {
            exact: self.exact,
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic {
    match_: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
        >,
    >,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElDynamic,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
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
                            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchEl,
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
::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    type O =
        terrars
        ::BlockAssignable<
            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(
        self,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> terrars
    ::ListRef<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElMatchElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    certificate_authority_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl { }

impl terrars
::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    #[doc = ""]
    pub certificate_authority_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl {
            certificate_authority_arns: self.certificate_authority_arns,
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl { }

impl terrars
::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl { }

impl terrars
::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl {
            secret_name: self.secret_name,
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElDynamic {
    acm: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>,
    >,
    file: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>,
    >,
    sds: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElDynamic,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
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
                            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmEl,
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
                            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileEl,
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
                            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsEl,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    type O =
        terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElAcmElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars
    ::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElDynamic {
    subject_alternative_names: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
        >,
    >,
    trust: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl>>,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElDynamic,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
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
                            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesEl,
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
                            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustEl,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> terrars
    ::ListRef<
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElSubjectAlternativeNamesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElTrustElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElDynamic {
    certificate: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    validation: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl>>,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElDynamic,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
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
                            AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateEl,
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
                        ::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationEl>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl {
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef {
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
    ) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElCertificateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElValidationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElDynamic {
    tls: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElDynamic,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl {
            tls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElTlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElBackendDefaultsElDynamic {
    client_policy: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElBackendDefaultsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_policy: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>>,
    dynamic: AppmeshVirtualGatewaySpecElBackendDefaultsElDynamic,
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsEl {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElBackendDefaultsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElBackendDefaultsEl {}

impl BuildAppmeshVirtualGatewaySpecElBackendDefaultsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElBackendDefaultsEl {
        AppmeshVirtualGatewaySpecElBackendDefaultsEl {
            client_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
        AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElBackendDefaultsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElClientPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    max_requests: terrars::PrimField<f64>,
}

impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    #[doc = ""]
    pub max_requests: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl { max_requests: self.max_requests }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    max_connections: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_requests: std::option::Option<terrars::PrimField<f64>>,
}

impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    #[doc = "Set the field `max_pending_requests`.\n"]
    pub fn set_max_pending_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pending_requests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    #[doc = ""]
    pub max_connections: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl {
            max_connections: self.max_connections,
            max_pending_requests: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef {
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
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    max_requests: terrars::PrimField<f64>,
}

impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    #[doc = ""]
    pub max_requests: terrars::PrimField<f64>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El { max_requests: self.max_requests }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>>,
    http: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>>,
    http2: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElConnectionPoolElDynamic,
}

impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcEl>>,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpEl>>,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2El>>,
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
}

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {}

impl BuildAppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
        AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElGrpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElHttp2ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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

impl AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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

impl BuildAppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
        AppmeshVirtualGatewaySpecElListenerElHealthCheckEl {
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

pub struct AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
        AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef {
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
pub struct AppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    port: terrars::PrimField<f64>,
    protocol: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElListenerElPortMappingEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElPortMappingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElPortMappingEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElPortMappingEl {
        AppmeshVirtualGatewaySpecElListenerElPortMappingEl {
            port: self.port,
            protocol: self.protocol,
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
        AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElPortMappingElRef {
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
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    certificate_arn: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    #[doc = ""]
    pub certificate_arn: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl { certificate_arn: self.certificate_arn }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
    private_key: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub private_key: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl {
            certificate_chain: self.certificate_chain,
            private_key: self.private_key,
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef {
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
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl { secret_name: self.secret_name }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElDynamic {
    acm: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>>,
    file: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>>,
    sds: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElDynamic,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmEl>>,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileEl>>,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElAcmElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl { }

impl terrars
::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    type O =
        terrars
        ::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    #[doc = ""]
    pub exact: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl { exact: self.exact }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElDynamic {
    match_: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>,
    >,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElDynamic,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
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
                            AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchEl,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    type O =
        terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl {
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    certificate_chain: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    #[doc = ""]
    pub certificate_chain: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl {
            certificate_chain: self.certificate_chain,
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    secret_name: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    #[doc = ""]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl { secret_name: self.secret_name }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElDynamic {
    file: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>>,
    sds: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElDynamic,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileEl>,
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
                        terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsEl>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElSdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElDynamic {
    subject_alternative_names: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>>,
    trust: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std
    ::option
    ::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElValidationElDynamic,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
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
                            AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesEl,
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
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElSubjectAlternativeNamesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElTrustElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElTlsElDynamic {
    certificate: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>>,
    validation: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerElTlsEl {
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElTlsElDynamic,
}

impl AppmeshVirtualGatewaySpecElListenerElTlsEl {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsElValidationEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerElTlsEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerElTlsEl {
    #[doc = ""]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElListenerElTlsEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerElTlsEl {
        AppmeshVirtualGatewaySpecElListenerElTlsEl {
            mode: self.mode,
            certificate: core::default::Default::default(),
            validation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElTlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElTlsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualGatewaySpecElListenerElTlsElRef {
        AppmeshVirtualGatewaySpecElListenerElTlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElTlsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElCertificateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElValidationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElListenerElDynamic {
    connection_pool: std
    ::option
    ::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>>,
    health_check: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElHealthCheckEl>>,
    port_mapping: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElPortMappingEl>>,
    tls: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerElTlsEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElListenerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pool: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElHealthCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_mapping: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElPortMappingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerElTlsEl>>,
    dynamic: AppmeshVirtualGatewaySpecElListenerElDynamic,
}

impl AppmeshVirtualGatewaySpecElListenerEl {
    #[doc = "Set the field `connection_pool`.\n"]
    pub fn set_connection_pool(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElConnectionPoolEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElHealthCheckEl>>,
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

    #[doc = "Set the field `port_mapping`.\n"]
    pub fn set_port_mapping(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElPortMappingEl>>,
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

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerElTlsEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElListenerEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElListenerEl {}

impl BuildAppmeshVirtualGatewaySpecElListenerEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElListenerEl {
        AppmeshVirtualGatewaySpecElListenerEl {
            connection_pool: core::default::Default::default(),
            health_check: core::default::Default::default(),
            port_mapping: core::default::Default::default(),
            tls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElListenerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElListenerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualGatewaySpecElListenerElRef {
        AppmeshVirtualGatewaySpecElListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElListenerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_pool` after provisioning.\n"]
    pub fn connection_pool(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElConnectionPoolElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElPortMappingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElTlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    path: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl { }

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl { path: self.path }
    }
}

pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElDynamic {
    file: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>>,
    dynamic: AppmeshVirtualGatewaySpecElLoggingElAccessLogElDynamic,
}

impl AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {}

impl BuildAppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogEl {
            file: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
        AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElLoggingElAccessLogElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElLoggingElDynamic {
    access_log: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElLoggingElAccessLogEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecElLoggingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_log: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElLoggingElAccessLogEl>>,
    dynamic: AppmeshVirtualGatewaySpecElLoggingElDynamic,
}

impl AppmeshVirtualGatewaySpecElLoggingEl {
    #[doc = "Set the field `access_log`.\n"]
    pub fn set_access_log(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElLoggingElAccessLogEl>>,
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

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecElLoggingEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecElLoggingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecElLoggingEl {}

impl BuildAppmeshVirtualGatewaySpecElLoggingEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecElLoggingEl {
        AppmeshVirtualGatewaySpecElLoggingEl {
            access_log: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElLoggingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElLoggingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualGatewaySpecElLoggingElRef {
        AppmeshVirtualGatewaySpecElLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElLoggingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_log` after provisioning.\n"]
    pub fn access_log(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElLoggingElAccessLogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_log", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewaySpecElDynamic {
    backend_defaults: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElBackendDefaultsEl>>,
    listener: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElListenerEl>>,
    logging: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecElLoggingEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualGatewaySpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_defaults: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElBackendDefaultsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    listener: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElListenerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<std::vec::Vec<AppmeshVirtualGatewaySpecElLoggingEl>>,
    dynamic: AppmeshVirtualGatewaySpecElDynamic,
}

impl AppmeshVirtualGatewaySpecEl {
    #[doc = "Set the field `backend_defaults`.\n"]
    pub fn set_backend_defaults(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElBackendDefaultsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElListenerEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualGatewaySpecElLoggingEl>>,
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
}

impl terrars::ToListMappable for AppmeshVirtualGatewaySpecEl {
    type O = terrars::BlockAssignable<AppmeshVirtualGatewaySpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualGatewaySpecEl {}

impl BuildAppmeshVirtualGatewaySpecEl {
    pub fn build(self) -> AppmeshVirtualGatewaySpecEl {
        AppmeshVirtualGatewaySpecEl {
            backend_defaults: core::default::Default::default(),
            listener: core::default::Default::default(),
            logging: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualGatewaySpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualGatewaySpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualGatewaySpecElRef {
        AppmeshVirtualGatewaySpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualGatewaySpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backend_defaults` after provisioning.\n"]
    pub fn backend_defaults(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElBackendDefaultsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backend_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElListenerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<AppmeshVirtualGatewaySpecElLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualGatewayDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppmeshVirtualGatewaySpecEl>>,
}

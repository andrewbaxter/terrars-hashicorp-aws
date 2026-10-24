use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodeartifactRepositoryData {
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
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    repository: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_connections: std::option::Option<std::vec::Vec<CodeartifactRepositoryExternalConnectionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstream: std::option::Option<std::vec::Vec<CodeartifactRepositoryUpstreamEl>>,
    dynamic: CodeartifactRepositoryDynamic,
}

struct CodeartifactRepository_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodeartifactRepositoryData>,
}

#[derive(Clone)]
pub struct CodeartifactRepository(std::rc::Rc<CodeartifactRepository_>);

impl CodeartifactRepository {
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

    #[doc = "Set the field `domain_owner`.\n"]
    pub fn set_domain_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domain_owner = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `external_connections`.\n"]
    pub fn set_external_connections(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodeartifactRepositoryExternalConnectionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_connections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_connections = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upstream`.\n"]
    pub fn set_upstream(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodeartifactRepositoryUpstreamEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upstream = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upstream = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `administrator_account` after provisioning.\n"]
    pub fn administrator_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.administrator_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_owner` after provisioning.\n"]
    pub fn domain_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_connections` after provisioning.\n"]
    pub fn external_connections(&self) -> terrars::ListRef<CodeartifactRepositoryExternalConnectionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_connections", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upstream` after provisioning.\n"]
    pub fn upstream(&self) -> terrars::ListRef<CodeartifactRepositoryUpstreamElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstream", self.extract_ref()))
    }
}

impl terrars::Referable for CodeartifactRepository {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodeartifactRepository { }

impl terrars::ToListMappable for CodeartifactRepository {
    type O = terrars::ListRef<CodeartifactRepositoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodeartifactRepository_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codeartifact_repository".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodeartifactRepository {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub repository: terrars::PrimField<std::string::String>,
}

impl BuildCodeartifactRepository {
    pub fn build(self, stack: &mut terrars::Stack) -> CodeartifactRepository {
        let out = CodeartifactRepository(std::rc::Rc::new(CodeartifactRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodeartifactRepositoryData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                domain: self.domain,
                domain_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                repository: self.repository,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                external_connections: core::default::Default::default(),
                upstream: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodeartifactRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodeartifactRepositoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodeartifactRepositoryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `administrator_account` after provisioning.\n"]
    pub fn administrator_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.administrator_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_owner` after provisioning.\n"]
    pub fn domain_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_connections` after provisioning.\n"]
    pub fn external_connections(&self) -> terrars::ListRef<CodeartifactRepositoryExternalConnectionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_connections", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upstream` after provisioning.\n"]
    pub fn upstream(&self) -> terrars::ListRef<CodeartifactRepositoryUpstreamElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstream", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodeartifactRepositoryExternalConnectionsEl {
    external_connection_name: terrars::PrimField<std::string::String>,
}

impl CodeartifactRepositoryExternalConnectionsEl { }

impl terrars::ToListMappable for CodeartifactRepositoryExternalConnectionsEl {
    type O = terrars::BlockAssignable<CodeartifactRepositoryExternalConnectionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodeartifactRepositoryExternalConnectionsEl {
    #[doc = ""]
    pub external_connection_name: terrars::PrimField<std::string::String>,
}

impl BuildCodeartifactRepositoryExternalConnectionsEl {
    pub fn build(self) -> CodeartifactRepositoryExternalConnectionsEl {
        CodeartifactRepositoryExternalConnectionsEl { external_connection_name: self.external_connection_name }
    }
}

pub struct CodeartifactRepositoryExternalConnectionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodeartifactRepositoryExternalConnectionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodeartifactRepositoryExternalConnectionsElRef {
        CodeartifactRepositoryExternalConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodeartifactRepositoryExternalConnectionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_connection_name` after provisioning.\n"]
    pub fn external_connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `package_format` after provisioning.\n"]
    pub fn package_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_format", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct CodeartifactRepositoryUpstreamEl {
    repository_name: terrars::PrimField<std::string::String>,
}

impl CodeartifactRepositoryUpstreamEl { }

impl terrars::ToListMappable for CodeartifactRepositoryUpstreamEl {
    type O = terrars::BlockAssignable<CodeartifactRepositoryUpstreamEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodeartifactRepositoryUpstreamEl {
    #[doc = ""]
    pub repository_name: terrars::PrimField<std::string::String>,
}

impl BuildCodeartifactRepositoryUpstreamEl {
    pub fn build(self) -> CodeartifactRepositoryUpstreamEl {
        CodeartifactRepositoryUpstreamEl { repository_name: self.repository_name }
    }
}

pub struct CodeartifactRepositoryUpstreamElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodeartifactRepositoryUpstreamElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodeartifactRepositoryUpstreamElRef {
        CodeartifactRepositoryUpstreamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodeartifactRepositoryUpstreamElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodeartifactRepositoryDynamic {
    external_connections: std::option::Option<terrars::DynamicBlock<CodeartifactRepositoryExternalConnectionsEl>>,
    upstream: std::option::Option<terrars::DynamicBlock<CodeartifactRepositoryUpstreamEl>>,
}

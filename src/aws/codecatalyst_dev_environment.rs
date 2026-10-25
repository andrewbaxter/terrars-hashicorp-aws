#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodecatalystDevEnvironmentData_ {
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
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inactivity_timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    instance_type: terrars::PrimField<std::string::String>,
    project_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    space_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ides: std::option::Option<std::vec::Vec<CodecatalystDevEnvironment_Ides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistent_storage: std::option::Option<std::vec::Vec<CodecatalystDevEnvironment_PersistentStorage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repositories: std::option::Option<std::vec::Vec<CodecatalystDevEnvironment_Repositories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CodecatalystDevEnvironment_Timeouts>,
    dynamic: CodecatalystDevEnvironmentDynamic,
}

struct CodecatalystDevEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodecatalystDevEnvironmentData_>,
}

#[derive(Clone)]
pub struct CodecatalystDevEnvironment(std::rc::Rc<CodecatalystDevEnvironment_>);

impl CodecatalystDevEnvironment {
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

    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inactivity_timeout_minutes`.\n"]
    pub fn set_inactivity_timeout_minutes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().inactivity_timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ides`.\n"]
    pub fn set_ides(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodecatalystDevEnvironment_Ides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `persistent_storage`.\n"]
    pub fn set_persistent_storage(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodecatalystDevEnvironment_PersistentStorage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().persistent_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.persistent_storage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `repositories`.\n"]
    pub fn set_repositories(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodecatalystDevEnvironment_Repositories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().repositories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.repositories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CodecatalystDevEnvironment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inactivity_timeout_minutes` after provisioning.\n"]
    pub fn inactivity_timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inactivity_timeout_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ides` after provisioning.\n"]
    pub fn ides(&self) -> terrars::ListRef<CodecatalystDevEnvironment_IdesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistent_storage` after provisioning.\n"]
    pub fn persistent_storage(&self) -> terrars::ListRef<CodecatalystDevEnvironment_PersistentStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistent_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> terrars::ListRef<CodecatalystDevEnvironment_RepositoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repositories", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CodecatalystDevEnvironment_TimeoutsRef {
        CodecatalystDevEnvironment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CodecatalystDevEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodecatalystDevEnvironment { }

impl terrars::ToListMappable for CodecatalystDevEnvironment {
    type O = terrars::ListRef<CodecatalystDevEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodecatalystDevEnvironment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codecatalyst_dev_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodecatalystDevEnvironment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub project_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub space_name: terrars::PrimField<std::string::String>,
}

impl BuildCodecatalystDevEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> CodecatalystDevEnvironment {
        let out = CodecatalystDevEnvironment(std::rc::Rc::new(CodecatalystDevEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodecatalystDevEnvironmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alias: core::default::Default::default(),
                id: core::default::Default::default(),
                inactivity_timeout_minutes: core::default::Default::default(),
                instance_type: self.instance_type,
                project_name: self.project_name,
                region: core::default::Default::default(),
                space_name: self.space_name,
                ides: core::default::Default::default(),
                persistent_storage: core::default::Default::default(),
                repositories: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodecatalystDevEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodecatalystDevEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodecatalystDevEnvironmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inactivity_timeout_minutes` after provisioning.\n"]
    pub fn inactivity_timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inactivity_timeout_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_name` after provisioning.\n"]
    pub fn space_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ides` after provisioning.\n"]
    pub fn ides(&self) -> terrars::ListRef<CodecatalystDevEnvironment_IdesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistent_storage` after provisioning.\n"]
    pub fn persistent_storage(&self) -> terrars::ListRef<CodecatalystDevEnvironment_PersistentStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistent_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> terrars::ListRef<CodecatalystDevEnvironment_RepositoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repositories", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CodecatalystDevEnvironment_TimeoutsRef {
        CodecatalystDevEnvironment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CodecatalystDevEnvironmentDynamic {
    ides: std::option::Option<terrars::DynamicBlock<CodecatalystDevEnvironment_Ides>>,
    persistent_storage: std::option::Option<terrars::DynamicBlock<CodecatalystDevEnvironment_PersistentStorage>>,
    repositories: std::option::Option<terrars::DynamicBlock<CodecatalystDevEnvironment_Repositories>>,
}

#[derive(Serialize)]
pub struct CodecatalystDevEnvironment_Ides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodecatalystDevEnvironment_Ides {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.runtime = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodecatalystDevEnvironment_Ides {
    type O = terrars::BlockAssignable<CodecatalystDevEnvironment_Ides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodecatalystDevEnvironment_Ides {}

impl BuildCodecatalystDevEnvironment_Ides {
    pub fn build(self) -> CodecatalystDevEnvironment_Ides {
        CodecatalystDevEnvironment_Ides {
            name: core::default::Default::default(),
            runtime: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodecatalystDevEnvironment_IdesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodecatalystDevEnvironment_IdesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodecatalystDevEnvironment_IdesRef {
        CodecatalystDevEnvironment_IdesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodecatalystDevEnvironment_IdesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }
}

#[derive(Serialize)]
pub struct CodecatalystDevEnvironment_PersistentStorage {
    size: terrars::PrimField<f64>,
}

impl CodecatalystDevEnvironment_PersistentStorage { }

impl terrars::ToListMappable for CodecatalystDevEnvironment_PersistentStorage {
    type O = terrars::BlockAssignable<CodecatalystDevEnvironment_PersistentStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodecatalystDevEnvironment_PersistentStorage {
    #[doc = ""]
    pub size: terrars::PrimField<f64>,
}

impl BuildCodecatalystDevEnvironment_PersistentStorage {
    pub fn build(self) -> CodecatalystDevEnvironment_PersistentStorage {
        CodecatalystDevEnvironment_PersistentStorage { size: self.size }
    }
}

#[allow(dead_code)]
pub struct CodecatalystDevEnvironment_PersistentStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodecatalystDevEnvironment_PersistentStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodecatalystDevEnvironment_PersistentStorageRef {
        CodecatalystDevEnvironment_PersistentStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodecatalystDevEnvironment_PersistentStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize)]
pub struct CodecatalystDevEnvironment_Repositories {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch_name: std::option::Option<terrars::PrimField<std::string::String>>,
    repository_name: terrars::PrimField<std::string::String>,
}

impl CodecatalystDevEnvironment_Repositories {
    #[doc = "Set the field `branch_name`.\n"]
    pub fn set_branch_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodecatalystDevEnvironment_Repositories {
    type O = terrars::BlockAssignable<CodecatalystDevEnvironment_Repositories>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodecatalystDevEnvironment_Repositories {
    #[doc = ""]
    pub repository_name: terrars::PrimField<std::string::String>,
}

impl BuildCodecatalystDevEnvironment_Repositories {
    pub fn build(self) -> CodecatalystDevEnvironment_Repositories {
        CodecatalystDevEnvironment_Repositories {
            branch_name: core::default::Default::default(),
            repository_name: self.repository_name,
        }
    }
}

#[allow(dead_code)]
pub struct CodecatalystDevEnvironment_RepositoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodecatalystDevEnvironment_RepositoriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodecatalystDevEnvironment_RepositoriesRef {
        CodecatalystDevEnvironment_RepositoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodecatalystDevEnvironment_RepositoriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodecatalystDevEnvironment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodecatalystDevEnvironment_Timeouts {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodecatalystDevEnvironment_Timeouts {
    type O = terrars::BlockAssignable<CodecatalystDevEnvironment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodecatalystDevEnvironment_Timeouts {}

impl BuildCodecatalystDevEnvironment_Timeouts {
    pub fn build(self) -> CodecatalystDevEnvironment_Timeouts {
        CodecatalystDevEnvironment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodecatalystDevEnvironment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodecatalystDevEnvironment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodecatalystDevEnvironment_TimeoutsRef {
        CodecatalystDevEnvironment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodecatalystDevEnvironment_TimeoutsRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

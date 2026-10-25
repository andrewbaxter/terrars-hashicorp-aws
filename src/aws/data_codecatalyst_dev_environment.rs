#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCodecatalystDevEnvironmentData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creator_id: std::option::Option<terrars::PrimField<std::string::String>>,
    env_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    project_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    space_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repositories: std::option::Option<std::vec::Vec<DataCodecatalystDevEnvironment_Repositories>>,
    dynamic: DataCodecatalystDevEnvironmentDynamic,
}

struct DataCodecatalystDevEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCodecatalystDevEnvironmentData_>,
}

#[derive(Clone)]
pub struct DataCodecatalystDevEnvironment(std::rc::Rc<DataCodecatalystDevEnvironment_>);

impl DataCodecatalystDevEnvironment {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `creator_id`.\n"]
    pub fn set_creator_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().creator_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `repositories`.\n"]
    pub fn set_repositories(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCodecatalystDevEnvironment_Repositories>>,
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

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_id` after provisioning.\n"]
    pub fn creator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `env_id` after provisioning.\n"]
    pub fn env_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ides` after provisioning.\n"]
    pub fn ides(&self) -> terrars::ListRef<DataCodecatalystDevEnvironment_IdesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inactivity_timeout_minutes` after provisioning.\n"]
    pub fn inactivity_timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inactivity_timeout_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistent_storage` after provisioning.\n"]
    pub fn persistent_storage(&self) -> terrars::ListRef<DataCodecatalystDevEnvironment_PersistentStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistent_storage", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> terrars::ListRef<DataCodecatalystDevEnvironment_RepositoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repositories", self.extract_ref()))
    }
}

impl terrars::Referable for DataCodecatalystDevEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCodecatalystDevEnvironment { }

impl terrars::ToListMappable for DataCodecatalystDevEnvironment {
    type O = terrars::ListRef<DataCodecatalystDevEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCodecatalystDevEnvironment_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_codecatalyst_dev_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCodecatalystDevEnvironment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub env_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub project_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub space_name: terrars::PrimField<std::string::String>,
}

impl BuildDataCodecatalystDevEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCodecatalystDevEnvironment {
        let out = DataCodecatalystDevEnvironment(std::rc::Rc::new(DataCodecatalystDevEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCodecatalystDevEnvironmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                alias: core::default::Default::default(),
                creator_id: core::default::Default::default(),
                env_id: self.env_id,
                id: core::default::Default::default(),
                project_name: self.project_name,
                region: core::default::Default::default(),
                space_name: self.space_name,
                tags: core::default::Default::default(),
                repositories: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCodecatalystDevEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodecatalystDevEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCodecatalystDevEnvironmentRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creator_id` after provisioning.\n"]
    pub fn creator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `env_id` after provisioning.\n"]
    pub fn env_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ides` after provisioning.\n"]
    pub fn ides(&self) -> terrars::ListRef<DataCodecatalystDevEnvironment_IdesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inactivity_timeout_minutes` after provisioning.\n"]
    pub fn inactivity_timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inactivity_timeout_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistent_storage` after provisioning.\n"]
    pub fn persistent_storage(&self) -> terrars::ListRef<DataCodecatalystDevEnvironment_PersistentStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistent_storage", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> terrars::ListRef<DataCodecatalystDevEnvironment_RepositoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repositories", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataCodecatalystDevEnvironmentDynamic {
    repositories: std::option::Option<terrars::DynamicBlock<DataCodecatalystDevEnvironment_Repositories>>,
}

#[derive(Serialize)]
pub struct DataCodecatalystDevEnvironment_Ides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCodecatalystDevEnvironment_Ides {
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

impl terrars::ToListMappable for DataCodecatalystDevEnvironment_Ides {
    type O = terrars::BlockAssignable<DataCodecatalystDevEnvironment_Ides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodecatalystDevEnvironment_Ides {}

impl BuildDataCodecatalystDevEnvironment_Ides {
    pub fn build(self) -> DataCodecatalystDevEnvironment_Ides {
        DataCodecatalystDevEnvironment_Ides {
            name: core::default::Default::default(),
            runtime: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCodecatalystDevEnvironment_IdesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodecatalystDevEnvironment_IdesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCodecatalystDevEnvironment_IdesRef {
        DataCodecatalystDevEnvironment_IdesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodecatalystDevEnvironment_IdesRef {
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
pub struct DataCodecatalystDevEnvironment_PersistentStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCodecatalystDevEnvironment_PersistentStorage {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCodecatalystDevEnvironment_PersistentStorage {
    type O = terrars::BlockAssignable<DataCodecatalystDevEnvironment_PersistentStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodecatalystDevEnvironment_PersistentStorage {}

impl BuildDataCodecatalystDevEnvironment_PersistentStorage {
    pub fn build(self) -> DataCodecatalystDevEnvironment_PersistentStorage {
        DataCodecatalystDevEnvironment_PersistentStorage { size: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCodecatalystDevEnvironment_PersistentStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodecatalystDevEnvironment_PersistentStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCodecatalystDevEnvironment_PersistentStorageRef {
        DataCodecatalystDevEnvironment_PersistentStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodecatalystDevEnvironment_PersistentStorageRef {
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
pub struct DataCodecatalystDevEnvironment_Repositories {}

impl DataCodecatalystDevEnvironment_Repositories { }

impl terrars::ToListMappable for DataCodecatalystDevEnvironment_Repositories {
    type O = terrars::BlockAssignable<DataCodecatalystDevEnvironment_Repositories>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodecatalystDevEnvironment_Repositories {}

impl BuildDataCodecatalystDevEnvironment_Repositories {
    pub fn build(self) -> DataCodecatalystDevEnvironment_Repositories {
        DataCodecatalystDevEnvironment_Repositories {}
    }
}

#[allow(dead_code)]
pub struct DataCodecatalystDevEnvironment_RepositoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodecatalystDevEnvironment_RepositoriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCodecatalystDevEnvironment_RepositoriesRef {
        DataCodecatalystDevEnvironment_RepositoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodecatalystDevEnvironment_RepositoriesRef {
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

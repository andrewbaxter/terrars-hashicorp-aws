use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEfsAccessPointData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    access_point_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataEfsAccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEfsAccessPointData>,
}

#[derive(Clone)]
pub struct DataEfsAccessPoint(std::rc::Rc<DataEfsAccessPoint_>);

impl DataEfsAccessPoint {
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

    #[doc = "Get a reference to the value of field `access_point_id` after provisioning.\n"]
    pub fn access_point_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_arn` after provisioning.\n"]
    pub fn file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> terrars::ListRef<DataEfsAccessPointPosixUserElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::ListRef<DataEfsAccessPointRootDirectoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataEfsAccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEfsAccessPoint { }

impl terrars::ToListMappable for DataEfsAccessPoint {
    type O = terrars::ListRef<DataEfsAccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEfsAccessPoint_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_efs_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEfsAccessPoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub access_point_id: terrars::PrimField<std::string::String>,
}

impl BuildDataEfsAccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEfsAccessPoint {
        let out = DataEfsAccessPoint(std::rc::Rc::new(DataEfsAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEfsAccessPointData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                access_point_id: self.access_point_id,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEfsAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEfsAccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEfsAccessPointRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_point_id` after provisioning.\n"]
    pub fn access_point_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_arn` after provisioning.\n"]
    pub fn file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> terrars::ListRef<DataEfsAccessPointPosixUserElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::ListRef<DataEfsAccessPointRootDirectoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEfsAccessPointPosixUserEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_gids: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEfsAccessPointPosixUserEl {
    #[doc = "Set the field `gid`.\n"]
    pub fn set_gid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_gids`.\n"]
    pub fn set_secondary_gids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.secondary_gids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.uid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEfsAccessPointPosixUserEl {
    type O = terrars::BlockAssignable<DataEfsAccessPointPosixUserEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEfsAccessPointPosixUserEl {}

impl BuildDataEfsAccessPointPosixUserEl {
    pub fn build(self) -> DataEfsAccessPointPosixUserEl {
        DataEfsAccessPointPosixUserEl {
            gid: core::default::Default::default(),
            secondary_gids: core::default::Default::default(),
            uid: core::default::Default::default(),
        }
    }
}

pub struct DataEfsAccessPointPosixUserElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEfsAccessPointPosixUserElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEfsAccessPointPosixUserElRef {
        DataEfsAccessPointPosixUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEfsAccessPointPosixUserElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_gids` after provisioning.\n"]
    pub fn secondary_gids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.secondary_gids", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEfsAccessPointRootDirectoryElCreationInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_gid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_uid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEfsAccessPointRootDirectoryElCreationInfoEl {
    #[doc = "Set the field `owner_gid`.\n"]
    pub fn set_owner_gid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.owner_gid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner_uid`.\n"]
    pub fn set_owner_uid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.owner_uid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permissions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEfsAccessPointRootDirectoryElCreationInfoEl {
    type O = terrars::BlockAssignable<DataEfsAccessPointRootDirectoryElCreationInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEfsAccessPointRootDirectoryElCreationInfoEl {}

impl BuildDataEfsAccessPointRootDirectoryElCreationInfoEl {
    pub fn build(self) -> DataEfsAccessPointRootDirectoryElCreationInfoEl {
        DataEfsAccessPointRootDirectoryElCreationInfoEl {
            owner_gid: core::default::Default::default(),
            owner_uid: core::default::Default::default(),
            permissions: core::default::Default::default(),
        }
    }
}

pub struct DataEfsAccessPointRootDirectoryElCreationInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEfsAccessPointRootDirectoryElCreationInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEfsAccessPointRootDirectoryElCreationInfoElRef {
        DataEfsAccessPointRootDirectoryElCreationInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEfsAccessPointRootDirectoryElCreationInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `owner_gid` after provisioning.\n"]
    pub fn owner_gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_gid", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_uid` after provisioning.\n"]
    pub fn owner_uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_uid", self.base))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEfsAccessPointRootDirectoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_info: std::option::Option<terrars::ListField<DataEfsAccessPointRootDirectoryElCreationInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEfsAccessPointRootDirectoryEl {
    #[doc = "Set the field `creation_info`.\n"]
    pub fn set_creation_info(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEfsAccessPointRootDirectoryElCreationInfoEl>>,
    ) -> Self {
        self.creation_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEfsAccessPointRootDirectoryEl {
    type O = terrars::BlockAssignable<DataEfsAccessPointRootDirectoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEfsAccessPointRootDirectoryEl {}

impl BuildDataEfsAccessPointRootDirectoryEl {
    pub fn build(self) -> DataEfsAccessPointRootDirectoryEl {
        DataEfsAccessPointRootDirectoryEl {
            creation_info: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct DataEfsAccessPointRootDirectoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEfsAccessPointRootDirectoryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEfsAccessPointRootDirectoryElRef {
        DataEfsAccessPointRootDirectoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEfsAccessPointRootDirectoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `creation_info` after provisioning.\n"]
    pub fn creation_info(&self) -> terrars::ListRef<DataEfsAccessPointRootDirectoryElCreationInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creation_info", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EfsAccessPointData {
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
    file_system_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    posix_user: std::option::Option<std::vec::Vec<EfsAccessPointPosixUserEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_directory: std::option::Option<std::vec::Vec<EfsAccessPointRootDirectoryEl>>,
    dynamic: EfsAccessPointDynamic,
}

struct EfsAccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EfsAccessPointData>,
}

#[derive(Clone)]
pub struct EfsAccessPoint(std::rc::Rc<EfsAccessPoint_>);

impl EfsAccessPoint {
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

    #[doc = "Set the field `posix_user`.\n"]
    pub fn set_posix_user(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EfsAccessPointPosixUserEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().posix_user = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.posix_user = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EfsAccessPointRootDirectoryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_directory = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_directory = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> terrars::ListRef<EfsAccessPointPosixUserElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::ListRef<EfsAccessPointRootDirectoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_directory", self.extract_ref()))
    }
}

impl terrars::Referable for EfsAccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EfsAccessPoint { }

impl terrars::ToListMappable for EfsAccessPoint {
    type O = terrars::ListRef<EfsAccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EfsAccessPoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_efs_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEfsAccessPoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
}

impl BuildEfsAccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> EfsAccessPoint {
        let out = EfsAccessPoint(std::rc::Rc::new(EfsAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EfsAccessPointData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                file_system_id: self.file_system_id,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                posix_user: core::default::Default::default(),
                root_directory: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EfsAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsAccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EfsAccessPointRef {
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> terrars::ListRef<EfsAccessPointPosixUserElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::ListRef<EfsAccessPointRootDirectoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_directory", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EfsAccessPointPosixUserEl {
    gid: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_gids: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    uid: terrars::PrimField<f64>,
}

impl EfsAccessPointPosixUserEl {
    #[doc = "Set the field `secondary_gids`.\n"]
    pub fn set_secondary_gids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.secondary_gids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EfsAccessPointPosixUserEl {
    type O = terrars::BlockAssignable<EfsAccessPointPosixUserEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEfsAccessPointPosixUserEl {
    #[doc = ""]
    pub gid: terrars::PrimField<f64>,
    #[doc = ""]
    pub uid: terrars::PrimField<f64>,
}

impl BuildEfsAccessPointPosixUserEl {
    pub fn build(self) -> EfsAccessPointPosixUserEl {
        EfsAccessPointPosixUserEl {
            gid: self.gid,
            secondary_gids: core::default::Default::default(),
            uid: self.uid,
        }
    }
}

pub struct EfsAccessPointPosixUserElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsAccessPointPosixUserElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EfsAccessPointPosixUserElRef {
        EfsAccessPointPosixUserElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EfsAccessPointPosixUserElRef {
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
pub struct EfsAccessPointRootDirectoryElCreationInfoEl {
    owner_gid: terrars::PrimField<f64>,
    owner_uid: terrars::PrimField<f64>,
    permissions: terrars::PrimField<std::string::String>,
}

impl EfsAccessPointRootDirectoryElCreationInfoEl { }

impl terrars::ToListMappable for EfsAccessPointRootDirectoryElCreationInfoEl {
    type O = terrars::BlockAssignable<EfsAccessPointRootDirectoryElCreationInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEfsAccessPointRootDirectoryElCreationInfoEl {
    #[doc = ""]
    pub owner_gid: terrars::PrimField<f64>,
    #[doc = ""]
    pub owner_uid: terrars::PrimField<f64>,
    #[doc = ""]
    pub permissions: terrars::PrimField<std::string::String>,
}

impl BuildEfsAccessPointRootDirectoryElCreationInfoEl {
    pub fn build(self) -> EfsAccessPointRootDirectoryElCreationInfoEl {
        EfsAccessPointRootDirectoryElCreationInfoEl {
            owner_gid: self.owner_gid,
            owner_uid: self.owner_uid,
            permissions: self.permissions,
        }
    }
}

pub struct EfsAccessPointRootDirectoryElCreationInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsAccessPointRootDirectoryElCreationInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EfsAccessPointRootDirectoryElCreationInfoElRef {
        EfsAccessPointRootDirectoryElCreationInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EfsAccessPointRootDirectoryElCreationInfoElRef {
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

#[derive(Serialize, Default)]
struct EfsAccessPointRootDirectoryElDynamic {
    creation_info: std::option::Option<terrars::DynamicBlock<EfsAccessPointRootDirectoryElCreationInfoEl>>,
}

#[derive(Serialize)]
pub struct EfsAccessPointRootDirectoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_info: std::option::Option<std::vec::Vec<EfsAccessPointRootDirectoryElCreationInfoEl>>,
    dynamic: EfsAccessPointRootDirectoryElDynamic,
}

impl EfsAccessPointRootDirectoryEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_info`.\n"]
    pub fn set_creation_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EfsAccessPointRootDirectoryElCreationInfoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.creation_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.creation_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EfsAccessPointRootDirectoryEl {
    type O = terrars::BlockAssignable<EfsAccessPointRootDirectoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEfsAccessPointRootDirectoryEl {}

impl BuildEfsAccessPointRootDirectoryEl {
    pub fn build(self) -> EfsAccessPointRootDirectoryEl {
        EfsAccessPointRootDirectoryEl {
            path: core::default::Default::default(),
            creation_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EfsAccessPointRootDirectoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsAccessPointRootDirectoryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EfsAccessPointRootDirectoryElRef {
        EfsAccessPointRootDirectoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EfsAccessPointRootDirectoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_info` after provisioning.\n"]
    pub fn creation_info(&self) -> terrars::ListRef<EfsAccessPointRootDirectoryElCreationInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creation_info", self.base))
    }
}

#[derive(Serialize, Default)]
struct EfsAccessPointDynamic {
    posix_user: std::option::Option<terrars::DynamicBlock<EfsAccessPointPosixUserEl>>,
    root_directory: std::option::Option<terrars::DynamicBlock<EfsAccessPointRootDirectoryEl>>,
}

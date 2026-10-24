use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MskconnectCustomPluginData {
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
    content_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<std::vec::Vec<MskconnectCustomPluginLocationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MskconnectCustomPluginTimeoutsEl>,
    dynamic: MskconnectCustomPluginDynamic,
}

struct MskconnectCustomPlugin_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MskconnectCustomPluginData>,
}

#[derive(Clone)]
pub struct MskconnectCustomPlugin(std::rc::Rc<MskconnectCustomPlugin_>);

impl MskconnectCustomPlugin {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectCustomPluginLocationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MskconnectCustomPluginTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latest_revision` after provisioning.\n"]
    pub fn latest_revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::ListRef<MskconnectCustomPluginLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskconnectCustomPluginTimeoutsElRef {
        MskconnectCustomPluginTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MskconnectCustomPlugin {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MskconnectCustomPlugin { }

impl terrars::ToListMappable for MskconnectCustomPlugin {
    type O = terrars::ListRef<MskconnectCustomPluginRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MskconnectCustomPlugin_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_mskconnect_custom_plugin".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMskconnectCustomPlugin {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMskconnectCustomPlugin {
    pub fn build(self, stack: &mut terrars::Stack) -> MskconnectCustomPlugin {
        let out = MskconnectCustomPlugin(std::rc::Rc::new(MskconnectCustomPlugin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MskconnectCustomPluginData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                content_type: self.content_type,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                location: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MskconnectCustomPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectCustomPluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MskconnectCustomPluginRef {
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

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latest_revision` after provisioning.\n"]
    pub fn latest_revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::ListRef<MskconnectCustomPluginLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskconnectCustomPluginTimeoutsElRef {
        MskconnectCustomPluginTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MskconnectCustomPluginLocationElS3El {
    bucket_arn: terrars::PrimField<std::string::String>,
    file_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectCustomPluginLocationElS3El {
    #[doc = "Set the field `object_version`.\n"]
    pub fn set_object_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MskconnectCustomPluginLocationElS3El {
    type O = terrars::BlockAssignable<MskconnectCustomPluginLocationElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectCustomPluginLocationElS3El {
    #[doc = ""]
    pub bucket_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_key: terrars::PrimField<std::string::String>,
}

impl BuildMskconnectCustomPluginLocationElS3El {
    pub fn build(self) -> MskconnectCustomPluginLocationElS3El {
        MskconnectCustomPluginLocationElS3El {
            bucket_arn: self.bucket_arn,
            file_key: self.file_key,
            object_version: core::default::Default::default(),
        }
    }
}

pub struct MskconnectCustomPluginLocationElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectCustomPluginLocationElS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectCustomPluginLocationElS3ElRef {
        MskconnectCustomPluginLocationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectCustomPluginLocationElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `file_key` after provisioning.\n"]
    pub fn file_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_key", self.base))
    }

    #[doc = "Get a reference to the value of field `object_version` after provisioning.\n"]
    pub fn object_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct MskconnectCustomPluginLocationElDynamic {
    s3: std::option::Option<terrars::DynamicBlock<MskconnectCustomPluginLocationElS3El>>,
}

#[derive(Serialize)]
pub struct MskconnectCustomPluginLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<MskconnectCustomPluginLocationElS3El>>,
    dynamic: MskconnectCustomPluginLocationElDynamic,
}

impl MskconnectCustomPluginLocationEl {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MskconnectCustomPluginLocationElS3El>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MskconnectCustomPluginLocationEl {
    type O = terrars::BlockAssignable<MskconnectCustomPluginLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectCustomPluginLocationEl {}

impl BuildMskconnectCustomPluginLocationEl {
    pub fn build(self) -> MskconnectCustomPluginLocationEl {
        MskconnectCustomPluginLocationEl {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MskconnectCustomPluginLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectCustomPluginLocationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectCustomPluginLocationElRef {
        MskconnectCustomPluginLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectCustomPluginLocationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<MskconnectCustomPluginLocationElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize)]
pub struct MskconnectCustomPluginTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MskconnectCustomPluginTimeoutsEl {
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

impl terrars::ToListMappable for MskconnectCustomPluginTimeoutsEl {
    type O = terrars::BlockAssignable<MskconnectCustomPluginTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMskconnectCustomPluginTimeoutsEl {}

impl BuildMskconnectCustomPluginTimeoutsEl {
    pub fn build(self) -> MskconnectCustomPluginTimeoutsEl {
        MskconnectCustomPluginTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct MskconnectCustomPluginTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MskconnectCustomPluginTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MskconnectCustomPluginTimeoutsElRef {
        MskconnectCustomPluginTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MskconnectCustomPluginTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct MskconnectCustomPluginDynamic {
    location: std::option::Option<terrars::DynamicBlock<MskconnectCustomPluginLocationEl>>,
}

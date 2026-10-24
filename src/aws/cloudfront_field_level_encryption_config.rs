use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontFieldLevelEncryptionConfigData {
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
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type_profile_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_arg_profile_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl>>,
    dynamic: CloudfrontFieldLevelEncryptionConfigDynamic,
}

struct CloudfrontFieldLevelEncryptionConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontFieldLevelEncryptionConfigData>,
}

#[derive(Clone)]
pub struct CloudfrontFieldLevelEncryptionConfig(std::rc::Rc<CloudfrontFieldLevelEncryptionConfig_>);

impl CloudfrontFieldLevelEncryptionConfig {
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

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type_profile_config`.\n"]
    pub fn set_content_type_profile_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_type_profile_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_type_profile_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_arg_profile_config`.\n"]
    pub fn set_query_arg_profile_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().query_arg_profile_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.query_arg_profile_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caller_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_type_profile_config` after provisioning.\n"]
    pub fn content_type_profile_config(
        &self,
    ) -> terrars::ListRef<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_type_profile_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_arg_profile_config` after provisioning.\n"]
    pub fn query_arg_profile_config(
        &self,
    ) -> terrars::ListRef<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_arg_profile_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontFieldLevelEncryptionConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontFieldLevelEncryptionConfig { }

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionConfig {
    type O = terrars::ListRef<CloudfrontFieldLevelEncryptionConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontFieldLevelEncryptionConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_field_level_encryption_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfig {
    pub tf_id: std::string::String,
}

impl BuildCloudfrontFieldLevelEncryptionConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontFieldLevelEncryptionConfig {
        let out = CloudfrontFieldLevelEncryptionConfig(std::rc::Rc::new(CloudfrontFieldLevelEncryptionConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontFieldLevelEncryptionConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                content_type_profile_config: core::default::Default::default(),
                query_arg_profile_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `caller_reference` after provisioning.\n"]
    pub fn caller_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caller_reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_type_profile_config` after provisioning.\n"]
    pub fn content_type_profile_config(
        &self,
    ) -> terrars::ListRef<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_type_profile_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_arg_profile_config` after provisioning.\n"]
    pub fn query_arg_profile_config(
        &self,
    ) -> terrars::ListRef<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_arg_profile_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    content_type: terrars::PrimField<std::string::String>,
    format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    #[doc = "Set the field `profile_id`.\n"]
    pub fn set_profile_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    type O =
        terrars
        ::BlockAssignable<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl {
            content_type: self.content_type,
            format: self.format,
            profile_id: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElDynamic {
    items: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std
    ::option
    ::Option<
        std::vec::Vec<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl>,
    >,
    dynamic: CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElDynamic,
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElItemsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
    type O =
        terrars
        ::BlockAssignable<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {}

impl BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl {
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElDynamic {
    content_type_profiles: std
    ::option
    ::Option<
        terrars::DynamicBlock<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    forward_when_content_type_is_unknown: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type_profiles: std
    ::option
    ::Option<std::vec::Vec<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl>>,
    dynamic: CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElDynamic,
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    #[doc = "Set the field `content_type_profiles`.\n"]
    pub fn set_content_type_profiles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.content_type_profiles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.content_type_profiles = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    type O = terrars::BlockAssignable<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    #[doc = ""]
    pub forward_when_content_type_is_unknown: terrars::PrimField<bool>,
}

impl BuildCloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl {
            forward_when_content_type_is_unknown: self.forward_when_content_type_is_unknown,
            content_type_profiles: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
        CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `forward_when_content_type_is_unknown` after provisioning.\n"]
    pub fn forward_when_content_type_is_unknown(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forward_when_content_type_is_unknown", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type_profiles` after provisioning.\n"]
    pub fn content_type_profiles(
        &self,
    ) -> terrars::ListRef<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigElContentTypeProfilesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_type_profiles", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
    profile_id: terrars::PrimField<std::string::String>,
    query_arg: terrars::PrimField<std::string::String>,
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl { }

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
    type O =
        terrars
        ::BlockAssignable<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
    #[doc = ""]
    pub profile_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub query_arg: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl {
            profile_id: self.profile_id,
            query_arg: self.query_arg,
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `query_arg` after provisioning.\n"]
    pub fn query_arg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_arg", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElDynamic {
    items: std
    ::option
    ::Option<
        terrars::DynamicBlock<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std
    ::option
    ::Option<std::vec::Vec<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl>>,
    dynamic: CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElDynamic,
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElItemsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
    type O = terrars::BlockAssignable<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {}

impl BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl {
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElDynamic {
    query_arg_profiles: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    forward_when_query_arg_profile_is_unknown: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_arg_profiles: std
    ::option
    ::Option<std::vec::Vec<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl>>,
    dynamic: CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElDynamic,
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    #[doc = "Set the field `query_arg_profiles`.\n"]
    pub fn set_query_arg_profiles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_arg_profiles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_arg_profiles = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    type O = terrars::BlockAssignable<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    #[doc = ""]
    pub forward_when_query_arg_profile_is_unknown: terrars::PrimField<bool>,
}

impl BuildCloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl {
            forward_when_query_arg_profile_is_unknown: self.forward_when_query_arg_profile_is_unknown,
            query_arg_profiles: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
        CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `forward_when_query_arg_profile_is_unknown` after provisioning.\n"]
    pub fn forward_when_query_arg_profile_is_unknown(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.forward_when_query_arg_profile_is_unknown", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_arg_profiles` after provisioning.\n"]
    pub fn query_arg_profiles(
        &self,
    ) -> terrars::ListRef<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigElQueryArgProfilesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_arg_profiles", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionConfigDynamic {
    content_type_profile_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontFieldLevelEncryptionConfigContentTypeProfileConfigEl>>,
    query_arg_profile_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontFieldLevelEncryptionConfigQueryArgProfileConfigEl>>,
}

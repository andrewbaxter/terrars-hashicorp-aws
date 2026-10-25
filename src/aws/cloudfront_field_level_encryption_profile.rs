#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontFieldLevelEncryptionProfileData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_entities: std::option::Option<std::vec::Vec<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities>>,
    dynamic: CloudfrontFieldLevelEncryptionProfileDynamic,
}

struct CloudfrontFieldLevelEncryptionProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontFieldLevelEncryptionProfileData_>,
}

#[derive(Clone)]
pub struct CloudfrontFieldLevelEncryptionProfile(std::rc::Rc<CloudfrontFieldLevelEncryptionProfile_>);

impl CloudfrontFieldLevelEncryptionProfile {
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

    #[doc = "Set the field `encryption_entities`.\n"]
    pub fn set_encryption_entities(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_entities = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_entities = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_entities` after provisioning.\n"]
    pub fn encryption_entities(&self) -> terrars::ListRef<CloudfrontFieldLevelEncryptionProfile_EncryptionEntitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_entities", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontFieldLevelEncryptionProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontFieldLevelEncryptionProfile { }

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionProfile {
    type O = terrars::ListRef<CloudfrontFieldLevelEncryptionProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontFieldLevelEncryptionProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_field_level_encryption_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionProfile {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontFieldLevelEncryptionProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontFieldLevelEncryptionProfile {
        let out = CloudfrontFieldLevelEncryptionProfile(std::rc::Rc::new(CloudfrontFieldLevelEncryptionProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontFieldLevelEncryptionProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                encryption_entities: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontFieldLevelEncryptionProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontFieldLevelEncryptionProfileRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_entities` after provisioning.\n"]
    pub fn encryption_entities(&self) -> terrars::ListRef<CloudfrontFieldLevelEncryptionProfile_EncryptionEntitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_entities", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionProfileDynamic {
    encryption_entities: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities>>,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns {
    type O = terrars::BlockAssignable<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns {}

impl BuildCloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns {
        CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns {
            items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatternsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatternsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatternsRef {
        CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatternsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatternsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_ItemsDynamic {
    field_patterns: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns>>,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items {
    provider_id: terrars::PrimField<std::string::String>,
    public_key_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_patterns: std
    ::option
    ::Option<std::vec::Vec<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns>>,
    dynamic: CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_ItemsDynamic,
}

impl CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items {
    #[doc = "Set the field `field_patterns`.\n"]
    pub fn set_field_patterns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatterns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_patterns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_patterns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items {
    type O = terrars::BlockAssignable<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items {
    #[doc = ""]
    pub provider_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub public_key_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items {
        CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items {
            provider_id: self.provider_id,
            public_key_id: self.public_key_id,
            field_patterns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_ItemsRef {
        CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provider_id` after provisioning.\n"]
    pub fn provider_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_id", self.base))
    }

    #[doc = "Get a reference to the value of field `public_key_id` after provisioning.\n"]
    pub fn public_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `field_patterns` after provisioning.\n"]
    pub fn field_patterns(
        &self,
    ) -> terrars::ListRef<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items_FieldPatternsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_patterns", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontFieldLevelEncryptionProfile_EncryptionEntitiesDynamic {
    items: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items>>,
}

#[derive(Serialize)]
pub struct CloudfrontFieldLevelEncryptionProfile_EncryptionEntities {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items>>,
    dynamic: CloudfrontFieldLevelEncryptionProfile_EncryptionEntitiesDynamic,
}

impl CloudfrontFieldLevelEncryptionProfile_EncryptionEntities {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities_Items>>,
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

impl terrars::ToListMappable for CloudfrontFieldLevelEncryptionProfile_EncryptionEntities {
    type O = terrars::BlockAssignable<CloudfrontFieldLevelEncryptionProfile_EncryptionEntities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontFieldLevelEncryptionProfile_EncryptionEntities {}

impl BuildCloudfrontFieldLevelEncryptionProfile_EncryptionEntities {
    pub fn build(self) -> CloudfrontFieldLevelEncryptionProfile_EncryptionEntities {
        CloudfrontFieldLevelEncryptionProfile_EncryptionEntities {
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontFieldLevelEncryptionProfile_EncryptionEntitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontFieldLevelEncryptionProfile_EncryptionEntitiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontFieldLevelEncryptionProfile_EncryptionEntitiesRef {
        CloudfrontFieldLevelEncryptionProfile_EncryptionEntitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontFieldLevelEncryptionProfile_EncryptionEntitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

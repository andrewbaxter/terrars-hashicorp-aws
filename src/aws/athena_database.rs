use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AthenaDatabaseData {
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
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_configuration: std::option::Option<std::vec::Vec<AthenaDatabaseAclConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<AthenaDatabaseEncryptionConfigurationEl>>,
    dynamic: AthenaDatabaseDynamic,
}

struct AthenaDatabase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AthenaDatabaseData>,
}

#[derive(Clone)]
pub struct AthenaDatabase(std::rc::Rc<AthenaDatabase_>);

impl AthenaDatabase {
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

    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `acl_configuration`.\n"]
    pub fn set_acl_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AthenaDatabaseAclConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().acl_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.acl_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AthenaDatabaseEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `acl_configuration` after provisioning.\n"]
    pub fn acl_configuration(&self) -> terrars::ListRef<AthenaDatabaseAclConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<AthenaDatabaseEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for AthenaDatabase {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AthenaDatabase { }

impl terrars::ToListMappable for AthenaDatabase {
    type O = terrars::ListRef<AthenaDatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AthenaDatabase_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_athena_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAthenaDatabase {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAthenaDatabase {
    pub fn build(self, stack: &mut terrars::Stack) -> AthenaDatabase {
        let out = AthenaDatabase(std::rc::Rc::new(AthenaDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AthenaDatabaseData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: core::default::Default::default(),
                comment: core::default::Default::default(),
                expected_bucket_owner: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                properties: core::default::Default::default(),
                acl_configuration: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AthenaDatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaDatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AthenaDatabaseRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `acl_configuration` after provisioning.\n"]
    pub fn acl_configuration(&self) -> terrars::ListRef<AthenaDatabaseAclConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<AthenaDatabaseEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AthenaDatabaseAclConfigurationEl {
    s3_acl_option: terrars::PrimField<std::string::String>,
}

impl AthenaDatabaseAclConfigurationEl { }

impl terrars::ToListMappable for AthenaDatabaseAclConfigurationEl {
    type O = terrars::BlockAssignable<AthenaDatabaseAclConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaDatabaseAclConfigurationEl {
    #[doc = ""]
    pub s3_acl_option: terrars::PrimField<std::string::String>,
}

impl BuildAthenaDatabaseAclConfigurationEl {
    pub fn build(self) -> AthenaDatabaseAclConfigurationEl {
        AthenaDatabaseAclConfigurationEl { s3_acl_option: self.s3_acl_option }
    }
}

pub struct AthenaDatabaseAclConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaDatabaseAclConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AthenaDatabaseAclConfigurationElRef {
        AthenaDatabaseAclConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaDatabaseAclConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_acl_option` after provisioning.\n"]
    pub fn s3_acl_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_acl_option", self.base))
    }
}

#[derive(Serialize)]
pub struct AthenaDatabaseEncryptionConfigurationEl {
    encryption_option: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AthenaDatabaseEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AthenaDatabaseEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<AthenaDatabaseEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAthenaDatabaseEncryptionConfigurationEl {
    #[doc = ""]
    pub encryption_option: terrars::PrimField<std::string::String>,
}

impl BuildAthenaDatabaseEncryptionConfigurationEl {
    pub fn build(self) -> AthenaDatabaseEncryptionConfigurationEl {
        AthenaDatabaseEncryptionConfigurationEl {
            encryption_option: self.encryption_option,
            kms_key: core::default::Default::default(),
        }
    }
}

pub struct AthenaDatabaseEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AthenaDatabaseEncryptionConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AthenaDatabaseEncryptionConfigurationElRef {
        AthenaDatabaseEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AthenaDatabaseEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_option` after provisioning.\n"]
    pub fn encryption_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_option", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct AthenaDatabaseDynamic {
    acl_configuration: std::option::Option<terrars::DynamicBlock<AthenaDatabaseAclConfigurationEl>>,
    encryption_configuration: std::option::Option<terrars::DynamicBlock<AthenaDatabaseEncryptionConfigurationEl>>,
}

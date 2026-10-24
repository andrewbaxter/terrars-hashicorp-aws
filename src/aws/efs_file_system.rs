use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EfsFileSystemData {
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
    availability_zone_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput_in_mibps: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_policy: std::option::Option<std::vec::Vec<EfsFileSystemLifecyclePolicyEl>>,
    dynamic: EfsFileSystemDynamic,
}

struct EfsFileSystem_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EfsFileSystemData>,
}

#[derive(Clone)]
pub struct EfsFileSystem(std::rc::Rc<EfsFileSystem_>);

impl EfsFileSystem {
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

    #[doc = "Set the field `availability_zone_name`.\n"]
    pub fn set_availability_zone_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().availability_zone_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_token`.\n"]
    pub fn set_creation_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().creation_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performance_mode`.\n"]
    pub fn set_performance_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().performance_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioned_throughput_in_mibps`.\n"]
    pub fn set_provisioned_throughput_in_mibps(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().provisioned_throughput_in_mibps = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `throughput_mode`.\n"]
    pub fn set_throughput_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().throughput_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_policy`.\n"]
    pub fn set_lifecycle_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EfsFileSystemLifecyclePolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lifecycle_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lifecycle_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_name` after provisioning.\n"]
    pub fn availability_zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_token` after provisioning.\n"]
    pub fn creation_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_mount_targets` after provisioning.\n"]
    pub fn number_of_mount_targets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_mount_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_mode` after provisioning.\n"]
    pub fn performance_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput_in_mibps` after provisioning.\n"]
    pub fn provisioned_throughput_in_mibps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput_in_mibps", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\n"]
    pub fn size_in_bytes(&self) -> terrars::ListRef<EfsFileSystemSizeInBytesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size_in_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_mode` after provisioning.\n"]
    pub fn throughput_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> terrars::ListRef<EfsFileSystemLifecyclePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_policy", self.extract_ref()))
    }
}

impl terrars::Referable for EfsFileSystem {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EfsFileSystem { }

impl terrars::ToListMappable for EfsFileSystem {
    type O = terrars::ListRef<EfsFileSystemRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EfsFileSystem_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_efs_file_system".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEfsFileSystem {
    pub tf_id: std::string::String,
}

impl BuildEfsFileSystem {
    pub fn build(self, stack: &mut terrars::Stack) -> EfsFileSystem {
        let out = EfsFileSystem(std::rc::Rc::new(EfsFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EfsFileSystemData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                availability_zone_name: core::default::Default::default(),
                creation_token: core::default::Default::default(),
                encrypted: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                performance_mode: core::default::Default::default(),
                provisioned_throughput_in_mibps: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                throughput_mode: core::default::Default::default(),
                lifecycle_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EfsFileSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsFileSystemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EfsFileSystemRef {
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

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_name` after provisioning.\n"]
    pub fn availability_zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_token` after provisioning.\n"]
    pub fn creation_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_mount_targets` after provisioning.\n"]
    pub fn number_of_mount_targets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_mount_targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_mode` after provisioning.\n"]
    pub fn performance_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput_in_mibps` after provisioning.\n"]
    pub fn provisioned_throughput_in_mibps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_throughput_in_mibps", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\n"]
    pub fn size_in_bytes(&self) -> terrars::ListRef<EfsFileSystemSizeInBytesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size_in_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `throughput_mode` after provisioning.\n"]
    pub fn throughput_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_policy` after provisioning.\n"]
    pub fn lifecycle_policy(&self) -> terrars::ListRef<EfsFileSystemLifecyclePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EfsFileSystemSizeInBytesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_in_ia: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_in_standard: std::option::Option<terrars::PrimField<f64>>,
}

impl EfsFileSystemSizeInBytesEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_in_ia`.\n"]
    pub fn set_value_in_ia(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value_in_ia = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_in_standard`.\n"]
    pub fn set_value_in_standard(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value_in_standard = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EfsFileSystemSizeInBytesEl {
    type O = terrars::BlockAssignable<EfsFileSystemSizeInBytesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEfsFileSystemSizeInBytesEl {}

impl BuildEfsFileSystemSizeInBytesEl {
    pub fn build(self) -> EfsFileSystemSizeInBytesEl {
        EfsFileSystemSizeInBytesEl {
            value: core::default::Default::default(),
            value_in_ia: core::default::Default::default(),
            value_in_standard: core::default::Default::default(),
        }
    }
}

pub struct EfsFileSystemSizeInBytesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsFileSystemSizeInBytesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EfsFileSystemSizeInBytesElRef {
        EfsFileSystemSizeInBytesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EfsFileSystemSizeInBytesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `value_in_ia` after provisioning.\n"]
    pub fn value_in_ia(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_in_ia", self.base))
    }

    #[doc = "Get a reference to the value of field `value_in_standard` after provisioning.\n"]
    pub fn value_in_standard(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_in_standard", self.base))
    }
}

#[derive(Serialize)]
pub struct EfsFileSystemLifecyclePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition_to_ia: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition_to_primary_storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EfsFileSystemLifecyclePolicyEl {
    #[doc = "Set the field `transition_to_ia`.\n"]
    pub fn set_transition_to_ia(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.transition_to_ia = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transition_to_primary_storage_class`.\n"]
    pub fn set_transition_to_primary_storage_class(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transition_to_primary_storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EfsFileSystemLifecyclePolicyEl {
    type O = terrars::BlockAssignable<EfsFileSystemLifecyclePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEfsFileSystemLifecyclePolicyEl {}

impl BuildEfsFileSystemLifecyclePolicyEl {
    pub fn build(self) -> EfsFileSystemLifecyclePolicyEl {
        EfsFileSystemLifecyclePolicyEl {
            transition_to_ia: core::default::Default::default(),
            transition_to_primary_storage_class: core::default::Default::default(),
        }
    }
}

pub struct EfsFileSystemLifecyclePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsFileSystemLifecyclePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EfsFileSystemLifecyclePolicyElRef {
        EfsFileSystemLifecyclePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EfsFileSystemLifecyclePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `transition_to_ia` after provisioning.\n"]
    pub fn transition_to_ia(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transition_to_ia", self.base))
    }

    #[doc = "Get a reference to the value of field `transition_to_primary_storage_class` after provisioning.\n"]
    pub fn transition_to_primary_storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transition_to_primary_storage_class", self.base))
    }
}

#[derive(Serialize, Default)]
struct EfsFileSystemDynamic {
    lifecycle_policy: std::option::Option<terrars::DynamicBlock<EfsFileSystemLifecyclePolicyEl>>,
}

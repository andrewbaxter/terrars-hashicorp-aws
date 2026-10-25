#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontkeyvaluestoreKeysExclusiveData_ {
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
    key_value_store_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_key_value_pair: std
    ::option
    ::Option<std::vec::Vec<CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair>>,
    dynamic: CloudfrontkeyvaluestoreKeysExclusiveDynamic,
}

struct CloudfrontkeyvaluestoreKeysExclusive_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontkeyvaluestoreKeysExclusiveData_>,
}

#[derive(Clone)]
pub struct CloudfrontkeyvaluestoreKeysExclusive(std::rc::Rc<CloudfrontkeyvaluestoreKeysExclusive_>);

impl CloudfrontkeyvaluestoreKeysExclusive {
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

    #[doc =
        "Set the field `max_batch_size`.\nMaximum resource key values pairs that you wills update in a single API request. AWS has a default quota of 50 keys or a 3 MB payload, whichever is reached first"]
    pub fn set_max_batch_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_key_value_pair`.\n"]
    pub fn set_resource_key_value_pair(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_key_value_pair = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_key_value_pair = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `key_value_store_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the Key Value Store."]
    pub fn key_value_store_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_value_store_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_batch_size` after provisioning.\nMaximum resource key values pairs that you wills update in a single API request. AWS has a default quota of 50 keys or a 3 MB payload, whichever is reached first"]
    pub fn max_batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_batch_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_size_in_bytes` after provisioning.\nTotal size of the Key Value Store in bytes."]
    pub fn total_size_in_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_size_in_bytes", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontkeyvaluestoreKeysExclusive {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontkeyvaluestoreKeysExclusive { }

impl terrars::ToListMappable for CloudfrontkeyvaluestoreKeysExclusive {
    type O = terrars::ListRef<CloudfrontkeyvaluestoreKeysExclusiveRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontkeyvaluestoreKeysExclusive_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfrontkeyvaluestore_keys_exclusive".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontkeyvaluestoreKeysExclusive {
    pub tf_id: std::string::String,
    #[doc = "The Amazon Resource Name (ARN) of the Key Value Store."]
    pub key_value_store_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontkeyvaluestoreKeysExclusive {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontkeyvaluestoreKeysExclusive {
        let out = CloudfrontkeyvaluestoreKeysExclusive(std::rc::Rc::new(CloudfrontkeyvaluestoreKeysExclusive_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontkeyvaluestoreKeysExclusiveData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                key_value_store_arn: self.key_value_store_arn,
                max_batch_size: core::default::Default::default(),
                resource_key_value_pair: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontkeyvaluestoreKeysExclusiveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontkeyvaluestoreKeysExclusiveRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontkeyvaluestoreKeysExclusiveRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key_value_store_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the Key Value Store."]
    pub fn key_value_store_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_value_store_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_batch_size` after provisioning.\nMaximum resource key values pairs that you wills update in a single API request. AWS has a default quota of 50 keys or a 3 MB payload, whichever is reached first"]
    pub fn max_batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_batch_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_size_in_bytes` after provisioning.\nTotal size of the Key Value Store in bytes."]
    pub fn total_size_in_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_size_in_bytes", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontkeyvaluestoreKeysExclusiveDynamic {
    resource_key_value_pair: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair>>,
}

#[derive(Serialize)]
pub struct CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair { }

impl terrars::ToListMappable for CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair {
    type O = terrars::BlockAssignable<CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair {
    #[doc = "The key to put."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "The value to put."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair {
    pub fn build(self) -> CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair {
        CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePair {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePairRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePairRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePairRef {
        CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePairRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontkeyvaluestoreKeysExclusive_ResourceKeyValuePairRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key to put."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value to put."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

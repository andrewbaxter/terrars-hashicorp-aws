#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchLogTransformerData_ {
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
    log_group_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformer_config: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig>>,
    dynamic: CloudwatchLogTransformerDynamic,
}

struct CloudwatchLogTransformer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchLogTransformerData_>,
}

#[derive(Clone)]
pub struct CloudwatchLogTransformer(std::rc::Rc<CloudwatchLogTransformer_>);

impl CloudwatchLogTransformer {
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
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transformer_config`.\n"]
    pub fn set_transformer_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transformer_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transformer_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transformer_config` after provisioning.\n"]
    pub fn transformer_config(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformer_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudwatchLogTransformer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudwatchLogTransformer { }

impl terrars::ToListMappable for CloudwatchLogTransformer {
    type O = terrars::ListRef<CloudwatchLogTransformerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudwatchLogTransformer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudwatch_log_transformer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchLogTransformer {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub log_group_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudwatchLogTransformer {
        let out = CloudwatchLogTransformer(std::rc::Rc::new(CloudwatchLogTransformer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudwatchLogTransformerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                log_group_arn: self.log_group_arn,
                region: core::default::Default::default(),
                transformer_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchLogTransformerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudwatchLogTransformerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transformer_config` after provisioning.\n"]
    pub fn transformer_config(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformer_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogTransformerDynamic {
    transformer_config: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig>>,
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_AddKeys_Entry {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_if_exists: std::option::Option<terrars::PrimField<bool>>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudwatchLogTransformer_TransformerConfig_AddKeys_Entry {
    #[doc = "Set the field `overwrite_if_exists`.\n"]
    pub fn set_overwrite_if_exists(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.overwrite_if_exists = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_AddKeys_Entry {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_AddKeys_Entry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_AddKeys_Entry {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_AddKeys_Entry {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_AddKeys_Entry {
        CloudwatchLogTransformer_TransformerConfig_AddKeys_Entry {
            key: self.key,
            overwrite_if_exists: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_AddKeys_EntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_AddKeys_EntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_AddKeys_EntryRef {
        CloudwatchLogTransformer_TransformerConfig_AddKeys_EntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_AddKeys_EntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_if_exists` after provisioning.\n"]
    pub fn overwrite_if_exists(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overwrite_if_exists", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogTransformer_TransformerConfig_AddKeysDynamic {
    entry: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_AddKeys_Entry>>,
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_AddKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_AddKeys_Entry>>,
    dynamic: CloudwatchLogTransformer_TransformerConfig_AddKeysDynamic,
}

impl CloudwatchLogTransformer_TransformerConfig_AddKeys {
    #[doc = "Set the field `entry`.\n"]
    pub fn set_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_AddKeys_Entry>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entry = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_AddKeys {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_AddKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_AddKeys {}

impl BuildCloudwatchLogTransformer_TransformerConfig_AddKeys {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_AddKeys {
        CloudwatchLogTransformer_TransformerConfig_AddKeys {
            entry: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_AddKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_AddKeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_AddKeysRef {
        CloudwatchLogTransformer_TransformerConfig_AddKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_AddKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry` after provisioning.\n"]
    pub fn entry(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_AddKeys_EntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_if_exists: std::option::Option<terrars::PrimField<bool>>,
    source: terrars::PrimField<std::string::String>,
    target: terrars::PrimField<std::string::String>,
}

impl CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry {
    #[doc = "Set the field `overwrite_if_exists`.\n"]
    pub fn set_overwrite_if_exists(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.overwrite_if_exists = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry {
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry {
        CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry {
            overwrite_if_exists: core::default::Default::default(),
            source: self.source,
            target: self.target,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_MoveKeys_EntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_MoveKeys_EntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_MoveKeys_EntryRef {
        CloudwatchLogTransformer_TransformerConfig_MoveKeys_EntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_MoveKeys_EntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `overwrite_if_exists` after provisioning.\n"]
    pub fn overwrite_if_exists(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overwrite_if_exists", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogTransformer_TransformerConfig_CopyValueDynamic {
    entry: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry>>,
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_CopyValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry>>,
    dynamic: CloudwatchLogTransformer_TransformerConfig_CopyValueDynamic,
}

impl CloudwatchLogTransformer_TransformerConfig_CopyValue {
    #[doc = "Set the field `entry`.\n"]
    pub fn set_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entry = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_CopyValue {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_CopyValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_CopyValue {}

impl BuildCloudwatchLogTransformer_TransformerConfig_CopyValue {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_CopyValue {
        CloudwatchLogTransformer_TransformerConfig_CopyValue {
            entry: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_CopyValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_CopyValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_CopyValueRef {
        CloudwatchLogTransformer_TransformerConfig_CopyValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_CopyValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry` after provisioning.\n"]
    pub fn entry(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_MoveKeys_EntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_Csv {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quote_character: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_Csv {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.columns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quote_character`.\n"]
    pub fn set_quote_character(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.quote_character = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_Csv {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_Csv>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_Csv {}

impl BuildCloudwatchLogTransformer_TransformerConfig_Csv {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_Csv {
        CloudwatchLogTransformer_TransformerConfig_Csv {
            columns: core::default::Default::default(),
            delimiter: core::default::Default::default(),
            quote_character: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_CsvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_CsvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_CsvRef {
        CloudwatchLogTransformer_TransformerConfig_CsvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_CsvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `quote_character` after provisioning.\n"]
    pub fn quote_character(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quote_character", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_DateTimeConverter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locale: std::option::Option<terrars::PrimField<std::string::String>>,
    match_patterns: terrars::ListField<terrars::PrimField<std::string::String>>,
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    target: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_timezone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_DateTimeConverter {
    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_timezone`.\n"]
    pub fn set_source_timezone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_format`.\n"]
    pub fn set_target_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_timezone`.\n"]
    pub fn set_target_timezone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_timezone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_DateTimeConverter {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_DateTimeConverter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_DateTimeConverter {
    #[doc = ""]
    pub match_patterns: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_DateTimeConverter {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_DateTimeConverter {
        CloudwatchLogTransformer_TransformerConfig_DateTimeConverter {
            locale: core::default::Default::default(),
            match_patterns: self.match_patterns,
            source: self.source,
            source_timezone: core::default::Default::default(),
            target: self.target,
            target_format: core::default::Default::default(),
            target_timezone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_DateTimeConverterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_DateTimeConverterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_DateTimeConverterRef {
        CloudwatchLogTransformer_TransformerConfig_DateTimeConverterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_DateTimeConverterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.base))
    }

    #[doc = "Get a reference to the value of field `match_patterns` after provisioning.\n"]
    pub fn match_patterns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_patterns", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `source_timezone` after provisioning.\n"]
    pub fn source_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_timezone", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `target_format` after provisioning.\n"]
    pub fn target_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_format", self.base))
    }

    #[doc = "Get a reference to the value of field `target_timezone` after provisioning.\n"]
    pub fn target_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_timezone", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_DeleteKeys {
    with_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_DeleteKeys { }

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_DeleteKeys {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_DeleteKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_DeleteKeys {
    #[doc = ""]
    pub with_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_DeleteKeys {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_DeleteKeys {
        CloudwatchLogTransformer_TransformerConfig_DeleteKeys { with_keys: self.with_keys }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_DeleteKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_DeleteKeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_DeleteKeysRef {
        CloudwatchLogTransformer_TransformerConfig_DeleteKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_DeleteKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `with_keys` after provisioning.\n"]
    pub fn with_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.with_keys", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_Grok {
    #[serde(rename = "match")]
    match_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_Grok {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_Grok {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_Grok>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_Grok {
    #[doc = ""]
    pub match_: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_Grok {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_Grok {
        CloudwatchLogTransformer_TransformerConfig_Grok {
            match_: self.match_,
            source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_GrokRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_GrokRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_GrokRef {
        CloudwatchLogTransformer_TransformerConfig_GrokRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_GrokRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_ListToMap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flatten: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flattened_element: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_ListToMap {
    #[doc = "Set the field `flatten`.\n"]
    pub fn set_flatten(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.flatten = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flattened_element`.\n"]
    pub fn set_flattened_element(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flattened_element = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_key`.\n"]
    pub fn set_value_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_ListToMap {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ListToMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_ListToMap {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_ListToMap {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_ListToMap {
        CloudwatchLogTransformer_TransformerConfig_ListToMap {
            flatten: core::default::Default::default(),
            flattened_element: core::default::Default::default(),
            key: self.key,
            source: self.source,
            target: core::default::Default::default(),
            value_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_ListToMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_ListToMapRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_ListToMapRef {
        CloudwatchLogTransformer_TransformerConfig_ListToMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_ListToMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flatten` after provisioning.\n"]
    pub fn flatten(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flatten", self.base))
    }

    #[doc = "Get a reference to the value of field `flattened_element` after provisioning.\n"]
    pub fn flattened_element(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flattened_element", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `value_key` after provisioning.\n"]
    pub fn value_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_key", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_LowerCaseString {
    with_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_LowerCaseString { }

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_LowerCaseString {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_LowerCaseString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_LowerCaseString {
    #[doc = ""]
    pub with_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_LowerCaseString {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_LowerCaseString {
        CloudwatchLogTransformer_TransformerConfig_LowerCaseString { with_keys: self.with_keys }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_LowerCaseStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_LowerCaseStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_LowerCaseStringRef {
        CloudwatchLogTransformer_TransformerConfig_LowerCaseStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_LowerCaseStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `with_keys` after provisioning.\n"]
    pub fn with_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.with_keys", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogTransformer_TransformerConfig_MoveKeysDynamic {
    entry: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry>>,
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_MoveKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry>>,
    dynamic: CloudwatchLogTransformer_TransformerConfig_MoveKeysDynamic,
}

impl CloudwatchLogTransformer_TransformerConfig_MoveKeys {
    #[doc = "Set the field `entry`.\n"]
    pub fn set_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_MoveKeys_Entry>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entry = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_MoveKeys {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_MoveKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_MoveKeys {}

impl BuildCloudwatchLogTransformer_TransformerConfig_MoveKeys {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_MoveKeys {
        CloudwatchLogTransformer_TransformerConfig_MoveKeys {
            entry: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_MoveKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_MoveKeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_MoveKeysRef {
        CloudwatchLogTransformer_TransformerConfig_MoveKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_MoveKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry` after provisioning.\n"]
    pub fn entry(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_MoveKeys_EntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseCloudfront {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_ParseCloudfront {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_ParseCloudfront {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseCloudfront>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_ParseCloudfront {}

impl BuildCloudwatchLogTransformer_TransformerConfig_ParseCloudfront {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_ParseCloudfront {
        CloudwatchLogTransformer_TransformerConfig_ParseCloudfront { source: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseCloudfrontRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_ParseCloudfrontRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_ParseCloudfrontRef {
        CloudwatchLogTransformer_TransformerConfig_ParseCloudfrontRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_ParseCloudfrontRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseJson {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_ParseJson {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_ParseJson {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseJson>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_ParseJson {}

impl BuildCloudwatchLogTransformer_TransformerConfig_ParseJson {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_ParseJson {
        CloudwatchLogTransformer_TransformerConfig_ParseJson {
            destination: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseJsonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_ParseJsonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_ParseJsonRef {
        CloudwatchLogTransformer_TransformerConfig_ParseJsonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_ParseJsonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseKeyValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_value_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_match_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_if_exists: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_ParseKeyValue {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_delimiter`.\n"]
    pub fn set_field_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_value_delimiter`.\n"]
    pub fn set_key_value_delimiter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_value_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `non_match_value`.\n"]
    pub fn set_non_match_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.non_match_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overwrite_if_exists`.\n"]
    pub fn set_overwrite_if_exists(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.overwrite_if_exists = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_ParseKeyValue {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseKeyValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_ParseKeyValue {}

impl BuildCloudwatchLogTransformer_TransformerConfig_ParseKeyValue {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_ParseKeyValue {
        CloudwatchLogTransformer_TransformerConfig_ParseKeyValue {
            destination: core::default::Default::default(),
            field_delimiter: core::default::Default::default(),
            key_prefix: core::default::Default::default(),
            key_value_delimiter: core::default::Default::default(),
            non_match_value: core::default::Default::default(),
            overwrite_if_exists: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseKeyValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_ParseKeyValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_ParseKeyValueRef {
        CloudwatchLogTransformer_TransformerConfig_ParseKeyValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_ParseKeyValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `field_delimiter` after provisioning.\n"]
    pub fn field_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `key_value_delimiter` after provisioning.\n"]
    pub fn key_value_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_value_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `non_match_value` after provisioning.\n"]
    pub fn non_match_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.non_match_value", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_if_exists` after provisioning.\n"]
    pub fn overwrite_if_exists(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overwrite_if_exists", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParsePostgres {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_ParsePostgres {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_ParsePostgres {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParsePostgres>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_ParsePostgres {}

impl BuildCloudwatchLogTransformer_TransformerConfig_ParsePostgres {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_ParsePostgres {
        CloudwatchLogTransformer_TransformerConfig_ParsePostgres { source: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParsePostgresRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_ParsePostgresRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_ParsePostgresRef {
        CloudwatchLogTransformer_TransformerConfig_ParsePostgresRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_ParsePostgresRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseRoute53 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_ParseRoute53 {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_ParseRoute53 {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseRoute53>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_ParseRoute53 {}

impl BuildCloudwatchLogTransformer_TransformerConfig_ParseRoute53 {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_ParseRoute53 {
        CloudwatchLogTransformer_TransformerConfig_ParseRoute53 { source: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseRoute53Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_ParseRoute53Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_ParseRoute53Ref {
        CloudwatchLogTransformer_TransformerConfig_ParseRoute53Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_ParseRoute53Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseToOcsf {
    event_source: terrars::PrimField<std::string::String>,
    ocsf_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_ParseToOcsf {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_ParseToOcsf {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseToOcsf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_ParseToOcsf {
    #[doc = ""]
    pub event_source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub ocsf_version: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_ParseToOcsf {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_ParseToOcsf {
        CloudwatchLogTransformer_TransformerConfig_ParseToOcsf {
            event_source: self.event_source,
            ocsf_version: self.ocsf_version,
            source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseToOcsfRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_ParseToOcsfRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_ParseToOcsfRef {
        CloudwatchLogTransformer_TransformerConfig_ParseToOcsfRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_ParseToOcsfRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_source` after provisioning.\n"]
    pub fn event_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_source", self.base))
    }

    #[doc = "Get a reference to the value of field `ocsf_version` after provisioning.\n"]
    pub fn ocsf_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocsf_version", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseVpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_ParseVpc {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_ParseVpc {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseVpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_ParseVpc {}

impl BuildCloudwatchLogTransformer_TransformerConfig_ParseVpc {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_ParseVpc {
        CloudwatchLogTransformer_TransformerConfig_ParseVpc { source: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseVpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_ParseVpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_ParseVpcRef {
        CloudwatchLogTransformer_TransformerConfig_ParseVpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_ParseVpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseWaf {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_ParseWaf {
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_ParseWaf {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseWaf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_ParseWaf {}

impl BuildCloudwatchLogTransformer_TransformerConfig_ParseWaf {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_ParseWaf {
        CloudwatchLogTransformer_TransformerConfig_ParseWaf { source: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_ParseWafRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_ParseWafRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_ParseWafRef {
        CloudwatchLogTransformer_TransformerConfig_ParseWafRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_ParseWafRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_if_exists: std::option::Option<terrars::PrimField<bool>>,
    rename_to: terrars::PrimField<std::string::String>,
}

impl CloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry {
    #[doc = "Set the field `overwrite_if_exists`.\n"]
    pub fn set_overwrite_if_exists(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.overwrite_if_exists = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rename_to: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry {
        CloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry {
            key: self.key,
            overwrite_if_exists: core::default::Default::default(),
            rename_to: self.rename_to,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_RenameKeys_EntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_RenameKeys_EntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_RenameKeys_EntryRef {
        CloudwatchLogTransformer_TransformerConfig_RenameKeys_EntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_RenameKeys_EntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_if_exists` after provisioning.\n"]
    pub fn overwrite_if_exists(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overwrite_if_exists", self.base))
    }

    #[doc = "Get a reference to the value of field `rename_to` after provisioning.\n"]
    pub fn rename_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rename_to", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogTransformer_TransformerConfig_RenameKeysDynamic {
    entry: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry>>,
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_RenameKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry>>,
    dynamic: CloudwatchLogTransformer_TransformerConfig_RenameKeysDynamic,
}

impl CloudwatchLogTransformer_TransformerConfig_RenameKeys {
    #[doc = "Set the field `entry`.\n"]
    pub fn set_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_RenameKeys_Entry>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entry = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_RenameKeys {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_RenameKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_RenameKeys {}

impl BuildCloudwatchLogTransformer_TransformerConfig_RenameKeys {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_RenameKeys {
        CloudwatchLogTransformer_TransformerConfig_RenameKeys {
            entry: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_RenameKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_RenameKeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_RenameKeysRef {
        CloudwatchLogTransformer_TransformerConfig_RenameKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_RenameKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry` after provisioning.\n"]
    pub fn entry(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_RenameKeys_EntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_SplitString_Entry {
    delimiter: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
}

impl CloudwatchLogTransformer_TransformerConfig_SplitString_Entry { }

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_SplitString_Entry {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_SplitString_Entry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_SplitString_Entry {
    #[doc = ""]
    pub delimiter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_SplitString_Entry {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_SplitString_Entry {
        CloudwatchLogTransformer_TransformerConfig_SplitString_Entry {
            delimiter: self.delimiter,
            source: self.source,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_SplitString_EntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_SplitString_EntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_SplitString_EntryRef {
        CloudwatchLogTransformer_TransformerConfig_SplitString_EntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_SplitString_EntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogTransformer_TransformerConfig_SplitStringDynamic {
    entry: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_SplitString_Entry>>,
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_SplitString {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_SplitString_Entry>>,
    dynamic: CloudwatchLogTransformer_TransformerConfig_SplitStringDynamic,
}

impl CloudwatchLogTransformer_TransformerConfig_SplitString {
    #[doc = "Set the field `entry`.\n"]
    pub fn set_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_SplitString_Entry>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entry = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_SplitString {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_SplitString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_SplitString {}

impl BuildCloudwatchLogTransformer_TransformerConfig_SplitString {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_SplitString {
        CloudwatchLogTransformer_TransformerConfig_SplitString {
            entry: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_SplitStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_SplitStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_SplitStringRef {
        CloudwatchLogTransformer_TransformerConfig_SplitStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_SplitStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry` after provisioning.\n"]
    pub fn entry(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_SplitString_EntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry {
    from: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
    to: terrars::PrimField<std::string::String>,
}

impl CloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry { }

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry {
    #[doc = ""]
    pub from: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub to: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry {
        CloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry {
            from: self.from,
            source: self.source,
            to: self.to,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_SubstituteString_EntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_SubstituteString_EntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_SubstituteString_EntryRef {
        CloudwatchLogTransformer_TransformerConfig_SubstituteString_EntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_SubstituteString_EntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogTransformer_TransformerConfig_SubstituteStringDynamic {
    entry: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry>>,
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_SubstituteString {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry>>,
    dynamic: CloudwatchLogTransformer_TransformerConfig_SubstituteStringDynamic,
}

impl CloudwatchLogTransformer_TransformerConfig_SubstituteString {
    #[doc = "Set the field `entry`.\n"]
    pub fn set_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_SubstituteString_Entry>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entry = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_SubstituteString {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_SubstituteString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_SubstituteString {}

impl BuildCloudwatchLogTransformer_TransformerConfig_SubstituteString {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_SubstituteString {
        CloudwatchLogTransformer_TransformerConfig_SubstituteString {
            entry: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_SubstituteStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_SubstituteStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_SubstituteStringRef {
        CloudwatchLogTransformer_TransformerConfig_SubstituteStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_SubstituteStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry` after provisioning.\n"]
    pub fn entry(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_SubstituteString_EntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_TrimString {
    with_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_TrimString { }

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_TrimString {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_TrimString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_TrimString {
    #[doc = ""]
    pub with_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_TrimString {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_TrimString {
        CloudwatchLogTransformer_TransformerConfig_TrimString { with_keys: self.with_keys }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_TrimStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_TrimStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_TrimStringRef {
        CloudwatchLogTransformer_TransformerConfig_TrimStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_TrimStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `with_keys` after provisioning.\n"]
    pub fn with_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.with_keys", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry {
    key: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry { }

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry {
        CloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry {
            key: self.key,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_TypeConverter_EntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_TypeConverter_EntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_TypeConverter_EntryRef {
        CloudwatchLogTransformer_TransformerConfig_TypeConverter_EntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_TypeConverter_EntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogTransformer_TransformerConfig_TypeConverterDynamic {
    entry: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry>>,
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_TypeConverter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry>>,
    dynamic: CloudwatchLogTransformer_TransformerConfig_TypeConverterDynamic,
}

impl CloudwatchLogTransformer_TransformerConfig_TypeConverter {
    #[doc = "Set the field `entry`.\n"]
    pub fn set_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_TypeConverter_Entry>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entry = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_TypeConverter {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_TypeConverter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_TypeConverter {}

impl BuildCloudwatchLogTransformer_TransformerConfig_TypeConverter {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_TypeConverter {
        CloudwatchLogTransformer_TransformerConfig_TypeConverter {
            entry: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_TypeConverterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_TypeConverterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_TypeConverterRef {
        CloudwatchLogTransformer_TransformerConfig_TypeConverterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_TypeConverterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entry` after provisioning.\n"]
    pub fn entry(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_TypeConverter_EntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig_UpperCaseString {
    with_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CloudwatchLogTransformer_TransformerConfig_UpperCaseString { }

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig_UpperCaseString {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_UpperCaseString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig_UpperCaseString {
    #[doc = ""]
    pub with_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchLogTransformer_TransformerConfig_UpperCaseString {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig_UpperCaseString {
        CloudwatchLogTransformer_TransformerConfig_UpperCaseString { with_keys: self.with_keys }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfig_UpperCaseStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfig_UpperCaseStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchLogTransformer_TransformerConfig_UpperCaseStringRef {
        CloudwatchLogTransformer_TransformerConfig_UpperCaseStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfig_UpperCaseStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `with_keys` after provisioning.\n"]
    pub fn with_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.with_keys", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchLogTransformer_TransformerConfigDynamic {
    add_keys: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_AddKeys>>,
    copy_value: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_CopyValue>>,
    csv: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_Csv>>,
    date_time_converter: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_DateTimeConverter>>,
    delete_keys: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_DeleteKeys>>,
    grok: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_Grok>>,
    list_to_map: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_ListToMap>>,
    lower_case_string: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_LowerCaseString>>,
    move_keys: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_MoveKeys>>,
    parse_cloudfront: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_ParseCloudfront>>,
    parse_json: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_ParseJson>>,
    parse_key_value: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_ParseKeyValue>>,
    parse_postgres: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_ParsePostgres>>,
    parse_route53: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_ParseRoute53>>,
    parse_to_ocsf: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_ParseToOcsf>>,
    parse_vpc: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_ParseVpc>>,
    parse_waf: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_ParseWaf>>,
    rename_keys: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_RenameKeys>>,
    split_string: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_SplitString>>,
    substitute_string: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_SubstituteString>>,
    trim_string: std::option::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_TrimString>>,
    type_converter: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_TypeConverter>>,
    upper_case_string: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchLogTransformer_TransformerConfig_UpperCaseString>>,
}

#[derive(Serialize)]
pub struct CloudwatchLogTransformer_TransformerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_keys: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_AddKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_value: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_CopyValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_Csv>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_time_converter: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_DateTimeConverter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_keys: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_DeleteKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grok: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_Grok>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    list_to_map: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_ListToMap>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lower_case_string: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_LowerCaseString>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    move_keys: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_MoveKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_cloudfront: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_ParseCloudfront>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_json: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_ParseJson>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_key_value: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_ParseKeyValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_postgres: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_ParsePostgres>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_route53: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_ParseRoute53>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_to_ocsf: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_ParseToOcsf>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_vpc: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_ParseVpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parse_waf: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_ParseWaf>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rename_keys: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_RenameKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    split_string: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_SplitString>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substitute_string: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_SubstituteString>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trim_string: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_TrimString>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    type_converter: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_TypeConverter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upper_case_string: std::option::Option<std::vec::Vec<CloudwatchLogTransformer_TransformerConfig_UpperCaseString>>,
    dynamic: CloudwatchLogTransformer_TransformerConfigDynamic,
}

impl CloudwatchLogTransformer_TransformerConfig {
    #[doc = "Set the field `add_keys`.\n"]
    pub fn set_add_keys(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_AddKeys>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.add_keys = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.add_keys = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `copy_value`.\n"]
    pub fn set_copy_value(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_CopyValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.copy_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.copy_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `csv`.\n"]
    pub fn set_csv(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_Csv>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csv = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csv = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `date_time_converter`.\n"]
    pub fn set_date_time_converter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_DateTimeConverter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_time_converter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_time_converter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delete_keys`.\n"]
    pub fn set_delete_keys(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_DeleteKeys>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delete_keys = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delete_keys = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grok`.\n"]
    pub fn set_grok(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_Grok>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grok = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grok = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `list_to_map`.\n"]
    pub fn set_list_to_map(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ListToMap>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.list_to_map = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.list_to_map = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lower_case_string`.\n"]
    pub fn set_lower_case_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_LowerCaseString>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lower_case_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lower_case_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `move_keys`.\n"]
    pub fn set_move_keys(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_MoveKeys>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.move_keys = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.move_keys = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parse_cloudfront`.\n"]
    pub fn set_parse_cloudfront(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseCloudfront>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parse_cloudfront = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parse_cloudfront = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parse_json`.\n"]
    pub fn set_parse_json(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseJson>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parse_json = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parse_json = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parse_key_value`.\n"]
    pub fn set_parse_key_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseKeyValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parse_key_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parse_key_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parse_postgres`.\n"]
    pub fn set_parse_postgres(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParsePostgres>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parse_postgres = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parse_postgres = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parse_route53`.\n"]
    pub fn set_parse_route53(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseRoute53>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parse_route53 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parse_route53 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parse_to_ocsf`.\n"]
    pub fn set_parse_to_ocsf(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseToOcsf>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parse_to_ocsf = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parse_to_ocsf = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parse_vpc`.\n"]
    pub fn set_parse_vpc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseVpc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parse_vpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parse_vpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parse_waf`.\n"]
    pub fn set_parse_waf(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_ParseWaf>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parse_waf = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parse_waf = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rename_keys`.\n"]
    pub fn set_rename_keys(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_RenameKeys>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rename_keys = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rename_keys = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `split_string`.\n"]
    pub fn set_split_string(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_SplitString>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.split_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.split_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `substitute_string`.\n"]
    pub fn set_substitute_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_SubstituteString>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.substitute_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.substitute_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trim_string`.\n"]
    pub fn set_trim_string(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_TrimString>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trim_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trim_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `type_converter`.\n"]
    pub fn set_type_converter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_TypeConverter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.type_converter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.type_converter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upper_case_string`.\n"]
    pub fn set_upper_case_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig_UpperCaseString>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.upper_case_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.upper_case_string = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchLogTransformer_TransformerConfig {
    type O = terrars::BlockAssignable<CloudwatchLogTransformer_TransformerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchLogTransformer_TransformerConfig {}

impl BuildCloudwatchLogTransformer_TransformerConfig {
    pub fn build(self) -> CloudwatchLogTransformer_TransformerConfig {
        CloudwatchLogTransformer_TransformerConfig {
            add_keys: core::default::Default::default(),
            copy_value: core::default::Default::default(),
            csv: core::default::Default::default(),
            date_time_converter: core::default::Default::default(),
            delete_keys: core::default::Default::default(),
            grok: core::default::Default::default(),
            list_to_map: core::default::Default::default(),
            lower_case_string: core::default::Default::default(),
            move_keys: core::default::Default::default(),
            parse_cloudfront: core::default::Default::default(),
            parse_json: core::default::Default::default(),
            parse_key_value: core::default::Default::default(),
            parse_postgres: core::default::Default::default(),
            parse_route53: core::default::Default::default(),
            parse_to_ocsf: core::default::Default::default(),
            parse_vpc: core::default::Default::default(),
            parse_waf: core::default::Default::default(),
            rename_keys: core::default::Default::default(),
            split_string: core::default::Default::default(),
            substitute_string: core::default::Default::default(),
            trim_string: core::default::Default::default(),
            type_converter: core::default::Default::default(),
            upper_case_string: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchLogTransformer_TransformerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchLogTransformer_TransformerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchLogTransformer_TransformerConfigRef {
        CloudwatchLogTransformer_TransformerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchLogTransformer_TransformerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add_keys` after provisioning.\n"]
    pub fn add_keys(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_AddKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.add_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_value` after provisioning.\n"]
    pub fn copy_value(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_CopyValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.copy_value", self.base))
    }

    #[doc = "Get a reference to the value of field `csv` after provisioning.\n"]
    pub fn csv(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_CsvRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv", self.base))
    }

    #[doc = "Get a reference to the value of field `date_time_converter` after provisioning.\n"]
    pub fn date_time_converter(
        &self,
    ) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_DateTimeConverterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_time_converter", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_keys` after provisioning.\n"]
    pub fn delete_keys(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_DeleteKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `grok` after provisioning.\n"]
    pub fn grok(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_GrokRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grok", self.base))
    }

    #[doc = "Get a reference to the value of field `list_to_map` after provisioning.\n"]
    pub fn list_to_map(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_ListToMapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list_to_map", self.base))
    }

    #[doc = "Get a reference to the value of field `lower_case_string` after provisioning.\n"]
    pub fn lower_case_string(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_LowerCaseStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lower_case_string", self.base))
    }

    #[doc = "Get a reference to the value of field `move_keys` after provisioning.\n"]
    pub fn move_keys(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_MoveKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.move_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `parse_cloudfront` after provisioning.\n"]
    pub fn parse_cloudfront(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_ParseCloudfrontRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parse_cloudfront", self.base))
    }

    #[doc = "Get a reference to the value of field `parse_json` after provisioning.\n"]
    pub fn parse_json(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_ParseJsonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parse_json", self.base))
    }

    #[doc = "Get a reference to the value of field `parse_key_value` after provisioning.\n"]
    pub fn parse_key_value(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_ParseKeyValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parse_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `parse_postgres` after provisioning.\n"]
    pub fn parse_postgres(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_ParsePostgresRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parse_postgres", self.base))
    }

    #[doc = "Get a reference to the value of field `parse_route53` after provisioning.\n"]
    pub fn parse_route53(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_ParseRoute53Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parse_route53", self.base))
    }

    #[doc = "Get a reference to the value of field `parse_to_ocsf` after provisioning.\n"]
    pub fn parse_to_ocsf(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_ParseToOcsfRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parse_to_ocsf", self.base))
    }

    #[doc = "Get a reference to the value of field `parse_vpc` after provisioning.\n"]
    pub fn parse_vpc(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_ParseVpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parse_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `parse_waf` after provisioning.\n"]
    pub fn parse_waf(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_ParseWafRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parse_waf", self.base))
    }

    #[doc = "Get a reference to the value of field `rename_keys` after provisioning.\n"]
    pub fn rename_keys(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_RenameKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rename_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `split_string` after provisioning.\n"]
    pub fn split_string(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_SplitStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.split_string", self.base))
    }

    #[doc = "Get a reference to the value of field `substitute_string` after provisioning.\n"]
    pub fn substitute_string(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_SubstituteStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.substitute_string", self.base))
    }

    #[doc = "Get a reference to the value of field `trim_string` after provisioning.\n"]
    pub fn trim_string(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_TrimStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trim_string", self.base))
    }

    #[doc = "Get a reference to the value of field `type_converter` after provisioning.\n"]
    pub fn type_converter(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_TypeConverterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.type_converter", self.base))
    }

    #[doc = "Get a reference to the value of field `upper_case_string` after provisioning.\n"]
    pub fn upper_case_string(&self) -> terrars::ListRef<CloudwatchLogTransformer_TransformerConfig_UpperCaseStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upper_case_string", self.base))
    }
}

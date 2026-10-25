#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafregionalWebAclData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    metric_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_action: std::option::Option<std::vec::Vec<WafregionalWebAcl_DefaultAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_configuration: std::option::Option<std::vec::Vec<WafregionalWebAcl_LoggingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<WafregionalWebAcl_Rule>>,
    dynamic: WafregionalWebAclDynamic,
}

struct WafregionalWebAcl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WafregionalWebAclData_>,
}

#[derive(Clone)]
pub struct WafregionalWebAcl(std::rc::Rc<WafregionalWebAcl_>);

impl WafregionalWebAcl {
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_action`.\n"]
    pub fn set_default_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAcl_DefaultAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAcl_LoggingConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAcl_Rule>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<WafregionalWebAcl_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<WafregionalWebAcl_LoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for WafregionalWebAcl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WafregionalWebAcl { }

impl terrars::ToListMappable for WafregionalWebAcl {
    type O = terrars::ListRef<WafregionalWebAclRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WafregionalWebAcl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_wafregional_web_acl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafregionalWebAcl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAcl {
    pub fn build(self, stack: &mut terrars::Stack) -> WafregionalWebAcl {
        let out = WafregionalWebAcl(std::rc::Rc::new(WafregionalWebAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WafregionalWebAclData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                metric_name: self.metric_name,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                default_action: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WafregionalWebAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAclRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WafregionalWebAclRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<WafregionalWebAcl_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<WafregionalWebAcl_LoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAclDynamic {
    default_action: std::option::Option<terrars::DynamicBlock<WafregionalWebAcl_DefaultAction>>,
    logging_configuration: std::option::Option<terrars::DynamicBlock<WafregionalWebAcl_LoggingConfiguration>>,
    rule: std::option::Option<terrars::DynamicBlock<WafregionalWebAcl_Rule>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAcl_DefaultAction {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafregionalWebAcl_DefaultAction { }

impl terrars::ToListMappable for WafregionalWebAcl_DefaultAction {
    type O = terrars::BlockAssignable<WafregionalWebAcl_DefaultAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAcl_DefaultAction {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAcl_DefaultAction {
    pub fn build(self) -> WafregionalWebAcl_DefaultAction {
        WafregionalWebAcl_DefaultAction { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct WafregionalWebAcl_DefaultActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAcl_DefaultActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAcl_DefaultActionRef {
        WafregionalWebAcl_DefaultActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAcl_DefaultActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    type O = terrars::BlockAssignable<WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    pub fn build(self) -> WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
        WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
        WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAcl_LoggingConfiguration_RedactedFieldsDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAcl_LoggingConfiguration_RedactedFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std
    ::option
    ::Option<std::vec::Vec<WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch>>,
    dynamic: WafregionalWebAcl_LoggingConfiguration_RedactedFieldsDynamic,
}

impl WafregionalWebAcl_LoggingConfiguration_RedactedFields {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<WafregionalWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WafregionalWebAcl_LoggingConfiguration_RedactedFields {
    type O = terrars::BlockAssignable<WafregionalWebAcl_LoggingConfiguration_RedactedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAcl_LoggingConfiguration_RedactedFields {}

impl BuildWafregionalWebAcl_LoggingConfiguration_RedactedFields {
    pub fn build(self) -> WafregionalWebAcl_LoggingConfiguration_RedactedFields {
        WafregionalWebAcl_LoggingConfiguration_RedactedFields {
            field_to_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WafregionalWebAcl_LoggingConfiguration_RedactedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAcl_LoggingConfiguration_RedactedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WafregionalWebAcl_LoggingConfiguration_RedactedFieldsRef {
        WafregionalWebAcl_LoggingConfiguration_RedactedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAcl_LoggingConfiguration_RedactedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAcl_LoggingConfigurationDynamic {
    redacted_fields: std
    ::option
    ::Option<terrars::DynamicBlock<WafregionalWebAcl_LoggingConfiguration_RedactedFields>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAcl_LoggingConfiguration {
    log_destination: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redacted_fields: std::option::Option<std::vec::Vec<WafregionalWebAcl_LoggingConfiguration_RedactedFields>>,
    dynamic: WafregionalWebAcl_LoggingConfigurationDynamic,
}

impl WafregionalWebAcl_LoggingConfiguration {
    #[doc = "Set the field `redacted_fields`.\n"]
    pub fn set_redacted_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAcl_LoggingConfiguration_RedactedFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redacted_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redacted_fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WafregionalWebAcl_LoggingConfiguration {
    type O = terrars::BlockAssignable<WafregionalWebAcl_LoggingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAcl_LoggingConfiguration {
    #[doc = ""]
    pub log_destination: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAcl_LoggingConfiguration {
    pub fn build(self) -> WafregionalWebAcl_LoggingConfiguration {
        WafregionalWebAcl_LoggingConfiguration {
            log_destination: self.log_destination,
            redacted_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WafregionalWebAcl_LoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAcl_LoggingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAcl_LoggingConfigurationRef {
        WafregionalWebAcl_LoggingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAcl_LoggingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> terrars::ListRef<WafregionalWebAcl_LoggingConfiguration_RedactedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAcl_Rule_Action {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafregionalWebAcl_Rule_Action { }

impl terrars::ToListMappable for WafregionalWebAcl_Rule_Action {
    type O = terrars::BlockAssignable<WafregionalWebAcl_Rule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAcl_Rule_Action {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAcl_Rule_Action {
    pub fn build(self) -> WafregionalWebAcl_Rule_Action {
        WafregionalWebAcl_Rule_Action { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct WafregionalWebAcl_Rule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAcl_Rule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAcl_Rule_ActionRef {
        WafregionalWebAcl_Rule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAcl_Rule_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAcl_Rule_OverrideAction {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafregionalWebAcl_Rule_OverrideAction { }

impl terrars::ToListMappable for WafregionalWebAcl_Rule_OverrideAction {
    type O = terrars::BlockAssignable<WafregionalWebAcl_Rule_OverrideAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAcl_Rule_OverrideAction {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAcl_Rule_OverrideAction {
    pub fn build(self) -> WafregionalWebAcl_Rule_OverrideAction {
        WafregionalWebAcl_Rule_OverrideAction { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct WafregionalWebAcl_Rule_OverrideActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAcl_Rule_OverrideActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAcl_Rule_OverrideActionRef {
        WafregionalWebAcl_Rule_OverrideActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAcl_Rule_OverrideActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAcl_RuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<WafregionalWebAcl_Rule_Action>>,
    override_action: std::option::Option<terrars::DynamicBlock<WafregionalWebAcl_Rule_OverrideAction>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAcl_Rule {
    priority: terrars::PrimField<f64>,
    rule_id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<WafregionalWebAcl_Rule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_action: std::option::Option<std::vec::Vec<WafregionalWebAcl_Rule_OverrideAction>>,
    dynamic: WafregionalWebAcl_RuleDynamic,
}

impl WafregionalWebAcl_Rule {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAcl_Rule_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `override_action`.\n"]
    pub fn set_override_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAcl_Rule_OverrideAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.override_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.override_action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WafregionalWebAcl_Rule {
    type O = terrars::BlockAssignable<WafregionalWebAcl_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAcl_Rule {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub rule_id: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAcl_Rule {
    pub fn build(self) -> WafregionalWebAcl_Rule {
        WafregionalWebAcl_Rule {
            priority: self.priority,
            rule_id: self.rule_id,
            type_: core::default::Default::default(),
            action: core::default::Default::default(),
            override_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WafregionalWebAcl_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAcl_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAcl_RuleRef {
        WafregionalWebAcl_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAcl_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<WafregionalWebAcl_Rule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `override_action` after provisioning.\n"]
    pub fn override_action(&self) -> terrars::ListRef<WafregionalWebAcl_Rule_OverrideActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_action", self.base))
    }
}

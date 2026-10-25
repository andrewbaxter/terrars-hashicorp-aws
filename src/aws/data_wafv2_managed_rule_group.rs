#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataWafv2ManagedRuleGroupData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    scope: terrars::PrimField<std::string::String>,
    vendor_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataWafv2ManagedRuleGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWafv2ManagedRuleGroupData_>,
}

#[derive(Clone)]
pub struct DataWafv2ManagedRuleGroup(std::rc::Rc<DataWafv2ManagedRuleGroup_>);

impl DataWafv2ManagedRuleGroup {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_name`.\n"]
    pub fn set_version_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `available_labels` after provisioning.\n"]
    pub fn available_labels(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_AvailableLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumed_labels` after provisioning.\n"]
    pub fn consumed_labels(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_ConsumedLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumed_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_namespace` after provisioning.\n"]
    pub fn label_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_namespace", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vendor_name` after provisioning.\n"]
    pub fn vendor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vendor_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataWafv2ManagedRuleGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWafv2ManagedRuleGroup { }

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup {
    type O = terrars::ListRef<DataWafv2ManagedRuleGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWafv2ManagedRuleGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_wafv2_managed_rule_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWafv2ManagedRuleGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub scope: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vendor_name: terrars::PrimField<std::string::String>,
}

impl BuildDataWafv2ManagedRuleGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWafv2ManagedRuleGroup {
        let out = DataWafv2ManagedRuleGroup(std::rc::Rc::new(DataWafv2ManagedRuleGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWafv2ManagedRuleGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name: self.name,
                region: core::default::Default::default(),
                scope: self.scope,
                vendor_name: self.vendor_name,
                version_name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWafv2ManagedRuleGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWafv2ManagedRuleGroupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `available_labels` after provisioning.\n"]
    pub fn available_labels(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_AvailableLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `consumed_labels` after provisioning.\n"]
    pub fn consumed_labels(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_ConsumedLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumed_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_namespace` after provisioning.\n"]
    pub fn label_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_namespace", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vendor_name` after provisioning.\n"]
    pub fn vendor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vendor_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_name` after provisioning.\n"]
    pub fn version_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_AvailableLabels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWafv2ManagedRuleGroup_AvailableLabels {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_AvailableLabels {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_AvailableLabels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_AvailableLabels {}

impl BuildDataWafv2ManagedRuleGroup_AvailableLabels {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_AvailableLabels {
        DataWafv2ManagedRuleGroup_AvailableLabels { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_AvailableLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_AvailableLabelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWafv2ManagedRuleGroup_AvailableLabelsRef {
        DataWafv2ManagedRuleGroup_AvailableLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_AvailableLabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_ConsumedLabels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWafv2ManagedRuleGroup_ConsumedLabels {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_ConsumedLabels {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_ConsumedLabels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_ConsumedLabels {}

impl BuildDataWafv2ManagedRuleGroup_ConsumedLabels {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_ConsumedLabels {
        DataWafv2ManagedRuleGroup_ConsumedLabels { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_ConsumedLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_ConsumedLabelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWafv2ManagedRuleGroup_ConsumedLabelsRef {
        DataWafv2ManagedRuleGroup_ConsumedLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_ConsumedLabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader {
    type O =
        terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader {
        DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeaderRef {
        DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insert_header: std
    ::option
    ::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling {
    #[doc = "Set the field `insert_header`.\n"]
    pub fn set_insert_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeader>,
                    >,
    ) -> Self {
        self.insert_header = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling {
        DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling {
            insert_header: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandlingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandlingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandlingRef {
        DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandlingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandlingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `insert_header` after provisioning.\n"]
    pub fn insert_header(
        &self,
    ) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling_InsertHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.insert_header", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Allow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std
    ::option
    ::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Allow {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling>>,
    ) -> Self {
        self.custom_request_handling = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action_Allow {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action_Allow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action_Allow {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action_Allow {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action_Allow {
        DataWafv2ManagedRuleGroup_Rules_Action_Allow { custom_request_handling: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_AllowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_Action_AllowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWafv2ManagedRuleGroup_Rules_Action_AllowRef {
        DataWafv2ManagedRuleGroup_Rules_Action_AllowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_Action_AllowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(
        &self,
    ) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandlingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader {
        DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeaderRef {
        DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_response_body_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header: std
    ::option
    ::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse {
    #[doc = "Set the field `custom_response_body_key`.\n"]
    pub fn set_custom_response_body_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_response_body_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_code`.\n"]
    pub fn set_response_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_header`.\n"]
    pub fn set_response_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeader>,
                    >,
    ) -> Self {
        self.response_header = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse {
        DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse {
            custom_response_body_key: core::default::Default::default(),
            response_code: core::default::Default::default(),
            response_header: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponseRef {
        DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_response_body_key` after provisioning.\n"]
    pub fn custom_response_body_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_response_body_key", self.base))
    }

    #[doc = "Get a reference to the value of field `response_code` after provisioning.\n"]
    pub fn response_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_code", self.base))
    }

    #[doc = "Get a reference to the value of field `response_header` after provisioning.\n"]
    pub fn response_header(
        &self,
    ) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse_ResponseHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_header", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Block {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_response: std
    ::option
    ::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Block {
    #[doc = "Set the field `custom_response`.\n"]
    pub fn set_custom_response(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponse>>,
    ) -> Self {
        self.custom_response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action_Block {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action_Block>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action_Block {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action_Block {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action_Block {
        DataWafv2ManagedRuleGroup_Rules_Action_Block { custom_response: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_BlockRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_Action_BlockRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWafv2ManagedRuleGroup_Rules_Action_BlockRef {
        DataWafv2ManagedRuleGroup_Rules_Action_BlockRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_Action_BlockRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_response` after provisioning.\n"]
    pub fn custom_response(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_Block_CustomResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_response", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Captcha {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std
    ::option
    ::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Captcha {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling>>,
    ) -> Self {
        self.custom_request_handling = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action_Captcha {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action_Captcha>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action_Captcha {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action_Captcha {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action_Captcha {
        DataWafv2ManagedRuleGroup_Rules_Action_Captcha { custom_request_handling: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_CaptchaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_Action_CaptchaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWafv2ManagedRuleGroup_Rules_Action_CaptchaRef {
        DataWafv2ManagedRuleGroup_Rules_Action_CaptchaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_Action_CaptchaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(
        &self,
    ) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandlingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Challenge {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std
    ::option
    ::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Challenge {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling>>,
    ) -> Self {
        self.custom_request_handling = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action_Challenge {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action_Challenge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action_Challenge {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action_Challenge {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action_Challenge {
        DataWafv2ManagedRuleGroup_Rules_Action_Challenge {
            custom_request_handling: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_ChallengeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_Action_ChallengeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWafv2ManagedRuleGroup_Rules_Action_ChallengeRef {
        DataWafv2ManagedRuleGroup_Rules_Action_ChallengeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_Action_ChallengeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(
        &self,
    ) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandlingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_Count {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std
    ::option
    ::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action_Count {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandling>>,
    ) -> Self {
        self.custom_request_handling = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action_Count {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action_Count>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action_Count {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action_Count {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action_Count {
        DataWafv2ManagedRuleGroup_Rules_Action_Count { custom_request_handling: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action_CountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_Action_CountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataWafv2ManagedRuleGroup_Rules_Action_CountRef {
        DataWafv2ManagedRuleGroup_Rules_Action_CountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_Action_CountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(
        &self,
    ) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_Allow_CustomRequestHandlingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block: std::option::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Block>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    captcha: std::option::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Captcha>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge: std::option::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Challenge>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Count>>,
}

impl DataWafv2ManagedRuleGroup_Rules_Action {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Allow>>,
    ) -> Self {
        self.allow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block`.\n"]
    pub fn set_block(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Block>>,
    ) -> Self {
        self.block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `captcha`.\n"]
    pub fn set_captcha(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Captcha>>,
    ) -> Self {
        self.captcha = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `challenge`.\n"]
    pub fn set_challenge(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Challenge>>,
    ) -> Self {
        self.challenge = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `count`.\n"]
    pub fn set_count(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action_Count>>,
    ) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules_Action {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules_Action {}

impl BuildDataWafv2ManagedRuleGroup_Rules_Action {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules_Action {
        DataWafv2ManagedRuleGroup_Rules_Action {
            allow: core::default::Default::default(),
            block: core::default::Default::default(),
            captcha: core::default::Default::default(),
            challenge: core::default::Default::default(),
            count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_Rules_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_Rules_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWafv2ManagedRuleGroup_Rules_ActionRef {
        DataWafv2ManagedRuleGroup_Rules_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_Rules_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_AllowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `block` after provisioning.\n"]
    pub fn block(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_BlockRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block", self.base))
    }

    #[doc = "Get a reference to the value of field `captcha` after provisioning.\n"]
    pub fn captcha(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_CaptchaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.captcha", self.base))
    }

    #[doc = "Get a reference to the value of field `challenge` after provisioning.\n"]
    pub fn challenge(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_ChallengeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge", self.base))
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_Action_CountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWafv2ManagedRuleGroup_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWafv2ManagedRuleGroup_Rules {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataWafv2ManagedRuleGroup_Rules_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWafv2ManagedRuleGroup_Rules {
    type O = terrars::BlockAssignable<DataWafv2ManagedRuleGroup_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWafv2ManagedRuleGroup_Rules {}

impl BuildDataWafv2ManagedRuleGroup_Rules {
    pub fn build(self) -> DataWafv2ManagedRuleGroup_Rules {
        DataWafv2ManagedRuleGroup_Rules {
            action: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataWafv2ManagedRuleGroup_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWafv2ManagedRuleGroup_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWafv2ManagedRuleGroup_RulesRef {
        DataWafv2ManagedRuleGroup_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWafv2ManagedRuleGroup_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataWafv2ManagedRuleGroup_Rules_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

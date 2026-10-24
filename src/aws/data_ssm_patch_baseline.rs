use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmPatchBaselineData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_baseline: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system: std::option::Option<terrars::PrimField<std::string::String>>,
    owner: terrars::PrimField<std::string::String>,
}

struct DataSsmPatchBaseline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSsmPatchBaselineData>,
}

#[derive(Clone)]
pub struct DataSsmPatchBaseline(std::rc::Rc<DataSsmPatchBaseline_>);

impl DataSsmPatchBaseline {
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

    #[doc = "Set the field `default_baseline`.\n"]
    pub fn set_default_baseline(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_baseline = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system`.\n"]
    pub fn set_operating_system(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().operating_system = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `approval_rule` after provisioning.\n"]
    pub fn approval_rule(&self) -> terrars::ListRef<DataSsmPatchBaselineApprovalRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approved_patches` after provisioning.\n"]
    pub fn approved_patches(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approved_patches", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approved_patches_compliance_level` after provisioning.\n"]
    pub fn approved_patches_compliance_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_compliance_level", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approved_patches_enable_non_security` after provisioning.\n"]
    pub fn approved_patches_enable_non_security(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_enable_non_security", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_baseline` after provisioning.\n"]
    pub fn default_baseline(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_baseline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_filter` after provisioning.\n"]
    pub fn global_filter(&self) -> terrars::ListRef<DataSsmPatchBaselineGlobalFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejected_patches` after provisioning.\n"]
    pub fn rejected_patches(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rejected_patches", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejected_patches_action` after provisioning.\n"]
    pub fn rejected_patches_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rejected_patches_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DataSsmPatchBaselineSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl terrars::Referable for DataSsmPatchBaseline {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSsmPatchBaseline { }

impl terrars::ToListMappable for DataSsmPatchBaseline {
    type O = terrars::ListRef<DataSsmPatchBaselineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSsmPatchBaseline_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ssm_patch_baseline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmPatchBaseline {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub owner: terrars::PrimField<std::string::String>,
}

impl BuildDataSsmPatchBaseline {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSsmPatchBaseline {
        let out = DataSsmPatchBaseline(std::rc::Rc::new(DataSsmPatchBaseline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSsmPatchBaselineData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                default_baseline: core::default::Default::default(),
                id: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                operating_system: core::default::Default::default(),
                owner: self.owner,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmPatchBaselineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmPatchBaselineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSsmPatchBaselineRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `approval_rule` after provisioning.\n"]
    pub fn approval_rule(&self) -> terrars::ListRef<DataSsmPatchBaselineApprovalRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approved_patches` after provisioning.\n"]
    pub fn approved_patches(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approved_patches", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approved_patches_compliance_level` after provisioning.\n"]
    pub fn approved_patches_compliance_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_compliance_level", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approved_patches_enable_non_security` after provisioning.\n"]
    pub fn approved_patches_enable_non_security(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_enable_non_security", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_baseline` after provisioning.\n"]
    pub fn default_baseline(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_baseline", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_filter` after provisioning.\n"]
    pub fn global_filter(&self) -> terrars::ListRef<DataSsmPatchBaselineGlobalFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejected_patches` after provisioning.\n"]
    pub fn rejected_patches(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rejected_patches", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejected_patches_action` after provisioning.\n"]
    pub fn rejected_patches_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rejected_patches_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DataSsmPatchBaselineSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
    type O = terrars::BlockAssignable<DataSsmPatchBaselineApprovalRuleElPatchFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmPatchBaselineApprovalRuleElPatchFilterEl {}

impl BuildDataSsmPatchBaselineApprovalRuleElPatchFilterEl {
    pub fn build(self) -> DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
        DataSsmPatchBaselineApprovalRuleElPatchFilterEl {
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
        DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmPatchBaselineApprovalRuleElPatchFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmPatchBaselineApprovalRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approve_after_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approve_until_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_non_security: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch_filter: std::option::Option<terrars::ListField<DataSsmPatchBaselineApprovalRuleElPatchFilterEl>>,
}

impl DataSsmPatchBaselineApprovalRuleEl {
    #[doc = "Set the field `approve_after_days`.\n"]
    pub fn set_approve_after_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.approve_after_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approve_until_date`.\n"]
    pub fn set_approve_until_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.approve_until_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compliance_level`.\n"]
    pub fn set_compliance_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compliance_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_non_security`.\n"]
    pub fn set_enable_non_security(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_non_security = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `patch_filter`.\n"]
    pub fn set_patch_filter(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSsmPatchBaselineApprovalRuleElPatchFilterEl>>,
    ) -> Self {
        self.patch_filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmPatchBaselineApprovalRuleEl {
    type O = terrars::BlockAssignable<DataSsmPatchBaselineApprovalRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmPatchBaselineApprovalRuleEl {}

impl BuildDataSsmPatchBaselineApprovalRuleEl {
    pub fn build(self) -> DataSsmPatchBaselineApprovalRuleEl {
        DataSsmPatchBaselineApprovalRuleEl {
            approve_after_days: core::default::Default::default(),
            approve_until_date: core::default::Default::default(),
            compliance_level: core::default::Default::default(),
            enable_non_security: core::default::Default::default(),
            patch_filter: core::default::Default::default(),
        }
    }
}

pub struct DataSsmPatchBaselineApprovalRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmPatchBaselineApprovalRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmPatchBaselineApprovalRuleElRef {
        DataSsmPatchBaselineApprovalRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmPatchBaselineApprovalRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approve_after_days` after provisioning.\n"]
    pub fn approve_after_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approve_after_days", self.base))
    }

    #[doc = "Get a reference to the value of field `approve_until_date` after provisioning.\n"]
    pub fn approve_until_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approve_until_date", self.base))
    }

    #[doc = "Get a reference to the value of field `compliance_level` after provisioning.\n"]
    pub fn compliance_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_level", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_non_security` after provisioning.\n"]
    pub fn enable_non_security(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_non_security", self.base))
    }

    #[doc = "Get a reference to the value of field `patch_filter` after provisioning.\n"]
    pub fn patch_filter(&self) -> terrars::ListRef<DataSsmPatchBaselineApprovalRuleElPatchFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.patch_filter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmPatchBaselineGlobalFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataSsmPatchBaselineGlobalFilterEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmPatchBaselineGlobalFilterEl {
    type O = terrars::BlockAssignable<DataSsmPatchBaselineGlobalFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmPatchBaselineGlobalFilterEl {}

impl BuildDataSsmPatchBaselineGlobalFilterEl {
    pub fn build(self) -> DataSsmPatchBaselineGlobalFilterEl {
        DataSsmPatchBaselineGlobalFilterEl {
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataSsmPatchBaselineGlobalFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmPatchBaselineGlobalFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmPatchBaselineGlobalFilterElRef {
        DataSsmPatchBaselineGlobalFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmPatchBaselineGlobalFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmPatchBaselineSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    products: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataSsmPatchBaselineSourceEl {
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `products`.\n"]
    pub fn set_products(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.products = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmPatchBaselineSourceEl {
    type O = terrars::BlockAssignable<DataSsmPatchBaselineSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmPatchBaselineSourceEl {}

impl BuildDataSsmPatchBaselineSourceEl {
    pub fn build(self) -> DataSsmPatchBaselineSourceEl {
        DataSsmPatchBaselineSourceEl {
            configuration: core::default::Default::default(),
            name: core::default::Default::default(),
            products: core::default::Default::default(),
        }
    }
}

pub struct DataSsmPatchBaselineSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmPatchBaselineSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmPatchBaselineSourceElRef {
        DataSsmPatchBaselineSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmPatchBaselineSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `products` after provisioning.\n"]
    pub fn products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.products", self.base))
    }
}

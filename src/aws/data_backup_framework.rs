use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBackupFrameworkData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataBackupFramework_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBackupFrameworkData>,
}

#[derive(Clone)]
pub struct DataBackupFramework(std::rc::Rc<DataBackupFramework_>);

impl DataBackupFramework {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control` after provisioning.\n"]
    pub fn control(&self) -> terrars::SetRef<DataBackupFrameworkControlElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.control", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataBackupFramework {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBackupFramework { }

impl terrars::ToListMappable for DataBackupFramework {
    type O = terrars::ListRef<DataBackupFrameworkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBackupFramework_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_backup_framework".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupFramework {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataBackupFramework {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBackupFramework {
        let out = DataBackupFramework(std::rc::Rc::new(DataBackupFramework_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBackupFrameworkData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupFrameworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupFrameworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBackupFrameworkRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control` after provisioning.\n"]
    pub fn control(&self) -> terrars::SetRef<DataBackupFrameworkControlElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.control", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupFrameworkControlElInputParameterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupFrameworkControlElInputParameterEl {
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

impl terrars::ToListMappable for DataBackupFrameworkControlElInputParameterEl {
    type O = terrars::BlockAssignable<DataBackupFrameworkControlElInputParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupFrameworkControlElInputParameterEl {}

impl BuildDataBackupFrameworkControlElInputParameterEl {
    pub fn build(self) -> DataBackupFrameworkControlElInputParameterEl {
        DataBackupFrameworkControlElInputParameterEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataBackupFrameworkControlElInputParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupFrameworkControlElInputParameterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBackupFrameworkControlElInputParameterElRef {
        DataBackupFrameworkControlElInputParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupFrameworkControlElInputParameterElRef {
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
pub struct DataBackupFrameworkControlElScopeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_resource_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_resource_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataBackupFrameworkControlElScopeEl {
    #[doc = "Set the field `compliance_resource_ids`.\n"]
    pub fn set_compliance_resource_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.compliance_resource_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compliance_resource_types`.\n"]
    pub fn set_compliance_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.compliance_resource_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupFrameworkControlElScopeEl {
    type O = terrars::BlockAssignable<DataBackupFrameworkControlElScopeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupFrameworkControlElScopeEl {}

impl BuildDataBackupFrameworkControlElScopeEl {
    pub fn build(self) -> DataBackupFrameworkControlElScopeEl {
        DataBackupFrameworkControlElScopeEl {
            compliance_resource_ids: core::default::Default::default(),
            compliance_resource_types: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataBackupFrameworkControlElScopeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupFrameworkControlElScopeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupFrameworkControlElScopeElRef {
        DataBackupFrameworkControlElScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupFrameworkControlElScopeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compliance_resource_ids` after provisioning.\n"]
    pub fn compliance_resource_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.compliance_resource_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `compliance_resource_types` after provisioning.\n"]
    pub fn compliance_resource_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.compliance_resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupFrameworkControlEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_parameter: std::option::Option<terrars::SetField<DataBackupFrameworkControlElInputParameterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::ListField<DataBackupFrameworkControlElScopeEl>>,
}

impl DataBackupFrameworkControlEl {
    #[doc = "Set the field `input_parameter`.\n"]
    pub fn set_input_parameter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataBackupFrameworkControlElInputParameterEl>>,
    ) -> Self {
        self.input_parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBackupFrameworkControlElScopeEl>>,
    ) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupFrameworkControlEl {
    type O = terrars::BlockAssignable<DataBackupFrameworkControlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupFrameworkControlEl {}

impl BuildDataBackupFrameworkControlEl {
    pub fn build(self) -> DataBackupFrameworkControlEl {
        DataBackupFrameworkControlEl {
            input_parameter: core::default::Default::default(),
            name: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

pub struct DataBackupFrameworkControlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupFrameworkControlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupFrameworkControlElRef {
        DataBackupFrameworkControlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupFrameworkControlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_parameter` after provisioning.\n"]
    pub fn input_parameter(&self) -> terrars::SetRef<DataBackupFrameworkControlElInputParameterElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.input_parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<DataBackupFrameworkControlElScopeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

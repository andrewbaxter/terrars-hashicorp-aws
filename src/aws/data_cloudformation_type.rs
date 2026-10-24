use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudformationTypeData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    type_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudformationType_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudformationTypeData>,
}

#[derive(Clone)]
pub struct DataCloudformationType(std::rc::Rc<DataCloudformationType_>);

impl DataCloudformationType {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_name`.\n"]
    pub fn set_type_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_version_id` after provisioning.\n"]
    pub fn default_version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deprecated_status` after provisioning.\n"]
    pub fn deprecated_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecated_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `documentation_url` after provisioning.\n"]
    pub fn documentation_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default_version` after provisioning.\n"]
    pub fn is_default_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataCloudformationTypeLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_type` after provisioning.\n"]
    pub fn provisioning_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_url` after provisioning.\n"]
    pub fn source_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_arn` after provisioning.\n"]
    pub fn type_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_name` after provisioning.\n"]
    pub fn type_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudformationType {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudformationType { }

impl terrars::ToListMappable for DataCloudformationType {
    type O = terrars::ListRef<DataCloudformationTypeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudformationType_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudformation_type".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudformationType {
    pub tf_id: std::string::String,
}

impl BuildDataCloudformationType {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudformationType {
        let out = DataCloudformationType(std::rc::Rc::new(DataCloudformationType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudformationTypeData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                type_: core::default::Default::default(),
                type_name: core::default::Default::default(),
                version_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudformationTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudformationTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudformationTypeRef {
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

    #[doc = "Get a reference to the value of field `default_version_id` after provisioning.\n"]
    pub fn default_version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deprecated_status` after provisioning.\n"]
    pub fn deprecated_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deprecated_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `documentation_url` after provisioning.\n"]
    pub fn documentation_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default_version` after provisioning.\n"]
    pub fn is_default_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataCloudformationTypeLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_type` after provisioning.\n"]
    pub fn provisioning_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_url` after provisioning.\n"]
    pub fn source_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_arn` after provisioning.\n"]
    pub fn type_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_name` after provisioning.\n"]
    pub fn type_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudformationTypeLoggingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudformationTypeLoggingConfigEl {
    #[doc = "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_role_arn`.\n"]
    pub fn set_log_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudformationTypeLoggingConfigEl {
    type O = terrars::BlockAssignable<DataCloudformationTypeLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudformationTypeLoggingConfigEl {}

impl BuildDataCloudformationTypeLoggingConfigEl {
    pub fn build(self) -> DataCloudformationTypeLoggingConfigEl {
        DataCloudformationTypeLoggingConfigEl {
            log_group_name: core::default::Default::default(),
            log_role_arn: core::default::Default::default(),
        }
    }
}

pub struct DataCloudformationTypeLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudformationTypeLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudformationTypeLoggingConfigElRef {
        DataCloudformationTypeLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudformationTypeLoggingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `log_role_arn` after provisioning.\n"]
    pub fn log_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_role_arn", self.base))
    }
}

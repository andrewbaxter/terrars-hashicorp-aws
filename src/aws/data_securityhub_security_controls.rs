#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSecurityhubSecurityControlsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSecurityhubSecurityControls_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSecurityhubSecurityControlsData_>,
}

#[derive(Clone)]
pub struct DataSecurityhubSecurityControls(std::rc::Rc<DataSecurityhubSecurityControls_>);

impl DataSecurityhubSecurityControls {
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

    #[doc = "Set the field `standards_arn`.\n"]
    pub fn set_standards_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().standards_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_control_definitions` after provisioning.\n"]
    pub fn security_control_definitions(
        &self,
    ) -> terrars::ListRef<DataSecurityhubSecurityControls_SecurityControlDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_control_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_arn` after provisioning.\n"]
    pub fn standards_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standards_arn", self.extract_ref()))
    }
}

impl terrars::Referable for DataSecurityhubSecurityControls {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSecurityhubSecurityControls { }

impl terrars::ToListMappable for DataSecurityhubSecurityControls {
    type O = terrars::ListRef<DataSecurityhubSecurityControlsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSecurityhubSecurityControls_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_securityhub_security_controls".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecurityhubSecurityControls {
    pub tf_id: std::string::String,
}

impl BuildDataSecurityhubSecurityControls {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSecurityhubSecurityControls {
        let out = DataSecurityhubSecurityControls(std::rc::Rc::new(DataSecurityhubSecurityControls_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSecurityhubSecurityControlsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                standards_arn: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecurityhubSecurityControlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecurityhubSecurityControlsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSecurityhubSecurityControlsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_control_definitions` after provisioning.\n"]
    pub fn security_control_definitions(
        &self,
    ) -> terrars::ListRef<DataSecurityhubSecurityControls_SecurityControlDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_control_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_arn` after provisioning.\n"]
    pub fn standards_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standards_arn", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSecurityhubSecurityControls_SecurityControlDefinitions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_region_availability: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customizable_properties: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remediation_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_control_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity_rating: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecurityhubSecurityControls_SecurityControlDefinitions {
    #[doc = "Set the field `current_region_availability`.\n"]
    pub fn set_current_region_availability(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.current_region_availability = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customizable_properties`.\n"]
    pub fn set_customizable_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.customizable_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remediation_url`.\n"]
    pub fn set_remediation_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.remediation_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_control_id`.\n"]
    pub fn set_security_control_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.security_control_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity_rating`.\n"]
    pub fn set_severity_rating(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity_rating = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecurityhubSecurityControls_SecurityControlDefinitions {
    type O = terrars::BlockAssignable<DataSecurityhubSecurityControls_SecurityControlDefinitions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecurityhubSecurityControls_SecurityControlDefinitions {}

impl BuildDataSecurityhubSecurityControls_SecurityControlDefinitions {
    pub fn build(self) -> DataSecurityhubSecurityControls_SecurityControlDefinitions {
        DataSecurityhubSecurityControls_SecurityControlDefinitions {
            current_region_availability: core::default::Default::default(),
            customizable_properties: core::default::Default::default(),
            description: core::default::Default::default(),
            remediation_url: core::default::Default::default(),
            security_control_id: core::default::Default::default(),
            severity_rating: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecurityhubSecurityControls_SecurityControlDefinitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecurityhubSecurityControls_SecurityControlDefinitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecurityhubSecurityControls_SecurityControlDefinitionsRef {
        DataSecurityhubSecurityControls_SecurityControlDefinitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecurityhubSecurityControls_SecurityControlDefinitionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_region_availability` after provisioning.\n"]
    pub fn current_region_availability(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_region_availability", self.base))
    }

    #[doc = "Get a reference to the value of field `customizable_properties` after provisioning.\n"]
    pub fn customizable_properties(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customizable_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `remediation_url` after provisioning.\n"]
    pub fn remediation_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation_url", self.base))
    }

    #[doc = "Get a reference to the value of field `security_control_id` after provisioning.\n"]
    pub fn security_control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_control_id", self.base))
    }

    #[doc = "Get a reference to the value of field `severity_rating` after provisioning.\n"]
    pub fn severity_rating(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity_rating", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

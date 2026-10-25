#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSecurityhubStandardsControlAssociationsData_ {
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
    security_control_id: terrars::PrimField<std::string::String>,
}

struct DataSecurityhubStandardsControlAssociations_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSecurityhubStandardsControlAssociationsData_>,
}

#[derive(Clone)]
pub struct DataSecurityhubStandardsControlAssociations(std::rc::Rc<DataSecurityhubStandardsControlAssociations_>);

impl DataSecurityhubStandardsControlAssociations {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_control_id` after provisioning.\n"]
    pub fn security_control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_control_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_control_associations` after provisioning.\n"]
    pub fn standards_control_associations(
        &self,
    ) -> terrars::ListRef<DataSecurityhubStandardsControlAssociations_StandardsControlAssociationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.standards_control_associations", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataSecurityhubStandardsControlAssociations {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSecurityhubStandardsControlAssociations { }

impl terrars::ToListMappable for DataSecurityhubStandardsControlAssociations {
    type O = terrars::ListRef<DataSecurityhubStandardsControlAssociationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSecurityhubStandardsControlAssociations_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_securityhub_standards_control_associations".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecurityhubStandardsControlAssociations {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub security_control_id: terrars::PrimField<std::string::String>,
}

impl BuildDataSecurityhubStandardsControlAssociations {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSecurityhubStandardsControlAssociations {
        let out =
            DataSecurityhubStandardsControlAssociations(
                std::rc::Rc::new(DataSecurityhubStandardsControlAssociations_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataSecurityhubStandardsControlAssociationsData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        region: core::default::Default::default(),
                        security_control_id: self.security_control_id,
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecurityhubStandardsControlAssociationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecurityhubStandardsControlAssociationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSecurityhubStandardsControlAssociationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_control_id` after provisioning.\n"]
    pub fn security_control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_control_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_control_associations` after provisioning.\n"]
    pub fn standards_control_associations(
        &self,
    ) -> terrars::ListRef<DataSecurityhubStandardsControlAssociations_StandardsControlAssociationsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.standards_control_associations", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataSecurityhubStandardsControlAssociations_StandardsControlAssociations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    association_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_requirements: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_control_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_control_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_control_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_control_title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecurityhubStandardsControlAssociations_StandardsControlAssociations {
    #[doc = "Set the field `association_status`.\n"]
    pub fn set_association_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.association_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `related_requirements`.\n"]
    pub fn set_related_requirements(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.related_requirements = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_control_arn`.\n"]
    pub fn set_security_control_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.security_control_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `standards_arn`.\n"]
    pub fn set_standards_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.standards_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standards_control_description`.\n"]
    pub fn set_standards_control_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.standards_control_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standards_control_title`.\n"]
    pub fn set_standards_control_title(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.standards_control_title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_reason`.\n"]
    pub fn set_updated_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecurityhubStandardsControlAssociations_StandardsControlAssociations {
    type O = terrars::BlockAssignable<DataSecurityhubStandardsControlAssociations_StandardsControlAssociations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecurityhubStandardsControlAssociations_StandardsControlAssociations {}

impl BuildDataSecurityhubStandardsControlAssociations_StandardsControlAssociations {
    pub fn build(self) -> DataSecurityhubStandardsControlAssociations_StandardsControlAssociations {
        DataSecurityhubStandardsControlAssociations_StandardsControlAssociations {
            association_status: core::default::Default::default(),
            related_requirements: core::default::Default::default(),
            security_control_arn: core::default::Default::default(),
            security_control_id: core::default::Default::default(),
            standards_arn: core::default::Default::default(),
            standards_control_description: core::default::Default::default(),
            standards_control_title: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            updated_reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecurityhubStandardsControlAssociations_StandardsControlAssociationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecurityhubStandardsControlAssociations_StandardsControlAssociationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecurityhubStandardsControlAssociations_StandardsControlAssociationsRef {
        DataSecurityhubStandardsControlAssociations_StandardsControlAssociationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecurityhubStandardsControlAssociations_StandardsControlAssociationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `association_status` after provisioning.\n"]
    pub fn association_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_status", self.base))
    }

    #[doc = "Get a reference to the value of field `related_requirements` after provisioning.\n"]
    pub fn related_requirements(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.related_requirements", self.base))
    }

    #[doc = "Get a reference to the value of field `security_control_arn` after provisioning.\n"]
    pub fn security_control_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_control_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `security_control_id` after provisioning.\n"]
    pub fn security_control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_control_id", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_arn` after provisioning.\n"]
    pub fn standards_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standards_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_control_description` after provisioning.\n"]
    pub fn standards_control_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standards_control_description", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_control_title` after provisioning.\n"]
    pub fn standards_control_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standards_control_title", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_reason` after provisioning.\n"]
    pub fn updated_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_reason", self.base))
    }
}

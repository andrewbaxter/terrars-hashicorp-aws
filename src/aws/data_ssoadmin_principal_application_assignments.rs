#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsoadminPrincipalApplicationAssignmentsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    instance_arn: terrars::PrimField<std::string::String>,
    principal_id: terrars::PrimField<std::string::String>,
    principal_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_assignments: std
    ::option
    ::Option<std::vec::Vec<DataSsoadminPrincipalApplicationAssignments_ApplicationAssignments>>,
    dynamic: DataSsoadminPrincipalApplicationAssignmentsDynamic,
}

struct DataSsoadminPrincipalApplicationAssignments_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSsoadminPrincipalApplicationAssignmentsData_>,
}

#[derive(Clone)]
pub struct DataSsoadminPrincipalApplicationAssignments(std::rc::Rc<DataSsoadminPrincipalApplicationAssignments_>);

impl DataSsoadminPrincipalApplicationAssignments {
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

    #[doc = "Set the field `application_assignments`.\n"]
    pub fn set_application_assignments(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataSsoadminPrincipalApplicationAssignments_ApplicationAssignments>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().application_assignments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.application_assignments = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_id` after provisioning.\n"]
    pub fn principal_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_assignments` after provisioning.\n"]
    pub fn application_assignments(
        &self,
    ) -> terrars::ListRef<DataSsoadminPrincipalApplicationAssignments_ApplicationAssignmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_assignments", self.extract_ref()))
    }
}

impl terrars::Referable for DataSsoadminPrincipalApplicationAssignments {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSsoadminPrincipalApplicationAssignments { }

impl terrars::ToListMappable for DataSsoadminPrincipalApplicationAssignments {
    type O = terrars::ListRef<DataSsoadminPrincipalApplicationAssignmentsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSsoadminPrincipalApplicationAssignments_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ssoadmin_principal_application_assignments".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsoadminPrincipalApplicationAssignments {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub principal_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub principal_type: terrars::PrimField<std::string::String>,
}

impl BuildDataSsoadminPrincipalApplicationAssignments {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSsoadminPrincipalApplicationAssignments {
        let out =
            DataSsoadminPrincipalApplicationAssignments(
                std::rc::Rc::new(DataSsoadminPrincipalApplicationAssignments_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataSsoadminPrincipalApplicationAssignmentsData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        instance_arn: self.instance_arn,
                        principal_id: self.principal_id,
                        principal_type: self.principal_type,
                        region: core::default::Default::default(),
                        application_assignments: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsoadminPrincipalApplicationAssignmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsoadminPrincipalApplicationAssignmentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSsoadminPrincipalApplicationAssignmentsRef {
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

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_id` after provisioning.\n"]
    pub fn principal_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_assignments` after provisioning.\n"]
    pub fn application_assignments(
        &self,
    ) -> terrars::ListRef<DataSsoadminPrincipalApplicationAssignments_ApplicationAssignmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.application_assignments", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataSsoadminPrincipalApplicationAssignmentsDynamic {
    application_assignments: std
    ::option
    ::Option<terrars::DynamicBlock<DataSsoadminPrincipalApplicationAssignments_ApplicationAssignments>>,
}

#[derive(Serialize)]
pub struct DataSsoadminPrincipalApplicationAssignments_ApplicationAssignments {}

impl DataSsoadminPrincipalApplicationAssignments_ApplicationAssignments { }

impl terrars::ToListMappable for DataSsoadminPrincipalApplicationAssignments_ApplicationAssignments {
    type O = terrars::BlockAssignable<DataSsoadminPrincipalApplicationAssignments_ApplicationAssignments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminPrincipalApplicationAssignments_ApplicationAssignments {}

impl BuildDataSsoadminPrincipalApplicationAssignments_ApplicationAssignments {
    pub fn build(self) -> DataSsoadminPrincipalApplicationAssignments_ApplicationAssignments {
        DataSsoadminPrincipalApplicationAssignments_ApplicationAssignments {}
    }
}

#[allow(dead_code)]
pub struct DataSsoadminPrincipalApplicationAssignments_ApplicationAssignmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsoadminPrincipalApplicationAssignments_ApplicationAssignmentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSsoadminPrincipalApplicationAssignments_ApplicationAssignmentsRef {
        DataSsoadminPrincipalApplicationAssignments_ApplicationAssignmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminPrincipalApplicationAssignments_ApplicationAssignmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_arn` after provisioning.\n"]
    pub fn application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_id` after provisioning.\n"]
    pub fn principal_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_id", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_type", self.base))
    }
}

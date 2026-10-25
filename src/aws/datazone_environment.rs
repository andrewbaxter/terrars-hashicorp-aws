#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatazoneEnvironmentData_ {
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
    account_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blueprint_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    domain_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glossary_terms: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    profile_identifier: terrars::PrimField<std::string::String>,
    project_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DatazoneEnvironment_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_parameters: std::option::Option<std::vec::Vec<DatazoneEnvironment_UserParameters>>,
    dynamic: DatazoneEnvironmentDynamic,
}

struct DatazoneEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatazoneEnvironmentData_>,
}

#[derive(Clone)]
pub struct DatazoneEnvironment(std::rc::Rc<DatazoneEnvironment_>);

impl DatazoneEnvironment {
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

    #[doc = "Set the field `account_identifier`.\n"]
    pub fn set_account_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `account_region`.\n"]
    pub fn set_account_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `blueprint_identifier`.\n"]
    pub fn set_blueprint_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().blueprint_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `glossary_terms`.\n"]
    pub fn set_glossary_terms(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().glossary_terms = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DatazoneEnvironment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_parameters`.\n"]
    pub fn set_user_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazoneEnvironment_UserParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_identifier` after provisioning.\n"]
    pub fn account_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_region` after provisioning.\n"]
    pub fn account_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blueprint_identifier` after provisioning.\n"]
    pub fn blueprint_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blueprint_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glossary_terms` after provisioning.\n"]
    pub fn glossary_terms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glossary_terms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_deployment` after provisioning.\n"]
    pub fn last_deployment(&self) -> terrars::ListRef<DatazoneEnvironment_LastDeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_identifier` after provisioning.\n"]
    pub fn profile_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_identifier` after provisioning.\n"]
    pub fn project_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_environment` after provisioning.\n"]
    pub fn provider_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_resources` after provisioning.\n"]
    pub fn provisioned_resources(&self) -> terrars::ListRef<DatazoneEnvironment_ProvisionedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneEnvironment_TimeoutsRef {
        DatazoneEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_parameters` after provisioning.\n"]
    pub fn user_parameters(&self) -> terrars::ListRef<DatazoneEnvironment_UserParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_parameters", self.extract_ref()))
    }
}

impl terrars::Referable for DatazoneEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatazoneEnvironment { }

impl terrars::ToListMappable for DatazoneEnvironment {
    type O = terrars::ListRef<DatazoneEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatazoneEnvironment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_datazone_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatazoneEnvironment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub profile_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub project_identifier: terrars::PrimField<std::string::String>,
}

impl BuildDatazoneEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> DatazoneEnvironment {
        let out = DatazoneEnvironment(std::rc::Rc::new(DatazoneEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatazoneEnvironmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_identifier: core::default::Default::default(),
                account_region: core::default::Default::default(),
                blueprint_identifier: core::default::Default::default(),
                description: core::default::Default::default(),
                domain_identifier: self.domain_identifier,
                glossary_terms: core::default::Default::default(),
                name: self.name,
                profile_identifier: self.profile_identifier,
                project_identifier: self.project_identifier,
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_parameters: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatazoneEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazoneEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatazoneEnvironmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_identifier` after provisioning.\n"]
    pub fn account_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_region` after provisioning.\n"]
    pub fn account_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blueprint_identifier` after provisioning.\n"]
    pub fn blueprint_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.blueprint_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glossary_terms` after provisioning.\n"]
    pub fn glossary_terms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glossary_terms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_deployment` after provisioning.\n"]
    pub fn last_deployment(&self) -> terrars::ListRef<DatazoneEnvironment_LastDeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `profile_identifier` after provisioning.\n"]
    pub fn profile_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_identifier` after provisioning.\n"]
    pub fn project_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_environment` after provisioning.\n"]
    pub fn provider_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_resources` after provisioning.\n"]
    pub fn provisioned_resources(&self) -> terrars::ListRef<DatazoneEnvironment_ProvisionedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatazoneEnvironment_TimeoutsRef {
        DatazoneEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_parameters` after provisioning.\n"]
    pub fn user_parameters(&self) -> terrars::ListRef<DatazoneEnvironment_UserParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_parameters", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DatazoneEnvironmentDynamic {
    user_parameters: std::option::Option<terrars::DynamicBlock<DatazoneEnvironment_UserParameters>>,
}

#[derive(Serialize)]
pub struct DatazoneEnvironment_LastDeployment_FailureReasons {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatazoneEnvironment_LastDeployment_FailureReasons {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazoneEnvironment_LastDeployment_FailureReasons {
    type O = terrars::BlockAssignable<DatazoneEnvironment_LastDeployment_FailureReasons>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazoneEnvironment_LastDeployment_FailureReasons {}

impl BuildDatazoneEnvironment_LastDeployment_FailureReasons {
    pub fn build(self) -> DatazoneEnvironment_LastDeployment_FailureReasons {
        DatazoneEnvironment_LastDeployment_FailureReasons {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazoneEnvironment_LastDeployment_FailureReasonsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazoneEnvironment_LastDeployment_FailureReasonsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazoneEnvironment_LastDeployment_FailureReasonsRef {
        DatazoneEnvironment_LastDeployment_FailureReasonsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazoneEnvironment_LastDeployment_FailureReasonsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazoneEnvironment_LastDeployment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_reasons: std::option::Option<terrars::ListField<DatazoneEnvironment_LastDeployment_FailureReasons>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_deployment_complete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DatazoneEnvironment_LastDeployment {
    #[doc = "Set the field `deployment_id`.\n"]
    pub fn set_deployment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_status`.\n"]
    pub fn set_deployment_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployment_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_type`.\n"]
    pub fn set_deployment_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_reasons`.\n"]
    pub fn set_failure_reasons(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DatazoneEnvironment_LastDeployment_FailureReasons>>,
    ) -> Self {
        self.failure_reasons = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_deployment_complete`.\n"]
    pub fn set_is_deployment_complete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_deployment_complete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.messages = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazoneEnvironment_LastDeployment {
    type O = terrars::BlockAssignable<DatazoneEnvironment_LastDeployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazoneEnvironment_LastDeployment {}

impl BuildDatazoneEnvironment_LastDeployment {
    pub fn build(self) -> DatazoneEnvironment_LastDeployment {
        DatazoneEnvironment_LastDeployment {
            deployment_id: core::default::Default::default(),
            deployment_status: core::default::Default::default(),
            deployment_type: core::default::Default::default(),
            failure_reasons: core::default::Default::default(),
            is_deployment_complete: core::default::Default::default(),
            messages: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazoneEnvironment_LastDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazoneEnvironment_LastDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazoneEnvironment_LastDeploymentRef {
        DatazoneEnvironment_LastDeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazoneEnvironment_LastDeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_id` after provisioning.\n"]
    pub fn deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_status", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_reasons` after provisioning.\n"]
    pub fn failure_reasons(&self) -> terrars::ListRef<DatazoneEnvironment_LastDeployment_FailureReasonsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_reasons", self.base))
    }

    #[doc = "Get a reference to the value of field `is_deployment_complete` after provisioning.\n"]
    pub fn is_deployment_complete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deployment_complete", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazoneEnvironment_ProvisionedResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatazoneEnvironment_ProvisionedResources {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazoneEnvironment_ProvisionedResources {
    type O = terrars::BlockAssignable<DatazoneEnvironment_ProvisionedResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazoneEnvironment_ProvisionedResources {}

impl BuildDatazoneEnvironment_ProvisionedResources {
    pub fn build(self) -> DatazoneEnvironment_ProvisionedResources {
        DatazoneEnvironment_ProvisionedResources {
            name: core::default::Default::default(),
            provider: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazoneEnvironment_ProvisionedResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazoneEnvironment_ProvisionedResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazoneEnvironment_ProvisionedResourcesRef {
        DatazoneEnvironment_ProvisionedResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazoneEnvironment_ProvisionedResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazoneEnvironment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatazoneEnvironment_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazoneEnvironment_Timeouts {
    type O = terrars::BlockAssignable<DatazoneEnvironment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazoneEnvironment_Timeouts {}

impl BuildDatazoneEnvironment_Timeouts {
    pub fn build(self) -> DatazoneEnvironment_Timeouts {
        DatazoneEnvironment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazoneEnvironment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazoneEnvironment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazoneEnvironment_TimeoutsRef {
        DatazoneEnvironment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazoneEnvironment_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazoneEnvironment_UserParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatazoneEnvironment_UserParameters {
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

impl terrars::ToListMappable for DatazoneEnvironment_UserParameters {
    type O = terrars::BlockAssignable<DatazoneEnvironment_UserParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazoneEnvironment_UserParameters {}

impl BuildDatazoneEnvironment_UserParameters {
    pub fn build(self) -> DatazoneEnvironment_UserParameters {
        DatazoneEnvironment_UserParameters {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazoneEnvironment_UserParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazoneEnvironment_UserParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazoneEnvironment_UserParametersRef {
        DatazoneEnvironment_UserParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazoneEnvironment_UserParametersRef {
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

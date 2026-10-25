#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIamPrincipalPolicySimulationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    action_names: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_policies_json: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caller_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions_boundary_policies_json: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    policy_source_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_handling_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_owner_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_policy_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<std::vec::Vec<DataIamPrincipalPolicySimulation_Context>>,
    dynamic: DataIamPrincipalPolicySimulationDynamic,
}

struct DataIamPrincipalPolicySimulation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamPrincipalPolicySimulationData_>,
}

#[derive(Clone)]
pub struct DataIamPrincipalPolicySimulation(std::rc::Rc<DataIamPrincipalPolicySimulation_>);

impl DataIamPrincipalPolicySimulation {
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

    #[doc = "Set the field `additional_policies_json`.\nAdditional principal-based policies to use in the simulation."]
    pub fn set_additional_policies_json(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().additional_policies_json = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `caller_arn`.\nARN of a user to use as the caller of the simulated requests. If not specified, defaults to the principal specified in policy_source_arn, if it is a user ARN."]
    pub fn set_caller_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().caller_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permissions_boundary_policies_json`.\nAdditional permission boundary policies to use in the simulation."]
    pub fn set_permissions_boundary_policies_json(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().permissions_boundary_policies_json = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_arns`.\nARNs of specific resources to use as the targets of the specified actions during simulation. If not specified, the simulator assumes \"*\" which represents general access across all resources."]
    pub fn set_resource_arns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_handling_option`.\nSpecifies the type of simulation to run. Some API operations need a particular resource handling option in order to produce a correct reesult."]
    pub fn set_resource_handling_option(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_handling_option = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_owner_account_id`.\nAn AWS account ID to use as the simulated owner for any resource whose ARN does not include a specific owner account ID. Defaults to the account given as part of caller_arn."]
    pub fn set_resource_owner_account_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_owner_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_policy_json`.\nA resource policy to associate with all of the target resources for simulation purposes. The policy simulator does not automatically retrieve resource-level policies, so if a resource policy is crucial to your test then you must specify here the same policy document associated with your target resource(s)."]
    pub fn set_resource_policy_json(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().resource_policy_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `context`.\n"]
    pub fn set_context(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIamPrincipalPolicySimulation_Context>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `action_names` after provisioning.\nOne or more names of actions, like \"iam:CreateUser\", that should be included in the simulation."]
    pub fn action_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.action_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `additional_policies_json` after provisioning.\nAdditional principal-based policies to use in the simulation."]
    pub fn additional_policies_json(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_policies_json", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `all_allowed` after provisioning.\nA summary of the results attribute which is true if all of the results have decision \"allowed\", and false otherwise."]
    pub fn all_allowed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_allowed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `caller_arn` after provisioning.\nARN of a user to use as the caller of the simulated requests. If not specified, defaults to the principal specified in policy_source_arn, if it is a user ARN."]
    pub fn caller_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caller_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDo not use"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `permissions_boundary_policies_json` after provisioning.\nAdditional permission boundary policies to use in the simulation."]
    pub fn permissions_boundary_policies_json(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.permissions_boundary_policies_json", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `policy_source_arn` after provisioning.\nARN of the principal (e.g. user, role) whose existing configured access policies will be used as the basis for the simulation. If you specify a role ARN here, you can also set caller_arn to simulate a particular user acting with the given role."]
    pub fn policy_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_source_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_arns` after provisioning.\nARNs of specific resources to use as the targets of the specified actions during simulation. If not specified, the simulator assumes \"*\" which represents general access across all resources."]
    pub fn resource_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_arns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_handling_option` after provisioning.\nSpecifies the type of simulation to run. Some API operations need a particular resource handling option in order to produce a correct reesult."]
    pub fn resource_handling_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_handling_option", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_owner_account_id` after provisioning.\nAn AWS account ID to use as the simulated owner for any resource whose ARN does not include a specific owner account ID. Defaults to the account given as part of caller_arn."]
    pub fn resource_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner_account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_policy_json` after provisioning.\nA resource policy to associate with all of the target resources for simulation purposes. The policy simulator does not automatically retrieve resource-level policies, so if a resource policy is crucial to your test then you must specify here the same policy document associated with your target resource(s)."]
    pub fn resource_policy_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_policy_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `results` after provisioning.\n"]
    pub fn results(&self) -> terrars::SetRef<DataIamPrincipalPolicySimulation_ResultsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.results", self.extract_ref()))
    }
}

impl terrars::Referable for DataIamPrincipalPolicySimulation {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamPrincipalPolicySimulation { }

impl terrars::ToListMappable for DataIamPrincipalPolicySimulation {
    type O = terrars::ListRef<DataIamPrincipalPolicySimulationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamPrincipalPolicySimulation_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_iam_principal_policy_simulation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamPrincipalPolicySimulation {
    pub tf_id: std::string::String,
    #[doc = "One or more names of actions, like \"iam:CreateUser\", that should be included in the simulation."]
    pub action_names: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc =
        "ARN of the principal (e.g. user, role) whose existing configured access policies will be used as the basis for the simulation. If you specify a role ARN here, you can also set caller_arn to simulate a particular user acting with the given role."]
    pub policy_source_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataIamPrincipalPolicySimulation {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamPrincipalPolicySimulation {
        let out = DataIamPrincipalPolicySimulation(std::rc::Rc::new(DataIamPrincipalPolicySimulation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamPrincipalPolicySimulationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                action_names: self.action_names,
                additional_policies_json: core::default::Default::default(),
                caller_arn: core::default::Default::default(),
                permissions_boundary_policies_json: core::default::Default::default(),
                policy_source_arn: self.policy_source_arn,
                resource_arns: core::default::Default::default(),
                resource_handling_option: core::default::Default::default(),
                resource_owner_account_id: core::default::Default::default(),
                resource_policy_json: core::default::Default::default(),
                context: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamPrincipalPolicySimulationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPrincipalPolicySimulationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamPrincipalPolicySimulationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `action_names` after provisioning.\nOne or more names of actions, like \"iam:CreateUser\", that should be included in the simulation."]
    pub fn action_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.action_names", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `additional_policies_json` after provisioning.\nAdditional principal-based policies to use in the simulation."]
    pub fn additional_policies_json(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_policies_json", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `all_allowed` after provisioning.\nA summary of the results attribute which is true if all of the results have decision \"allowed\", and false otherwise."]
    pub fn all_allowed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_allowed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `caller_arn` after provisioning.\nARN of a user to use as the caller of the simulated requests. If not specified, defaults to the principal specified in policy_source_arn, if it is a user ARN."]
    pub fn caller_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.caller_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nDo not use"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `permissions_boundary_policies_json` after provisioning.\nAdditional permission boundary policies to use in the simulation."]
    pub fn permissions_boundary_policies_json(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.permissions_boundary_policies_json", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `policy_source_arn` after provisioning.\nARN of the principal (e.g. user, role) whose existing configured access policies will be used as the basis for the simulation. If you specify a role ARN here, you can also set caller_arn to simulate a particular user acting with the given role."]
    pub fn policy_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_source_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_arns` after provisioning.\nARNs of specific resources to use as the targets of the specified actions during simulation. If not specified, the simulator assumes \"*\" which represents general access across all resources."]
    pub fn resource_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_arns", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_handling_option` after provisioning.\nSpecifies the type of simulation to run. Some API operations need a particular resource handling option in order to produce a correct reesult."]
    pub fn resource_handling_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_handling_option", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_owner_account_id` after provisioning.\nAn AWS account ID to use as the simulated owner for any resource whose ARN does not include a specific owner account ID. Defaults to the account given as part of caller_arn."]
    pub fn resource_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner_account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_policy_json` after provisioning.\nA resource policy to associate with all of the target resources for simulation purposes. The policy simulator does not automatically retrieve resource-level policies, so if a resource policy is crucial to your test then you must specify here the same policy document associated with your target resource(s)."]
    pub fn resource_policy_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_policy_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `results` after provisioning.\n"]
    pub fn results(&self) -> terrars::SetRef<DataIamPrincipalPolicySimulation_ResultsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.results", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataIamPrincipalPolicySimulationDynamic {
    context: std::option::Option<terrars::DynamicBlock<DataIamPrincipalPolicySimulation_Context>>,
}

#[derive(Serialize)]
pub struct DataIamPrincipalPolicySimulation_Results_MatchedStatements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_policy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_policy_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamPrincipalPolicySimulation_Results_MatchedStatements {
    #[doc = "Set the field `source_policy_id`.\n"]
    pub fn set_source_policy_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_policy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_policy_type`.\n"]
    pub fn set_source_policy_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_policy_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamPrincipalPolicySimulation_Results_MatchedStatements {
    type O = terrars::BlockAssignable<DataIamPrincipalPolicySimulation_Results_MatchedStatements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPrincipalPolicySimulation_Results_MatchedStatements {}

impl BuildDataIamPrincipalPolicySimulation_Results_MatchedStatements {
    pub fn build(self) -> DataIamPrincipalPolicySimulation_Results_MatchedStatements {
        DataIamPrincipalPolicySimulation_Results_MatchedStatements {
            source_policy_id: core::default::Default::default(),
            source_policy_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamPrincipalPolicySimulation_Results_MatchedStatementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPrincipalPolicySimulation_Results_MatchedStatementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIamPrincipalPolicySimulation_Results_MatchedStatementsRef {
        DataIamPrincipalPolicySimulation_Results_MatchedStatementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPrincipalPolicySimulation_Results_MatchedStatementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_policy_id` after provisioning.\n"]
    pub fn source_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `source_policy_type` after provisioning.\n"]
    pub fn source_policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_policy_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamPrincipalPolicySimulation_Results {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decision_details: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matched_statements: std
    ::option
    ::Option<terrars::SetField<DataIamPrincipalPolicySimulation_Results_MatchedStatements>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    missing_context_keys: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamPrincipalPolicySimulation_Results {
    #[doc = "Set the field `action_name`.\n"]
    pub fn set_action_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed`.\n"]
    pub fn set_allowed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allowed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `decision`.\n"]
    pub fn set_decision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.decision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `decision_details`.\n"]
    pub fn set_decision_details(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.decision_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matched_statements`.\n"]
    pub fn set_matched_statements(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataIamPrincipalPolicySimulation_Results_MatchedStatements>>,
    ) -> Self {
        self.matched_statements = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `missing_context_keys`.\n"]
    pub fn set_missing_context_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.missing_context_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamPrincipalPolicySimulation_Results {
    type O = terrars::BlockAssignable<DataIamPrincipalPolicySimulation_Results>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPrincipalPolicySimulation_Results {}

impl BuildDataIamPrincipalPolicySimulation_Results {
    pub fn build(self) -> DataIamPrincipalPolicySimulation_Results {
        DataIamPrincipalPolicySimulation_Results {
            action_name: core::default::Default::default(),
            allowed: core::default::Default::default(),
            decision: core::default::Default::default(),
            decision_details: core::default::Default::default(),
            matched_statements: core::default::Default::default(),
            missing_context_keys: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamPrincipalPolicySimulation_ResultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPrincipalPolicySimulation_ResultsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamPrincipalPolicySimulation_ResultsRef {
        DataIamPrincipalPolicySimulation_ResultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPrincipalPolicySimulation_ResultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_name` after provisioning.\n"]
    pub fn action_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_name", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed` after provisioning.\n"]
    pub fn allowed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed", self.base))
    }

    #[doc = "Get a reference to the value of field `decision` after provisioning.\n"]
    pub fn decision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.decision", self.base))
    }

    #[doc = "Get a reference to the value of field `decision_details` after provisioning.\n"]
    pub fn decision_details(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.decision_details", self.base))
    }

    #[doc = "Get a reference to the value of field `matched_statements` after provisioning.\n"]
    pub fn matched_statements(&self) -> terrars::SetRef<DataIamPrincipalPolicySimulation_Results_MatchedStatementsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.matched_statements", self.base))
    }

    #[doc = "Get a reference to the value of field `missing_context_keys` after provisioning.\n"]
    pub fn missing_context_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.missing_context_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIamPrincipalPolicySimulation_Context {
    key: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataIamPrincipalPolicySimulation_Context { }

impl terrars::ToListMappable for DataIamPrincipalPolicySimulation_Context {
    type O = terrars::BlockAssignable<DataIamPrincipalPolicySimulation_Context>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamPrincipalPolicySimulation_Context {
    #[doc = "The key name of the context entry, such as \"aws:CurrentTime\"."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "The type that the simulator should use to interpret the strings given in argument \"values\"."]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc =
        "One or more values to assign to the context key, given as a string in a syntax appropriate for the selected value type."]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataIamPrincipalPolicySimulation_Context {
    pub fn build(self) -> DataIamPrincipalPolicySimulation_Context {
        DataIamPrincipalPolicySimulation_Context {
            key: self.key,
            type_: self.type_,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataIamPrincipalPolicySimulation_ContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamPrincipalPolicySimulation_ContextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamPrincipalPolicySimulation_ContextRef {
        DataIamPrincipalPolicySimulation_ContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamPrincipalPolicySimulation_ContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThe key name of the context entry, such as \"aws:CurrentTime\"."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type that the simulator should use to interpret the strings given in argument \"values\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nOne or more values to assign to the context key, given as a string in a syntax appropriate for the selected value type."]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

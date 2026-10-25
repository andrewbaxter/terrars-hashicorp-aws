#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudformationStackSetData_ {
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
    administration_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    call_as: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_deployment: std::option::Option<std::vec::Vec<CloudformationStackSet_AutoDeployment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_execution: std::option::Option<std::vec::Vec<CloudformationStackSet_ManagedExecution>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_preferences: std::option::Option<std::vec::Vec<CloudformationStackSet_OperationPreferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudformationStackSet_Timeouts>,
    dynamic: CloudformationStackSetDynamic,
}

struct CloudformationStackSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudformationStackSetData_>,
}

#[derive(Clone)]
pub struct CloudformationStackSet(std::rc::Rc<CloudformationStackSet_>);

impl CloudformationStackSet {
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

    #[doc = "Set the field `administration_role_arn`.\n"]
    pub fn set_administration_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().administration_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `call_as`.\n"]
    pub fn set_call_as(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().call_as = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_name`.\n"]
    pub fn set_execution_role_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_role_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permission_model`.\n"]
    pub fn set_permission_model(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().permission_model = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `template_body`.\n"]
    pub fn set_template_body(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().template_body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `template_url`.\n"]
    pub fn set_template_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().template_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_deployment`.\n"]
    pub fn set_auto_deployment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudformationStackSet_AutoDeployment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_deployment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_deployment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_execution`.\n"]
    pub fn set_managed_execution(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudformationStackSet_ManagedExecution>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_execution = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_execution = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operation_preferences`.\n"]
    pub fn set_operation_preferences(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudformationStackSet_OperationPreferences>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().operation_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.operation_preferences = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudformationStackSet_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `administration_role_arn` after provisioning.\n"]
    pub fn administration_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.administration_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.call_as", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capabilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_name` after provisioning.\n"]
    pub fn execution_role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_model` after provisioning.\n"]
    pub fn permission_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_id` after provisioning.\n"]
    pub fn stack_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_url` after provisioning.\n"]
    pub fn template_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_deployment` after provisioning.\n"]
    pub fn auto_deployment(&self) -> terrars::ListRef<CloudformationStackSet_AutoDeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_execution` after provisioning.\n"]
    pub fn managed_execution(&self) -> terrars::ListRef<CloudformationStackSet_ManagedExecutionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_execution", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(&self) -> terrars::ListRef<CloudformationStackSet_OperationPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation_preferences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackSet_TimeoutsRef {
        CloudformationStackSet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudformationStackSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudformationStackSet { }

impl terrars::ToListMappable for CloudformationStackSet {
    type O = terrars::ListRef<CloudformationStackSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudformationStackSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudformation_stack_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudformationStackSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudformationStackSet {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudformationStackSet {
        let out = CloudformationStackSet(std::rc::Rc::new(CloudformationStackSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudformationStackSetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                administration_role_arn: core::default::Default::default(),
                call_as: core::default::Default::default(),
                capabilities: core::default::Default::default(),
                description: core::default::Default::default(),
                execution_role_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parameters: core::default::Default::default(),
                permission_model: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                template_body: core::default::Default::default(),
                template_url: core::default::Default::default(),
                auto_deployment: core::default::Default::default(),
                managed_execution: core::default::Default::default(),
                operation_preferences: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudformationStackSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudformationStackSetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `administration_role_arn` after provisioning.\n"]
    pub fn administration_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.administration_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `call_as` after provisioning.\n"]
    pub fn call_as(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.call_as", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capabilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_name` after provisioning.\n"]
    pub fn execution_role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_model` after provisioning.\n"]
    pub fn permission_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_id` after provisioning.\n"]
    pub fn stack_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_url` after provisioning.\n"]
    pub fn template_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_deployment` after provisioning.\n"]
    pub fn auto_deployment(&self) -> terrars::ListRef<CloudformationStackSet_AutoDeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_execution` after provisioning.\n"]
    pub fn managed_execution(&self) -> terrars::ListRef<CloudformationStackSet_ManagedExecutionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_execution", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_preferences` after provisioning.\n"]
    pub fn operation_preferences(&self) -> terrars::ListRef<CloudformationStackSet_OperationPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation_preferences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudformationStackSet_TimeoutsRef {
        CloudformationStackSet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudformationStackSetDynamic {
    auto_deployment: std::option::Option<terrars::DynamicBlock<CloudformationStackSet_AutoDeployment>>,
    managed_execution: std::option::Option<terrars::DynamicBlock<CloudformationStackSet_ManagedExecution>>,
    operation_preferences: std::option::Option<terrars::DynamicBlock<CloudformationStackSet_OperationPreferences>>,
}

#[derive(Serialize)]
pub struct CloudformationStackSet_AutoDeployment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    depends_on_stack_sets: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_stacks_on_account_removal: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudformationStackSet_AutoDeployment {
    #[doc = "Set the field `depends_on_stack_sets`.\n"]
    pub fn set_depends_on_stack_sets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.depends_on_stack_sets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retain_stacks_on_account_removal`.\n"]
    pub fn set_retain_stacks_on_account_removal(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.retain_stacks_on_account_removal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudformationStackSet_AutoDeployment {
    type O = terrars::BlockAssignable<CloudformationStackSet_AutoDeployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSet_AutoDeployment {}

impl BuildCloudformationStackSet_AutoDeployment {
    pub fn build(self) -> CloudformationStackSet_AutoDeployment {
        CloudformationStackSet_AutoDeployment {
            depends_on_stack_sets: core::default::Default::default(),
            enabled: core::default::Default::default(),
            retain_stacks_on_account_removal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudformationStackSet_AutoDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSet_AutoDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudformationStackSet_AutoDeploymentRef {
        CloudformationStackSet_AutoDeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSet_AutoDeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `depends_on_stack_sets` after provisioning.\n"]
    pub fn depends_on_stack_sets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.depends_on_stack_sets", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_stacks_on_account_removal` after provisioning.\n"]
    pub fn retain_stacks_on_account_removal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_stacks_on_account_removal", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackSet_ManagedExecution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active: std::option::Option<terrars::PrimField<bool>>,
}

impl CloudformationStackSet_ManagedExecution {
    #[doc = "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.active = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudformationStackSet_ManagedExecution {
    type O = terrars::BlockAssignable<CloudformationStackSet_ManagedExecution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSet_ManagedExecution {}

impl BuildCloudformationStackSet_ManagedExecution {
    pub fn build(self) -> CloudformationStackSet_ManagedExecution {
        CloudformationStackSet_ManagedExecution { active: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudformationStackSet_ManagedExecutionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSet_ManagedExecutionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudformationStackSet_ManagedExecutionRef {
        CloudformationStackSet_ManagedExecutionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSet_ManagedExecutionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackSet_OperationPreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_tolerance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_tolerance_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_concurrency_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_order: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CloudformationStackSet_OperationPreferences {
    #[doc = "Set the field `failure_tolerance_count`.\n"]
    pub fn set_failure_tolerance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_tolerance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_tolerance_percentage`.\n"]
    pub fn set_failure_tolerance_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_tolerance_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_count`.\n"]
    pub fn set_max_concurrent_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_percentage`.\n"]
    pub fn set_max_concurrent_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_concurrency_type`.\n"]
    pub fn set_region_concurrency_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.region_concurrency_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_order`.\n"]
    pub fn set_region_order(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.region_order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudformationStackSet_OperationPreferences {
    type O = terrars::BlockAssignable<CloudformationStackSet_OperationPreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSet_OperationPreferences {}

impl BuildCloudformationStackSet_OperationPreferences {
    pub fn build(self) -> CloudformationStackSet_OperationPreferences {
        CloudformationStackSet_OperationPreferences {
            failure_tolerance_count: core::default::Default::default(),
            failure_tolerance_percentage: core::default::Default::default(),
            max_concurrent_count: core::default::Default::default(),
            max_concurrent_percentage: core::default::Default::default(),
            region_concurrency_type: core::default::Default::default(),
            region_order: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudformationStackSet_OperationPreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSet_OperationPreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudformationStackSet_OperationPreferencesRef {
        CloudformationStackSet_OperationPreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSet_OperationPreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_count` after provisioning.\n"]
    pub fn failure_tolerance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_tolerance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_percentage` after provisioning.\n"]
    pub fn failure_tolerance_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_tolerance_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `max_concurrent_count` after provisioning.\n"]
    pub fn max_concurrent_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_concurrent_percentage` after provisioning.\n"]
    pub fn max_concurrent_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `region_concurrency_type` after provisioning.\n"]
    pub fn region_concurrency_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_concurrency_type", self.base))
    }

    #[doc = "Get a reference to the value of field `region_order` after provisioning.\n"]
    pub fn region_order(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region_order", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudformationStackSet_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudformationStackSet_Timeouts {
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudformationStackSet_Timeouts {
    type O = terrars::BlockAssignable<CloudformationStackSet_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudformationStackSet_Timeouts {}

impl BuildCloudformationStackSet_Timeouts {
    pub fn build(self) -> CloudformationStackSet_Timeouts {
        CloudformationStackSet_Timeouts { update: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudformationStackSet_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudformationStackSet_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudformationStackSet_TimeoutsRef {
        CloudformationStackSet_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudformationStackSet_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

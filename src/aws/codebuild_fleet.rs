#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodebuildFleetData_ {
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
    base_capacity: terrars::PrimField<f64>,
    compute_type: terrars::PrimField<std::string::String>,
    environment_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_service_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overflow_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_configuration: std::option::Option<std::vec::Vec<CodebuildFleet_ComputeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_configuration: std::option::Option<std::vec::Vec<CodebuildFleet_ScalingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<CodebuildFleet_VpcConfig>>,
    dynamic: CodebuildFleetDynamic,
}

struct CodebuildFleet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodebuildFleetData_>,
}

#[derive(Clone)]
pub struct CodebuildFleet(std::rc::Rc<CodebuildFleet_>);

impl CodebuildFleet {
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

    #[doc = "Set the field `fleet_service_role`.\n"]
    pub fn set_fleet_service_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fleet_service_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_id`.\n"]
    pub fn set_image_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().image_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overflow_behavior`.\n"]
    pub fn set_overflow_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().overflow_behavior = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `compute_configuration`.\n"]
    pub fn set_compute_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildFleet_ComputeConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_configuration`.\n"]
    pub fn set_scaling_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildFleet_ScalingConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(self, v: impl std::convert::Into<terrars::BlockAssignable<CodebuildFleet_VpcConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_type` after provisioning.\n"]
    pub fn environment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_service_role` after provisioning.\n"]
    pub fn fleet_service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `overflow_behavior` after provisioning.\n"]
    pub fn overflow_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overflow_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::SetRef<CodebuildFleet_StatusRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_configuration` after provisioning.\n"]
    pub fn compute_configuration(&self) -> terrars::ListRef<CodebuildFleet_ComputeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> terrars::ListRef<CodebuildFleet_ScalingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<CodebuildFleet_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for CodebuildFleet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodebuildFleet { }

impl terrars::ToListMappable for CodebuildFleet {
    type O = terrars::ListRef<CodebuildFleetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodebuildFleet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codebuild_fleet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodebuildFleet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub base_capacity: terrars::PrimField<f64>,
    #[doc = ""]
    pub compute_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub environment_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildFleet {
    pub fn build(self, stack: &mut terrars::Stack) -> CodebuildFleet {
        let out = CodebuildFleet(std::rc::Rc::new(CodebuildFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodebuildFleetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                base_capacity: self.base_capacity,
                compute_type: self.compute_type,
                environment_type: self.environment_type,
                fleet_service_role: core::default::Default::default(),
                image_id: core::default::Default::default(),
                name: self.name,
                overflow_behavior: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                compute_configuration: core::default::Default::default(),
                scaling_configuration: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodebuildFleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildFleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodebuildFleetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_type` after provisioning.\n"]
    pub fn environment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_service_role` after provisioning.\n"]
    pub fn fleet_service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `overflow_behavior` after provisioning.\n"]
    pub fn overflow_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overflow_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::SetRef<CodebuildFleet_StatusRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_configuration` after provisioning.\n"]
    pub fn compute_configuration(&self) -> terrars::ListRef<CodebuildFleet_ComputeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> terrars::ListRef<CodebuildFleet_ScalingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<CodebuildFleet_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CodebuildFleetDynamic {
    compute_configuration: std::option::Option<terrars::DynamicBlock<CodebuildFleet_ComputeConfiguration>>,
    scaling_configuration: std::option::Option<terrars::DynamicBlock<CodebuildFleet_ScalingConfiguration>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<CodebuildFleet_VpcConfig>>,
}

#[derive(Serialize)]
pub struct CodebuildFleet_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildFleet_Status {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildFleet_Status {
    type O = terrars::BlockAssignable<CodebuildFleet_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleet_Status {}

impl BuildCodebuildFleet_Status {
    pub fn build(self) -> CodebuildFleet_Status {
        CodebuildFleet_Status {
            context: core::default::Default::default(),
            message: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildFleet_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildFleet_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildFleet_StatusRef {
        CodebuildFleet_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleet_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildFleet_ComputeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu: std::option::Option<terrars::PrimField<f64>>,
}

impl CodebuildFleet_ComputeConfiguration {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vcpu`.\n"]
    pub fn set_vcpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vcpu = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildFleet_ComputeConfiguration {
    type O = terrars::BlockAssignable<CodebuildFleet_ComputeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleet_ComputeConfiguration {}

impl BuildCodebuildFleet_ComputeConfiguration {
    pub fn build(self) -> CodebuildFleet_ComputeConfiguration {
        CodebuildFleet_ComputeConfiguration {
            disk: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            memory: core::default::Default::default(),
            vcpu: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildFleet_ComputeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildFleet_ComputeConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildFleet_ComputeConfigurationRef {
        CodebuildFleet_ComputeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleet_ComputeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu` after provisioning.\n"]
    pub fn vcpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcpu", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_value: std::option::Option<terrars::PrimField<f64>>,
}

impl CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
    #[doc = "Set the field `metric_type`.\n"]
    pub fn set_metric_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_value`.\n"]
    pub fn set_target_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
    type O = terrars::BlockAssignable<CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {}

impl BuildCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
    pub fn build(self) -> CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
        CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
            metric_type: core::default::Default::default(),
            target_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
        CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_type` after provisioning.\n"]
    pub fn metric_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildFleet_ScalingConfigurationDynamic {
    target_tracking_scaling_configs: std
    ::option
    ::Option<terrars::DynamicBlock<CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs>>,
}

#[derive(Serialize)]
pub struct CodebuildFleet_ScalingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tracking_scaling_configs: std
    ::option
    ::Option<std::vec::Vec<CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs>>,
    dynamic: CodebuildFleet_ScalingConfigurationDynamic,
}

impl CodebuildFleet_ScalingConfiguration {
    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_type`.\n"]
    pub fn set_scaling_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scaling_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_tracking_scaling_configs`.\n"]
    pub fn set_target_tracking_scaling_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_tracking_scaling_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_tracking_scaling_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodebuildFleet_ScalingConfiguration {
    type O = terrars::BlockAssignable<CodebuildFleet_ScalingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleet_ScalingConfiguration {}

impl BuildCodebuildFleet_ScalingConfiguration {
    pub fn build(self) -> CodebuildFleet_ScalingConfiguration {
        CodebuildFleet_ScalingConfiguration {
            max_capacity: core::default::Default::default(),
            scaling_type: core::default::Default::default(),
            target_tracking_scaling_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildFleet_ScalingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildFleet_ScalingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildFleet_ScalingConfigurationRef {
        CodebuildFleet_ScalingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleet_ScalingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_type` after provisioning.\n"]
    pub fn scaling_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_tracking_scaling_configs` after provisioning.\n"]
    pub fn target_tracking_scaling_configs(
        &self,
    ) -> terrars::ListRef<CodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_tracking_scaling_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct CodebuildFleet_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    vpc_id: terrars::PrimField<std::string::String>,
}

impl CodebuildFleet_VpcConfig { }

impl terrars::ToListMappable for CodebuildFleet_VpcConfig {
    type O = terrars::BlockAssignable<CodebuildFleet_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildFleet_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildFleet_VpcConfig {
    pub fn build(self) -> CodebuildFleet_VpcConfig {
        CodebuildFleet_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnets: self.subnets,
            vpc_id: self.vpc_id,
        }
    }
}

#[allow(dead_code)]
pub struct CodebuildFleet_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildFleet_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildFleet_VpcConfigRef {
        CodebuildFleet_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildFleet_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

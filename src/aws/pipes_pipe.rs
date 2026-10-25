#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct PipesPipeData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enrichment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    target: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enrichment_parameters: std::option::Option<std::vec::Vec<PipesPipe_EnrichmentParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_configuration: std::option::Option<std::vec::Vec<PipesPipe_LogConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_parameters: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PipesPipe_Timeouts>,
    dynamic: PipesPipeDynamic,
}

struct PipesPipe_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PipesPipeData_>,
}

#[derive(Clone)]
pub struct PipesPipe(std::rc::Rc<PipesPipe_>);

impl PipesPipe {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_state`.\n"]
    pub fn set_desired_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().desired_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enrichment`.\n"]
    pub fn set_enrichment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().enrichment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_identifier`.\n"]
    pub fn set_kms_key_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `enrichment_parameters`.\n"]
    pub fn set_enrichment_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_EnrichmentParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enrichment_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enrichment_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_configuration`.\n"]
    pub fn set_log_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_LogConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_parameters`.\n"]
    pub fn set_source_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_SourceParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_parameters`.\n"]
    pub fn set_target_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PipesPipe_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\n"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enrichment` after provisioning.\n"]
    pub fn enrichment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enrichment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enrichment_parameters` after provisioning.\n"]
    pub fn enrichment_parameters(&self) -> terrars::ListRef<PipesPipe_EnrichmentParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enrichment_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<PipesPipe_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_parameters` after provisioning.\n"]
    pub fn source_parameters(&self) -> terrars::ListRef<PipesPipe_SourceParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_parameters` after provisioning.\n"]
    pub fn target_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PipesPipe_TimeoutsRef {
        PipesPipe_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PipesPipe {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PipesPipe { }

impl terrars::ToListMappable for PipesPipe {
    type O = terrars::ListRef<PipesPipeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PipesPipe_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_pipes_pipe".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPipesPipe {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe {
    pub fn build(self, stack: &mut terrars::Stack) -> PipesPipe {
        let out = PipesPipe(std::rc::Rc::new(PipesPipe_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PipesPipeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                enrichment: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_identifier: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                source: self.source,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target: self.target,
                enrichment_parameters: core::default::Default::default(),
                log_configuration: core::default::Default::default(),
                source_parameters: core::default::Default::default(),
                target_parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PipesPipeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PipesPipeRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\n"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enrichment` after provisioning.\n"]
    pub fn enrichment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enrichment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enrichment_parameters` after provisioning.\n"]
    pub fn enrichment_parameters(&self) -> terrars::ListRef<PipesPipe_EnrichmentParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enrichment_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::ListRef<PipesPipe_LogConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_parameters` after provisioning.\n"]
    pub fn source_parameters(&self) -> terrars::ListRef<PipesPipe_SourceParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_parameters` after provisioning.\n"]
    pub fn target_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PipesPipe_TimeoutsRef {
        PipesPipe_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PipesPipeDynamic {
    enrichment_parameters: std::option::Option<terrars::DynamicBlock<PipesPipe_EnrichmentParameters>>,
    log_configuration: std::option::Option<terrars::DynamicBlock<PipesPipe_LogConfiguration>>,
    source_parameters: std::option::Option<terrars::DynamicBlock<PipesPipe_SourceParameters>>,
    target_parameters: std::option::Option<terrars::DynamicBlock<PipesPipe_TargetParameters>>,
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_HttpParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_parameter_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl PipesPipe_TargetParameters_HttpParameters {
    #[doc = "Set the field `header_parameters`.\n"]
    pub fn set_header_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.header_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path_parameter_values`.\n"]
    pub fn set_path_parameter_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.path_parameter_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string_parameters`.\n"]
    pub fn set_query_string_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_string_parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_HttpParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_HttpParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_HttpParameters {}

impl BuildPipesPipe_TargetParameters_HttpParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_HttpParameters {
        PipesPipe_TargetParameters_HttpParameters {
            header_parameters: core::default::Default::default(),
            path_parameter_values: core::default::Default::default(),
            query_string_parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_HttpParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_HttpParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipesPipe_TargetParameters_HttpParametersRef {
        PipesPipe_TargetParameters_HttpParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_HttpParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_parameters` after provisioning.\n"]
    pub fn header_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.header_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `path_parameter_values` after provisioning.\n"]
    pub fn path_parameter_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_parameter_values", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string_parameters` after provisioning.\n"]
    pub fn query_string_parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.query_string_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_EnrichmentParametersDynamic {
    http_parameters: std::option::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_HttpParameters>>,
}

#[derive(Serialize)]
pub struct PipesPipe_EnrichmentParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_HttpParameters>>,
    dynamic: PipesPipe_EnrichmentParametersDynamic,
}

impl PipesPipe_EnrichmentParameters {
    #[doc = "Set the field `input_template`.\n"]
    pub fn set_input_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_parameters`.\n"]
    pub fn set_http_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_HttpParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_EnrichmentParameters {
    type O = terrars::BlockAssignable<PipesPipe_EnrichmentParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_EnrichmentParameters {}

impl BuildPipesPipe_EnrichmentParameters {
    pub fn build(self) -> PipesPipe_EnrichmentParameters {
        PipesPipe_EnrichmentParameters {
            input_template: core::default::Default::default(),
            http_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_EnrichmentParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_EnrichmentParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipesPipe_EnrichmentParametersRef {
        PipesPipe_EnrichmentParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_EnrichmentParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_template` after provisioning.\n"]
    pub fn input_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_template", self.base))
    }

    #[doc = "Get a reference to the value of field `http_parameters` after provisioning.\n"]
    pub fn http_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParameters_HttpParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_LogConfiguration_CloudwatchLogsLogDestination {
    log_group_arn: terrars::PrimField<std::string::String>,
}

impl PipesPipe_LogConfiguration_CloudwatchLogsLogDestination { }

impl terrars::ToListMappable for PipesPipe_LogConfiguration_CloudwatchLogsLogDestination {
    type O = terrars::BlockAssignable<PipesPipe_LogConfiguration_CloudwatchLogsLogDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_LogConfiguration_CloudwatchLogsLogDestination {
    #[doc = ""]
    pub log_group_arn: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_LogConfiguration_CloudwatchLogsLogDestination {
    pub fn build(self) -> PipesPipe_LogConfiguration_CloudwatchLogsLogDestination {
        PipesPipe_LogConfiguration_CloudwatchLogsLogDestination { log_group_arn: self.log_group_arn }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_LogConfiguration_CloudwatchLogsLogDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_LogConfiguration_CloudwatchLogsLogDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_LogConfiguration_CloudwatchLogsLogDestinationRef {
        PipesPipe_LogConfiguration_CloudwatchLogsLogDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_LogConfiguration_CloudwatchLogsLogDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_arn` after provisioning.\n"]
    pub fn log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_LogConfiguration_FirehoseLogDestination {
    delivery_stream_arn: terrars::PrimField<std::string::String>,
}

impl PipesPipe_LogConfiguration_FirehoseLogDestination { }

impl terrars::ToListMappable for PipesPipe_LogConfiguration_FirehoseLogDestination {
    type O = terrars::BlockAssignable<PipesPipe_LogConfiguration_FirehoseLogDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_LogConfiguration_FirehoseLogDestination {
    #[doc = ""]
    pub delivery_stream_arn: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_LogConfiguration_FirehoseLogDestination {
    pub fn build(self) -> PipesPipe_LogConfiguration_FirehoseLogDestination {
        PipesPipe_LogConfiguration_FirehoseLogDestination { delivery_stream_arn: self.delivery_stream_arn }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_LogConfiguration_FirehoseLogDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_LogConfiguration_FirehoseLogDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_LogConfiguration_FirehoseLogDestinationRef {
        PipesPipe_LogConfiguration_FirehoseLogDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_LogConfiguration_FirehoseLogDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream_arn` after provisioning.\n"]
    pub fn delivery_stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_LogConfiguration_S3LogDestination {
    bucket_name: terrars::PrimField<std::string::String>,
    bucket_owner: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_LogConfiguration_S3LogDestination {
    #[doc = "Set the field `output_format`.\n"]
    pub fn set_output_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_LogConfiguration_S3LogDestination {
    type O = terrars::BlockAssignable<PipesPipe_LogConfiguration_S3LogDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_LogConfiguration_S3LogDestination {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bucket_owner: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_LogConfiguration_S3LogDestination {
    pub fn build(self) -> PipesPipe_LogConfiguration_S3LogDestination {
        PipesPipe_LogConfiguration_S3LogDestination {
            bucket_name: self.bucket_name,
            bucket_owner: self.bucket_owner,
            output_format: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_LogConfiguration_S3LogDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_LogConfiguration_S3LogDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_LogConfiguration_S3LogDestinationRef {
        PipesPipe_LogConfiguration_S3LogDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_LogConfiguration_S3LogDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_owner` after provisioning.\n"]
    pub fn bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_owner", self.base))
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_LogConfigurationDynamic {
    cloudwatch_logs_log_destination: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_LogConfiguration_CloudwatchLogsLogDestination>>,
    firehose_log_destination: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_LogConfiguration_FirehoseLogDestination>>,
    s3_log_destination: std::option::Option<terrars::DynamicBlock<PipesPipe_LogConfiguration_S3LogDestination>>,
}

#[derive(Serialize)]
pub struct PipesPipe_LogConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_execution_data: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    level: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs_log_destination: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_LogConfiguration_CloudwatchLogsLogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose_log_destination: std::option::Option<std::vec::Vec<PipesPipe_LogConfiguration_FirehoseLogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_log_destination: std::option::Option<std::vec::Vec<PipesPipe_LogConfiguration_S3LogDestination>>,
    dynamic: PipesPipe_LogConfigurationDynamic,
}

impl PipesPipe_LogConfiguration {
    #[doc = "Set the field `include_execution_data`.\n"]
    pub fn set_include_execution_data(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_execution_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_logs_log_destination`.\n"]
    pub fn set_cloudwatch_logs_log_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_LogConfiguration_CloudwatchLogsLogDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs_log_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs_log_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firehose_log_destination`.\n"]
    pub fn set_firehose_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_LogConfiguration_FirehoseLogDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.firehose_log_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose_log_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_log_destination`.\n"]
    pub fn set_s3_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_LogConfiguration_S3LogDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_log_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_log_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_LogConfiguration {
    type O = terrars::BlockAssignable<PipesPipe_LogConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_LogConfiguration {
    #[doc = ""]
    pub level: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_LogConfiguration {
    pub fn build(self) -> PipesPipe_LogConfiguration {
        PipesPipe_LogConfiguration {
            include_execution_data: core::default::Default::default(),
            level: self.level,
            cloudwatch_logs_log_destination: core::default::Default::default(),
            firehose_log_destination: core::default::Default::default(),
            s3_log_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_LogConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_LogConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipesPipe_LogConfigurationRef {
        PipesPipe_LogConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_LogConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_execution_data` after provisioning.\n"]
    pub fn include_execution_data(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.include_execution_data", self.base))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs_log_destination` after provisioning.\n"]
    pub fn cloudwatch_logs_log_destination(
        &self,
    ) -> terrars::ListRef<PipesPipe_LogConfiguration_CloudwatchLogsLogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs_log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose_log_destination` after provisioning.\n"]
    pub fn firehose_log_destination(&self) -> terrars::ListRef<PipesPipe_LogConfiguration_FirehoseLogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose_log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_log_destination` after provisioning.\n"]
    pub fn s3_log_destination(&self) -> terrars::ListRef<PipesPipe_LogConfiguration_S3LogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_log_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials {
    basic_auth: terrars::PrimField<std::string::String>,
}

impl PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials { }

impl terrars::ToListMappable for PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials {
    #[doc = ""]
    pub basic_auth: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials {
    pub fn build(self) -> PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials {
        PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials { basic_auth: self.basic_auth }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_ActivemqBrokerParameters_CredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_ActivemqBrokerParameters_CredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_ActivemqBrokerParameters_CredentialsRef {
        PipesPipe_SourceParameters_ActivemqBrokerParameters_CredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_ActivemqBrokerParameters_CredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_SourceParameters_ActivemqBrokerParametersDynamic {
    credentials: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials>>,
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_ActivemqBrokerParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batching_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    queue_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials>>,
    dynamic: PipesPipe_SourceParameters_ActivemqBrokerParametersDynamic,
}

impl PipesPipe_SourceParameters_ActivemqBrokerParameters {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.maximum_batching_window_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.credentials = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_ActivemqBrokerParameters {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_ActivemqBrokerParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_ActivemqBrokerParameters {
    #[doc = ""]
    pub queue_name: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_SourceParameters_ActivemqBrokerParameters {
    pub fn build(self) -> PipesPipe_SourceParameters_ActivemqBrokerParameters {
        PipesPipe_SourceParameters_ActivemqBrokerParameters {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            queue_name: self.queue_name,
            credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_ActivemqBrokerParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_ActivemqBrokerParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_ActivemqBrokerParametersRef {
        PipesPipe_SourceParameters_ActivemqBrokerParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_ActivemqBrokerParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_batching_window_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> terrars::ListRef<PipesPipe_SourceParameters_ActivemqBrokerParameters_CredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig {}

impl BuildPipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig {
    pub fn build(self) -> PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig {
        PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig {
            arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfigRef {
        PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_SourceParameters_DynamodbStreamParametersDynamic {
    dead_letter_config: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig>>,
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_DynamodbStreamParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batching_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_record_age_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_partial_batch_item_failure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelization_factor: std::option::Option<terrars::PrimField<f64>>,
    starting_position: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_config: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig>>,
    dynamic: PipesPipe_SourceParameters_DynamodbStreamParametersDynamic,
}

impl PipesPipe_SourceParameters_DynamodbStreamParameters {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.maximum_batching_window_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_record_age_in_seconds`.\n"]
    pub fn set_maximum_record_age_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_record_age_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_retry_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_partial_batch_item_failure`.\n"]
    pub fn set_on_partial_batch_item_failure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_partial_batch_item_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallelization_factor`.\n"]
    pub fn set_parallelization_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallelization_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dead_letter_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dead_letter_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_DynamodbStreamParameters {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_DynamodbStreamParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_DynamodbStreamParameters {
    #[doc = ""]
    pub starting_position: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_SourceParameters_DynamodbStreamParameters {
    pub fn build(self) -> PipesPipe_SourceParameters_DynamodbStreamParameters {
        PipesPipe_SourceParameters_DynamodbStreamParameters {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            maximum_record_age_in_seconds: core::default::Default::default(),
            maximum_retry_attempts: core::default::Default::default(),
            on_partial_batch_item_failure: core::default::Default::default(),
            parallelization_factor: core::default::Default::default(),
            starting_position: self.starting_position,
            dead_letter_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_DynamodbStreamParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_DynamodbStreamParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_DynamodbStreamParametersRef {
        PipesPipe_SourceParameters_DynamodbStreamParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_DynamodbStreamParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_batching_window_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_record_age_in_seconds` after provisioning.\n"]
    pub fn maximum_record_age_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_record_age_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `on_partial_batch_item_failure` after provisioning.\n"]
    pub fn on_partial_batch_item_failure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_partial_batch_item_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelization_factor` after provisioning.\n"]
    pub fn parallelization_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelization_factor", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.base))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(
        &self,
    ) -> terrars::ListRef<PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_FilterCriteria_Filter {
    pattern: terrars::PrimField<std::string::String>,
}

impl PipesPipe_SourceParameters_FilterCriteria_Filter { }

impl terrars::ToListMappable for PipesPipe_SourceParameters_FilterCriteria_Filter {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_FilterCriteria_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_FilterCriteria_Filter {
    #[doc = ""]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_SourceParameters_FilterCriteria_Filter {
    pub fn build(self) -> PipesPipe_SourceParameters_FilterCriteria_Filter {
        PipesPipe_SourceParameters_FilterCriteria_Filter { pattern: self.pattern }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_FilterCriteria_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_FilterCriteria_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_FilterCriteria_FilterRef {
        PipesPipe_SourceParameters_FilterCriteria_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_FilterCriteria_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_SourceParameters_FilterCriteriaDynamic {
    filter: std::option::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_FilterCriteria_Filter>>,
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_FilterCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_FilterCriteria_Filter>>,
    dynamic: PipesPipe_SourceParameters_FilterCriteriaDynamic,
}

impl PipesPipe_SourceParameters_FilterCriteria {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_FilterCriteria_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_FilterCriteria {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_FilterCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_FilterCriteria {}

impl BuildPipesPipe_SourceParameters_FilterCriteria {
    pub fn build(self) -> PipesPipe_SourceParameters_FilterCriteria {
        PipesPipe_SourceParameters_FilterCriteria {
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_FilterCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_FilterCriteriaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipesPipe_SourceParameters_FilterCriteriaRef {
        PipesPipe_SourceParameters_FilterCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_FilterCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<PipesPipe_SourceParameters_FilterCriteria_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_SourceParameters_KinesisStreamParametersDynamic {
    dead_letter_config: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig>>,
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_KinesisStreamParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batching_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_record_age_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_partial_batch_item_failure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallelization_factor: std::option::Option<terrars::PrimField<f64>>,
    starting_position: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_config: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig>>,
    dynamic: PipesPipe_SourceParameters_KinesisStreamParametersDynamic,
}

impl PipesPipe_SourceParameters_KinesisStreamParameters {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.maximum_batching_window_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_record_age_in_seconds`.\n"]
    pub fn set_maximum_record_age_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_record_age_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_retry_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_partial_batch_item_failure`.\n"]
    pub fn set_on_partial_batch_item_failure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_partial_batch_item_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parallelization_factor`.\n"]
    pub fn set_parallelization_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.parallelization_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_position_timestamp`.\n"]
    pub fn set_starting_position_timestamp(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.starting_position_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dead_letter_config`.\n"]
    pub fn set_dead_letter_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dead_letter_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dead_letter_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_KinesisStreamParameters {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_KinesisStreamParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_KinesisStreamParameters {
    #[doc = ""]
    pub starting_position: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_SourceParameters_KinesisStreamParameters {
    pub fn build(self) -> PipesPipe_SourceParameters_KinesisStreamParameters {
        PipesPipe_SourceParameters_KinesisStreamParameters {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            maximum_record_age_in_seconds: core::default::Default::default(),
            maximum_retry_attempts: core::default::Default::default(),
            on_partial_batch_item_failure: core::default::Default::default(),
            parallelization_factor: core::default::Default::default(),
            starting_position: self.starting_position,
            starting_position_timestamp: core::default::Default::default(),
            dead_letter_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_KinesisStreamParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_KinesisStreamParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_KinesisStreamParametersRef {
        PipesPipe_SourceParameters_KinesisStreamParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_KinesisStreamParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_batching_window_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_record_age_in_seconds` after provisioning.\n"]
    pub fn maximum_record_age_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_record_age_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.base))
    }

    #[doc = "Get a reference to the value of field `on_partial_batch_item_failure` after provisioning.\n"]
    pub fn on_partial_batch_item_failure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_partial_batch_item_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `parallelization_factor` after provisioning.\n"]
    pub fn parallelization_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallelization_factor", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_position_timestamp` after provisioning.\n"]
    pub fn starting_position_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `dead_letter_config` after provisioning.\n"]
    pub fn dead_letter_config(
        &self,
    ) -> terrars::ListRef<PipesPipe_SourceParameters_KinesisStreamParameters_DeadLetterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_config", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_tls_auth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl_scram_512_auth: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials {
    #[doc = "Set the field `client_certificate_tls_auth`.\n"]
    pub fn set_client_certificate_tls_auth(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate_tls_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sasl_scram_512_auth`.\n"]
    pub fn set_sasl_scram_512_auth(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sasl_scram_512_auth = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials {}

impl BuildPipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials {
    pub fn build(self) -> PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials {
        PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials {
            client_certificate_tls_auth: core::default::Default::default(),
            sasl_scram_512_auth: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_CredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_CredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_CredentialsRef {
        PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_CredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_CredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_certificate_tls_auth` after provisioning.\n"]
    pub fn client_certificate_tls_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_tls_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl_scram_512_auth` after provisioning.\n"]
    pub fn sasl_scram_512_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sasl_scram_512_auth", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_SourceParameters_ManagedStreamingKafkaParametersDynamic {
    credentials: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials>>,
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_ManagedStreamingKafkaParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batching_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position: std::option::Option<terrars::PrimField<std::string::String>>,
    topic_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials>>,
    dynamic: PipesPipe_SourceParameters_ManagedStreamingKafkaParametersDynamic,
}

impl PipesPipe_SourceParameters_ManagedStreamingKafkaParameters {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.maximum_batching_window_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starting_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_Credentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.credentials = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_ManagedStreamingKafkaParameters {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_ManagedStreamingKafkaParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_ManagedStreamingKafkaParameters {
    #[doc = ""]
    pub topic_name: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_SourceParameters_ManagedStreamingKafkaParameters {
    pub fn build(self) -> PipesPipe_SourceParameters_ManagedStreamingKafkaParameters {
        PipesPipe_SourceParameters_ManagedStreamingKafkaParameters {
            batch_size: core::default::Default::default(),
            consumer_group_id: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            starting_position: core::default::Default::default(),
            topic_name: self.topic_name,
            credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_ManagedStreamingKafkaParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_ManagedStreamingKafkaParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_ManagedStreamingKafkaParametersRef {
        PipesPipe_SourceParameters_ManagedStreamingKafkaParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_ManagedStreamingKafkaParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_batching_window_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_name` after provisioning.\n"]
    pub fn topic_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_name", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(
        &self,
    ) -> terrars::ListRef<PipesPipe_SourceParameters_ManagedStreamingKafkaParameters_CredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_SourceParameters_RabbitmqBrokerParametersDynamic {
    credentials: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials>>,
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_RabbitmqBrokerParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batching_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    queue_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials>>,
    dynamic: PipesPipe_SourceParameters_RabbitmqBrokerParametersDynamic,
}

impl PipesPipe_SourceParameters_RabbitmqBrokerParameters {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.maximum_batching_window_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_host`.\n"]
    pub fn set_virtual_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_ActivemqBrokerParameters_Credentials>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.credentials = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_RabbitmqBrokerParameters {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_RabbitmqBrokerParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_RabbitmqBrokerParameters {
    #[doc = ""]
    pub queue_name: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_SourceParameters_RabbitmqBrokerParameters {
    pub fn build(self) -> PipesPipe_SourceParameters_RabbitmqBrokerParameters {
        PipesPipe_SourceParameters_RabbitmqBrokerParameters {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            queue_name: self.queue_name,
            virtual_host: core::default::Default::default(),
            credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_RabbitmqBrokerParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_RabbitmqBrokerParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_RabbitmqBrokerParametersRef {
        PipesPipe_SourceParameters_RabbitmqBrokerParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_RabbitmqBrokerParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_batching_window_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_name` after provisioning.\n"]
    pub fn queue_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_name", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_host` after provisioning.\n"]
    pub fn virtual_host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_host", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> terrars::ListRef<PipesPipe_SourceParameters_ActivemqBrokerParameters_CredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_tls_auth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl_scram_256_auth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sasl_scram_512_auth: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials {
    #[doc = "Set the field `basic_auth`.\n"]
    pub fn set_basic_auth(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.basic_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate_tls_auth`.\n"]
    pub fn set_client_certificate_tls_auth(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate_tls_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sasl_scram_256_auth`.\n"]
    pub fn set_sasl_scram_256_auth(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sasl_scram_256_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sasl_scram_512_auth`.\n"]
    pub fn set_sasl_scram_512_auth(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sasl_scram_512_auth = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials {}

impl BuildPipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials {
    pub fn build(self) -> PipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials {
        PipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials {
            basic_auth: core::default::Default::default(),
            client_certificate_tls_auth: core::default::Default::default(),
            sasl_scram_256_auth: core::default::Default::default(),
            sasl_scram_512_auth: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_SelfManagedKafkaParameters_CredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_SelfManagedKafkaParameters_CredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_SelfManagedKafkaParameters_CredentialsRef {
        PipesPipe_SourceParameters_SelfManagedKafkaParameters_CredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_SelfManagedKafkaParameters_CredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `client_certificate_tls_auth` after provisioning.\n"]
    pub fn client_certificate_tls_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_tls_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl_scram_256_auth` after provisioning.\n"]
    pub fn sasl_scram_256_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sasl_scram_256_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `sasl_scram_512_auth` after provisioning.\n"]
    pub fn sasl_scram_512_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sasl_scram_512_auth", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl PipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc {}

impl BuildPipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc {
    pub fn build(self) -> PipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc {
        PipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc {
            security_groups: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_SelfManagedKafkaParameters_VpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_SelfManagedKafkaParameters_VpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_SelfManagedKafkaParameters_VpcRef {
        PipesPipe_SourceParameters_SelfManagedKafkaParameters_VpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_SelfManagedKafkaParameters_VpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_SourceParameters_SelfManagedKafkaParametersDynamic {
    credentials: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials>>,
    vpc: std::option::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc>>,
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_SelfManagedKafkaParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_bootstrap_servers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batching_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_root_ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_position: std::option::Option<terrars::PrimField<std::string::String>>,
    topic_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc>>,
    dynamic: PipesPipe_SourceParameters_SelfManagedKafkaParametersDynamic,
}

impl PipesPipe_SourceParameters_SelfManagedKafkaParameters {
    #[doc = "Set the field `additional_bootstrap_servers`.\n"]
    pub fn set_additional_bootstrap_servers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_bootstrap_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_group_id`.\n"]
    pub fn set_consumer_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.maximum_batching_window_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_root_ca_certificate`.\n"]
    pub fn set_server_root_ca_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.server_root_ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_position`.\n"]
    pub fn set_starting_position(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starting_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PipesPipe_SourceParameters_SelfManagedKafkaParameters_Credentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_SelfManagedKafkaParameters_Vpc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_SelfManagedKafkaParameters {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_SelfManagedKafkaParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_SelfManagedKafkaParameters {
    #[doc = ""]
    pub topic_name: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_SourceParameters_SelfManagedKafkaParameters {
    pub fn build(self) -> PipesPipe_SourceParameters_SelfManagedKafkaParameters {
        PipesPipe_SourceParameters_SelfManagedKafkaParameters {
            additional_bootstrap_servers: core::default::Default::default(),
            batch_size: core::default::Default::default(),
            consumer_group_id: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
            server_root_ca_certificate: core::default::Default::default(),
            starting_position: core::default::Default::default(),
            topic_name: self.topic_name,
            credentials: core::default::Default::default(),
            vpc: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_SelfManagedKafkaParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_SelfManagedKafkaParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_SelfManagedKafkaParametersRef {
        PipesPipe_SourceParameters_SelfManagedKafkaParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_SelfManagedKafkaParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_bootstrap_servers` after provisioning.\n"]
    pub fn additional_bootstrap_servers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.additional_bootstrap_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_group_id` after provisioning.\n"]
    pub fn consumer_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_batching_window_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `server_root_ca_certificate` after provisioning.\n"]
    pub fn server_root_ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_root_ca_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_position` after provisioning.\n"]
    pub fn starting_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_position", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_name` after provisioning.\n"]
    pub fn topic_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_name", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> terrars::ListRef<PipesPipe_SourceParameters_SelfManagedKafkaParameters_CredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<PipesPipe_SourceParameters_SelfManagedKafkaParameters_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters_SqsQueueParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batching_window_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl PipesPipe_SourceParameters_SqsQueueParameters {
    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batching_window_in_seconds`.\n"]
    pub fn set_maximum_batching_window_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.maximum_batching_window_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters_SqsQueueParameters {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters_SqsQueueParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters_SqsQueueParameters {}

impl BuildPipesPipe_SourceParameters_SqsQueueParameters {
    pub fn build(self) -> PipesPipe_SourceParameters_SqsQueueParameters {
        PipesPipe_SourceParameters_SqsQueueParameters {
            batch_size: core::default::Default::default(),
            maximum_batching_window_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParameters_SqsQueueParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParameters_SqsQueueParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_SourceParameters_SqsQueueParametersRef {
        PipesPipe_SourceParameters_SqsQueueParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParameters_SqsQueueParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_batching_window_in_seconds` after provisioning.\n"]
    pub fn maximum_batching_window_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_batching_window_in_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_SourceParametersDynamic {
    activemq_broker_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_ActivemqBrokerParameters>>,
    dynamodb_stream_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_DynamodbStreamParameters>>,
    filter_criteria: std::option::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_FilterCriteria>>,
    kinesis_stream_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_KinesisStreamParameters>>,
    managed_streaming_kafka_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_ManagedStreamingKafkaParameters>>,
    rabbitmq_broker_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_RabbitmqBrokerParameters>>,
    self_managed_kafka_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_SelfManagedKafkaParameters>>,
    sqs_queue_parameters: std::option::Option<terrars::DynamicBlock<PipesPipe_SourceParameters_SqsQueueParameters>>,
}

#[derive(Serialize)]
pub struct PipesPipe_SourceParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activemq_broker_parameters: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_ActivemqBrokerParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamodb_stream_parameters: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_DynamodbStreamParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_criteria: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_FilterCriteria>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream_parameters: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_KinesisStreamParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_streaming_kafka_parameters: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_SourceParameters_ManagedStreamingKafkaParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rabbitmq_broker_parameters: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_RabbitmqBrokerParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_kafka_parameters: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_SourceParameters_SelfManagedKafkaParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs_queue_parameters: std::option::Option<std::vec::Vec<PipesPipe_SourceParameters_SqsQueueParameters>>,
    dynamic: PipesPipe_SourceParametersDynamic,
}

impl PipesPipe_SourceParameters {
    #[doc = "Set the field `activemq_broker_parameters`.\n"]
    pub fn set_activemq_broker_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_ActivemqBrokerParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.activemq_broker_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.activemq_broker_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynamodb_stream_parameters`.\n"]
    pub fn set_dynamodb_stream_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_DynamodbStreamParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dynamodb_stream_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dynamodb_stream_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter_criteria`.\n"]
    pub fn set_filter_criteria(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_FilterCriteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_stream_parameters`.\n"]
    pub fn set_kinesis_stream_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_KinesisStreamParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_stream_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_streaming_kafka_parameters`.\n"]
    pub fn set_managed_streaming_kafka_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_ManagedStreamingKafkaParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_streaming_kafka_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_streaming_kafka_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rabbitmq_broker_parameters`.\n"]
    pub fn set_rabbitmq_broker_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_RabbitmqBrokerParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rabbitmq_broker_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rabbitmq_broker_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed_kafka_parameters`.\n"]
    pub fn set_self_managed_kafka_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_SelfManagedKafkaParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.self_managed_kafka_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.self_managed_kafka_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs_queue_parameters`.\n"]
    pub fn set_sqs_queue_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_SourceParameters_SqsQueueParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqs_queue_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_queue_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_SourceParameters {
    type O = terrars::BlockAssignable<PipesPipe_SourceParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_SourceParameters {}

impl BuildPipesPipe_SourceParameters {
    pub fn build(self) -> PipesPipe_SourceParameters {
        PipesPipe_SourceParameters {
            activemq_broker_parameters: core::default::Default::default(),
            dynamodb_stream_parameters: core::default::Default::default(),
            filter_criteria: core::default::Default::default(),
            kinesis_stream_parameters: core::default::Default::default(),
            managed_streaming_kafka_parameters: core::default::Default::default(),
            rabbitmq_broker_parameters: core::default::Default::default(),
            self_managed_kafka_parameters: core::default::Default::default(),
            sqs_queue_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_SourceParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_SourceParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipesPipe_SourceParametersRef {
        PipesPipe_SourceParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_SourceParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activemq_broker_parameters` after provisioning.\n"]
    pub fn activemq_broker_parameters(&self) -> terrars::ListRef<PipesPipe_SourceParameters_ActivemqBrokerParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.activemq_broker_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `dynamodb_stream_parameters` after provisioning.\n"]
    pub fn dynamodb_stream_parameters(&self) -> terrars::ListRef<PipesPipe_SourceParameters_DynamodbStreamParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamodb_stream_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> terrars::ListRef<PipesPipe_SourceParameters_FilterCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream_parameters` after provisioning.\n"]
    pub fn kinesis_stream_parameters(&self) -> terrars::ListRef<PipesPipe_SourceParameters_KinesisStreamParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_streaming_kafka_parameters` after provisioning.\n"]
    pub fn managed_streaming_kafka_parameters(
        &self,
    ) -> terrars::ListRef<PipesPipe_SourceParameters_ManagedStreamingKafkaParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_streaming_kafka_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `rabbitmq_broker_parameters` after provisioning.\n"]
    pub fn rabbitmq_broker_parameters(&self) -> terrars::ListRef<PipesPipe_SourceParameters_RabbitmqBrokerParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rabbitmq_broker_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_kafka_parameters` after provisioning.\n"]
    pub fn self_managed_kafka_parameters(
        &self,
    ) -> terrars::ListRef<PipesPipe_SourceParameters_SelfManagedKafkaParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_kafka_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs_queue_parameters` after provisioning.\n"]
    pub fn sqs_queue_parameters(&self) -> terrars::ListRef<PipesPipe_SourceParameters_SqsQueueParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_queue_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_ArrayProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl PipesPipe_TargetParameters_BatchJobParameters_ArrayProperties {
    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_BatchJobParameters_ArrayProperties {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_ArrayProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_BatchJobParameters_ArrayProperties {}

impl BuildPipesPipe_TargetParameters_BatchJobParameters_ArrayProperties {
    pub fn build(self) -> PipesPipe_TargetParameters_BatchJobParameters_ArrayProperties {
        PipesPipe_TargetParameters_BatchJobParameters_ArrayProperties { size: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_ArrayPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_BatchJobParameters_ArrayPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_BatchJobParameters_ArrayPropertiesRef {
        PipesPipe_TargetParameters_BatchJobParameters_ArrayPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_BatchJobParameters_ArrayPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment {
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

impl terrars::ToListMappable for PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment {}

impl BuildPipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment {
    pub fn build(self) -> PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment {
        PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_EnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_EnvironmentRef {
        PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_EnvironmentRef {
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

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement { }

impl terrars::ToListMappable for PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement {
    type O =
        terrars
        ::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement {
    pub fn build(self) -> PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement {
        PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirementRef {
        PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

#[derive(Serialize, Default)]
struct PipesPipe_TargetParameters_BatchJobParameters_ContainerOverridesDynamic {
    environment: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment>>,
    resource_requirement: std
    ::option
    ::Option<
        terrars::DynamicBlock<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement>,
    >,
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_requirement: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement>>,
    dynamic: PipesPipe_TargetParameters_BatchJobParameters_ContainerOverridesDynamic,
}

impl PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_requirement`.\n"]
    pub fn set_resource_requirement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_requirement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_requirement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides {}

impl BuildPipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides {
    pub fn build(self) -> PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides {
        PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides {
            command: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            environment: core::default::Default::default(),
            resource_requirement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_ContainerOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_BatchJobParameters_ContainerOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_BatchJobParameters_ContainerOverridesRef {
        PipesPipe_TargetParameters_BatchJobParameters_ContainerOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_BatchJobParameters_ContainerOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_requirement` after provisioning.\n"]
    pub fn resource_requirement(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_requirement", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_DependsOn {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_TargetParameters_BatchJobParameters_DependsOn {
    #[doc = "Set the field `job_id`.\n"]
    pub fn set_job_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.job_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_BatchJobParameters_DependsOn {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_DependsOn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_BatchJobParameters_DependsOn {}

impl BuildPipesPipe_TargetParameters_BatchJobParameters_DependsOn {
    pub fn build(self) -> PipesPipe_TargetParameters_BatchJobParameters_DependsOn {
        PipesPipe_TargetParameters_BatchJobParameters_DependsOn {
            job_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_DependsOnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_BatchJobParameters_DependsOnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_BatchJobParameters_DependsOnRef {
        PipesPipe_TargetParameters_BatchJobParameters_DependsOnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_BatchJobParameters_DependsOnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `job_id` after provisioning.\n"]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_RetryStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl PipesPipe_TargetParameters_BatchJobParameters_RetryStrategy {
    #[doc = "Set the field `attempts`.\n"]
    pub fn set_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.attempts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_BatchJobParameters_RetryStrategy {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_RetryStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_BatchJobParameters_RetryStrategy {}

impl BuildPipesPipe_TargetParameters_BatchJobParameters_RetryStrategy {
    pub fn build(self) -> PipesPipe_TargetParameters_BatchJobParameters_RetryStrategy {
        PipesPipe_TargetParameters_BatchJobParameters_RetryStrategy { attempts: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_BatchJobParameters_RetryStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_BatchJobParameters_RetryStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_BatchJobParameters_RetryStrategyRef {
        PipesPipe_TargetParameters_BatchJobParameters_RetryStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_BatchJobParameters_RetryStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attempts` after provisioning.\n"]
    pub fn attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempts", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_TargetParameters_BatchJobParametersDynamic {
    array_properties: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_BatchJobParameters_ArrayProperties>>,
    container_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides>>,
    depends_on: std::option::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_BatchJobParameters_DependsOn>>,
    retry_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_BatchJobParameters_RetryStrategy>>,
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_BatchJobParameters {
    job_definition: terrars::PrimField<std::string::String>,
    job_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    array_properties: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_BatchJobParameters_ArrayProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_overrides: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    depends_on: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_BatchJobParameters_DependsOn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_strategy: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_BatchJobParameters_RetryStrategy>>,
    dynamic: PipesPipe_TargetParameters_BatchJobParametersDynamic,
}

impl PipesPipe_TargetParameters_BatchJobParameters {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `array_properties`.\n"]
    pub fn set_array_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_ArrayProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.array_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.array_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container_overrides`.\n"]
    pub fn set_container_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `depends_on`.\n"]
    pub fn set_depends_on(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_DependsOn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.depends_on = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.depends_on = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_strategy`.\n"]
    pub fn set_retry_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters_RetryStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_strategy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_BatchJobParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_BatchJobParameters {
    #[doc = ""]
    pub job_definition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub job_name: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_TargetParameters_BatchJobParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_BatchJobParameters {
        PipesPipe_TargetParameters_BatchJobParameters {
            job_definition: self.job_definition,
            job_name: self.job_name,
            parameters: core::default::Default::default(),
            array_properties: core::default::Default::default(),
            container_overrides: core::default::Default::default(),
            depends_on: core::default::Default::default(),
            retry_strategy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_BatchJobParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_BatchJobParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_BatchJobParametersRef {
        PipesPipe_TargetParameters_BatchJobParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_BatchJobParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `job_definition` after provisioning.\n"]
    pub fn job_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `job_name` after provisioning.\n"]
    pub fn job_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `array_properties` after provisioning.\n"]
    pub fn array_properties(&self) -> terrars::ListRef<PipesPipe_TargetParameters_BatchJobParameters_ArrayPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.array_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `container_overrides` after provisioning.\n"]
    pub fn container_overrides(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `depends_on` after provisioning.\n"]
    pub fn depends_on(&self) -> terrars::ListRef<PipesPipe_TargetParameters_BatchJobParameters_DependsOnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.depends_on", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_strategy` after provisioning.\n"]
    pub fn retry_strategy(&self) -> terrars::ListRef<PipesPipe_TargetParameters_BatchJobParameters_RetryStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_CloudwatchLogsParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_TargetParameters_CloudwatchLogsParameters {
    #[doc = "Set the field `log_stream_name`.\n"]
    pub fn set_log_stream_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_stream_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp`.\n"]
    pub fn set_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timestamp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_CloudwatchLogsParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_CloudwatchLogsParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_CloudwatchLogsParameters {}

impl BuildPipesPipe_TargetParameters_CloudwatchLogsParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_CloudwatchLogsParameters {
        PipesPipe_TargetParameters_CloudwatchLogsParameters {
            log_stream_name: core::default::Default::default(),
            timestamp: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_CloudwatchLogsParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_CloudwatchLogsParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_CloudwatchLogsParametersRef {
        PipesPipe_TargetParameters_CloudwatchLogsParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_CloudwatchLogsParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_stream_name` after provisioning.\n"]
    pub fn log_stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream_name", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy {
        PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategyRef {
        PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.assign_public_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration {
    type O =
        terrars
        ::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration {}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration {
        PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfigurationRef {
        PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assign_public_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfigurationDynamic {
    aws_vpc_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_vpc_configuration: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration>>,
    dynamic: PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfigurationDynamic,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration {
    #[doc = "Set the field `aws_vpc_configuration`.\n"]
    pub fn set_aws_vpc_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_vpc_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_vpc_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration {}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration {
        PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration {
            aws_vpc_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfigurationRef {
        PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_vpc_configuration` after provisioning.\n"]
    pub fn aws_vpc_configuration(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration_AwsVpcConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_vpc_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile { }

impl terrars
::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile {
    type O =
        terrars
        ::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile {
        PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFileRef {
        PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

#[derive(Serialize, Default)]
struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverrideDynamic {
    environment: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment>>,
    environment_file: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile>,
    >,
    resource_requirement: std
    ::option
    ::Option<
        terrars::DynamicBlock<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement>,
    >,
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_reservation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_file: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_requirement: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement>>,
    dynamic: PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverrideDynamic,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_reservation`.\n"]
    pub fn set_memory_reservation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_reservation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_Environment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment_file`.\n"]
    pub fn set_environment_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFile,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment_file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment_file = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_requirement`.\n"]
    pub fn set_resource_requirement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_requirement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_requirement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride {}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride {
        PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride {
            command: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
            memory_reservation: core::default::Default::default(),
            name: core::default::Default::default(),
            environment: core::default::Default::default(),
            environment_file: core::default::Default::default(),
            resource_requirement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverrideRef {
        PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_reservation` after provisioning.\n"]
    pub fn memory_reservation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_reservation", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_file` after provisioning.\n"]
    pub fn environment_file(
        &self,
    ) -> terrars
    ::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride_EnvironmentFileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_file", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_requirement` after provisioning.\n"]
    pub fn resource_requirement(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_BatchJobParameters_ContainerOverrides_ResourceRequirementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_requirement", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage {
    size_in_gib: terrars::PrimField<f64>,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage { }

impl terrars::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage {
    #[doc = ""]
    pub size_in_gib: terrars::PrimField<f64>,
}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage {
        PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage { size_in_gib: self.size_in_gib }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorageRef {
        PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size_in_gib` after provisioning.\n"]
    pub fn size_in_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_type`.\n"]
    pub fn set_device_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride {
    type O =
        terrars
        ::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride {}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride {
        PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride {
            device_name: core::default::Default::default(),
            device_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverrideRef {
        PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type` after provisioning.\n"]
    pub fn device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_TargetParameters_EcsTaskParameters_OverridesDynamic {
    container_override: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride>>,
    ephemeral_storage: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage>>,
    inference_accelerator_override: std
    ::option
    ::Option<
        terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride>,
    >,
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_Overrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_override: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_accelerator_override: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride>>,
    dynamic: PipesPipe_TargetParameters_EcsTaskParameters_OverridesDynamic,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_Overrides {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_role_arn`.\n"]
    pub fn set_task_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.task_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_override`.\n"]
    pub fn set_container_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverride>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_override = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorage>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ephemeral_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ephemeral_storage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inference_accelerator_override`.\n"]
    pub fn set_inference_accelerator_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverride,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inference_accelerator_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inference_accelerator_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_Overrides {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_Overrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides {}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_Overrides {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_Overrides {
        PipesPipe_TargetParameters_EcsTaskParameters_Overrides {
            cpu: core::default::Default::default(),
            execution_role_arn: core::default::Default::default(),
            memory: core::default::Default::default(),
            task_role_arn: core::default::Default::default(),
            container_override: core::default::Default::default(),
            ephemeral_storage: core::default::Default::default(),
            inference_accelerator_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_OverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_OverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_OverridesRef {
        PipesPipe_TargetParameters_EcsTaskParameters_OverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_OverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `container_override` after provisioning.\n"]
    pub fn container_override(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_ContainerOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_override", self.base))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_accelerator_override` after provisioning.\n"]
    pub fn inference_accelerator_override(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_Overrides_InferenceAcceleratorOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_accelerator_override", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint {}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint {
        PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint {
            expression: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraintRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraintRef {
        PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy {}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy {
        PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy {
            field: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategyRef {
        PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_TargetParameters_EcsTaskParametersDynamic {
    capacity_provider_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy>>,
    network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration>>,
    overrides: std::option::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_Overrides>>,
    placement_constraint: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint>>,
    placement_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy>>,
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EcsTaskParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ecs_managed_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_execute_command: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_count: std::option::Option<terrars::PrimField<f64>>,
    task_definition_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider_strategy: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overrides: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_Overrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_constraint: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_strategy: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy>>,
    dynamic: PipesPipe_TargetParameters_EcsTaskParametersDynamic,
}

impl PipesPipe_TargetParameters_EcsTaskParameters {
    #[doc = "Set the field `enable_ecs_managed_tags`.\n"]
    pub fn set_enable_ecs_managed_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ecs_managed_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_execute_command`.\n"]
    pub fn set_enable_execute_command(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_execute_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_type`.\n"]
    pub fn set_launch_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.launch_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.platform_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.propagate_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reference_id`.\n"]
    pub fn set_reference_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reference_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `task_count`.\n"]
    pub fn set_task_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.task_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity_provider_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_provider_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_Overrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_constraint`.\n"]
    pub fn set_placement_constraint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placement_constraint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_constraint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_strategy`.\n"]
    pub fn set_placement_strategy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.placement_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.placement_strategy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_EcsTaskParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EcsTaskParameters {
    #[doc = ""]
    pub task_definition_arn: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_TargetParameters_EcsTaskParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_EcsTaskParameters {
        PipesPipe_TargetParameters_EcsTaskParameters {
            enable_ecs_managed_tags: core::default::Default::default(),
            enable_execute_command: core::default::Default::default(),
            group: core::default::Default::default(),
            launch_type: core::default::Default::default(),
            platform_version: core::default::Default::default(),
            propagate_tags: core::default::Default::default(),
            reference_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            task_count: core::default::Default::default(),
            task_definition_arn: self.task_definition_arn,
            capacity_provider_strategy: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            overrides: core::default::Default::default(),
            placement_constraint: core::default::Default::default(),
            placement_strategy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EcsTaskParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EcsTaskParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EcsTaskParametersRef {
        PipesPipe_TargetParameters_EcsTaskParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EcsTaskParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.base))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.base))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.base))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `task_count` after provisioning.\n"]
    pub fn task_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }

    #[doc = "Get a reference to the value of field `task_definition_arn` after provisioning.\n"]
    pub fn task_definition_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider_strategy` after provisioning.\n"]
    pub fn capacity_provider_strategy(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_CapacityProviderStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_provider_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_OverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_constraint` after provisioning.\n"]
    pub fn placement_constraint(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_PlacementConstraintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_constraint", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_strategy` after provisioning.\n"]
    pub fn placement_strategy(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParameters_PlacementStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_EventbridgeEventBusParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detail_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_TargetParameters_EventbridgeEventBusParameters {
    #[doc = "Set the field `detail_type`.\n"]
    pub fn set_detail_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.detail_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time`.\n"]
    pub fn set_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_EventbridgeEventBusParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_EventbridgeEventBusParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_EventbridgeEventBusParameters {}

impl BuildPipesPipe_TargetParameters_EventbridgeEventBusParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_EventbridgeEventBusParameters {
        PipesPipe_TargetParameters_EventbridgeEventBusParameters {
            detail_type: core::default::Default::default(),
            endpoint_id: core::default::Default::default(),
            resources: core::default::Default::default(),
            source: core::default::Default::default(),
            time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_EventbridgeEventBusParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_EventbridgeEventBusParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_EventbridgeEventBusParametersRef {
        PipesPipe_TargetParameters_EventbridgeEventBusParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_EventbridgeEventBusParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `detail_type` after provisioning.\n"]
    pub fn detail_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detail_type", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_KinesisStreamParameters {
    partition_key: terrars::PrimField<std::string::String>,
}

impl PipesPipe_TargetParameters_KinesisStreamParameters { }

impl terrars::ToListMappable for PipesPipe_TargetParameters_KinesisStreamParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_KinesisStreamParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_KinesisStreamParameters {
    #[doc = ""]
    pub partition_key: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_TargetParameters_KinesisStreamParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_KinesisStreamParameters {
        PipesPipe_TargetParameters_KinesisStreamParameters { partition_key: self.partition_key }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_KinesisStreamParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_KinesisStreamParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_KinesisStreamParametersRef {
        PipesPipe_TargetParameters_KinesisStreamParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_KinesisStreamParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `partition_key` after provisioning.\n"]
    pub fn partition_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_key", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_LambdaFunctionParameters {
    invocation_type: terrars::PrimField<std::string::String>,
}

impl PipesPipe_TargetParameters_LambdaFunctionParameters { }

impl terrars::ToListMappable for PipesPipe_TargetParameters_LambdaFunctionParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_LambdaFunctionParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_LambdaFunctionParameters {
    #[doc = ""]
    pub invocation_type: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_TargetParameters_LambdaFunctionParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_LambdaFunctionParameters {
        PipesPipe_TargetParameters_LambdaFunctionParameters { invocation_type: self.invocation_type }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_LambdaFunctionParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_LambdaFunctionParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_LambdaFunctionParametersRef {
        PipesPipe_TargetParameters_LambdaFunctionParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_LambdaFunctionParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invocation_type` after provisioning.\n"]
    pub fn invocation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_type", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_RedshiftDataParameters {
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    sqls: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_event: std::option::Option<terrars::PrimField<bool>>,
}

impl PipesPipe_TargetParameters_RedshiftDataParameters {
    #[doc = "Set the field `db_user`.\n"]
    pub fn set_db_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_manager_arn`.\n"]
    pub fn set_secret_manager_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_manager_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statement_name`.\n"]
    pub fn set_statement_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.statement_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_event`.\n"]
    pub fn set_with_event(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.with_event = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_RedshiftDataParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_RedshiftDataParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_RedshiftDataParameters {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sqls: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPipesPipe_TargetParameters_RedshiftDataParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_RedshiftDataParameters {
        PipesPipe_TargetParameters_RedshiftDataParameters {
            database: self.database,
            db_user: core::default::Default::default(),
            secret_manager_arn: core::default::Default::default(),
            sqls: self.sqls,
            statement_name: core::default::Default::default(),
            with_event: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_RedshiftDataParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_RedshiftDataParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_RedshiftDataParametersRef {
        PipesPipe_TargetParameters_RedshiftDataParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_RedshiftDataParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_manager_arn` after provisioning.\n"]
    pub fn secret_manager_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_manager_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sqls` after provisioning.\n"]
    pub fn sqls(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.sqls", self.base))
    }

    #[doc = "Get a reference to the value of field `statement_name` after provisioning.\n"]
    pub fn statement_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_name", self.base))
    }

    #[doc = "Get a reference to the value of field `with_event` after provisioning.\n"]
    pub fn with_event(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_event", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter { }

impl terrars::ToListMappable for PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter {
    pub fn build(self) -> PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter {
        PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameterRef {
        PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameterRef {
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

#[derive(Serialize, Default)]
struct PipesPipe_TargetParameters_SagemakerPipelineParametersDynamic {
    pipeline_parameter: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter>>,
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_SagemakerPipelineParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_parameter: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter>>,
    dynamic: PipesPipe_TargetParameters_SagemakerPipelineParametersDynamic,
}

impl PipesPipe_TargetParameters_SagemakerPipelineParameters {
    #[doc = "Set the field `pipeline_parameter`.\n"]
    pub fn set_pipeline_parameter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameter>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pipeline_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pipeline_parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_SagemakerPipelineParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_SagemakerPipelineParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_SagemakerPipelineParameters {}

impl BuildPipesPipe_TargetParameters_SagemakerPipelineParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_SagemakerPipelineParameters {
        PipesPipe_TargetParameters_SagemakerPipelineParameters {
            pipeline_parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_SagemakerPipelineParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_SagemakerPipelineParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_SagemakerPipelineParametersRef {
        PipesPipe_TargetParameters_SagemakerPipelineParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_SagemakerPipelineParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pipeline_parameter` after provisioning.\n"]
    pub fn pipeline_parameter(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_SagemakerPipelineParameters_PipelineParameterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pipeline_parameter", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_SqsQueueParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_deduplication_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_TargetParameters_SqsQueueParameters {
    #[doc = "Set the field `message_deduplication_id`.\n"]
    pub fn set_message_deduplication_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.message_deduplication_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_group_id`.\n"]
    pub fn set_message_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_group_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters_SqsQueueParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_SqsQueueParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_SqsQueueParameters {}

impl BuildPipesPipe_TargetParameters_SqsQueueParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_SqsQueueParameters {
        PipesPipe_TargetParameters_SqsQueueParameters {
            message_deduplication_id: core::default::Default::default(),
            message_group_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_SqsQueueParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_SqsQueueParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_SqsQueueParametersRef {
        PipesPipe_TargetParameters_SqsQueueParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_SqsQueueParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_deduplication_id` after provisioning.\n"]
    pub fn message_deduplication_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_deduplication_id", self.base))
    }

    #[doc = "Get a reference to the value of field `message_group_id` after provisioning.\n"]
    pub fn message_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters_StepFunctionStateMachineParameters {
    invocation_type: terrars::PrimField<std::string::String>,
}

impl PipesPipe_TargetParameters_StepFunctionStateMachineParameters { }

impl terrars::ToListMappable for PipesPipe_TargetParameters_StepFunctionStateMachineParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters_StepFunctionStateMachineParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters_StepFunctionStateMachineParameters {
    #[doc = ""]
    pub invocation_type: terrars::PrimField<std::string::String>,
}

impl BuildPipesPipe_TargetParameters_StepFunctionStateMachineParameters {
    pub fn build(self) -> PipesPipe_TargetParameters_StepFunctionStateMachineParameters {
        PipesPipe_TargetParameters_StepFunctionStateMachineParameters { invocation_type: self.invocation_type }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParameters_StepFunctionStateMachineParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParameters_StepFunctionStateMachineParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PipesPipe_TargetParameters_StepFunctionStateMachineParametersRef {
        PipesPipe_TargetParameters_StepFunctionStateMachineParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParameters_StepFunctionStateMachineParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invocation_type` after provisioning.\n"]
    pub fn invocation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct PipesPipe_TargetParametersDynamic {
    batch_job_parameters: std::option::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_BatchJobParameters>>,
    cloudwatch_logs_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_CloudwatchLogsParameters>>,
    ecs_task_parameters: std::option::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_EcsTaskParameters>>,
    eventbridge_event_bus_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_EventbridgeEventBusParameters>>,
    http_parameters: std::option::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_HttpParameters>>,
    kinesis_stream_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_KinesisStreamParameters>>,
    lambda_function_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_LambdaFunctionParameters>>,
    redshift_data_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_RedshiftDataParameters>>,
    sagemaker_pipeline_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_SagemakerPipelineParameters>>,
    sqs_queue_parameters: std::option::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_SqsQueueParameters>>,
    step_function_state_machine_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<PipesPipe_TargetParameters_StepFunctionStateMachineParameters>>,
}

#[derive(Serialize)]
pub struct PipesPipe_TargetParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_job_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_BatchJobParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_CloudwatchLogsParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_task_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_EcsTaskParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eventbridge_event_bus_parameters: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_EventbridgeEventBusParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_HttpParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kinesis_stream_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_KinesisStreamParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_LambdaFunctionParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift_data_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_RedshiftDataParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sagemaker_pipeline_parameters: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_SagemakerPipelineParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs_queue_parameters: std::option::Option<std::vec::Vec<PipesPipe_TargetParameters_SqsQueueParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_function_state_machine_parameters: std
    ::option
    ::Option<std::vec::Vec<PipesPipe_TargetParameters_StepFunctionStateMachineParameters>>,
    dynamic: PipesPipe_TargetParametersDynamic,
}

impl PipesPipe_TargetParameters {
    #[doc = "Set the field `input_template`.\n"]
    pub fn set_input_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `batch_job_parameters`.\n"]
    pub fn set_batch_job_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_BatchJobParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.batch_job_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.batch_job_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudwatch_logs_parameters`.\n"]
    pub fn set_cloudwatch_logs_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_CloudwatchLogsParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecs_task_parameters`.\n"]
    pub fn set_ecs_task_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_EcsTaskParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecs_task_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecs_task_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eventbridge_event_bus_parameters`.\n"]
    pub fn set_eventbridge_event_bus_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_EventbridgeEventBusParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eventbridge_event_bus_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eventbridge_event_bus_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_parameters`.\n"]
    pub fn set_http_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_HttpParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kinesis_stream_parameters`.\n"]
    pub fn set_kinesis_stream_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_KinesisStreamParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kinesis_stream_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function_parameters`.\n"]
    pub fn set_lambda_function_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_LambdaFunctionParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift_data_parameters`.\n"]
    pub fn set_redshift_data_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_RedshiftDataParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift_data_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift_data_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sagemaker_pipeline_parameters`.\n"]
    pub fn set_sagemaker_pipeline_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_SagemakerPipelineParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sagemaker_pipeline_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sagemaker_pipeline_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs_queue_parameters`.\n"]
    pub fn set_sqs_queue_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_SqsQueueParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqs_queue_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_queue_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `step_function_state_machine_parameters`.\n"]
    pub fn set_step_function_state_machine_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PipesPipe_TargetParameters_StepFunctionStateMachineParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.step_function_state_machine_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.step_function_state_machine_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PipesPipe_TargetParameters {
    type O = terrars::BlockAssignable<PipesPipe_TargetParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_TargetParameters {}

impl BuildPipesPipe_TargetParameters {
    pub fn build(self) -> PipesPipe_TargetParameters {
        PipesPipe_TargetParameters {
            input_template: core::default::Default::default(),
            batch_job_parameters: core::default::Default::default(),
            cloudwatch_logs_parameters: core::default::Default::default(),
            ecs_task_parameters: core::default::Default::default(),
            eventbridge_event_bus_parameters: core::default::Default::default(),
            http_parameters: core::default::Default::default(),
            kinesis_stream_parameters: core::default::Default::default(),
            lambda_function_parameters: core::default::Default::default(),
            redshift_data_parameters: core::default::Default::default(),
            sagemaker_pipeline_parameters: core::default::Default::default(),
            sqs_queue_parameters: core::default::Default::default(),
            step_function_state_machine_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TargetParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TargetParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipesPipe_TargetParametersRef {
        PipesPipe_TargetParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TargetParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_template` after provisioning.\n"]
    pub fn input_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_template", self.base))
    }

    #[doc = "Get a reference to the value of field `batch_job_parameters` after provisioning.\n"]
    pub fn batch_job_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParameters_BatchJobParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.batch_job_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs_parameters` after provisioning.\n"]
    pub fn cloudwatch_logs_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParameters_CloudwatchLogsParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `ecs_task_parameters` after provisioning.\n"]
    pub fn ecs_task_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParameters_EcsTaskParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_task_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `eventbridge_event_bus_parameters` after provisioning.\n"]
    pub fn eventbridge_event_bus_parameters(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_EventbridgeEventBusParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eventbridge_event_bus_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `http_parameters` after provisioning.\n"]
    pub fn http_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParameters_HttpParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `kinesis_stream_parameters` after provisioning.\n"]
    pub fn kinesis_stream_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParameters_KinesisStreamParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kinesis_stream_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_function_parameters` after provisioning.\n"]
    pub fn lambda_function_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParameters_LambdaFunctionParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_function_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift_data_parameters` after provisioning.\n"]
    pub fn redshift_data_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParameters_RedshiftDataParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift_data_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `sagemaker_pipeline_parameters` after provisioning.\n"]
    pub fn sagemaker_pipeline_parameters(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_SagemakerPipelineParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sagemaker_pipeline_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs_queue_parameters` after provisioning.\n"]
    pub fn sqs_queue_parameters(&self) -> terrars::ListRef<PipesPipe_TargetParameters_SqsQueueParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_queue_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `step_function_state_machine_parameters` after provisioning.\n"]
    pub fn step_function_state_machine_parameters(
        &self,
    ) -> terrars::ListRef<PipesPipe_TargetParameters_StepFunctionStateMachineParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step_function_state_machine_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct PipesPipe_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PipesPipe_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PipesPipe_Timeouts {
    type O = terrars::BlockAssignable<PipesPipe_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPipesPipe_Timeouts {}

impl BuildPipesPipe_Timeouts {
    pub fn build(self) -> PipesPipe_Timeouts {
        PipesPipe_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PipesPipe_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PipesPipe_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PipesPipe_TimeoutsRef {
        PipesPipe_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PipesPipe_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

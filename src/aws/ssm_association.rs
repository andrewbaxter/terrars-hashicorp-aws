use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsmAssociationData {
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
    apply_only_at_cron_interval: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    association_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automation_target_parameter_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_errors: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_success_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_location: std::option::Option<std::vec::Vec<SsmAssociationOutputLocationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets: std::option::Option<std::vec::Vec<SsmAssociationTargetsEl>>,
    dynamic: SsmAssociationDynamic,
}

struct SsmAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsmAssociationData>,
}

#[derive(Clone)]
pub struct SsmAssociation(std::rc::Rc<SsmAssociation_>);

impl SsmAssociation {
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

    #[doc = "Set the field `apply_only_at_cron_interval`.\n"]
    pub fn set_apply_only_at_cron_interval(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_only_at_cron_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `association_name`.\n"]
    pub fn set_association_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().association_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automation_target_parameter_name`.\n"]
    pub fn set_automation_target_parameter_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().automation_target_parameter_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compliance_severity`.\n"]
    pub fn set_compliance_severity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().compliance_severity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document_version`.\n"]
    pub fn set_document_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().document_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrency`.\n"]
    pub fn set_max_concurrency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().max_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_errors`.\n"]
    pub fn set_max_errors(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().max_errors = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `schedule_expression`.\n"]
    pub fn set_schedule_expression(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schedule_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_success_timeout_seconds`.\n"]
    pub fn set_wait_for_success_timeout_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wait_for_success_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_location`.\n"]
    pub fn set_output_location(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmAssociationOutputLocationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(self, v: impl std::convert::Into<terrars::BlockAssignable<SsmAssociationTargetsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.targets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `apply_only_at_cron_interval` after provisioning.\n"]
    pub fn apply_only_at_cron_interval(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_only_at_cron_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_name` after provisioning.\n"]
    pub fn association_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automation_target_parameter_name` after provisioning.\n"]
    pub fn automation_target_parameter_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.automation_target_parameter_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compliance_severity` after provisioning.\n"]
    pub fn compliance_severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_severity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_errors` after provisioning.\n"]
    pub fn max_errors(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_success_timeout_seconds` after provisioning.\n"]
    pub fn wait_for_success_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_success_timeout_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_location` after provisioning.\n"]
    pub fn output_location(&self) -> terrars::ListRef<SsmAssociationOutputLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::ListRef<SsmAssociationTargetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }
}

impl terrars::Referable for SsmAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsmAssociation { }

impl terrars::ToListMappable for SsmAssociation {
    type O = terrars::ListRef<SsmAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsmAssociation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssm_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmAssociation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSsmAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> SsmAssociation {
        let out = SsmAssociation(std::rc::Rc::new(SsmAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsmAssociationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                apply_only_at_cron_interval: core::default::Default::default(),
                association_name: core::default::Default::default(),
                automation_target_parameter_name: core::default::Default::default(),
                compliance_severity: core::default::Default::default(),
                document_version: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: core::default::Default::default(),
                max_concurrency: core::default::Default::default(),
                max_errors: core::default::Default::default(),
                name: self.name,
                parameters: core::default::Default::default(),
                schedule_expression: core::default::Default::default(),
                wait_for_success_timeout_seconds: core::default::Default::default(),
                output_location: core::default::Default::default(),
                targets: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsmAssociationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apply_only_at_cron_interval` after provisioning.\n"]
    pub fn apply_only_at_cron_interval(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apply_only_at_cron_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_name` after provisioning.\n"]
    pub fn association_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automation_target_parameter_name` after provisioning.\n"]
    pub fn automation_target_parameter_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.automation_target_parameter_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `compliance_severity` after provisioning.\n"]
    pub fn compliance_severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_severity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_version` after provisioning.\n"]
    pub fn document_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_concurrency` after provisioning.\n"]
    pub fn max_concurrency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_errors` after provisioning.\n"]
    pub fn max_errors(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_success_timeout_seconds` after provisioning.\n"]
    pub fn wait_for_success_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.wait_for_success_timeout_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_location` after provisioning.\n"]
    pub fn output_location(&self) -> terrars::ListRef<SsmAssociationOutputLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::ListRef<SsmAssociationTargetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SsmAssociationOutputLocationEl {
    s3_bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SsmAssociationOutputLocationEl {
    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_key_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_region`.\n"]
    pub fn set_s3_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SsmAssociationOutputLocationEl {
    type O = terrars::BlockAssignable<SsmAssociationOutputLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmAssociationOutputLocationEl {
    #[doc = ""]
    pub s3_bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildSsmAssociationOutputLocationEl {
    pub fn build(self) -> SsmAssociationOutputLocationEl {
        SsmAssociationOutputLocationEl {
            s3_bucket_name: self.s3_bucket_name,
            s3_key_prefix: core::default::Default::default(),
            s3_region: core::default::Default::default(),
        }
    }
}

pub struct SsmAssociationOutputLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmAssociationOutputLocationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmAssociationOutputLocationElRef {
        SsmAssociationOutputLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmAssociationOutputLocationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_region` after provisioning.\n"]
    pub fn s3_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_region", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmAssociationTargetsEl {
    key: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SsmAssociationTargetsEl { }

impl terrars::ToListMappable for SsmAssociationTargetsEl {
    type O = terrars::BlockAssignable<SsmAssociationTargetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmAssociationTargetsEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSsmAssociationTargetsEl {
    pub fn build(self) -> SsmAssociationTargetsEl {
        SsmAssociationTargetsEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct SsmAssociationTargetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmAssociationTargetsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmAssociationTargetsElRef {
        SsmAssociationTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmAssociationTargetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmAssociationDynamic {
    output_location: std::option::Option<terrars::DynamicBlock<SsmAssociationOutputLocationEl>>,
    targets: std::option::Option<terrars::DynamicBlock<SsmAssociationTargetsEl>>,
}

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FlowLogData {
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
    eni_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_aggregation_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_attachment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_options: std::option::Option<std::vec::Vec<FlowLogDestinationOptionsEl>>,
    dynamic: FlowLogDynamic,
}

struct FlowLog_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FlowLogData>,
}

#[derive(Clone)]
pub struct FlowLog(std::rc::Rc<FlowLog_>);

impl FlowLog {
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

    #[doc = "Set the field `eni_id`.\n"]
    pub fn set_eni_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().eni_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_role_arn`.\n"]
    pub fn set_iam_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().iam_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination_type`.\n"]
    pub fn set_log_destination_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_destination_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_format`.\n"]
    pub fn set_log_format(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_aggregation_interval`.\n"]
    pub fn set_max_aggregation_interval(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_aggregation_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subnet_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `traffic_type`.\n"]
    pub fn set_traffic_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().traffic_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment_id`.\n"]
    pub fn set_transit_gateway_attachment_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().transit_gateway_attachment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_options`.\n"]
    pub fn set_destination_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FlowLogDestinationOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eni_id` after provisioning.\n"]
    pub fn eni_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eni_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_destination_type` after provisioning.\n"]
    pub fn log_destination_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_aggregation_interval` after provisioning.\n"]
    pub fn max_aggregation_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_aggregation_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_type` after provisioning.\n"]
    pub fn traffic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_options` after provisioning.\n"]
    pub fn destination_options(&self) -> terrars::ListRef<FlowLogDestinationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_options", self.extract_ref()))
    }
}

impl terrars::Referable for FlowLog {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FlowLog { }

impl terrars::ToListMappable for FlowLog {
    type O = terrars::ListRef<FlowLogRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FlowLog_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_flow_log".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFlowLog {
    pub tf_id: std::string::String,
}

impl BuildFlowLog {
    pub fn build(self, stack: &mut terrars::Stack) -> FlowLog {
        let out = FlowLog(std::rc::Rc::new(FlowLog_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FlowLogData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                eni_id: core::default::Default::default(),
                iam_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                log_destination: core::default::Default::default(),
                log_destination_type: core::default::Default::default(),
                log_format: core::default::Default::default(),
                log_group_name: core::default::Default::default(),
                max_aggregation_interval: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                traffic_type: core::default::Default::default(),
                transit_gateway_attachment_id: core::default::Default::default(),
                transit_gateway_id: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                destination_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FlowLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlowLogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FlowLogRef {
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

    #[doc = "Get a reference to the value of field `eni_id` after provisioning.\n"]
    pub fn eni_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eni_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_destination_type` after provisioning.\n"]
    pub fn log_destination_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_aggregation_interval` after provisioning.\n"]
    pub fn max_aggregation_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_aggregation_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_type` after provisioning.\n"]
    pub fn traffic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_id` after provisioning.\n"]
    pub fn transit_gateway_attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_attachment_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_options` after provisioning.\n"]
    pub fn destination_options(&self) -> terrars::ListRef<FlowLogDestinationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FlowLogDestinationOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_compatible_partitions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_hour_partition: std::option::Option<terrars::PrimField<bool>>,
}

impl FlowLogDestinationOptionsEl {
    #[doc = "Set the field `file_format`.\n"]
    pub fn set_file_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hive_compatible_partitions`.\n"]
    pub fn set_hive_compatible_partitions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.hive_compatible_partitions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_hour_partition`.\n"]
    pub fn set_per_hour_partition(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.per_hour_partition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FlowLogDestinationOptionsEl {
    type O = terrars::BlockAssignable<FlowLogDestinationOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFlowLogDestinationOptionsEl {}

impl BuildFlowLogDestinationOptionsEl {
    pub fn build(self) -> FlowLogDestinationOptionsEl {
        FlowLogDestinationOptionsEl {
            file_format: core::default::Default::default(),
            hive_compatible_partitions: core::default::Default::default(),
            per_hour_partition: core::default::Default::default(),
        }
    }
}

pub struct FlowLogDestinationOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FlowLogDestinationOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FlowLogDestinationOptionsElRef {
        FlowLogDestinationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FlowLogDestinationOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_format` after provisioning.\n"]
    pub fn file_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_format", self.base))
    }

    #[doc = "Get a reference to the value of field `hive_compatible_partitions` after provisioning.\n"]
    pub fn hive_compatible_partitions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hive_compatible_partitions", self.base))
    }

    #[doc = "Get a reference to the value of field `per_hour_partition` after provisioning.\n"]
    pub fn per_hour_partition(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_hour_partition", self.base))
    }
}

#[derive(Serialize, Default)]
struct FlowLogDynamic {
    destination_options: std::option::Option<terrars::DynamicBlock<FlowLogDestinationOptionsEl>>,
}

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppflowFlowData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_flow_config: std::option::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_flow_config: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task: std::option::Option<std::vec::Vec<AppflowFlowTaskEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_config: std::option::Option<std::vec::Vec<AppflowFlowTriggerConfigEl>>,
    dynamic: AppflowFlowDynamic,
}

struct AppflowFlow_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppflowFlowData>,
}

#[derive(Clone)]
pub struct AppflowFlow(std::rc::Rc<AppflowFlow_>);

impl AppflowFlow {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_arn`.\n"]
    pub fn set_kms_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `destination_flow_config`.\n"]
    pub fn set_destination_flow_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlowDestinationFlowConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_flow_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_flow_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_flow_config`.\n"]
    pub fn set_source_flow_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlowSourceFlowConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_flow_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_flow_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `task`.\n"]
    pub fn set_task(self, v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlowTaskEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().task = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.task = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trigger_config`.\n"]
    pub fn set_trigger_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlowTriggerConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger_config = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_flow_config` after provisioning.\n"]
    pub fn source_flow_config(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_config` after provisioning.\n"]
    pub fn trigger_config(&self) -> terrars::ListRef<AppflowFlowTriggerConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_config", self.extract_ref()))
    }
}

impl terrars::Referable for AppflowFlow {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppflowFlow { }

impl terrars::ToListMappable for AppflowFlow {
    type O = terrars::ListRef<AppflowFlowRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppflowFlow_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appflow_flow".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppflowFlow {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlow {
    pub fn build(self, stack: &mut terrars::Stack) -> AppflowFlow {
        let out = AppflowFlow(std::rc::Rc::new(AppflowFlow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppflowFlowData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_arn: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                destination_flow_config: core::default::Default::default(),
                source_flow_config: core::default::Default::default(),
                task: core::default::Default::default(),
                trigger_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppflowFlowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppflowFlowRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_arn` after provisioning.\n"]
    pub fn kms_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_flow_config` after provisioning.\n"]
    pub fn source_flow_config(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_flow_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trigger_config` after provisioning.\n"]
    pub fn trigger_config(&self) -> terrars::ListRef<AppflowFlowTriggerConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    entity_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_operation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl,
        >,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    #[doc = "Set the field `custom_properties`.\n"]
    pub fn set_custom_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id_field_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.write_operation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    type O =
        terrars
        ::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    #[doc = ""]
    pub entity_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl {
            custom_properties: core::default::Default::default(),
            entity_name: self.entity_name,
            id_field_names: core::default::Default::default(),
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_properties` after provisioning.\n"]
    pub fn custom_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_name` after provisioning.\n"]
    pub fn entity_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElErrorHandlingConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_type_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    #[doc = "Set the field `object_type_name`.\n"]
    pub fn set_object_type_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_type_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    type O =
        terrars
        ::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl {
            domain_name: self.domain_name,
            object_type_name: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object_type_name` after provisioning.\n"]
    pub fn object_type_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_type_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    type O =
        terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl {
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElErrorHandlingConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl {
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElErrorHandlingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl { }

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {
    type O =
        terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl {}
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl {
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElErrorHandlingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    intermediate_bucket_name: terrars::PrimField<std::string::String>,
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    #[doc = ""]
    pub intermediate_bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl {
            bucket_prefix: core::default::Default::default(),
            intermediate_bucket_name: self.intermediate_bucket_name,
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `intermediate_bucket_name` after provisioning.\n"]
    pub fn intermediate_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intermediate_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElErrorHandlingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
    #[doc = "Set the field `aggregation_type`.\n"]
    pub fn set_aggregation_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aggregation_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl {
            aggregation_type: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregation_type` after provisioning.\n"]
    pub fn aggregation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregation_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
    #[doc = "Set the field `prefix_format`.\n"]
    pub fn set_prefix_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_type`.\n"]
    pub fn set_prefix_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl {
            prefix_format: core::default::Default::default(),
            prefix_type: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix_format` after provisioning.\n"]
    pub fn prefix_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_type` after provisioning.\n"]
    pub fn prefix_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElDynamic {
    aggregation_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl,
        >,
    >,
    prefix_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl,
        >,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
    #[doc = "Set the field `file_type`.\n"]
    pub fn set_file_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aggregation_config`.\n"]
    pub fn set_aggregation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aggregation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix_config`.\n"]
    pub fn set_prefix_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prefix_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl {
            file_type: core::default::Default::default(),
            aggregation_config: core::default::Default::default(),
            prefix_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }

    #[doc = "Get a reference to the value of field `aggregation_config` after provisioning.\n"]
    pub fn aggregation_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElAggregationConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aggregation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_config` after provisioning.\n"]
    pub fn prefix_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElPrefixConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElDynamic {
    s3_output_format_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_format_config: std
    ::option
    ::Option<
        std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_format_config`.\n"]
    pub fn set_s3_output_format_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_output_format_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output_format_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            s3_output_format_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_format_config` after provisioning.\n"]
    pub fn s3_output_format_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElS3OutputFormatConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_output_format_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_operation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id_field_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.write_operation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    type O =
        terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl {
            id_field_names: core::default::Default::default(),
            object: self.object,
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElErrorHandlingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl,
        >,
    >,
    success_response_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    object_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_operation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_response_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl,
        >,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id_field_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.write_operation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `success_response_handling_config`.\n"]
    pub fn set_success_response_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.success_response_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.success_response_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    #[doc = ""]
    pub object_path: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl {
            id_field_names: core::default::Default::default(),
            object_path: self.object_path,
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            success_response_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `object_path` after provisioning.\n"]
    pub fn object_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_path", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElErrorHandlingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `success_response_handling_config` after provisioning.\n"]
    pub fn success_response_handling_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElSuccessResponseHandlingConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.success_response_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    intermediate_bucket_name: terrars::PrimField<std::string::String>,
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    #[doc = ""]
    pub intermediate_bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl {
            bucket_prefix: core::default::Default::default(),
            intermediate_bucket_name: self.intermediate_bucket_name,
            object: self.object,
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `intermediate_bucket_name` after provisioning.\n"]
    pub fn intermediate_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intermediate_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElErrorHandlingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
    #[doc = "Set the field `aggregation_type`.\n"]
    pub fn set_aggregation_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aggregation_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl {
            aggregation_type: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregation_type` after provisioning.\n"]
    pub fn aggregation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregation_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_format: std::option::Option<terrars::PrimField<std::string::String>>,
    prefix_type: terrars::PrimField<std::string::String>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    #[doc = "Set the field `prefix_format`.\n"]
    pub fn set_prefix_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    #[doc = ""]
    pub prefix_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl {
            prefix_format: core::default::Default::default(),
            prefix_type: self.prefix_type,
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix_format` after provisioning.\n"]
    pub fn prefix_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_format", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_type` after provisioning.\n"]
    pub fn prefix_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElDynamic {
    aggregation_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl,
        >,
    >,
    prefix_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl,
        >,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
    #[doc = "Set the field `file_type`.\n"]
    pub fn set_file_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aggregation_config`.\n"]
    pub fn set_aggregation_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aggregation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aggregation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix_config`.\n"]
    pub fn set_prefix_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prefix_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl {
            file_type: core::default::Default::default(),
            aggregation_config: core::default::Default::default(),
            prefix_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }

    #[doc = "Get a reference to the value of field `aggregation_config` after provisioning.\n"]
    pub fn aggregation_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElAggregationConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aggregation_config", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_config` after provisioning.\n"]
    pub fn prefix_config(
        &self,
    ) -> terrars
    ::ListRef<
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElPrefixConfigElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElDynamic {
    s3_output_format_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_output_format_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_output_format_config`.\n"]
    pub fn set_s3_output_format_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_output_format_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_output_format_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            s3_output_format_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_format_config` after provisioning.\n"]
    pub fn s3_output_format_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElS3OutputFormatConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_output_format_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_on_first_destination_error: std::option::Option<terrars::PrimField<bool>>,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_on_first_destination_error`.\n"]
    pub fn set_fail_on_first_destination_error(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_on_first_destination_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
    pub fn build(
        self,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl {
            bucket_name: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            fail_on_first_destination_error: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_on_first_destination_error` after provisioning.\n"]
    pub fn fail_on_first_destination_error(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_on_first_destination_error", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElDynamic {
    error_handling_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_field_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    object: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_operation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl>,
    >,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    #[doc = "Set the field `id_field_names`.\n"]
    pub fn set_id_field_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.id_field_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_operation_type`.\n"]
    pub fn set_write_operation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.write_operation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_config`.\n"]
    pub fn set_error_handling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_handling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_handling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl {
            id_field_names: core::default::Default::default(),
            object: self.object,
            write_operation_type: core::default::Default::default(),
            error_handling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id_field_names` after provisioning.\n"]
    pub fn id_field_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.id_field_names", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `write_operation_type` after provisioning.\n"]
    pub fn write_operation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_operation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_config` after provisioning.\n"]
    pub fn error_handling_config(
        &self,
    ) -> terrars
    ::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElErrorHandlingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElDynamic {
    custom_connector: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl>,
    >,
    customer_profiles: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl>,
    >,
    event_bridge: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl>>,
    honeycode: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl>>,
    lookout_metrics: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl>,
    >,
    marketo: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl>>,
    redshift: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl>>,
    s3: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El>>,
    salesforce: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl>>,
    sapo_data: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl>>,
    snowflake: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl>>,
    upsolver: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl>>,
    zendesk: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_profiles: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_bridge: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    honeycode: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lookout_metrics: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upsolver: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl>>,
    dynamic: AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElDynamic,
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `customer_profiles`.\n"]
    pub fn set_customer_profiles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customer_profiles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_profiles = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_bridge`.\n"]
    pub fn set_event_bridge(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.event_bridge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.event_bridge = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `honeycode`.\n"]
    pub fn set_honeycode(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.honeycode = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.honeycode = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lookout_metrics`.\n"]
    pub fn set_lookout_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lookout_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lookout_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.marketo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3El>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sapo_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake`.\n"]
    pub fn set_snowflake(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snowflake = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upsolver`.\n"]
    pub fn set_upsolver(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.upsolver = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.upsolver = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zendesk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {}

impl BuildAppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl {
            custom_connector: core::default::Default::default(),
            customer_profiles: core::default::Default::default(),
            event_bridge: core::default::Default::default(),
            honeycode: core::default::Default::default(),
            lookout_metrics: core::default::Default::default(),
            marketo: core::default::Default::default(),
            redshift: core::default::Default::default(),
            s3: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            snowflake: core::default::Default::default(),
            upsolver: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
        AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomConnectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_profiles` after provisioning.\n"]
    pub fn customer_profiles(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElCustomerProfilesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_profiles", self.base))
    }

    #[doc = "Get a reference to the value of field `event_bridge` after provisioning.\n"]
    pub fn event_bridge(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElEventBridgeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_bridge", self.base))
    }

    #[doc = "Get a reference to the value of field `honeycode` after provisioning.\n"]
    pub fn honeycode(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElHoneycodeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.honeycode", self.base))
    }

    #[doc = "Get a reference to the value of field `lookout_metrics` after provisioning.\n"]
    pub fn lookout_metrics(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElLookoutMetricsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lookout_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElMarketoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRedshiftElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSalesforceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSapoDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElSnowflakeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }

    #[doc = "Get a reference to the value of field `upsolver` after provisioning.\n"]
    pub fn upsolver(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElUpsolverElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upsolver", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElZendeskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDestinationFlowConfigElDynamic {
    destination_connector_properties: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowDestinationFlowConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_profile_name: std::option::Option<terrars::PrimField<std::string::String>>,
    connector_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_connector_properties: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl>>,
    dynamic: AppflowFlowDestinationFlowConfigElDynamic,
}

impl AppflowFlowDestinationFlowConfigEl {
    #[doc = "Set the field `api_version`.\n"]
    pub fn set_api_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_profile_name`.\n"]
    pub fn set_connector_profile_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connector_profile_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_connector_properties`.\n"]
    pub fn set_destination_connector_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_connector_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_connector_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowDestinationFlowConfigEl {
    type O = terrars::BlockAssignable<AppflowFlowDestinationFlowConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowDestinationFlowConfigEl {
    #[doc = ""]
    pub connector_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowDestinationFlowConfigEl {
    pub fn build(self) -> AppflowFlowDestinationFlowConfigEl {
        AppflowFlowDestinationFlowConfigEl {
            api_version: core::default::Default::default(),
            connector_profile_name: core::default::Default::default(),
            connector_type: self.connector_type,
            destination_connector_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowDestinationFlowConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowDestinationFlowConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlowDestinationFlowConfigElRef {
        AppflowFlowDestinationFlowConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowDestinationFlowConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_version` after provisioning.\n"]
    pub fn api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_profile_name` after provisioning.\n"]
    pub fn connector_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_profile_name", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_connector_properties` after provisioning.\n"]
    pub fn destination_connector_properties(
        &self,
    ) -> terrars::ListRef<AppflowFlowDestinationFlowConfigElDestinationConnectorPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_connector_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datetime_type_field_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
    #[doc = "Set the field `datetime_type_field_name`.\n"]
    pub fn set_datetime_type_field_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.datetime_type_field_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElIncrementalPullConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElIncrementalPullConfigEl {}

impl BuildAppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
        AppflowFlowSourceFlowConfigElIncrementalPullConfigEl {
            datetime_type_field_name: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
        AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datetime_type_field_name` after provisioning.\n"]
    pub fn datetime_type_field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datetime_type_field_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    entity_name: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    #[doc = "Set the field `custom_properties`.\n"]
    pub fn set_custom_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    #[doc = ""]
    pub entity_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl {
            custom_properties: core::default::Default::default(),
            entity_name: self.entity_name,
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_properties` after provisioning.\n"]
    pub fn custom_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_name` after provisioning.\n"]
    pub fn entity_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_name", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_file_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
    #[doc = "Set the field `s3_input_file_type`.\n"]
    pub fn set_s3_input_file_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_input_file_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
    type O =
        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl {
            s3_input_file_type: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_input_file_type` after provisioning.\n"]
    pub fn s3_input_file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_input_file_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElDynamic {
    s3_input_format_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_input_format_config: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl>>,
    dynamic: AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElDynamic,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_input_format_config`.\n"]
    pub fn set_s3_input_format_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_input_format_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_input_format_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El {
            bucket_name: self.bucket_name,
            bucket_prefix: core::default::Default::default(),
            s3_input_format_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_input_format_config` after provisioning.\n"]
    pub fn s3_input_format_config(
        &self,
    ) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElS3InputFormatConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_input_format_config", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_dynamic_field_update: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_deleted_records: std::option::Option<terrars::PrimField<bool>>,
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    #[doc = "Set the field `enable_dynamic_field_update`.\n"]
    pub fn set_enable_dynamic_field_update(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_dynamic_field_update = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_deleted_records`.\n"]
    pub fn set_include_deleted_records(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_deleted_records = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl {
            enable_dynamic_field_update: core::default::Default::default(),
            include_deleted_records: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_dynamic_field_update` after provisioning.\n"]
    pub fn enable_dynamic_field_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dynamic_field_update", self.base))
    }

    #[doc = "Get a reference to the value of field `include_deleted_records` after provisioning.\n"]
    pub fn include_deleted_records(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_deleted_records", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_all_versions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_renditions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_source_files: std::option::Option<terrars::PrimField<bool>>,
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    #[doc = "Set the field `document_type`.\n"]
    pub fn set_document_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_all_versions`.\n"]
    pub fn set_include_all_versions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_all_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_renditions`.\n"]
    pub fn set_include_renditions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_renditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_source_files`.\n"]
    pub fn set_include_source_files(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_source_files = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl {
            document_type: core::default::Default::default(),
            include_all_versions: core::default::Default::default(),
            include_renditions: core::default::Default::default(),
            include_source_files: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_type` after provisioning.\n"]
    pub fn document_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_type", self.base))
    }

    #[doc = "Get a reference to the value of field `include_all_versions` after provisioning.\n"]
    pub fn include_all_versions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_all_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `include_renditions` after provisioning.\n"]
    pub fn include_renditions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_renditions", self.base))
    }

    #[doc = "Get a reference to the value of field `include_source_files` after provisioning.\n"]
    pub fn include_source_files(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_source_files", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
    object: terrars::PrimField<std::string::String>,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl { }

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
    #[doc = ""]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl { object: self.object }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynamic {
    amplitude: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl>>,
    custom_connector: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl>>,
    datadog: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl>>,
    dynatrace: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl>>,
    google_analytics: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl>>,
    infor_nexus: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl>>,
    marketo: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl>>,
    s3: std::option::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El>>,
    salesforce: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl>>,
    sapo_data: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl>>,
    service_now: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl>>,
    singular: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl>>,
    slack: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl>>,
    trendmicro: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl>>,
    veeva: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl>>,
    zendesk: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amplitude: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynatrace: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_analytics: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infor_nexus: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    singular: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trendmicro: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    veeva: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl>>,
    dynamic: AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynamic,
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amplitude = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amplitude = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datadog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dynatrace = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dynatrace = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_analytics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_analytics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.infor_nexus = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.infor_nexus = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.marketo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.marketo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3El>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sapo_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sapo_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_now = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.singular = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.singular = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trendmicro = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trendmicro = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.veeva = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.veeva = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zendesk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zendesk = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {}

impl BuildAppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            s3: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
        AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElAmplitudeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(
        &self,
    ) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElCustomConnectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElDynatraceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(
        &self,
    ) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElGoogleAnalyticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(
        &self,
    ) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElInforNexusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElMarketoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(
        &self,
    ) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSalesforceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSapoDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(
        &self,
    ) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElServiceNowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSingularElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElSlackElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(
        &self,
    ) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElTrendmicroElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElVeevaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElZendeskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowSourceFlowConfigElDynamic {
    incremental_pull_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElIncrementalPullConfigEl>>,
    source_connector_properties: std
    ::option
    ::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowSourceFlowConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_profile_name: std::option::Option<terrars::PrimField<std::string::String>>,
    connector_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incremental_pull_config: std::option::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElIncrementalPullConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_connector_properties: std
    ::option
    ::Option<std::vec::Vec<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl>>,
    dynamic: AppflowFlowSourceFlowConfigElDynamic,
}

impl AppflowFlowSourceFlowConfigEl {
    #[doc = "Set the field `api_version`.\n"]
    pub fn set_api_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_profile_name`.\n"]
    pub fn set_connector_profile_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connector_profile_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `incremental_pull_config`.\n"]
    pub fn set_incremental_pull_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlowSourceFlowConfigElIncrementalPullConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.incremental_pull_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.incremental_pull_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_connector_properties`.\n"]
    pub fn set_source_connector_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_connector_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_connector_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowSourceFlowConfigEl {
    type O = terrars::BlockAssignable<AppflowFlowSourceFlowConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowSourceFlowConfigEl {
    #[doc = ""]
    pub connector_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowSourceFlowConfigEl {
    pub fn build(self) -> AppflowFlowSourceFlowConfigEl {
        AppflowFlowSourceFlowConfigEl {
            api_version: core::default::Default::default(),
            connector_profile_name: core::default::Default::default(),
            connector_type: self.connector_type,
            incremental_pull_config: core::default::Default::default(),
            source_connector_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowSourceFlowConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowSourceFlowConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlowSourceFlowConfigElRef {
        AppflowFlowSourceFlowConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowSourceFlowConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_version` after provisioning.\n"]
    pub fn api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_profile_name` after provisioning.\n"]
    pub fn connector_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_profile_name", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_type` after provisioning.\n"]
    pub fn connector_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.base))
    }

    #[doc = "Get a reference to the value of field `incremental_pull_config` after provisioning.\n"]
    pub fn incremental_pull_config(&self) -> terrars::ListRef<AppflowFlowSourceFlowConfigElIncrementalPullConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.incremental_pull_config", self.base))
    }

    #[doc = "Get a reference to the value of field `source_connector_properties` after provisioning.\n"]
    pub fn source_connector_properties(
        &self,
    ) -> terrars::ListRef<AppflowFlowSourceFlowConfigElSourceConnectorPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_connector_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowTaskElConnectorOperatorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amplitude: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_connector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynatrace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_analytics: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infor_nexus: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sapo_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    singular: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trendmicro: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    veeva: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zendesk: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlowTaskElConnectorOperatorEl {
    #[doc = "Set the field `amplitude`.\n"]
    pub fn set_amplitude(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.amplitude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_connector`.\n"]
    pub fn set_custom_connector(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_connector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datadog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dynatrace`.\n"]
    pub fn set_dynatrace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dynatrace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_analytics`.\n"]
    pub fn set_google_analytics(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.google_analytics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `infor_nexus`.\n"]
    pub fn set_infor_nexus(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.infor_nexus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `marketo`.\n"]
    pub fn set_marketo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.marketo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `salesforce`.\n"]
    pub fn set_salesforce(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.salesforce = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sapo_data`.\n"]
    pub fn set_sapo_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sapo_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_now = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `singular`.\n"]
    pub fn set_singular(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.singular = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slack = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trendmicro`.\n"]
    pub fn set_trendmicro(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trendmicro = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `veeva`.\n"]
    pub fn set_veeva(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.veeva = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zendesk`.\n"]
    pub fn set_zendesk(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zendesk = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlowTaskElConnectorOperatorEl {
    type O = terrars::BlockAssignable<AppflowFlowTaskElConnectorOperatorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTaskElConnectorOperatorEl {}

impl BuildAppflowFlowTaskElConnectorOperatorEl {
    pub fn build(self) -> AppflowFlowTaskElConnectorOperatorEl {
        AppflowFlowTaskElConnectorOperatorEl {
            amplitude: core::default::Default::default(),
            custom_connector: core::default::Default::default(),
            datadog: core::default::Default::default(),
            dynatrace: core::default::Default::default(),
            google_analytics: core::default::Default::default(),
            infor_nexus: core::default::Default::default(),
            marketo: core::default::Default::default(),
            s3: core::default::Default::default(),
            salesforce: core::default::Default::default(),
            sapo_data: core::default::Default::default(),
            service_now: core::default::Default::default(),
            singular: core::default::Default::default(),
            slack: core::default::Default::default(),
            trendmicro: core::default::Default::default(),
            veeva: core::default::Default::default(),
            zendesk: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowTaskElConnectorOperatorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowTaskElConnectorOperatorElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlowTaskElConnectorOperatorElRef {
        AppflowFlowTaskElConnectorOperatorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTaskElConnectorOperatorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amplitude` after provisioning.\n"]
    pub fn amplitude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amplitude", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_connector` after provisioning.\n"]
    pub fn custom_connector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_connector", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `dynatrace` after provisioning.\n"]
    pub fn dynatrace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dynatrace", self.base))
    }

    #[doc = "Get a reference to the value of field `google_analytics` after provisioning.\n"]
    pub fn google_analytics(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `infor_nexus` after provisioning.\n"]
    pub fn infor_nexus(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infor_nexus", self.base))
    }

    #[doc = "Get a reference to the value of field `marketo` after provisioning.\n"]
    pub fn marketo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.marketo", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce` after provisioning.\n"]
    pub fn salesforce(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.salesforce", self.base))
    }

    #[doc = "Get a reference to the value of field `sapo_data` after provisioning.\n"]
    pub fn sapo_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sapo_data", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `singular` after provisioning.\n"]
    pub fn singular(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.singular", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slack", self.base))
    }

    #[doc = "Get a reference to the value of field `trendmicro` after provisioning.\n"]
    pub fn trendmicro(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trendmicro", self.base))
    }

    #[doc = "Get a reference to the value of field `veeva` after provisioning.\n"]
    pub fn veeva(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.veeva", self.base))
    }

    #[doc = "Get a reference to the value of field `zendesk` after provisioning.\n"]
    pub fn zendesk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zendesk", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowTaskElDynamic {
    connector_operator: std::option::Option<terrars::DynamicBlock<AppflowFlowTaskElConnectorOperatorEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowTaskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_field: std::option::Option<terrars::PrimField<std::string::String>>,
    source_fields: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    task_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_operator: std::option::Option<std::vec::Vec<AppflowFlowTaskElConnectorOperatorEl>>,
    dynamic: AppflowFlowTaskElDynamic,
}

impl AppflowFlowTaskEl {
    #[doc = "Set the field `destination_field`.\n"]
    pub fn set_destination_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_properties`.\n"]
    pub fn set_task_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.task_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connector_operator`.\n"]
    pub fn set_connector_operator(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlowTaskElConnectorOperatorEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connector_operator = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connector_operator = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowTaskEl {
    type O = terrars::BlockAssignable<AppflowFlowTaskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTaskEl {
    #[doc = ""]
    pub source_fields: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub task_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowTaskEl {
    pub fn build(self) -> AppflowFlowTaskEl {
        AppflowFlowTaskEl {
            destination_field: core::default::Default::default(),
            source_fields: self.source_fields,
            task_properties: core::default::Default::default(),
            task_type: self.task_type,
            connector_operator: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowTaskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowTaskElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlowTaskElRef {
        AppflowFlowTaskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTaskElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_field` after provisioning.\n"]
    pub fn destination_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_field", self.base))
    }

    #[doc = "Get a reference to the value of field `source_fields` after provisioning.\n"]
    pub fn source_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `task_properties` after provisioning.\n"]
    pub fn task_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.task_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `task_type` after provisioning.\n"]
    pub fn task_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.base))
    }

    #[doc = "Get a reference to the value of field `connector_operator` after provisioning.\n"]
    pub fn connector_operator(&self) -> terrars::ListRef<AppflowFlowTaskElConnectorOperatorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_operator", self.base))
    }
}

#[derive(Serialize)]
pub struct AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_pull_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_execution_from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    schedule_expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_offset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    #[doc = "Set the field `data_pull_mode`.\n"]
    pub fn set_data_pull_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_pull_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_execution_from`.\n"]
    pub fn set_first_execution_from(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.first_execution_from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_end_time`.\n"]
    pub fn set_schedule_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule_end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_offset`.\n"]
    pub fn set_schedule_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.schedule_offset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_start_time`.\n"]
    pub fn set_schedule_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schedule_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timezone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    type O = terrars::BlockAssignable<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    #[doc = ""]
    pub schedule_expression: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
    pub fn build(self) -> AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
        AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl {
            data_pull_mode: core::default::Default::default(),
            first_execution_from: core::default::Default::default(),
            schedule_end_time: core::default::Default::default(),
            schedule_expression: self.schedule_expression,
            schedule_offset: core::default::Default::default(),
            schedule_start_time: core::default::Default::default(),
            timezone: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
        AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_pull_mode` after provisioning.\n"]
    pub fn data_pull_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_pull_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `first_execution_from` after provisioning.\n"]
    pub fn first_execution_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_execution_from", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_end_time` after provisioning.\n"]
    pub fn schedule_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_offset` after provisioning.\n"]
    pub fn schedule_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_offset", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_start_time` after provisioning.\n"]
    pub fn schedule_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowTriggerConfigElTriggerPropertiesElDynamic {
    scheduled: std::option::Option<terrars::DynamicBlock<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowTriggerConfigElTriggerPropertiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled: std::option::Option<std::vec::Vec<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl>>,
    dynamic: AppflowFlowTriggerConfigElTriggerPropertiesElDynamic,
}

impl AppflowFlowTriggerConfigElTriggerPropertiesEl {
    #[doc = "Set the field `scheduled`.\n"]
    pub fn set_scheduled(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduled = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduled = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowTriggerConfigElTriggerPropertiesEl {
    type O = terrars::BlockAssignable<AppflowFlowTriggerConfigElTriggerPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTriggerConfigElTriggerPropertiesEl {}

impl BuildAppflowFlowTriggerConfigElTriggerPropertiesEl {
    pub fn build(self) -> AppflowFlowTriggerConfigElTriggerPropertiesEl {
        AppflowFlowTriggerConfigElTriggerPropertiesEl {
            scheduled: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowTriggerConfigElTriggerPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowTriggerConfigElTriggerPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppflowFlowTriggerConfigElTriggerPropertiesElRef {
        AppflowFlowTriggerConfigElTriggerPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTriggerConfigElTriggerPropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scheduled` after provisioning.\n"]
    pub fn scheduled(&self) -> terrars::ListRef<AppflowFlowTriggerConfigElTriggerPropertiesElScheduledElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowTriggerConfigElDynamic {
    trigger_properties: std::option::Option<terrars::DynamicBlock<AppflowFlowTriggerConfigElTriggerPropertiesEl>>,
}

#[derive(Serialize)]
pub struct AppflowFlowTriggerConfigEl {
    trigger_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_properties: std::option::Option<std::vec::Vec<AppflowFlowTriggerConfigElTriggerPropertiesEl>>,
    dynamic: AppflowFlowTriggerConfigElDynamic,
}

impl AppflowFlowTriggerConfigEl {
    #[doc = "Set the field `trigger_properties`.\n"]
    pub fn set_trigger_properties(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppflowFlowTriggerConfigElTriggerPropertiesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trigger_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trigger_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppflowFlowTriggerConfigEl {
    type O = terrars::BlockAssignable<AppflowFlowTriggerConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppflowFlowTriggerConfigEl {
    #[doc = ""]
    pub trigger_type: terrars::PrimField<std::string::String>,
}

impl BuildAppflowFlowTriggerConfigEl {
    pub fn build(self) -> AppflowFlowTriggerConfigEl {
        AppflowFlowTriggerConfigEl {
            trigger_type: self.trigger_type,
            trigger_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppflowFlowTriggerConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppflowFlowTriggerConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppflowFlowTriggerConfigElRef {
        AppflowFlowTriggerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppflowFlowTriggerConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trigger_type` after provisioning.\n"]
    pub fn trigger_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_type", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_properties` after provisioning.\n"]
    pub fn trigger_properties(&self) -> terrars::ListRef<AppflowFlowTriggerConfigElTriggerPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trigger_properties", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppflowFlowDynamic {
    destination_flow_config: std::option::Option<terrars::DynamicBlock<AppflowFlowDestinationFlowConfigEl>>,
    source_flow_config: std::option::Option<terrars::DynamicBlock<AppflowFlowSourceFlowConfigEl>>,
    task: std::option::Option<terrars::DynamicBlock<AppflowFlowTaskEl>>,
    trigger_config: std::option::Option<terrars::DynamicBlock<AppflowFlowTriggerConfigEl>>,
}

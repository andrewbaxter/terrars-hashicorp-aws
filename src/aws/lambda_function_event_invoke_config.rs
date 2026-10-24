use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaFunctionEventInvokeConfigData {
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
    function_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_event_age_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_config: std::option::Option<std::vec::Vec<LambdaFunctionEventInvokeConfigDestinationConfigEl>>,
    dynamic: LambdaFunctionEventInvokeConfigDynamic,
}

struct LambdaFunctionEventInvokeConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaFunctionEventInvokeConfigData>,
}

#[derive(Clone)]
pub struct LambdaFunctionEventInvokeConfig(std::rc::Rc<LambdaFunctionEventInvokeConfig_>);

impl LambdaFunctionEventInvokeConfig {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_event_age_in_seconds`.\n"]
    pub fn set_maximum_event_age_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_event_age_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_retry_attempts`.\n"]
    pub fn set_maximum_retry_attempts(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_retry_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().qualifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_config`.\n"]
    pub fn set_destination_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_event_age_in_seconds` after provisioning.\n"]
    pub fn maximum_event_age_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_event_age_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<LambdaFunctionEventInvokeConfigDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }
}

impl terrars::Referable for LambdaFunctionEventInvokeConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LambdaFunctionEventInvokeConfig { }

impl terrars::ToListMappable for LambdaFunctionEventInvokeConfig {
    type O = terrars::ListRef<LambdaFunctionEventInvokeConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LambdaFunctionEventInvokeConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lambda_function_event_invoke_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaFunctionEventInvokeConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub function_name: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunctionEventInvokeConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> LambdaFunctionEventInvokeConfig {
        let out = LambdaFunctionEventInvokeConfig(std::rc::Rc::new(LambdaFunctionEventInvokeConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LambdaFunctionEventInvokeConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                function_name: self.function_name,
                id: core::default::Default::default(),
                maximum_event_age_in_seconds: core::default::Default::default(),
                maximum_retry_attempts: core::default::Default::default(),
                qualifier: core::default::Default::default(),
                destination_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaFunctionEventInvokeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionEventInvokeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaFunctionEventInvokeConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_event_age_in_seconds` after provisioning.\n"]
    pub fn maximum_event_age_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_event_age_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_retry_attempts` after provisioning.\n"]
    pub fn maximum_retry_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_attempts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<LambdaFunctionEventInvokeConfigDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
    destination: terrars::PrimField<std::string::String>,
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl { }

impl terrars::ToListMappable for LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
    type O = terrars::BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
    #[doc = ""]
    pub destination: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
    pub fn build(self) -> LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl {
        LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl { destination: self.destination }
    }
}

pub struct LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
        LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
    destination: terrars::PrimField<std::string::String>,
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl { }

impl terrars::ToListMappable for LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
    type O = terrars::BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
    #[doc = ""]
    pub destination: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
    pub fn build(self) -> LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl {
        LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl { destination: self.destination }
    }
}

pub struct LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
        LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaFunctionEventInvokeConfigDestinationConfigElDynamic {
    on_failure: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl>>,
    on_success: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl>>,
}

#[derive(Serialize)]
pub struct LambdaFunctionEventInvokeConfigDestinationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_failure: std::option::Option<std::vec::Vec<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_success: std::option::Option<std::vec::Vec<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl>>,
    dynamic: LambdaFunctionEventInvokeConfigDestinationConfigElDynamic,
}

impl LambdaFunctionEventInvokeConfigDestinationConfigEl {
    #[doc = "Set the field `on_failure`.\n"]
    pub fn set_on_failure(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_failure = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_failure = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_success`.\n"]
    pub fn set_on_success(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_success = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_success = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LambdaFunctionEventInvokeConfigDestinationConfigEl {
    type O = terrars::BlockAssignable<LambdaFunctionEventInvokeConfigDestinationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionEventInvokeConfigDestinationConfigEl {}

impl BuildLambdaFunctionEventInvokeConfigDestinationConfigEl {
    pub fn build(self) -> LambdaFunctionEventInvokeConfigDestinationConfigEl {
        LambdaFunctionEventInvokeConfigDestinationConfigEl {
            on_failure: core::default::Default::default(),
            on_success: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LambdaFunctionEventInvokeConfigDestinationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionEventInvokeConfigDestinationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaFunctionEventInvokeConfigDestinationConfigElRef {
        LambdaFunctionEventInvokeConfigDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionEventInvokeConfigDestinationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_failure` after provisioning.\n"]
    pub fn on_failure(&self) -> terrars::ListRef<LambdaFunctionEventInvokeConfigDestinationConfigElOnFailureElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `on_success` after provisioning.\n"]
    pub fn on_success(&self) -> terrars::ListRef<LambdaFunctionEventInvokeConfigDestinationConfigElOnSuccessElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_success", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaFunctionEventInvokeConfigDynamic {
    destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaFunctionEventInvokeConfigDestinationConfigEl>>,
}

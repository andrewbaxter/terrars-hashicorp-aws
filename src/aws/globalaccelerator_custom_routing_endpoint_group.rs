#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlobalacceleratorCustomRoutingEndpointGroupData_ {
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
    endpoint_group_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    listener_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_configuration: std
    ::option
    ::Option<std::vec::Vec<GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_configuration: std
    ::option
    ::Option<std::vec::Vec<GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GlobalacceleratorCustomRoutingEndpointGroup_Timeouts>,
    dynamic: GlobalacceleratorCustomRoutingEndpointGroupDynamic,
}

struct GlobalacceleratorCustomRoutingEndpointGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlobalacceleratorCustomRoutingEndpointGroupData_>,
}

#[derive(Clone)]
pub struct GlobalacceleratorCustomRoutingEndpointGroup(std::rc::Rc<GlobalacceleratorCustomRoutingEndpointGroup_>);

impl GlobalacceleratorCustomRoutingEndpointGroup {
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

    #[doc = "Set the field `endpoint_group_region`.\n"]
    pub fn set_endpoint_group_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().endpoint_group_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_configuration`.\n"]
    pub fn set_destination_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint_configuration`.\n"]
    pub fn set_endpoint_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GlobalacceleratorCustomRoutingEndpointGroup_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_group_region` after provisioning.\n"]
    pub fn endpoint_group_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_group_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorCustomRoutingEndpointGroup_TimeoutsRef {
        GlobalacceleratorCustomRoutingEndpointGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for GlobalacceleratorCustomRoutingEndpointGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlobalacceleratorCustomRoutingEndpointGroup { }

impl terrars::ToListMappable for GlobalacceleratorCustomRoutingEndpointGroup {
    type O = terrars::ListRef<GlobalacceleratorCustomRoutingEndpointGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlobalacceleratorCustomRoutingEndpointGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_globalaccelerator_custom_routing_endpoint_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlobalacceleratorCustomRoutingEndpointGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub listener_arn: terrars::PrimField<std::string::String>,
}

impl BuildGlobalacceleratorCustomRoutingEndpointGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> GlobalacceleratorCustomRoutingEndpointGroup {
        let out =
            GlobalacceleratorCustomRoutingEndpointGroup(
                std::rc::Rc::new(GlobalacceleratorCustomRoutingEndpointGroup_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(GlobalacceleratorCustomRoutingEndpointGroupData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        endpoint_group_region: core::default::Default::default(),
                        id: core::default::Default::default(),
                        listener_arn: self.listener_arn,
                        destination_configuration: core::default::Default::default(),
                        endpoint_configuration: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlobalacceleratorCustomRoutingEndpointGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorCustomRoutingEndpointGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlobalacceleratorCustomRoutingEndpointGroupRef {
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

    #[doc = "Get a reference to the value of field `endpoint_group_region` after provisioning.\n"]
    pub fn endpoint_group_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_group_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_arn` after provisioning.\n"]
    pub fn listener_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GlobalacceleratorCustomRoutingEndpointGroup_TimeoutsRef {
        GlobalacceleratorCustomRoutingEndpointGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct GlobalacceleratorCustomRoutingEndpointGroupDynamic {
    destination_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration>>,
    endpoint_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration>>,
}

#[derive(Serialize)]
pub struct GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration {
    from_port: terrars::PrimField<f64>,
    protocols: terrars::SetField<terrars::PrimField<std::string::String>>,
    to_port: terrars::PrimField<f64>,
}

impl GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration { }

impl terrars::ToListMappable for GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration {
    type O = terrars::BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration {
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub protocols: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub to_port: terrars::PrimField<f64>,
}

impl BuildGlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration {
    pub fn build(self) -> GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration {
        GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfiguration {
            from_port: self.from_port,
            protocols: self.protocols,
            to_port: self.to_port,
        }
    }
}

#[allow(dead_code)]
pub struct GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfigurationRef {
        GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorCustomRoutingEndpointGroup_DestinationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration {
    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration {
    type O = terrars::BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration {}

impl BuildGlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration {
    pub fn build(self) -> GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration {
        GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfiguration {
            endpoint_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfigurationRef {
        GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorCustomRoutingEndpointGroup_EndpointConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }
}

#[derive(Serialize)]
pub struct GlobalacceleratorCustomRoutingEndpointGroup_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlobalacceleratorCustomRoutingEndpointGroup_Timeouts {
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
}

impl terrars::ToListMappable for GlobalacceleratorCustomRoutingEndpointGroup_Timeouts {
    type O = terrars::BlockAssignable<GlobalacceleratorCustomRoutingEndpointGroup_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlobalacceleratorCustomRoutingEndpointGroup_Timeouts {}

impl BuildGlobalacceleratorCustomRoutingEndpointGroup_Timeouts {
    pub fn build(self) -> GlobalacceleratorCustomRoutingEndpointGroup_Timeouts {
        GlobalacceleratorCustomRoutingEndpointGroup_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlobalacceleratorCustomRoutingEndpointGroup_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlobalacceleratorCustomRoutingEndpointGroup_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlobalacceleratorCustomRoutingEndpointGroup_TimeoutsRef {
        GlobalacceleratorCustomRoutingEndpointGroup_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlobalacceleratorCustomRoutingEndpointGroup_TimeoutsRef {
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
}

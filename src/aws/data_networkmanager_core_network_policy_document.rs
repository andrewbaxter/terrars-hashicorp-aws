#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNetworkmanagerCoreNetworkPolicyDocumentData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment_policies: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment_routing_policy_rules: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_network_configuration: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_function_groups: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_policies: std::option::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_actions: std::option::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segments: std::option::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_Segments>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocumentDynamic,
}

struct DataNetworkmanagerCoreNetworkPolicyDocument_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkmanagerCoreNetworkPolicyDocumentData_>,
}

#[derive(Clone)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument(std::rc::Rc<DataNetworkmanagerCoreNetworkPolicyDocument_>);

impl DataNetworkmanagerCoreNetworkPolicyDocument {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attachment_policies`.\n"]
    pub fn set_attachment_policies(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attachment_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attachment_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `attachment_routing_policy_rules`.\n"]
    pub fn set_attachment_routing_policy_rules(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attachment_routing_policy_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attachment_routing_policy_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `core_network_configuration`.\n"]
    pub fn set_core_network_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().core_network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.core_network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_function_groups`.\n"]
    pub fn set_network_function_groups(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_function_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_function_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routing_policies`.\n"]
    pub fn set_routing_policies(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `segment_actions`.\n"]
    pub fn set_segment_actions(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().segment_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.segment_actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `segments`.\n"]
    pub fn set_segments(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_Segments>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().segments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.segments = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_policies` after provisioning.\n"]
    pub fn attachment_policies(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_routing_policy_rules` after provisioning.\n"]
    pub fn attachment_routing_policy_rules(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRulesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.attachment_routing_policy_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `core_network_configuration` after provisioning.\n"]
    pub fn core_network_configuration(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_function_groups` after provisioning.\n"]
    pub fn network_function_groups(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_function_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_policies` after provisioning.\n"]
    pub fn routing_policies(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segment_actions` after provisioning.\n"]
    pub fn segment_actions(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segment_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segments", self.extract_ref()))
    }
}

impl terrars::Referable for DataNetworkmanagerCoreNetworkPolicyDocument {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNetworkmanagerCoreNetworkPolicyDocument { }

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument {
    type O = terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocumentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNetworkmanagerCoreNetworkPolicyDocument_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_networkmanager_core_network_policy_document".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument {
    pub tf_id: std::string::String,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNetworkmanagerCoreNetworkPolicyDocument {
        let out =
            DataNetworkmanagerCoreNetworkPolicyDocument(
                std::rc::Rc::new(DataNetworkmanagerCoreNetworkPolicyDocument_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataNetworkmanagerCoreNetworkPolicyDocumentData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        id: core::default::Default::default(),
                        version: core::default::Default::default(),
                        attachment_policies: core::default::Default::default(),
                        attachment_routing_policy_rules: core::default::Default::default(),
                        core_network_configuration: core::default::Default::default(),
                        network_function_groups: core::default::Default::default(),
                        routing_policies: core::default::Default::default(),
                        segment_actions: core::default::Default::default(),
                        segments: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkmanagerCoreNetworkPolicyDocumentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocumentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocumentRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_policies` after provisioning.\n"]
    pub fn attachment_policies(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_routing_policy_rules` after provisioning.\n"]
    pub fn attachment_routing_policy_rules(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRulesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.attachment_routing_policy_rules", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `core_network_configuration` after provisioning.\n"]
    pub fn core_network_configuration(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_function_groups` after provisioning.\n"]
    pub fn network_function_groups(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_function_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_policies` after provisioning.\n"]
    pub fn routing_policies(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segment_actions` after provisioning.\n"]
    pub fn segment_actions(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segment_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segments", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocumentDynamic {
    attachment_policies: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies>>,
    attachment_routing_policy_rules: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules>>,
    core_network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration>>,
    network_function_groups: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups>>,
    routing_policies: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies>>,
    segment_actions: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions>>,
    segments: std::option::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_Segments>>,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_to_network_function_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    association_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_acceptance: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_value_of_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action {
    #[doc = "Set the field `add_to_network_function_group`.\n"]
    pub fn set_add_to_network_function_group(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.add_to_network_function_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `association_method`.\n"]
    pub fn set_association_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.association_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_acceptance`.\n"]
    pub fn set_require_acceptance(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_acceptance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segment`.\n"]
    pub fn set_segment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.segment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_value_of_key`.\n"]
    pub fn set_tag_value_of_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_value_of_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action {}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action {
            add_to_network_function_group: core::default::Default::default(),
            association_method: core::default::Default::default(),
            require_acceptance: core::default::Default::default(),
            segment: core::default::Default::default(),
            tag_value_of_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ActionRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add_to_network_function_group` after provisioning.\n"]
    pub fn add_to_network_function_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.add_to_network_function_group", self.base))
    }

    #[doc = "Get a reference to the value of field `association_method` after provisioning.\n"]
    pub fn association_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_method", self.base))
    }

    #[doc = "Get a reference to the value of field `require_acceptance` after provisioning.\n"]
    pub fn require_acceptance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_acceptance", self.base))
    }

    #[doc = "Get a reference to the value of field `segment` after provisioning.\n"]
    pub fn segment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_value_of_key` after provisioning.\n"]
    pub fn tag_value_of_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_value_of_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            type_: self.type_,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ConditionsRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
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
struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPoliciesDynamic {
    action: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action>>,
    conditions: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions>>,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_logic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPoliciesDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies {
    #[doc = "Set the field `condition_logic`.\n"]
    pub fn set_condition_logic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.condition_logic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Action>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_Conditions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies {
    #[doc = ""]
    pub rule_number: terrars::PrimField<f64>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies {
            condition_logic: core::default::Default::default(),
            description: core::default::Default::default(),
            rule_number: self.rule_number,
            action: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPoliciesRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_logic` after provisioning.\n"]
    pub fn condition_logic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition_logic", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentPolicies_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action {
    associate_routing_policies: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action { }

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action {
    type O =
        terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action {
    #[doc = ""]
    pub associate_routing_policies: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action {
            associate_routing_policies: self.associate_routing_policies,
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ActionRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `associate_routing_policies` after provisioning.\n"]
    pub fn associate_routing_policies(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.associate_routing_policies", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions { }

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions {
    type O =
        terrars
        ::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ConditionsRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ConditionsRef {
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
struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRulesDynamic {
    action: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action>>,
    conditions: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions>,
    >,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    rule_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRulesDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_locations`.\n"]
    pub fn set_edge_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.edge_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Action,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_Conditions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules {
    #[doc = ""]
    pub rule_number: terrars::PrimField<f64>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules {
            description: core::default::Default::default(),
            edge_locations: core::default::Default::default(),
            rule_number: self.rule_number,
            action: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRulesRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.edge_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_AttachmentRoutingPolicyRules_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inside_cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations {
    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inside_cidr_blocks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations {
    type O =
        terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations {
    #[doc = ""]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations {
        DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations {
            asn: core::default::Default::default(),
            inside_cidr_blocks: core::default::Default::default(),
            location: self.location,
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocationsRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfigurationDynamic {
    edge_locations: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations>,
    >,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration {
    asn_ranges: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inside_cidr_blocks: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_referencing_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_ecmp_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_locations: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfigurationDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration {
    #[doc = "Set the field `dns_support`.\n"]
    pub fn set_dns_support(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.dns_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inside_cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_referencing_support`.\n"]
    pub fn set_security_group_referencing_support(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.security_group_referencing_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_ecmp_support`.\n"]
    pub fn set_vpn_ecmp_support(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.vpn_ecmp_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_locations`.\n"]
    pub fn set_edge_locations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocations,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.edge_locations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.edge_locations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration {
    #[doc = ""]
    pub asn_ranges: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration {
        DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration {
            asn_ranges: self.asn_ranges,
            dns_support: core::default::Default::default(),
            inside_cidr_blocks: core::default::Default::default(),
            security_group_referencing_support: core::default::Default::default(),
            vpn_ecmp_support: core::default::Default::default(),
            edge_locations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfigurationRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn_ranges` after provisioning.\n"]
    pub fn asn_ranges(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.asn_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_support` after provisioning.\n"]
    pub fn dns_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_support", self.base))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_referencing_support` after provisioning.\n"]
    pub fn security_group_referencing_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_referencing_support", self.base))
    }

    #[doc = "Get a reference to the value of field `vpn_ecmp_support` after provisioning.\n"]
    pub fn vpn_ecmp_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_ecmp_support", self.base))
    }

    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_CoreNetworkConfiguration_EdgeLocationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    require_attachment_acceptance: terrars::PrimField<bool>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub require_attachment_acceptance: terrars::PrimField<bool>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups {
        DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroups {
            description: core::default::Default::default(),
            name: self.name,
            require_attachment_acceptance: self.require_attachment_acceptance,
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroupsRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_NetworkFunctionGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `require_attachment_acceptance` after provisioning.\n"]
    pub fn require_attachment_acceptance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_attachment_acceptance", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action {
    type O =
        terrars
        ::BlockAssignable<
            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action {
    pub fn build(
        self,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action {
            type_: self.type_,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_ActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_ActionRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_ActionRef {
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

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions { }

impl terrars
::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions {
    type O =
        terrars
        ::BlockAssignable<
            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions {
    pub fn build(
        self,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditionsRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditionsRef {
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
struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinitionDynamic {
    action: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action,
        >,
    >,
    match_conditions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_logic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_conditions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions,
        >,
    >,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinitionDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition {
    #[doc = "Set the field `condition_logic`.\n"]
    pub fn set_condition_logic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.condition_logic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_Action,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_conditions`.\n"]
    pub fn set_match_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition {
    type O =
        terrars
        ::BlockAssignable<
            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition {}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition {
    pub fn build(
        self,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition {
            condition_logic: core::default::Default::default(),
            action: core::default::Default::default(),
            match_conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinitionRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_logic` after provisioning.\n"]
    pub fn condition_logic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition_logic", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(
        &self,
    ) -> terrars
    ::ListRef<
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_ActionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_conditions` after provisioning.\n"]
    pub fn match_conditions(
        &self,
    ) -> terrars
    ::ListRef<
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition_MatchConditionsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_conditions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRulesDynamic {
    rule_definition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition>,
    >,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules {
    rule_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_definition: std
    ::option
    ::Option<
        std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition>,
    >,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRulesDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules {
    #[doc = "Set the field `rule_definition`.\n"]
    pub fn set_rule_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules {
    type O =
        terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules {
    #[doc = ""]
    pub rule_number: terrars::PrimField<f64>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules {
            rule_number: self.rule_number,
            rule_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRulesRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_definition` after provisioning.\n"]
    pub fn rule_definition(
        &self,
    ) -> terrars
    ::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules_RuleDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_definition", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPoliciesDynamic {
    routing_policy_rules: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules>>,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_policy_description: std::option::Option<terrars::PrimField<std::string::String>>,
    routing_policy_direction: terrars::PrimField<std::string::String>,
    routing_policy_name: terrars::PrimField<std::string::String>,
    routing_policy_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_policy_rules: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPoliciesDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies {
    #[doc = "Set the field `routing_policy_description`.\n"]
    pub fn set_routing_policy_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.routing_policy_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_policy_rules`.\n"]
    pub fn set_routing_policy_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRules,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.routing_policy_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.routing_policy_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies {
    #[doc = ""]
    pub routing_policy_direction: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub routing_policy_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub routing_policy_number: terrars::PrimField<f64>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies {
            routing_policy_description: core::default::Default::default(),
            routing_policy_direction: self.routing_policy_direction,
            routing_policy_name: self.routing_policy_name,
            routing_policy_number: self.routing_policy_number,
            routing_policy_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPoliciesRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `routing_policy_description` after provisioning.\n"]
    pub fn routing_policy_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_policy_description", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy_direction` after provisioning.\n"]
    pub fn routing_policy_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_policy_direction", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy_name` after provisioning.\n"]
    pub fn routing_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_policy_name", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy_number` after provisioning.\n"]
    pub fn routing_policy_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_policy_number", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy_rules` after provisioning.\n"]
    pub fn routing_policy_rules(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_RoutingPolicies_RoutingPolicyRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_policy_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation {
    edge_location: terrars::PrimField<std::string::String>,
    peer_edge_location: terrars::PrimField<std::string::String>,
    routing_policy_names: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation { }

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation {
    type O =
        terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation {
    #[doc = ""]
    pub edge_location: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub peer_edge_location: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub routing_policy_names: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation {
            edge_location: self.edge_location,
            peer_edge_location: self.peer_edge_location,
            routing_policy_names: self.routing_policy_names,
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociationRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_edge_location` after provisioning.\n"]
    pub fn peer_edge_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_edge_location", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy_names` after provisioning.\n"]
    pub fn routing_policy_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.routing_policy_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_sets: std::option::Option<terrars::SetField<terrars::SetField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_edge: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_edge_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride {
    #[doc = "Set the field `edge_sets`.\n"]
    pub fn set_edge_sets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::SetField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.edge_sets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_edge`.\n"]
    pub fn set_use_edge(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.use_edge = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_edge_location`.\n"]
    pub fn set_use_edge_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.use_edge_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride {
    type O =
        terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride {}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride {
            edge_sets: core::default::Default::default(),
            use_edge: core::default::Default::default(),
            use_edge_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverrideRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `edge_sets` after provisioning.\n"]
    pub fn edge_sets(&self) -> terrars::SetRef<terrars::SetRef<terrars::PrimExpr<std::string::String>>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.edge_sets", self.base))
    }

    #[doc = "Get a reference to the value of field `use_edge` after provisioning.\n"]
    pub fn use_edge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_edge", self.base))
    }

    #[doc = "Get a reference to the value of field `use_edge_location` after provisioning.\n"]
    pub fn use_edge_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_edge_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_ViaDynamic {
    with_edge_override: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride>>,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_function_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_edge_override: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_ViaDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via {
    #[doc = "Set the field `network_function_groups`.\n"]
    pub fn set_network_function_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.network_function_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_edge_override`.\n"]
    pub fn set_with_edge_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverride,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.with_edge_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.with_edge_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via {}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via {
            network_function_groups: core::default::Default::default(),
            with_edge_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_ViaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_ViaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_ViaRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_ViaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_ViaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_function_groups` after provisioning.\n"]
    pub fn network_function_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.network_function_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `with_edge_override` after provisioning.\n"]
    pub fn with_edge_override(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via_WithEdgeOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.with_edge_override", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segments: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo {
    #[doc = "Set the field `segments`.\n"]
    pub fn set_segments(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.segments = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo {}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo {
            segments: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentToRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentToRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentToRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentToRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentToRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.segments", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActionsDynamic {
    edge_location_association: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation>,
    >,
    via: std::option::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via>>,
    when_sent_to: std
    ::option
    ::Option<terrars::DynamicBlock<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo>>,
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_cidr_blocks: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_policy_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    segment: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_with: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_with_except: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_location_association: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    via: std::option::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    when_sent_to: std
    ::option
    ::Option<std::vec::Vec<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo>>,
    dynamic: DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActionsDynamic,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_cidr_blocks`.\n"]
    pub fn set_destination_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destinations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routing_policy_names`.\n"]
    pub fn set_routing_policy_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.routing_policy_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_with`.\n"]
    pub fn set_share_with(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.share_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_with_except`.\n"]
    pub fn set_share_with_except(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.share_with_except = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_location_association`.\n"]
    pub fn set_edge_location_association(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.edge_location_association = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.edge_location_association = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `via`.\n"]
    pub fn set_via(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_Via>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.via = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.via = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `when_sent_to`.\n"]
    pub fn set_when_sent_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentTo>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.when_sent_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.when_sent_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub segment: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions {
            action: self.action,
            description: core::default::Default::default(),
            destination_cidr_blocks: core::default::Default::default(),
            destinations: core::default::Default::default(),
            mode: core::default::Default::default(),
            routing_policy_names: core::default::Default::default(),
            segment: self.segment,
            share_with: core::default::Default::default(),
            share_with_except: core::default::Default::default(),
            edge_location_association: core::default::Default::default(),
            via: core::default::Default::default(),
            when_sent_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActionsRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_cidr_blocks` after provisioning.\n"]
    pub fn destination_cidr_blocks(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.destination_cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy_names` after provisioning.\n"]
    pub fn routing_policy_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.routing_policy_names", self.base))
    }

    #[doc = "Get a reference to the value of field `segment` after provisioning.\n"]
    pub fn segment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment", self.base))
    }

    #[doc = "Get a reference to the value of field `share_with` after provisioning.\n"]
    pub fn share_with(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.share_with", self.base))
    }

    #[doc = "Get a reference to the value of field `share_with_except` after provisioning.\n"]
    pub fn share_with_except(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.share_with_except", self.base))
    }

    #[doc = "Get a reference to the value of field `edge_location_association` after provisioning.\n"]
    pub fn edge_location_association(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_EdgeLocationAssociationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_location_association", self.base))
    }

    #[doc = "Get a reference to the value of field `via` after provisioning.\n"]
    pub fn via(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_ViaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.via", self.base))
    }

    #[doc = "Get a reference to the value of field `when_sent_to` after provisioning.\n"]
    pub fn when_sent_to(
        &self,
    ) -> terrars::ListRef<DataNetworkmanagerCoreNetworkPolicyDocument_SegmentActions_WhenSentToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.when_sent_to", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_Segments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_filter: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_filter: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    isolate_attachments: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_attachment_acceptance: std::option::Option<terrars::PrimField<bool>>,
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_Segments {
    #[doc = "Set the field `allow_filter`.\n"]
    pub fn set_allow_filter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deny_filter`.\n"]
    pub fn set_deny_filter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.deny_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_locations`.\n"]
    pub fn set_edge_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.edge_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `isolate_attachments`.\n"]
    pub fn set_isolate_attachments(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.isolate_attachments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_attachment_acceptance`.\n"]
    pub fn set_require_attachment_acceptance(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_attachment_acceptance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetworkPolicyDocument_Segments {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetworkPolicyDocument_Segments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetworkPolicyDocument_Segments {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkmanagerCoreNetworkPolicyDocument_Segments {
    pub fn build(self) -> DataNetworkmanagerCoreNetworkPolicyDocument_Segments {
        DataNetworkmanagerCoreNetworkPolicyDocument_Segments {
            allow_filter: core::default::Default::default(),
            deny_filter: core::default::Default::default(),
            description: core::default::Default::default(),
            edge_locations: core::default::Default::default(),
            isolate_attachments: core::default::Default::default(),
            name: self.name,
            require_attachment_acceptance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetworkPolicyDocument_SegmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkPolicyDocument_SegmentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetworkPolicyDocument_SegmentsRef {
        DataNetworkmanagerCoreNetworkPolicyDocument_SegmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetworkPolicyDocument_SegmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_filter` after provisioning.\n"]
    pub fn allow_filter(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `deny_filter` after provisioning.\n"]
    pub fn deny_filter(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.deny_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.edge_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `isolate_attachments` after provisioning.\n"]
    pub fn isolate_attachments(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.isolate_attachments", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `require_attachment_acceptance` after provisioning.\n"]
    pub fn require_attachment_acceptance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_attachment_acceptance", self.base))
    }
}

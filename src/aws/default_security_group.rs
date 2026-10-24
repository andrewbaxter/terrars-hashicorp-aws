use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DefaultSecurityGroupData {
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
    egress: std::option::Option<terrars::SetField<DefaultSecurityGroupEgressEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<terrars::SetField<DefaultSecurityGroupIngressEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoke_rules_on_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DefaultSecurityGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DefaultSecurityGroupData>,
}

#[derive(Clone)]
pub struct DefaultSecurityGroup(std::rc::Rc<DefaultSecurityGroup_>);

impl DefaultSecurityGroup {
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

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(self, v: impl std::convert::Into<terrars::SetField<DefaultSecurityGroupEgressEl>>) -> Self {
        self.0.data.borrow_mut().egress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(self, v: impl std::convert::Into<terrars::SetField<DefaultSecurityGroupIngressEl>>) -> Self {
        self.0.data.borrow_mut().ingress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revoke_rules_on_delete`.\n"]
    pub fn set_revoke_rules_on_delete(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().revoke_rules_on_delete = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::SetRef<DefaultSecurityGroupEgressElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.egress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> terrars::SetRef<DefaultSecurityGroupIngressElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revoke_rules_on_delete` after provisioning.\n"]
    pub fn revoke_rules_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revoke_rules_on_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

impl terrars::Referable for DefaultSecurityGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DefaultSecurityGroup { }

impl terrars::ToListMappable for DefaultSecurityGroup {
    type O = terrars::ListRef<DefaultSecurityGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DefaultSecurityGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_default_security_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDefaultSecurityGroup {
    pub tf_id: std::string::String,
}

impl BuildDefaultSecurityGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DefaultSecurityGroup {
        let out = DefaultSecurityGroup(std::rc::Rc::new(DefaultSecurityGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DefaultSecurityGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                egress: core::default::Default::default(),
                id: core::default::Default::default(),
                ingress: core::default::Default::default(),
                revoke_rules_on_delete: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DefaultSecurityGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DefaultSecurityGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DefaultSecurityGroupRef {
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

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::SetRef<DefaultSecurityGroupEgressElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.egress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> terrars::SetRef<DefaultSecurityGroupIngressElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revoke_rules_on_delete` after provisioning.\n"]
    pub fn revoke_rules_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revoke_rules_on_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DefaultSecurityGroupEgressEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "self", skip_serializing_if = "std::option::Option::is_none")]
    self_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DefaultSecurityGroupEgressEl {
    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_blocks`.\n"]
    pub fn set_ipv6_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv6_cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_ids`.\n"]
    pub fn set_prefix_list_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.prefix_list_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `self_`.\n"]
    pub fn set_self(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.self_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DefaultSecurityGroupEgressEl {
    type O = terrars::BlockAssignable<DefaultSecurityGroupEgressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDefaultSecurityGroupEgressEl {}

impl BuildDefaultSecurityGroupEgressEl {
    pub fn build(self) -> DefaultSecurityGroupEgressEl {
        DefaultSecurityGroupEgressEl {
            cidr_blocks: core::default::Default::default(),
            description: core::default::Default::default(),
            from_port: core::default::Default::default(),
            ipv6_cidr_blocks: core::default::Default::default(),
            prefix_list_ids: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            self_: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct DefaultSecurityGroupEgressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DefaultSecurityGroupEgressElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DefaultSecurityGroupEgressElRef {
        DefaultSecurityGroupEgressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DefaultSecurityGroupEgressElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_blocks` after provisioning.\n"]
    pub fn ipv6_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_ids` after provisioning.\n"]
    pub fn prefix_list_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_list_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `self_` after provisioning.\n"]
    pub fn self_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DefaultSecurityGroupIngressEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "self", skip_serializing_if = "std::option::Option::is_none")]
    self_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DefaultSecurityGroupIngressEl {
    #[doc = "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_blocks`.\n"]
    pub fn set_ipv6_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv6_cidr_blocks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_ids`.\n"]
    pub fn set_prefix_list_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.prefix_list_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `self_`.\n"]
    pub fn set_self(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.self_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DefaultSecurityGroupIngressEl {
    type O = terrars::BlockAssignable<DefaultSecurityGroupIngressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDefaultSecurityGroupIngressEl {}

impl BuildDefaultSecurityGroupIngressEl {
    pub fn build(self) -> DefaultSecurityGroupIngressEl {
        DefaultSecurityGroupIngressEl {
            cidr_blocks: core::default::Default::default(),
            description: core::default::Default::default(),
            from_port: core::default::Default::default(),
            ipv6_cidr_blocks: core::default::Default::default(),
            prefix_list_ids: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            self_: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct DefaultSecurityGroupIngressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DefaultSecurityGroupIngressElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DefaultSecurityGroupIngressElRef {
        DefaultSecurityGroupIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DefaultSecurityGroupIngressElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_blocks` after provisioning.\n"]
    pub fn ipv6_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_blocks", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_ids` after provisioning.\n"]
    pub fn prefix_list_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_list_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `self_` after provisioning.\n"]
    pub fn self_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

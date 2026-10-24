use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DefaultNetworkAclData {
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
    default_network_acl_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<std::vec::Vec<DefaultNetworkAclEgressEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<std::vec::Vec<DefaultNetworkAclIngressEl>>,
    dynamic: DefaultNetworkAclDynamic,
}

struct DefaultNetworkAcl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DefaultNetworkAclData>,
}

#[derive(Clone)]
pub struct DefaultNetworkAcl(std::rc::Rc<DefaultNetworkAcl_>);

impl DefaultNetworkAcl {
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

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().subnet_ids = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(self, v: impl std::convert::Into<terrars::BlockAssignable<DefaultNetworkAclEgressEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().egress = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.egress = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(self, v: impl std::convert::Into<terrars::BlockAssignable<DefaultNetworkAclIngressEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_network_acl_id` after provisioning.\n"]
    pub fn default_network_acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_network_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
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

impl terrars::Referable for DefaultNetworkAcl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DefaultNetworkAcl { }

impl terrars::ToListMappable for DefaultNetworkAcl {
    type O = terrars::ListRef<DefaultNetworkAclRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DefaultNetworkAcl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_default_network_acl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDefaultNetworkAcl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub default_network_acl_id: terrars::PrimField<std::string::String>,
}

impl BuildDefaultNetworkAcl {
    pub fn build(self, stack: &mut terrars::Stack) -> DefaultNetworkAcl {
        let out = DefaultNetworkAcl(std::rc::Rc::new(DefaultNetworkAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DefaultNetworkAclData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_network_acl_id: self.default_network_acl_id,
                id: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                egress: core::default::Default::default(),
                ingress: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DefaultNetworkAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DefaultNetworkAclRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DefaultNetworkAclRef {
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

    #[doc = "Get a reference to the value of field `default_network_acl_id` after provisioning.\n"]
    pub fn default_network_acl_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_network_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
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
pub struct DefaultNetworkAclEgressEl {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    from_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icmp_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icmp_type: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    protocol: terrars::PrimField<std::string::String>,
    rule_no: terrars::PrimField<f64>,
    to_port: terrars::PrimField<f64>,
}

impl DefaultNetworkAclEgressEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_code`.\n"]
    pub fn set_icmp_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.icmp_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_type`.\n"]
    pub fn set_icmp_type(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.icmp_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DefaultNetworkAclEgressEl {
    type O = terrars::BlockAssignable<DefaultNetworkAclEgressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDefaultNetworkAclEgressEl {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_no: terrars::PrimField<f64>,
    #[doc = ""]
    pub to_port: terrars::PrimField<f64>,
}

impl BuildDefaultNetworkAclEgressEl {
    pub fn build(self) -> DefaultNetworkAclEgressEl {
        DefaultNetworkAclEgressEl {
            action: self.action,
            cidr_block: core::default::Default::default(),
            from_port: self.from_port,
            icmp_code: core::default::Default::default(),
            icmp_type: core::default::Default::default(),
            ipv6_cidr_block: core::default::Default::default(),
            protocol: self.protocol,
            rule_no: self.rule_no,
            to_port: self.to_port,
        }
    }
}

pub struct DefaultNetworkAclEgressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DefaultNetworkAclEgressElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DefaultNetworkAclEgressElRef {
        DefaultNetworkAclEgressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DefaultNetworkAclEgressElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_code` after provisioning.\n"]
    pub fn icmp_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icmp_code", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type` after provisioning.\n"]
    pub fn icmp_type(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icmp_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_no` after provisioning.\n"]
    pub fn rule_no(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_no", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DefaultNetworkAclIngressEl {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    from_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icmp_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icmp_type: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    protocol: terrars::PrimField<std::string::String>,
    rule_no: terrars::PrimField<f64>,
    to_port: terrars::PrimField<f64>,
}

impl DefaultNetworkAclIngressEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_code`.\n"]
    pub fn set_icmp_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.icmp_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_type`.\n"]
    pub fn set_icmp_type(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.icmp_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DefaultNetworkAclIngressEl {
    type O = terrars::BlockAssignable<DefaultNetworkAclIngressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDefaultNetworkAclIngressEl {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_no: terrars::PrimField<f64>,
    #[doc = ""]
    pub to_port: terrars::PrimField<f64>,
}

impl BuildDefaultNetworkAclIngressEl {
    pub fn build(self) -> DefaultNetworkAclIngressEl {
        DefaultNetworkAclIngressEl {
            action: self.action,
            cidr_block: core::default::Default::default(),
            from_port: self.from_port,
            icmp_code: core::default::Default::default(),
            icmp_type: core::default::Default::default(),
            ipv6_cidr_block: core::default::Default::default(),
            protocol: self.protocol,
            rule_no: self.rule_no,
            to_port: self.to_port,
        }
    }
}

pub struct DefaultNetworkAclIngressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DefaultNetworkAclIngressElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DefaultNetworkAclIngressElRef {
        DefaultNetworkAclIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DefaultNetworkAclIngressElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_code` after provisioning.\n"]
    pub fn icmp_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icmp_code", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type` after provisioning.\n"]
    pub fn icmp_type(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icmp_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_no` after provisioning.\n"]
    pub fn rule_no(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_no", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct DefaultNetworkAclDynamic {
    egress: std::option::Option<terrars::DynamicBlock<DefaultNetworkAclEgressEl>>,
    ingress: std::option::Option<terrars::DynamicBlock<DefaultNetworkAclIngressEl>>,
}

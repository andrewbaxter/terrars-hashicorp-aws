use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecurityGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<SetField<SecurityGroupEgressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress: Option<SetField<SecurityGroupIngressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoke_rules_on_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecurityGroupTimeoutsEl>,
}

struct SecurityGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecurityGroupData>,
}

#[derive(Clone)]
pub struct SecurityGroup(Rc<SecurityGroup_>);

impl SecurityGroup {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `egress`.\n"]
    pub fn set_egress(self, v: impl Into<SetField<SecurityGroupEgressEl>>) -> Self {
        self.0.data.borrow_mut().egress = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ingress`.\n"]
    pub fn set_ingress(self, v: impl Into<SetField<SecurityGroupIngressEl>>) -> Self {
        self.0.data.borrow_mut().ingress = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `revoke_rules_on_delete`.\n"]
    pub fn set_revoke_rules_on_delete(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().revoke_rules_on_delete = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecurityGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> SetRef<SecurityGroupEgressElRef> {
        SetRef::new(self.shared().clone(), format!("{}.egress", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> SetRef<SecurityGroupIngressElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revoke_rules_on_delete` after provisioning.\n"]
    pub fn revoke_rules_on_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoke_rules_on_delete", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecurityGroupTimeoutsElRef {
        SecurityGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for SecurityGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for SecurityGroup {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for SecurityGroup {
    type O = ListRef<SecurityGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for SecurityGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_security_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityGroup {
    pub tf_id: String,
}

impl BuildSecurityGroup {
    pub fn build(self, stack: &mut Stack) -> SecurityGroup {
        let out = SecurityGroup(Rc::new(SecurityGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecurityGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                egress: core::default::Default::default(),
                id: core::default::Default::default(),
                ingress: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                revoke_rules_on_delete: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecurityGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> SetRef<SecurityGroupEgressElRef> {
        SetRef::new(self.shared().clone(), format!("{}.egress", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> SetRef<SecurityGroupIngressElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revoke_rules_on_delete` after provisioning.\n"]
    pub fn revoke_rules_on_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.revoke_rules_on_delete", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecurityGroupTimeoutsElRef {
        SecurityGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SecurityGroupEgressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_blocks: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_blocks: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    self_: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl SecurityGroupEgressEl {
    #[doc= "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.cidr_blocks = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_cidr_blocks`.\n"]
    pub fn set_ipv6_cidr_blocks(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ipv6_cidr_blocks = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix_list_ids`.\n"]
    pub fn set_prefix_list_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.prefix_list_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `self_`.\n"]
    pub fn set_self(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.self_ = Some(v.into());
        self
    }

    #[doc= "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for SecurityGroupEgressEl {
    type O = BlockAssignable<SecurityGroupEgressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityGroupEgressEl {}

impl BuildSecurityGroupEgressEl {
    pub fn build(self) -> SecurityGroupEgressEl {
        SecurityGroupEgressEl {
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

pub struct SecurityGroupEgressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityGroupEgressElRef {
    fn new(shared: StackShared, base: String) -> SecurityGroupEgressElRef {
        SecurityGroupEgressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityGroupEgressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_cidr_blocks` after provisioning.\n"]
    pub fn ipv6_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_blocks", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix_list_ids` after provisioning.\n"]
    pub fn prefix_list_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.prefix_list_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `self_` after provisioning.\n"]
    pub fn self_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.self", self.base))
    }

    #[doc= "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityGroupIngressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_blocks: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_blocks: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_list_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_groups: Option<SetField<PrimField<String>>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    self_: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl SecurityGroupIngressEl {
    #[doc= "Set the field `cidr_blocks`.\n"]
    pub fn set_cidr_blocks(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.cidr_blocks = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_cidr_blocks`.\n"]
    pub fn set_ipv6_cidr_blocks(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ipv6_cidr_blocks = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix_list_ids`.\n"]
    pub fn set_prefix_list_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.prefix_list_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `security_groups`.\n"]
    pub fn set_security_groups(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `self_`.\n"]
    pub fn set_self(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.self_ = Some(v.into());
        self
    }

    #[doc= "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for SecurityGroupIngressEl {
    type O = BlockAssignable<SecurityGroupIngressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityGroupIngressEl {}

impl BuildSecurityGroupIngressEl {
    pub fn build(self) -> SecurityGroupIngressEl {
        SecurityGroupIngressEl {
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

pub struct SecurityGroupIngressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityGroupIngressElRef {
    fn new(shared: StackShared, base: String) -> SecurityGroupIngressElRef {
        SecurityGroupIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityGroupIngressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cidr_blocks` after provisioning.\n"]
    pub fn cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.cidr_blocks", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_cidr_blocks` after provisioning.\n"]
    pub fn ipv6_cidr_blocks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_cidr_blocks", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix_list_ids` after provisioning.\n"]
    pub fn prefix_list_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.prefix_list_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `self_` after provisioning.\n"]
    pub fn self_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.self", self.base))
    }

    #[doc= "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl SecurityGroupTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for SecurityGroupTimeoutsEl {
    type O = BlockAssignable<SecurityGroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityGroupTimeoutsEl {}

impl BuildSecurityGroupTimeoutsEl {
    pub fn build(self) -> SecurityGroupTimeoutsEl {
        SecurityGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct SecurityGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecurityGroupTimeoutsElRef {
        SecurityGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityGroupTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

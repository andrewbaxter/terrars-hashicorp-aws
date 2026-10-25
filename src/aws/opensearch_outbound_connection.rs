#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpensearchOutboundConnectionData_ {
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
    accept_connection: std::option::Option<terrars::PrimField<bool>>,
    connection_alias: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_properties: std::option::Option<std::vec::Vec<OpensearchOutboundConnection_ConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_domain_info: std::option::Option<std::vec::Vec<OpensearchOutboundConnection_LocalDomainInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_domain_info: std::option::Option<std::vec::Vec<OpensearchOutboundConnection_RemoteDomainInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OpensearchOutboundConnection_Timeouts>,
    dynamic: OpensearchOutboundConnectionDynamic,
}

struct OpensearchOutboundConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpensearchOutboundConnectionData_>,
}

#[derive(Clone)]
pub struct OpensearchOutboundConnection(std::rc::Rc<OpensearchOutboundConnection_>);

impl OpensearchOutboundConnection {
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

    #[doc = "Set the field `accept_connection`.\n"]
    pub fn set_accept_connection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().accept_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_mode`.\n"]
    pub fn set_connection_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().connection_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_properties`.\n"]
    pub fn set_connection_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchOutboundConnection_ConnectionProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `local_domain_info`.\n"]
    pub fn set_local_domain_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchOutboundConnection_LocalDomainInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().local_domain_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.local_domain_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_domain_info`.\n"]
    pub fn set_remote_domain_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchOutboundConnection_RemoteDomainInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_domain_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_domain_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OpensearchOutboundConnection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accept_connection` after provisioning.\n"]
    pub fn accept_connection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_connection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_alias` after provisioning.\n"]
    pub fn connection_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_mode` after provisioning.\n"]
    pub fn connection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_status` after provisioning.\n"]
    pub fn connection_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_properties` after provisioning.\n"]
    pub fn connection_properties(&self) -> terrars::ListRef<OpensearchOutboundConnection_ConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_domain_info` after provisioning.\n"]
    pub fn local_domain_info(&self) -> terrars::ListRef<OpensearchOutboundConnection_LocalDomainInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_domain_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_domain_info` after provisioning.\n"]
    pub fn remote_domain_info(&self) -> terrars::ListRef<OpensearchOutboundConnection_RemoteDomainInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_domain_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchOutboundConnection_TimeoutsRef {
        OpensearchOutboundConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OpensearchOutboundConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OpensearchOutboundConnection { }

impl terrars::ToListMappable for OpensearchOutboundConnection {
    type O = terrars::ListRef<OpensearchOutboundConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OpensearchOutboundConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_opensearch_outbound_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpensearchOutboundConnection {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub connection_alias: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchOutboundConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> OpensearchOutboundConnection {
        let out = OpensearchOutboundConnection(std::rc::Rc::new(OpensearchOutboundConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OpensearchOutboundConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                accept_connection: core::default::Default::default(),
                connection_alias: self.connection_alias,
                connection_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                connection_properties: core::default::Default::default(),
                local_domain_info: core::default::Default::default(),
                remote_domain_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpensearchOutboundConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchOutboundConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpensearchOutboundConnectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_connection` after provisioning.\n"]
    pub fn accept_connection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_connection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_alias` after provisioning.\n"]
    pub fn connection_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_mode` after provisioning.\n"]
    pub fn connection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_status` after provisioning.\n"]
    pub fn connection_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_properties` after provisioning.\n"]
    pub fn connection_properties(&self) -> terrars::ListRef<OpensearchOutboundConnection_ConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_domain_info` after provisioning.\n"]
    pub fn local_domain_info(&self) -> terrars::ListRef<OpensearchOutboundConnection_LocalDomainInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_domain_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_domain_info` after provisioning.\n"]
    pub fn remote_domain_info(&self) -> terrars::ListRef<OpensearchOutboundConnection_RemoteDomainInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_domain_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchOutboundConnection_TimeoutsRef {
        OpensearchOutboundConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OpensearchOutboundConnectionDynamic {
    connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchOutboundConnection_ConnectionProperties>>,
    local_domain_info: std::option::Option<terrars::DynamicBlock<OpensearchOutboundConnection_LocalDomainInfo>>,
    remote_domain_info: std::option::Option<terrars::DynamicBlock<OpensearchOutboundConnection_RemoteDomainInfo>>,
}

#[derive(Serialize)]
pub struct OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_unavailable: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch {
    #[doc = "Set the field `skip_unavailable`.\n"]
    pub fn set_skip_unavailable(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.skip_unavailable = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch {
    type O = terrars::BlockAssignable<OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch {}

impl BuildOpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch {
    pub fn build(self) -> OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch {
        OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch {
            skip_unavailable: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearchRef {
        OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `skip_unavailable` after provisioning.\n"]
    pub fn skip_unavailable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_unavailable", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpensearchOutboundConnection_ConnectionPropertiesDynamic {
    cross_cluster_search: std
    ::option
    ::Option<terrars::DynamicBlock<OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch>>,
}

#[derive(Serialize)]
pub struct OpensearchOutboundConnection_ConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_cluster_search: std
    ::option
    ::Option<std::vec::Vec<OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch>>,
    dynamic: OpensearchOutboundConnection_ConnectionPropertiesDynamic,
}

impl OpensearchOutboundConnection_ConnectionProperties {
    #[doc = "Set the field `cross_cluster_search`.\n"]
    pub fn set_cross_cluster_search(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearch>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cross_cluster_search = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cross_cluster_search = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpensearchOutboundConnection_ConnectionProperties {
    type O = terrars::BlockAssignable<OpensearchOutboundConnection_ConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnection_ConnectionProperties {}

impl BuildOpensearchOutboundConnection_ConnectionProperties {
    pub fn build(self) -> OpensearchOutboundConnection_ConnectionProperties {
        OpensearchOutboundConnection_ConnectionProperties {
            cross_cluster_search: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchOutboundConnection_ConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchOutboundConnection_ConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchOutboundConnection_ConnectionPropertiesRef {
        OpensearchOutboundConnection_ConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnection_ConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_cluster_search` after provisioning.\n"]
    pub fn cross_cluster_search(
        &self,
    ) -> terrars::ListRef<OpensearchOutboundConnection_ConnectionProperties_CrossClusterSearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_cluster_search", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchOutboundConnection_LocalDomainInfo {
    domain_name: terrars::PrimField<std::string::String>,
    owner_id: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
}

impl OpensearchOutboundConnection_LocalDomainInfo { }

impl terrars::ToListMappable for OpensearchOutboundConnection_LocalDomainInfo {
    type O = terrars::BlockAssignable<OpensearchOutboundConnection_LocalDomainInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnection_LocalDomainInfo {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub owner_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchOutboundConnection_LocalDomainInfo {
    pub fn build(self) -> OpensearchOutboundConnection_LocalDomainInfo {
        OpensearchOutboundConnection_LocalDomainInfo {
            domain_name: self.domain_name,
            owner_id: self.owner_id,
            region: self.region,
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchOutboundConnection_LocalDomainInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchOutboundConnection_LocalDomainInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchOutboundConnection_LocalDomainInfoRef {
        OpensearchOutboundConnection_LocalDomainInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnection_LocalDomainInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchOutboundConnection_RemoteDomainInfo {
    domain_name: terrars::PrimField<std::string::String>,
    owner_id: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
}

impl OpensearchOutboundConnection_RemoteDomainInfo { }

impl terrars::ToListMappable for OpensearchOutboundConnection_RemoteDomainInfo {
    type O = terrars::BlockAssignable<OpensearchOutboundConnection_RemoteDomainInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnection_RemoteDomainInfo {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub owner_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchOutboundConnection_RemoteDomainInfo {
    pub fn build(self) -> OpensearchOutboundConnection_RemoteDomainInfo {
        OpensearchOutboundConnection_RemoteDomainInfo {
            domain_name: self.domain_name,
            owner_id: self.owner_id,
            region: self.region,
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchOutboundConnection_RemoteDomainInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchOutboundConnection_RemoteDomainInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchOutboundConnection_RemoteDomainInfoRef {
        OpensearchOutboundConnection_RemoteDomainInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnection_RemoteDomainInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchOutboundConnection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchOutboundConnection_Timeouts {
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

impl terrars::ToListMappable for OpensearchOutboundConnection_Timeouts {
    type O = terrars::BlockAssignable<OpensearchOutboundConnection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnection_Timeouts {}

impl BuildOpensearchOutboundConnection_Timeouts {
    pub fn build(self) -> OpensearchOutboundConnection_Timeouts {
        OpensearchOutboundConnection_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchOutboundConnection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchOutboundConnection_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchOutboundConnection_TimeoutsRef {
        OpensearchOutboundConnection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnection_TimeoutsRef {
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

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpensearchOutboundConnectionData {
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
    connection_alias: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_domain_info: std::option::Option<std::vec::Vec<OpensearchOutboundConnectionLocalDomainInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_domain_info: std::option::Option<std::vec::Vec<OpensearchOutboundConnectionRemoteDomainInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OpensearchOutboundConnectionTimeoutsEl>,
    dynamic: OpensearchOutboundConnectionDynamic,
}

struct OpensearchOutboundConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpensearchOutboundConnectionData>,
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_domain_info`.\n"]
    pub fn set_local_domain_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchOutboundConnectionLocalDomainInfoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<OpensearchOutboundConnectionRemoteDomainInfoEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<OpensearchOutboundConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `connection_alias` after provisioning.\n"]
    pub fn connection_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_status` after provisioning.\n"]
    pub fn connection_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_domain_info` after provisioning.\n"]
    pub fn local_domain_info(&self) -> terrars::ListRef<OpensearchOutboundConnectionLocalDomainInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_domain_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_domain_info` after provisioning.\n"]
    pub fn remote_domain_info(&self) -> terrars::ListRef<OpensearchOutboundConnectionRemoteDomainInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_domain_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchOutboundConnectionTimeoutsElRef {
        OpensearchOutboundConnectionTimeoutsElRef::new(
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
            data: std::cell::RefCell::new(OpensearchOutboundConnectionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_alias: self.connection_alias,
                id: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `connection_alias` after provisioning.\n"]
    pub fn connection_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_status` after provisioning.\n"]
    pub fn connection_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_domain_info` after provisioning.\n"]
    pub fn local_domain_info(&self) -> terrars::ListRef<OpensearchOutboundConnectionLocalDomainInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_domain_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_domain_info` after provisioning.\n"]
    pub fn remote_domain_info(&self) -> terrars::ListRef<OpensearchOutboundConnectionRemoteDomainInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_domain_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchOutboundConnectionTimeoutsElRef {
        OpensearchOutboundConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct OpensearchOutboundConnectionLocalDomainInfoEl {
    domain_name: terrars::PrimField<std::string::String>,
    owner_id: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
}

impl OpensearchOutboundConnectionLocalDomainInfoEl { }

impl terrars::ToListMappable for OpensearchOutboundConnectionLocalDomainInfoEl {
    type O = terrars::BlockAssignable<OpensearchOutboundConnectionLocalDomainInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnectionLocalDomainInfoEl {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub owner_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchOutboundConnectionLocalDomainInfoEl {
    pub fn build(self) -> OpensearchOutboundConnectionLocalDomainInfoEl {
        OpensearchOutboundConnectionLocalDomainInfoEl {
            domain_name: self.domain_name,
            owner_id: self.owner_id,
            region: self.region,
        }
    }
}

pub struct OpensearchOutboundConnectionLocalDomainInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchOutboundConnectionLocalDomainInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchOutboundConnectionLocalDomainInfoElRef {
        OpensearchOutboundConnectionLocalDomainInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnectionLocalDomainInfoElRef {
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
pub struct OpensearchOutboundConnectionRemoteDomainInfoEl {
    domain_name: terrars::PrimField<std::string::String>,
    owner_id: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
}

impl OpensearchOutboundConnectionRemoteDomainInfoEl { }

impl terrars::ToListMappable for OpensearchOutboundConnectionRemoteDomainInfoEl {
    type O = terrars::BlockAssignable<OpensearchOutboundConnectionRemoteDomainInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnectionRemoteDomainInfoEl {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub owner_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchOutboundConnectionRemoteDomainInfoEl {
    pub fn build(self) -> OpensearchOutboundConnectionRemoteDomainInfoEl {
        OpensearchOutboundConnectionRemoteDomainInfoEl {
            domain_name: self.domain_name,
            owner_id: self.owner_id,
            region: self.region,
        }
    }
}

pub struct OpensearchOutboundConnectionRemoteDomainInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchOutboundConnectionRemoteDomainInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchOutboundConnectionRemoteDomainInfoElRef {
        OpensearchOutboundConnectionRemoteDomainInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnectionRemoteDomainInfoElRef {
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
pub struct OpensearchOutboundConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpensearchOutboundConnectionTimeoutsEl {
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

impl terrars::ToListMappable for OpensearchOutboundConnectionTimeoutsEl {
    type O = terrars::BlockAssignable<OpensearchOutboundConnectionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnectionTimeoutsEl {}

impl BuildOpensearchOutboundConnectionTimeoutsEl {
    pub fn build(self) -> OpensearchOutboundConnectionTimeoutsEl {
        OpensearchOutboundConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct OpensearchOutboundConnectionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchOutboundConnectionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpensearchOutboundConnectionTimeoutsElRef {
        OpensearchOutboundConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnectionTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct OpensearchOutboundConnectionDynamic {
    local_domain_info: std::option::Option<terrars::DynamicBlock<OpensearchOutboundConnectionLocalDomainInfoEl>>,
    remote_domain_info: std::option::Option<terrars::DynamicBlock<OpensearchOutboundConnectionRemoteDomainInfoEl>>,
}

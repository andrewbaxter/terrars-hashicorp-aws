use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpensearchOutboundConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    connection_alias: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_domain_info: Option<Vec<OpensearchOutboundConnectionLocalDomainInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_domain_info: Option<Vec<OpensearchOutboundConnectionRemoteDomainInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OpensearchOutboundConnectionTimeoutsEl>,
    dynamic: OpensearchOutboundConnectionDynamic,
}

struct OpensearchOutboundConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpensearchOutboundConnectionData>,
}

#[derive(Clone)]
pub struct OpensearchOutboundConnection(Rc<OpensearchOutboundConnection_>);

impl OpensearchOutboundConnection {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `local_domain_info`.\n"]
    pub fn set_local_domain_info(
        self,
        v: impl Into<BlockAssignable<OpensearchOutboundConnectionLocalDomainInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().local_domain_info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.local_domain_info = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `remote_domain_info`.\n"]
    pub fn set_remote_domain_info(
        self,
        v: impl Into<BlockAssignable<OpensearchOutboundConnectionRemoteDomainInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_domain_info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_domain_info = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OpensearchOutboundConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `connection_alias` after provisioning.\n"]
    pub fn connection_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connection_status` after provisioning.\n"]
    pub fn connection_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `local_domain_info` after provisioning.\n"]
    pub fn local_domain_info(&self) -> ListRef<OpensearchOutboundConnectionLocalDomainInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_domain_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remote_domain_info` after provisioning.\n"]
    pub fn remote_domain_info(&self) -> ListRef<OpensearchOutboundConnectionRemoteDomainInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_domain_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchOutboundConnectionTimeoutsElRef {
        OpensearchOutboundConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for OpensearchOutboundConnection {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for OpensearchOutboundConnection { }

impl ToListMappable for OpensearchOutboundConnection {
    type O = ListRef<OpensearchOutboundConnectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OpensearchOutboundConnection_ {
    fn extract_resource_type(&self) -> String {
        "aws_opensearch_outbound_connection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpensearchOutboundConnection {
    pub tf_id: String,
    #[doc= ""]
    pub connection_alias: PrimField<String>,
}

impl BuildOpensearchOutboundConnection {
    pub fn build(self, stack: &mut Stack) -> OpensearchOutboundConnection {
        let out = OpensearchOutboundConnection(Rc::new(OpensearchOutboundConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpensearchOutboundConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connection_alias: self.connection_alias,
                id: core::default::Default::default(),
                local_domain_info: core::default::Default::default(),
                remote_domain_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpensearchOutboundConnectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchOutboundConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpensearchOutboundConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `connection_alias` after provisioning.\n"]
    pub fn connection_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connection_status` after provisioning.\n"]
    pub fn connection_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `local_domain_info` after provisioning.\n"]
    pub fn local_domain_info(&self) -> ListRef<OpensearchOutboundConnectionLocalDomainInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_domain_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remote_domain_info` after provisioning.\n"]
    pub fn remote_domain_info(&self) -> ListRef<OpensearchOutboundConnectionRemoteDomainInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_domain_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchOutboundConnectionTimeoutsElRef {
        OpensearchOutboundConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct OpensearchOutboundConnectionLocalDomainInfoEl {
    domain_name: PrimField<String>,
    owner_id: PrimField<String>,
    region: PrimField<String>,
}

impl OpensearchOutboundConnectionLocalDomainInfoEl { }

impl ToListMappable for OpensearchOutboundConnectionLocalDomainInfoEl {
    type O = BlockAssignable<OpensearchOutboundConnectionLocalDomainInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnectionLocalDomainInfoEl {
    #[doc= ""]
    pub domain_name: PrimField<String>,
    #[doc= ""]
    pub owner_id: PrimField<String>,
    #[doc= ""]
    pub region: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchOutboundConnectionLocalDomainInfoElRef {
    fn new(shared: StackShared, base: String) -> OpensearchOutboundConnectionLocalDomainInfoElRef {
        OpensearchOutboundConnectionLocalDomainInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnectionLocalDomainInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchOutboundConnectionRemoteDomainInfoEl {
    domain_name: PrimField<String>,
    owner_id: PrimField<String>,
    region: PrimField<String>,
}

impl OpensearchOutboundConnectionRemoteDomainInfoEl { }

impl ToListMappable for OpensearchOutboundConnectionRemoteDomainInfoEl {
    type O = BlockAssignable<OpensearchOutboundConnectionRemoteDomainInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchOutboundConnectionRemoteDomainInfoEl {
    #[doc= ""]
    pub domain_name: PrimField<String>,
    #[doc= ""]
    pub owner_id: PrimField<String>,
    #[doc= ""]
    pub region: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchOutboundConnectionRemoteDomainInfoElRef {
    fn new(shared: StackShared, base: String) -> OpensearchOutboundConnectionRemoteDomainInfoElRef {
        OpensearchOutboundConnectionRemoteDomainInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnectionRemoteDomainInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct OpensearchOutboundConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl OpensearchOutboundConnectionTimeoutsEl {
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

impl ToListMappable for OpensearchOutboundConnectionTimeoutsEl {
    type O = BlockAssignable<OpensearchOutboundConnectionTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchOutboundConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchOutboundConnectionTimeoutsElRef {
        OpensearchOutboundConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchOutboundConnectionTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct OpensearchOutboundConnectionDynamic {
    local_domain_info: Option<DynamicBlock<OpensearchOutboundConnectionLocalDomainInfoEl>>,
    remote_domain_info: Option<DynamicBlock<OpensearchOutboundConnectionRemoteDomainInfoEl>>,
}

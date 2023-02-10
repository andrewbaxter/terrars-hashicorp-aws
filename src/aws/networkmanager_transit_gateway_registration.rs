use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkmanagerTransitGatewayRegistrationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    global_network_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    transit_gateway_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkmanagerTransitGatewayRegistrationTimeoutsEl>,
}

struct NetworkmanagerTransitGatewayRegistration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkmanagerTransitGatewayRegistrationData>,
}

#[derive(Clone)]
pub struct NetworkmanagerTransitGatewayRegistration(Rc<NetworkmanagerTransitGatewayRegistration_>);

impl NetworkmanagerTransitGatewayRegistration {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkmanagerTransitGatewayRegistrationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_network_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_arn` after provisioning.\n"]
    pub fn transit_gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerTransitGatewayRegistrationTimeoutsElRef {
        NetworkmanagerTransitGatewayRegistrationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Resource for NetworkmanagerTransitGatewayRegistration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for NetworkmanagerTransitGatewayRegistration {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for NetworkmanagerTransitGatewayRegistration {
    type O = ListRef<NetworkmanagerTransitGatewayRegistrationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for NetworkmanagerTransitGatewayRegistration_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkmanager_transit_gateway_registration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkmanagerTransitGatewayRegistration {
    pub tf_id: String,
    #[doc= ""]
    pub global_network_id: PrimField<String>,
    #[doc= ""]
    pub transit_gateway_arn: PrimField<String>,
}

impl BuildNetworkmanagerTransitGatewayRegistration {
    pub fn build(self, stack: &mut Stack) -> NetworkmanagerTransitGatewayRegistration {
        let out = NetworkmanagerTransitGatewayRegistration(Rc::new(NetworkmanagerTransitGatewayRegistration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkmanagerTransitGatewayRegistrationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                global_network_id: self.global_network_id,
                id: core::default::Default::default(),
                transit_gateway_arn: self.transit_gateway_arn,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkmanagerTransitGatewayRegistrationRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerTransitGatewayRegistrationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkmanagerTransitGatewayRegistrationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.global_network_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_arn` after provisioning.\n"]
    pub fn transit_gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkmanagerTransitGatewayRegistrationTimeoutsElRef {
        NetworkmanagerTransitGatewayRegistrationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct NetworkmanagerTransitGatewayRegistrationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl NetworkmanagerTransitGatewayRegistrationTimeoutsEl {
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

impl ToListMappable for NetworkmanagerTransitGatewayRegistrationTimeoutsEl {
    type O = BlockAssignable<NetworkmanagerTransitGatewayRegistrationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkmanagerTransitGatewayRegistrationTimeoutsEl {}

impl BuildNetworkmanagerTransitGatewayRegistrationTimeoutsEl {
    pub fn build(self) -> NetworkmanagerTransitGatewayRegistrationTimeoutsEl {
        NetworkmanagerTransitGatewayRegistrationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct NetworkmanagerTransitGatewayRegistrationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkmanagerTransitGatewayRegistrationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkmanagerTransitGatewayRegistrationTimeoutsElRef {
        NetworkmanagerTransitGatewayRegistrationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkmanagerTransitGatewayRegistrationTimeoutsElRef {
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

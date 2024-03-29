use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcEndpointSubnetAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    subnet_id: PrimField<String>,
    vpc_endpoint_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VpcEndpointSubnetAssociationTimeoutsEl>,
}

struct VpcEndpointSubnetAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcEndpointSubnetAssociationData>,
}

#[derive(Clone)]
pub struct VpcEndpointSubnetAssociation(Rc<VpcEndpointSubnetAssociation_>);

impl VpcEndpointSubnetAssociation {
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

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VpcEndpointSubnetAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcEndpointSubnetAssociationTimeoutsElRef {
        VpcEndpointSubnetAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for VpcEndpointSubnetAssociation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for VpcEndpointSubnetAssociation { }

impl ToListMappable for VpcEndpointSubnetAssociation {
    type O = ListRef<VpcEndpointSubnetAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpcEndpointSubnetAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_endpoint_subnet_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcEndpointSubnetAssociation {
    pub tf_id: String,
    #[doc= ""]
    pub subnet_id: PrimField<String>,
    #[doc= ""]
    pub vpc_endpoint_id: PrimField<String>,
}

impl BuildVpcEndpointSubnetAssociation {
    pub fn build(self, stack: &mut Stack) -> VpcEndpointSubnetAssociation {
        let out = VpcEndpointSubnetAssociation(Rc::new(VpcEndpointSubnetAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcEndpointSubnetAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                subnet_id: self.subnet_id,
                vpc_endpoint_id: self.vpc_endpoint_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcEndpointSubnetAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcEndpointSubnetAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpcEndpointSubnetAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcEndpointSubnetAssociationTimeoutsElRef {
        VpcEndpointSubnetAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct VpcEndpointSubnetAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl VpcEndpointSubnetAssociationTimeoutsEl {
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

impl ToListMappable for VpcEndpointSubnetAssociationTimeoutsEl {
    type O = BlockAssignable<VpcEndpointSubnetAssociationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcEndpointSubnetAssociationTimeoutsEl {}

impl BuildVpcEndpointSubnetAssociationTimeoutsEl {
    pub fn build(self) -> VpcEndpointSubnetAssociationTimeoutsEl {
        VpcEndpointSubnetAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct VpcEndpointSubnetAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcEndpointSubnetAssociationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VpcEndpointSubnetAssociationTimeoutsElRef {
        VpcEndpointSubnetAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcEndpointSubnetAssociationTimeoutsElRef {
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

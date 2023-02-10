use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2TransitGatewayDxGatewayAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dx_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_gateway_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2TransitGatewayDxGatewayAttachmentFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl>,
    dynamic: DataEc2TransitGatewayDxGatewayAttachmentDynamic,
}

struct DataEc2TransitGatewayDxGatewayAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2TransitGatewayDxGatewayAttachmentData>,
}

#[derive(Clone)]
pub struct DataEc2TransitGatewayDxGatewayAttachment(Rc<DataEc2TransitGatewayDxGatewayAttachment_>);

impl DataEc2TransitGatewayDxGatewayAttachment {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `dx_gateway_id`.\n"]
    pub fn set_dx_gateway_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().dx_gateway_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_id = Some(v.into());
        self
    }

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataEc2TransitGatewayDxGatewayAttachmentFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `dx_gateway_id` after provisioning.\n"]
    pub fn dx_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayDxGatewayAttachmentTimeoutsElRef {
        DataEc2TransitGatewayDxGatewayAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataEc2TransitGatewayDxGatewayAttachment {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEc2TransitGatewayDxGatewayAttachment { }

impl ToListMappable for DataEc2TransitGatewayDxGatewayAttachment {
    type O = ListRef<DataEc2TransitGatewayDxGatewayAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2TransitGatewayDxGatewayAttachment_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_transit_gateway_dx_gateway_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2TransitGatewayDxGatewayAttachment {
    pub tf_id: String,
}

impl BuildDataEc2TransitGatewayDxGatewayAttachment {
    pub fn build(self, stack: &mut Stack) -> DataEc2TransitGatewayDxGatewayAttachment {
        let out = DataEc2TransitGatewayDxGatewayAttachment(Rc::new(DataEc2TransitGatewayDxGatewayAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2TransitGatewayDxGatewayAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                dx_gateway_id: core::default::Default::default(),
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                transit_gateway_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2TransitGatewayDxGatewayAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayDxGatewayAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2TransitGatewayDxGatewayAttachmentRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `dx_gateway_id` after provisioning.\n"]
    pub fn dx_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dx_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayDxGatewayAttachmentTimeoutsElRef {
        DataEc2TransitGatewayDxGatewayAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayDxGatewayAttachmentFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2TransitGatewayDxGatewayAttachmentFilterEl { }

impl ToListMappable for DataEc2TransitGatewayDxGatewayAttachmentFilterEl {
    type O = BlockAssignable<DataEc2TransitGatewayDxGatewayAttachmentFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayDxGatewayAttachmentFilterEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2TransitGatewayDxGatewayAttachmentFilterEl {
    pub fn build(self) -> DataEc2TransitGatewayDxGatewayAttachmentFilterEl {
        DataEc2TransitGatewayDxGatewayAttachmentFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2TransitGatewayDxGatewayAttachmentFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayDxGatewayAttachmentFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayDxGatewayAttachmentFilterElRef {
        DataEc2TransitGatewayDxGatewayAttachmentFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayDxGatewayAttachmentFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl {
    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl {
    type O = BlockAssignable<DataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl {}

impl BuildDataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl {
    pub fn build(self) -> DataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl {
        DataEc2TransitGatewayDxGatewayAttachmentTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2TransitGatewayDxGatewayAttachmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2TransitGatewayDxGatewayAttachmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2TransitGatewayDxGatewayAttachmentTimeoutsElRef {
        DataEc2TransitGatewayDxGatewayAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayDxGatewayAttachmentTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2TransitGatewayDxGatewayAttachmentDynamic {
    filter: Option<DynamicBlock<DataEc2TransitGatewayDxGatewayAttachmentFilterEl>>,
}

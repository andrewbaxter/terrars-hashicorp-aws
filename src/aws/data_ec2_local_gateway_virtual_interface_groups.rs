use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2LocalGatewayVirtualInterfaceGroupsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2LocalGatewayVirtualInterfaceGroupsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl>,
    dynamic: DataEc2LocalGatewayVirtualInterfaceGroupsDynamic,
}

struct DataEc2LocalGatewayVirtualInterfaceGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2LocalGatewayVirtualInterfaceGroupsData>,
}

#[derive(Clone)]
pub struct DataEc2LocalGatewayVirtualInterfaceGroups(Rc<DataEc2LocalGatewayVirtualInterfaceGroups_>);

impl DataEc2LocalGatewayVirtualInterfaceGroups {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataEc2LocalGatewayVirtualInterfaceGroupsFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `local_gateway_virtual_interface_ids` after provisioning.\n"]
    pub fn local_gateway_virtual_interface_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.local_gateway_virtual_interface_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Datasource for DataEc2LocalGatewayVirtualInterfaceGroups {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataEc2LocalGatewayVirtualInterfaceGroups {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataEc2LocalGatewayVirtualInterfaceGroups {
    type O = ListRef<DataEc2LocalGatewayVirtualInterfaceGroupsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataEc2LocalGatewayVirtualInterfaceGroups_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_local_gateway_virtual_interface_groups".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceGroups {
    pub tf_id: String,
}

impl BuildDataEc2LocalGatewayVirtualInterfaceGroups {
    pub fn build(self, stack: &mut Stack) -> DataEc2LocalGatewayVirtualInterfaceGroups {
        let out = DataEc2LocalGatewayVirtualInterfaceGroups(Rc::new(DataEc2LocalGatewayVirtualInterfaceGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2LocalGatewayVirtualInterfaceGroupsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceGroupsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayVirtualInterfaceGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceGroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `local_gateway_virtual_interface_ids` after provisioning.\n"]
    pub fn local_gateway_virtual_interface_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.local_gateway_virtual_interface_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2LocalGatewayVirtualInterfaceGroupsFilterEl {
    name: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl DataEc2LocalGatewayVirtualInterfaceGroupsFilterEl { }

impl ToListMappable for DataEc2LocalGatewayVirtualInterfaceGroupsFilterEl {
    type O = BlockAssignable<DataEc2LocalGatewayVirtualInterfaceGroupsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceGroupsFilterEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataEc2LocalGatewayVirtualInterfaceGroupsFilterEl {
    pub fn build(self) -> DataEc2LocalGatewayVirtualInterfaceGroupsFilterEl {
        DataEc2LocalGatewayVirtualInterfaceGroupsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceGroupsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayVirtualInterfaceGroupsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2LocalGatewayVirtualInterfaceGroupsFilterElRef {
        DataEc2LocalGatewayVirtualInterfaceGroupsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceGroupsFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl {
    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl {
    type O = BlockAssignable<DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl {}

impl BuildDataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl {
    pub fn build(self) -> DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl {
        DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceGroupsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2LocalGatewayVirtualInterfaceGroupsDynamic {
    filter: Option<DynamicBlock<DataEc2LocalGatewayVirtualInterfaceGroupsFilterEl>>,
}

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNetworkInterfacesData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataNetworkInterfacesFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataNetworkInterfacesTimeoutsEl>,
    dynamic: DataNetworkInterfacesDynamic,
}

struct DataNetworkInterfaces_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataNetworkInterfacesData>,
}

#[derive(Clone)]
pub struct DataNetworkInterfaces(Rc<DataNetworkInterfaces_>);

impl DataNetworkInterfaces {
    fn shared(&self) -> &StackShared {
        &self.0.shared
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
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataNetworkInterfacesFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<DataNetworkInterfacesTimeoutsEl>) -> Self {
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

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataNetworkInterfacesTimeoutsElRef {
        DataNetworkInterfacesTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Datasource for DataNetworkInterfaces {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataNetworkInterfaces {
    type O = ListRef<DataNetworkInterfacesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataNetworkInterfaces_ {
    fn extract_datasource_type(&self) -> String {
        "aws_network_interfaces".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkInterfaces {
    pub tf_id: String,
}

impl BuildDataNetworkInterfaces {
    pub fn build(self, stack: &mut Stack) -> DataNetworkInterfaces {
        let out = DataNetworkInterfaces(Rc::new(DataNetworkInterfaces_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataNetworkInterfacesData {
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

pub struct DataNetworkInterfacesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkInterfacesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkInterfacesRef {
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

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataNetworkInterfacesTimeoutsElRef {
        DataNetworkInterfacesTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNetworkInterfacesFilterEl {
    name: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl DataNetworkInterfacesFilterEl { }

impl ToListMappable for DataNetworkInterfacesFilterEl {
    type O = BlockAssignable<DataNetworkInterfacesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfacesFilterEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataNetworkInterfacesFilterEl {
    pub fn build(self) -> DataNetworkInterfacesFilterEl {
        DataNetworkInterfacesFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataNetworkInterfacesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkInterfacesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkInterfacesFilterElRef {
        DataNetworkInterfacesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfacesFilterElRef {
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
pub struct DataNetworkInterfacesTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataNetworkInterfacesTimeoutsEl {
    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataNetworkInterfacesTimeoutsEl {
    type O = BlockAssignable<DataNetworkInterfacesTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfacesTimeoutsEl {}

impl BuildDataNetworkInterfacesTimeoutsEl {
    pub fn build(self) -> DataNetworkInterfacesTimeoutsEl {
        DataNetworkInterfacesTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataNetworkInterfacesTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataNetworkInterfacesTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataNetworkInterfacesTimeoutsElRef {
        DataNetworkInterfacesTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfacesTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkInterfacesDynamic {
    filter: Option<DynamicBlock<DataNetworkInterfacesFilterEl>>,
}

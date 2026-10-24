use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2InstanceTypeOfferingsData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2InstanceTypeOfferingsFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataEc2InstanceTypeOfferingsTimeoutsEl>,
    dynamic: DataEc2InstanceTypeOfferingsDynamic,
}

struct DataEc2InstanceTypeOfferings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2InstanceTypeOfferingsData>,
}

#[derive(Clone)]
pub struct DataEc2InstanceTypeOfferings(std::rc::Rc<DataEc2InstanceTypeOfferings_>);

impl DataEc2InstanceTypeOfferings {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `location_type`.\n"]
    pub fn set_location_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2InstanceTypeOfferingsFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2InstanceTypeOfferingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_type` after provisioning.\n"]
    pub fn location_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_types` after provisioning.\n"]
    pub fn location_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceTypeOfferingsTimeoutsElRef {
        DataEc2InstanceTypeOfferingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataEc2InstanceTypeOfferings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2InstanceTypeOfferings { }

impl terrars::ToListMappable for DataEc2InstanceTypeOfferings {
    type O = terrars::ListRef<DataEc2InstanceTypeOfferingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2InstanceTypeOfferings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_instance_type_offerings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2InstanceTypeOfferings {
    pub tf_id: std::string::String,
}

impl BuildDataEc2InstanceTypeOfferings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2InstanceTypeOfferings {
        let out = DataEc2InstanceTypeOfferings(std::rc::Rc::new(DataEc2InstanceTypeOfferings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2InstanceTypeOfferingsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location_type: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2InstanceTypeOfferingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceTypeOfferingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2InstanceTypeOfferingsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_type` after provisioning.\n"]
    pub fn location_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_types` after provisioning.\n"]
    pub fn location_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locations` after provisioning.\n"]
    pub fn locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceTypeOfferingsTimeoutsElRef {
        DataEc2InstanceTypeOfferingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeOfferingsFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceTypeOfferingsFilterEl { }

impl terrars::ToListMappable for DataEc2InstanceTypeOfferingsFilterEl {
    type O = terrars::BlockAssignable<DataEc2InstanceTypeOfferingsFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceTypeOfferingsFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2InstanceTypeOfferingsFilterEl {
    pub fn build(self) -> DataEc2InstanceTypeOfferingsFilterEl {
        DataEc2InstanceTypeOfferingsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2InstanceTypeOfferingsFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceTypeOfferingsFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceTypeOfferingsFilterElRef {
        DataEc2InstanceTypeOfferingsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeOfferingsFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeOfferingsTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceTypeOfferingsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceTypeOfferingsTimeoutsEl {
    type O = terrars::BlockAssignable<DataEc2InstanceTypeOfferingsTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceTypeOfferingsTimeoutsEl {}

impl BuildDataEc2InstanceTypeOfferingsTimeoutsEl {
    pub fn build(self) -> DataEc2InstanceTypeOfferingsTimeoutsEl {
        DataEc2InstanceTypeOfferingsTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2InstanceTypeOfferingsTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceTypeOfferingsTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceTypeOfferingsTimeoutsElRef {
        DataEc2InstanceTypeOfferingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeOfferingsTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2InstanceTypeOfferingsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2InstanceTypeOfferingsFilterEl>>,
}

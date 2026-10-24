use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcIpamPoolData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_resource_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataVpcIpamPoolFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataVpcIpamPoolTimeoutsEl>,
    dynamic: DataVpcIpamPoolDynamic,
}

struct DataVpcIpamPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcIpamPoolData>,
}

#[derive(Clone)]
pub struct DataVpcIpamPool(std::rc::Rc<DataVpcIpamPool_>);

impl DataVpcIpamPool {
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

    #[doc = "Set the field `allocation_resource_tags`.\n"]
    pub fn set_allocation_resource_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allocation_resource_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_pool_id`.\n"]
    pub fn set_ipam_pool_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipam_pool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataVpcIpamPoolFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataVpcIpamPoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_default_netmask_length` after provisioning.\n"]
    pub fn allocation_default_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_default_netmask_length", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allocation_max_netmask_length` after provisioning.\n"]
    pub fn allocation_max_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_max_netmask_length", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allocation_min_netmask_length` after provisioning.\n"]
    pub fn allocation_min_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_min_netmask_length", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allocation_resource_tags` after provisioning.\n"]
    pub fn allocation_resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.allocation_resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_import` after provisioning.\n"]
    pub fn auto_import(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_import", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_service` after provisioning.\n"]
    pub fn aws_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_id` after provisioning.\n"]
    pub fn ipam_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_type` after provisioning.\n"]
    pub fn ipam_scope_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_depth` after provisioning.\n"]
    pub fn pool_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_depth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_advertisable` after provisioning.\n"]
    pub fn publicly_advertisable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_advertisable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ipam_pool_id` after provisioning.\n"]
    pub fn source_ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPoolTimeoutsElRef {
        DataVpcIpamPoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpcIpamPool {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpcIpamPool { }

impl terrars::ToListMappable for DataVpcIpamPool {
    type O = terrars::ListRef<DataVpcIpamPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpcIpamPool_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpc_ipam_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcIpamPool {
    pub tf_id: std::string::String,
}

impl BuildDataVpcIpamPool {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpcIpamPool {
        let out = DataVpcIpamPool(std::rc::Rc::new(DataVpcIpamPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpcIpamPoolData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                allocation_resource_tags: core::default::Default::default(),
                id: core::default::Default::default(),
                ipam_pool_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcIpamPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcIpamPoolRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocation_default_netmask_length` after provisioning.\n"]
    pub fn allocation_default_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_default_netmask_length", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allocation_max_netmask_length` after provisioning.\n"]
    pub fn allocation_max_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_max_netmask_length", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allocation_min_netmask_length` after provisioning.\n"]
    pub fn allocation_min_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocation_min_netmask_length", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `allocation_resource_tags` after provisioning.\n"]
    pub fn allocation_resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.allocation_resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_import` after provisioning.\n"]
    pub fn auto_import(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_import", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_service` after provisioning.\n"]
    pub fn aws_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_id` after provisioning.\n"]
    pub fn ipam_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_type` after provisioning.\n"]
    pub fn ipam_scope_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_depth` after provisioning.\n"]
    pub fn pool_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_depth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_advertisable` after provisioning.\n"]
    pub fn publicly_advertisable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_advertisable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ipam_pool_id` after provisioning.\n"]
    pub fn source_ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPoolTimeoutsElRef {
        DataVpcIpamPoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpamPoolFilterEl { }

impl terrars::ToListMappable for DataVpcIpamPoolFilterEl {
    type O = terrars::BlockAssignable<DataVpcIpamPoolFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataVpcIpamPoolFilterEl {
    pub fn build(self) -> DataVpcIpamPoolFilterEl {
        DataVpcIpamPoolFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpcIpamPoolFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpamPoolFilterElRef {
        DataVpcIpamPoolFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolFilterElRef {
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
pub struct DataVpcIpamPoolTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpamPoolTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcIpamPoolTimeoutsEl {
    type O = terrars::BlockAssignable<DataVpcIpamPoolTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolTimeoutsEl {}

impl BuildDataVpcIpamPoolTimeoutsEl {
    pub fn build(self) -> DataVpcIpamPoolTimeoutsEl {
        DataVpcIpamPoolTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataVpcIpamPoolTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpamPoolTimeoutsElRef {
        DataVpcIpamPoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataVpcIpamPoolDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataVpcIpamPoolFilterEl>>,
}

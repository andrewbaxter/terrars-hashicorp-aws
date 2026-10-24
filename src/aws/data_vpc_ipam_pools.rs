use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcIpamPoolsData {
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
    filter: std::option::Option<std::vec::Vec<DataVpcIpamPoolsFilterEl>>,
    dynamic: DataVpcIpamPoolsDynamic,
}

struct DataVpcIpamPools_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcIpamPoolsData>,
}

#[derive(Clone)]
pub struct DataVpcIpamPools(std::rc::Rc<DataVpcIpamPools_>);

impl DataVpcIpamPools {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataVpcIpamPoolsFilterEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pools` after provisioning.\n"]
    pub fn ipam_pools(&self) -> terrars::SetRef<DataVpcIpamPoolsIpamPoolsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipam_pools", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpcIpamPools {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpcIpamPools { }

impl terrars::ToListMappable for DataVpcIpamPools {
    type O = terrars::ListRef<DataVpcIpamPoolsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpcIpamPools_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpc_ipam_pools".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcIpamPools {
    pub tf_id: std::string::String,
}

impl BuildDataVpcIpamPools {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpcIpamPools {
        let out = DataVpcIpamPools(std::rc::Rc::new(DataVpcIpamPools_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpcIpamPoolsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcIpamPoolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcIpamPoolsRef {
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

    #[doc = "Get a reference to the value of field `ipam_pools` after provisioning.\n"]
    pub fn ipam_pools(&self) -> terrars::SetRef<DataVpcIpamPoolsIpamPoolsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipam_pools", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolsIpamPoolsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_default_netmask_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_max_netmask_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_min_netmask_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_resource_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_import: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_scope_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_scope_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locale: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publicly_advertisable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ipam_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataVpcIpamPoolsIpamPoolsEl {
    #[doc = "Set the field `address_family`.\n"]
    pub fn set_address_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address_family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_default_netmask_length`.\n"]
    pub fn set_allocation_default_netmask_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.allocation_default_netmask_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_max_netmask_length`.\n"]
    pub fn set_allocation_max_netmask_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.allocation_max_netmask_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_min_netmask_length`.\n"]
    pub fn set_allocation_min_netmask_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.allocation_min_netmask_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_resource_tags`.\n"]
    pub fn set_allocation_resource_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allocation_resource_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_import`.\n"]
    pub fn set_auto_import(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_import = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_service`.\n"]
    pub fn set_aws_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_pool_id`.\n"]
    pub fn set_ipam_pool_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipam_pool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_scope_id`.\n"]
    pub fn set_ipam_scope_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipam_scope_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_scope_type`.\n"]
    pub fn set_ipam_scope_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipam_scope_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_depth`.\n"]
    pub fn set_pool_depth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pool_depth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_advertisable`.\n"]
    pub fn set_publicly_advertisable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.publicly_advertisable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_ipam_pool_id`.\n"]
    pub fn set_source_ipam_pool_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_ipam_pool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcIpamPoolsIpamPoolsEl {
    type O = terrars::BlockAssignable<DataVpcIpamPoolsIpamPoolsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolsIpamPoolsEl {}

impl BuildDataVpcIpamPoolsIpamPoolsEl {
    pub fn build(self) -> DataVpcIpamPoolsIpamPoolsEl {
        DataVpcIpamPoolsIpamPoolsEl {
            address_family: core::default::Default::default(),
            allocation_default_netmask_length: core::default::Default::default(),
            allocation_max_netmask_length: core::default::Default::default(),
            allocation_min_netmask_length: core::default::Default::default(),
            allocation_resource_tags: core::default::Default::default(),
            arn: core::default::Default::default(),
            auto_import: core::default::Default::default(),
            aws_service: core::default::Default::default(),
            description: core::default::Default::default(),
            id: core::default::Default::default(),
            ipam_pool_id: core::default::Default::default(),
            ipam_scope_id: core::default::Default::default(),
            ipam_scope_type: core::default::Default::default(),
            locale: core::default::Default::default(),
            pool_depth: core::default::Default::default(),
            publicly_advertisable: core::default::Default::default(),
            source_ipam_pool_id: core::default::Default::default(),
            state: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataVpcIpamPoolsIpamPoolsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolsIpamPoolsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpamPoolsIpamPoolsElRef {
        DataVpcIpamPoolsIpamPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolsIpamPoolsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.base))
    }

    #[doc = "Get a reference to the value of field `allocation_default_netmask_length` after provisioning.\n"]
    pub fn allocation_default_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_default_netmask_length", self.base))
    }

    #[doc = "Get a reference to the value of field `allocation_max_netmask_length` after provisioning.\n"]
    pub fn allocation_max_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_max_netmask_length", self.base))
    }

    #[doc = "Get a reference to the value of field `allocation_min_netmask_length` after provisioning.\n"]
    pub fn allocation_min_netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_min_netmask_length", self.base))
    }

    #[doc = "Get a reference to the value of field `allocation_resource_tags` after provisioning.\n"]
    pub fn allocation_resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.allocation_resource_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_import` after provisioning.\n"]
    pub fn auto_import(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_import", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_service` after provisioning.\n"]
    pub fn aws_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_service", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_id` after provisioning.\n"]
    pub fn ipam_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_type` after provisioning.\n"]
    pub fn ipam_scope_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_type", self.base))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locale", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_depth` after provisioning.\n"]
    pub fn pool_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `publicly_advertisable` after provisioning.\n"]
    pub fn publicly_advertisable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_advertisable", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ipam_pool_id` after provisioning.\n"]
    pub fn source_ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ipam_pool_id", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolsFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpamPoolsFilterEl { }

impl terrars::ToListMappable for DataVpcIpamPoolsFilterEl {
    type O = terrars::BlockAssignable<DataVpcIpamPoolsFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolsFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataVpcIpamPoolsFilterEl {
    pub fn build(self) -> DataVpcIpamPoolsFilterEl {
        DataVpcIpamPoolsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpcIpamPoolsFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolsFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpamPoolsFilterElRef {
        DataVpcIpamPoolsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolsFilterElRef {
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

#[derive(Serialize, Default)]
struct DataVpcIpamPoolsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataVpcIpamPoolsFilterEl>>,
}

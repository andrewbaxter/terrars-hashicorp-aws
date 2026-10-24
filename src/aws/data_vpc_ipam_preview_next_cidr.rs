use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcIpamPreviewNextCidrData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disallowed_cidrs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    ipam_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    netmask_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataVpcIpamPreviewNextCidrTimeoutsEl>,
}

struct DataVpcIpamPreviewNextCidr_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcIpamPreviewNextCidrData>,
}

#[derive(Clone)]
pub struct DataVpcIpamPreviewNextCidr(std::rc::Rc<DataVpcIpamPreviewNextCidr_>);

impl DataVpcIpamPreviewNextCidr {
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

    #[doc = "Set the field `disallowed_cidrs`.\n"]
    pub fn set_disallowed_cidrs(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().disallowed_cidrs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `netmask_length`.\n"]
    pub fn set_netmask_length(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().netmask_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataVpcIpamPreviewNextCidrTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disallowed_cidrs` after provisioning.\n"]
    pub fn disallowed_cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.disallowed_cidrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `netmask_length` after provisioning.\n"]
    pub fn netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPreviewNextCidrTimeoutsElRef {
        DataVpcIpamPreviewNextCidrTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataVpcIpamPreviewNextCidr {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpcIpamPreviewNextCidr { }

impl terrars::ToListMappable for DataVpcIpamPreviewNextCidr {
    type O = terrars::ListRef<DataVpcIpamPreviewNextCidrRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpcIpamPreviewNextCidr_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpc_ipam_preview_next_cidr".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcIpamPreviewNextCidr {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub ipam_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildDataVpcIpamPreviewNextCidr {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpcIpamPreviewNextCidr {
        let out = DataVpcIpamPreviewNextCidr(std::rc::Rc::new(DataVpcIpamPreviewNextCidr_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpcIpamPreviewNextCidrData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                disallowed_cidrs: core::default::Default::default(),
                id: core::default::Default::default(),
                ipam_pool_id: self.ipam_pool_id,
                netmask_length: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcIpamPreviewNextCidrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPreviewNextCidrRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcIpamPreviewNextCidrRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disallowed_cidrs` after provisioning.\n"]
    pub fn disallowed_cidrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.disallowed_cidrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `netmask_length` after provisioning.\n"]
    pub fn netmask_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.netmask_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPreviewNextCidrTimeoutsElRef {
        DataVpcIpamPreviewNextCidrTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPreviewNextCidrTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpamPreviewNextCidrTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcIpamPreviewNextCidrTimeoutsEl {
    type O = terrars::BlockAssignable<DataVpcIpamPreviewNextCidrTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPreviewNextCidrTimeoutsEl {}

impl BuildDataVpcIpamPreviewNextCidrTimeoutsEl {
    pub fn build(self) -> DataVpcIpamPreviewNextCidrTimeoutsEl {
        DataVpcIpamPreviewNextCidrTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataVpcIpamPreviewNextCidrTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPreviewNextCidrTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpamPreviewNextCidrTimeoutsElRef {
        DataVpcIpamPreviewNextCidrTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPreviewNextCidrTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

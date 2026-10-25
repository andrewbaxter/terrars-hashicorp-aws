#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcIpamsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataVpcIpams_Filter>>,
    dynamic: DataVpcIpamsDynamic,
}

struct DataVpcIpams_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcIpamsData_>,
}

#[derive(Clone)]
pub struct DataVpcIpams(std::rc::Rc<DataVpcIpams_>);

impl DataVpcIpams {
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

    #[doc = "Set the field `ipam_ids`.\n"]
    pub fn set_ipam_ids(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ipam_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataVpcIpams_Filter>>) -> Self {
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

    #[doc = "Get a reference to the value of field `ipam_ids` after provisioning.\n"]
    pub fn ipam_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipam_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipams` after provisioning.\n"]
    pub fn ipams(&self) -> terrars::ListRef<DataVpcIpams_IpamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipams", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpcIpams {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpcIpams { }

impl terrars::ToListMappable for DataVpcIpams {
    type O = terrars::ListRef<DataVpcIpamsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpcIpams_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpc_ipams".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcIpams {
    pub tf_id: std::string::String,
}

impl BuildDataVpcIpams {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpcIpams {
        let out = DataVpcIpams(std::rc::Rc::new(DataVpcIpams_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpcIpamsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                ipam_ids: core::default::Default::default(),
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcIpamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcIpamsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `ipam_ids` after provisioning.\n"]
    pub fn ipam_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipam_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipams` after provisioning.\n"]
    pub fn ipams(&self) -> terrars::ListRef<DataVpcIpams_IpamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipams", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataVpcIpamsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataVpcIpams_Filter>>,
}

#[derive(Serialize)]
pub struct DataVpcIpams_Ipams_OperatingRegions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpams_Ipams_OperatingRegions {
    #[doc = "Set the field `region_name`.\n"]
    pub fn set_region_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcIpams_Ipams_OperatingRegions {
    type O = terrars::BlockAssignable<DataVpcIpams_Ipams_OperatingRegions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpams_Ipams_OperatingRegions {}

impl BuildDataVpcIpams_Ipams_OperatingRegions {
    pub fn build(self) -> DataVpcIpams_Ipams_OperatingRegions {
        DataVpcIpams_Ipams_OperatingRegions { region_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataVpcIpams_Ipams_OperatingRegionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpams_Ipams_OperatingRegionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpams_Ipams_OperatingRegionsRef {
        DataVpcIpams_Ipams_OperatingRegionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpams_Ipams_OperatingRegionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpams_Ipams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_discovery_association_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_resource_discovery_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_private_gua: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipam_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metered_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_regions: std::option::Option<terrars::ListField<DataVpcIpams_Ipams_OperatingRegions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_default_scope_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_default_scope_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_discovery_association_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpams_Ipams {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_resource_discovery_association_id`.\n"]
    pub fn set_default_resource_discovery_association_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_resource_discovery_association_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_resource_discovery_id`.\n"]
    pub fn set_default_resource_discovery_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_resource_discovery_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_private_gua`.\n"]
    pub fn set_enable_private_gua(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_private_gua = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_region`.\n"]
    pub fn set_ipam_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipam_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metered_account`.\n"]
    pub fn set_metered_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metered_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_regions`.\n"]
    pub fn set_operating_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVpcIpams_Ipams_OperatingRegions>>,
    ) -> Self {
        self.operating_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner_id`.\n"]
    pub fn set_owner_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_default_scope_id`.\n"]
    pub fn set_private_default_scope_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_default_scope_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_default_scope_id`.\n"]
    pub fn set_public_default_scope_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_default_scope_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_discovery_association_count`.\n"]
    pub fn set_resource_discovery_association_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.resource_discovery_association_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope_count`.\n"]
    pub fn set_scope_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scope_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tier`.\n"]
    pub fn set_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcIpams_Ipams {
    type O = terrars::BlockAssignable<DataVpcIpams_Ipams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpams_Ipams {}

impl BuildDataVpcIpams_Ipams {
    pub fn build(self) -> DataVpcIpams_Ipams {
        DataVpcIpams_Ipams {
            arn: core::default::Default::default(),
            default_resource_discovery_association_id: core::default::Default::default(),
            default_resource_discovery_id: core::default::Default::default(),
            description: core::default::Default::default(),
            enable_private_gua: core::default::Default::default(),
            id: core::default::Default::default(),
            ipam_region: core::default::Default::default(),
            metered_account: core::default::Default::default(),
            operating_regions: core::default::Default::default(),
            owner_id: core::default::Default::default(),
            private_default_scope_id: core::default::Default::default(),
            public_default_scope_id: core::default::Default::default(),
            resource_discovery_association_count: core::default::Default::default(),
            scope_count: core::default::Default::default(),
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
            tier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVpcIpams_IpamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpams_IpamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpams_IpamsRef {
        DataVpcIpams_IpamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpams_IpamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_discovery_association_id` after provisioning.\n"]
    pub fn default_resource_discovery_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_resource_discovery_association_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `default_resource_discovery_id` after provisioning.\n"]
    pub fn default_resource_discovery_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_resource_discovery_id", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_private_gua` after provisioning.\n"]
    pub fn enable_private_gua(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_gua", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipam_region` after provisioning.\n"]
    pub fn ipam_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_region", self.base))
    }

    #[doc = "Get a reference to the value of field `metered_account` after provisioning.\n"]
    pub fn metered_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metered_account", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_regions` after provisioning.\n"]
    pub fn operating_regions(&self) -> terrars::ListRef<DataVpcIpams_Ipams_OperatingRegionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operating_regions", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }

    #[doc = "Get a reference to the value of field `private_default_scope_id` after provisioning.\n"]
    pub fn private_default_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_default_scope_id", self.base))
    }

    #[doc = "Get a reference to the value of field `public_default_scope_id` after provisioning.\n"]
    pub fn public_default_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_default_scope_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_discovery_association_count` after provisioning.\n"]
    pub fn resource_discovery_association_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_discovery_association_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scope_count` after provisioning.\n"]
    pub fn scope_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope_count", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }

    #[doc = "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpams_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpams_Filter { }

impl terrars::ToListMappable for DataVpcIpams_Filter {
    type O = terrars::BlockAssignable<DataVpcIpams_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpams_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataVpcIpams_Filter {
    pub fn build(self) -> DataVpcIpams_Filter {
        DataVpcIpams_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataVpcIpams_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpams_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpams_FilterRef {
        DataVpcIpams_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpams_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAlbData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataAlbTimeoutsEl>,
}

struct DataAlb_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAlbData>,
}

#[derive(Clone)]
pub struct DataAlb(Rc<DataAlb_>);

impl DataAlb {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().arn = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataAlbTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> ListRef<DataAlbAccessLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn_suffix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.desync_mitigation_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `drop_invalid_header_fields` after provisioning.\n"]
    pub fn drop_invalid_header_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.drop_invalid_header_fields", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_cross_zone_load_balancing` after provisioning.\n"]
    pub fn enable_cross_zone_load_balancing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_cross_zone_load_balancing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_deletion_protection` after provisioning.\n"]
    pub fn enable_deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_http2` after provisioning.\n"]
    pub fn enable_http2(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_http2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_waf_fail_open` after provisioning.\n"]
    pub fn enable_waf_fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_waf_fail_open", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `load_balancer_type` after provisioning.\n"]
    pub fn load_balancer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preserve_host_header` after provisioning.\n"]
    pub fn preserve_host_header(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_host_header", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> SetRef<DataAlbSubnetMappingElRef> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataAlbTimeoutsElRef {
        DataAlbTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Datasource for DataAlb {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataAlb {
    type O = ListRef<DataAlbRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAlb_ {
    fn extract_datasource_type(&self) -> String {
        "aws_alb".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAlb {
    pub tf_id: String,
}

impl BuildDataAlb {
    pub fn build(self, stack: &mut Stack) -> DataAlb {
        let out = DataAlb(Rc::new(DataAlb_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAlbData {
                provider: None,
                for_each: None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAlbRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAlbRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> ListRef<DataAlbAccessLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn_suffix` after provisioning.\n"]
    pub fn arn_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn_suffix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.desync_mitigation_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `drop_invalid_header_fields` after provisioning.\n"]
    pub fn drop_invalid_header_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.drop_invalid_header_fields", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_cross_zone_load_balancing` after provisioning.\n"]
    pub fn enable_cross_zone_load_balancing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_cross_zone_load_balancing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_deletion_protection` after provisioning.\n"]
    pub fn enable_deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_http2` after provisioning.\n"]
    pub fn enable_http2(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_http2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_waf_fail_open` after provisioning.\n"]
    pub fn enable_waf_fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_waf_fail_open", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `load_balancer_type` after provisioning.\n"]
    pub fn load_balancer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preserve_host_header` after provisioning.\n"]
    pub fn preserve_host_header(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_host_header", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> SetRef<DataAlbSubnetMappingElRef> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataAlbTimeoutsElRef {
        DataAlbTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAlbAccessLogsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

impl DataAlbAccessLogsEl {
    #[doc= "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbAccessLogsEl {
    type O = BlockAssignable<DataAlbAccessLogsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbAccessLogsEl {}

impl BuildDataAlbAccessLogsEl {
    pub fn build(self) -> DataAlbAccessLogsEl {
        DataAlbAccessLogsEl {
            bucket: core::default::Default::default(),
            enabled: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct DataAlbAccessLogsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbAccessLogsElRef {
    fn new(shared: StackShared, base: String) -> DataAlbAccessLogsElRef {
        DataAlbAccessLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbAccessLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbSubnetMappingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outpost_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ipv4_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
}

impl DataAlbSubnetMappingEl {
    #[doc= "Set the field `allocation_id`.\n"]
    pub fn set_allocation_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocation_id = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_address`.\n"]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }

    #[doc= "Set the field `outpost_id`.\n"]
    pub fn set_outpost_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.outpost_id = Some(v.into());
        self
    }

    #[doc= "Set the field `private_ipv4_address`.\n"]
    pub fn set_private_ipv4_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ipv4_address = Some(v.into());
        self
    }

    #[doc= "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbSubnetMappingEl {
    type O = BlockAssignable<DataAlbSubnetMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbSubnetMappingEl {}

impl BuildDataAlbSubnetMappingEl {
    pub fn build(self) -> DataAlbSubnetMappingEl {
        DataAlbSubnetMappingEl {
            allocation_id: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            outpost_id: core::default::Default::default(),
            private_ipv4_address: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct DataAlbSubnetMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbSubnetMappingElRef {
    fn new(shared: StackShared, base: String) -> DataAlbSubnetMappingElRef {
        DataAlbSubnetMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbSubnetMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allocation_id` after provisioning.\n"]
    pub fn allocation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_id", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc= "Get a reference to the value of field `outpost_id` after provisioning.\n"]
    pub fn outpost_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.outpost_id", self.base))
    }

    #[doc= "Get a reference to the value of field `private_ipv4_address` after provisioning.\n"]
    pub fn private_ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ipv4_address", self.base))
    }

    #[doc= "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlbTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataAlbTimeoutsEl {
    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataAlbTimeoutsEl {
    type O = BlockAssignable<DataAlbTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlbTimeoutsEl {}

impl BuildDataAlbTimeoutsEl {
    pub fn build(self) -> DataAlbTimeoutsEl {
        DataAlbTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataAlbTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAlbTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataAlbTimeoutsElRef {
        DataAlbTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlbTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataInstanceData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    get_password_data: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    get_user_data: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataInstanceFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataInstanceTimeoutsEl>,
    dynamic: DataInstanceDynamic,
}

struct DataInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataInstanceData>,
}

#[derive(Clone)]
pub struct DataInstance(std::rc::Rc<DataInstance_>);

impl DataInstance {
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

    #[doc = "Set the field `get_password_data`.\n"]
    pub fn set_get_password_data(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().get_password_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `get_user_data`.\n"]
    pub fn set_get_user_data(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().get_user_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_tags`.\n"]
    pub fn set_instance_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().instance_tags = std::option::Option::Some(v.into());
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
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataInstanceFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `ami` after provisioning.\n"]
    pub fn ami(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associate_public_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<DataInstanceCreditSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_stop` after provisioning.\n"]
    pub fn disable_api_stop(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_api_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_termination` after provisioning.\n"]
    pub fn disable_api_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_api_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_block_device` after provisioning.\n"]
    pub fn ebs_block_device(&self) -> terrars::SetRef<DataInstanceEbsBlockDeviceElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ebs_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<DataInstanceEnclaveOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_block_device` after provisioning.\n"]
    pub fn ephemeral_block_device(&self) -> terrars::ListRef<DataInstanceEphemeralBlockDeviceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `get_password_data` after provisioning.\n"]
    pub fn get_password_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.get_password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `get_user_data` after provisioning.\n"]
    pub fn get_user_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.get_user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_resource_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_state` after provisioning.\n"]
    pub fn instance_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_tags` after provisioning.\n"]
    pub fn instance_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.instance_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<DataInstanceMaintenanceOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<DataInstanceMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_data` after provisioning.\n"]
    pub fn password_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_partition_number` after provisioning.\n"]
    pub fn placement_partition_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_partition_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<DataInstancePrivateDnsNameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> terrars::SetRef<DataInstanceRootBlockDeviceElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.root_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_private_ips` after provisioning.\n"]
    pub fn secondary_private_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.secondary_private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dest_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataInstanceTimeoutsElRef {
        DataInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataInstance { }

impl terrars::ToListMappable for DataInstance {
    type O = terrars::ListRef<DataInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataInstance_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataInstance {
    pub tf_id: std::string::String,
}

impl BuildDataInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DataInstance {
        let out = DataInstance(std::rc::Rc::new(DataInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                get_password_data: core::default::Default::default(),
                get_user_data: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: core::default::Default::default(),
                instance_tags: core::default::Default::default(),
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

pub struct DataInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataInstanceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `ami` after provisioning.\n"]
    pub fn ami(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associate_public_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<DataInstanceCreditSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_stop` after provisioning.\n"]
    pub fn disable_api_stop(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_api_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_termination` after provisioning.\n"]
    pub fn disable_api_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_api_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_block_device` after provisioning.\n"]
    pub fn ebs_block_device(&self) -> terrars::SetRef<DataInstanceEbsBlockDeviceElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ebs_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<DataInstanceEnclaveOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_block_device` after provisioning.\n"]
    pub fn ephemeral_block_device(&self) -> terrars::ListRef<DataInstanceEphemeralBlockDeviceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `get_password_data` after provisioning.\n"]
    pub fn get_password_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.get_password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `get_user_data` after provisioning.\n"]
    pub fn get_user_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.get_user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_resource_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_state` after provisioning.\n"]
    pub fn instance_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_tags` after provisioning.\n"]
    pub fn instance_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.instance_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<DataInstanceMaintenanceOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<DataInstanceMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_data` after provisioning.\n"]
    pub fn password_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_partition_number` after provisioning.\n"]
    pub fn placement_partition_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_partition_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<DataInstancePrivateDnsNameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> terrars::SetRef<DataInstanceRootBlockDeviceElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.root_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_private_ips` after provisioning.\n"]
    pub fn secondary_private_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.secondary_private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dest_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataInstanceTimeoutsElRef {
        DataInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataInstanceCreditSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_credits: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataInstanceCreditSpecificationEl {
    #[doc = "Set the field `cpu_credits`.\n"]
    pub fn set_cpu_credits(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_credits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataInstanceCreditSpecificationEl {
    type O = terrars::BlockAssignable<DataInstanceCreditSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceCreditSpecificationEl {}

impl BuildDataInstanceCreditSpecificationEl {
    pub fn build(self) -> DataInstanceCreditSpecificationEl {
        DataInstanceCreditSpecificationEl { cpu_credits: core::default::Default::default() }
    }
}

pub struct DataInstanceCreditSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceCreditSpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstanceCreditSpecificationElRef {
        DataInstanceCreditSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceCreditSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_credits` after provisioning.\n"]
    pub fn cpu_credits(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_credits", self.base))
    }
}

#[derive(Serialize)]
pub struct DataInstanceEbsBlockDeviceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataInstanceEbsBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_id`.\n"]
    pub fn set_volume_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataInstanceEbsBlockDeviceEl {
    type O = terrars::BlockAssignable<DataInstanceEbsBlockDeviceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceEbsBlockDeviceEl {}

impl BuildDataInstanceEbsBlockDeviceEl {
    pub fn build(self) -> DataInstanceEbsBlockDeviceEl {
        DataInstanceEbsBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            device_name: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_id: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct DataInstanceEbsBlockDeviceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceEbsBlockDeviceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstanceEbsBlockDeviceElRef {
        DataInstanceEbsBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceEbsBlockDeviceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataInstanceEnclaveOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataInstanceEnclaveOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataInstanceEnclaveOptionsEl {
    type O = terrars::BlockAssignable<DataInstanceEnclaveOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceEnclaveOptionsEl {}

impl BuildDataInstanceEnclaveOptionsEl {
    pub fn build(self) -> DataInstanceEnclaveOptionsEl {
        DataInstanceEnclaveOptionsEl { enabled: core::default::Default::default() }
    }
}

pub struct DataInstanceEnclaveOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceEnclaveOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstanceEnclaveOptionsElRef {
        DataInstanceEnclaveOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceEnclaveOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataInstanceEphemeralBlockDeviceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_device: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataInstanceEphemeralBlockDeviceEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_device = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataInstanceEphemeralBlockDeviceEl {
    type O = terrars::BlockAssignable<DataInstanceEphemeralBlockDeviceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceEphemeralBlockDeviceEl {}

impl BuildDataInstanceEphemeralBlockDeviceEl {
    pub fn build(self) -> DataInstanceEphemeralBlockDeviceEl {
        DataInstanceEphemeralBlockDeviceEl {
            device_name: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
        }
    }
}

pub struct DataInstanceEphemeralBlockDeviceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceEphemeralBlockDeviceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstanceEphemeralBlockDeviceElRef {
        DataInstanceEphemeralBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceEphemeralBlockDeviceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataInstanceMaintenanceOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_recovery: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataInstanceMaintenanceOptionsEl {
    #[doc = "Set the field `auto_recovery`.\n"]
    pub fn set_auto_recovery(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_recovery = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataInstanceMaintenanceOptionsEl {
    type O = terrars::BlockAssignable<DataInstanceMaintenanceOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceMaintenanceOptionsEl {}

impl BuildDataInstanceMaintenanceOptionsEl {
    pub fn build(self) -> DataInstanceMaintenanceOptionsEl {
        DataInstanceMaintenanceOptionsEl { auto_recovery: core::default::Default::default() }
    }
}

pub struct DataInstanceMaintenanceOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceMaintenanceOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstanceMaintenanceOptionsElRef {
        DataInstanceMaintenanceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceMaintenanceOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_recovery` after provisioning.\n"]
    pub fn auto_recovery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery", self.base))
    }
}

#[derive(Serialize)]
pub struct DataInstanceMetadataOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_put_response_hop_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_tokens: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_metadata_tags: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataInstanceMetadataOptionsEl {
    #[doc = "Set the field `http_endpoint`.\n"]
    pub fn set_http_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_put_response_hop_limit`.\n"]
    pub fn set_http_put_response_hop_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.http_put_response_hop_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_tokens`.\n"]
    pub fn set_http_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_metadata_tags`.\n"]
    pub fn set_instance_metadata_tags(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_metadata_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataInstanceMetadataOptionsEl {
    type O = terrars::BlockAssignable<DataInstanceMetadataOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceMetadataOptionsEl {}

impl BuildDataInstanceMetadataOptionsEl {
    pub fn build(self) -> DataInstanceMetadataOptionsEl {
        DataInstanceMetadataOptionsEl {
            http_endpoint: core::default::Default::default(),
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
            instance_metadata_tags: core::default::Default::default(),
        }
    }
}

pub struct DataInstanceMetadataOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceMetadataOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstanceMetadataOptionsElRef {
        DataInstanceMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceMetadataOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_endpoint` after provisioning.\n"]
    pub fn http_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `http_put_response_hop_limit` after provisioning.\n"]
    pub fn http_put_response_hop_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_put_response_hop_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `http_tokens` after provisioning.\n"]
    pub fn http_tokens(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_metadata_tags` after provisioning.\n"]
    pub fn instance_metadata_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_metadata_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataInstancePrivateDnsNameOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_a_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_aaaa_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataInstancePrivateDnsNameOptionsEl {
    #[doc = "Set the field `enable_resource_name_dns_a_record`.\n"]
    pub fn set_enable_resource_name_dns_a_record(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_resource_name_dns_a_record = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_resource_name_dns_aaaa_record`.\n"]
    pub fn set_enable_resource_name_dns_aaaa_record(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_resource_name_dns_aaaa_record = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname_type`.\n"]
    pub fn set_hostname_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataInstancePrivateDnsNameOptionsEl {
    type O = terrars::BlockAssignable<DataInstancePrivateDnsNameOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstancePrivateDnsNameOptionsEl {}

impl BuildDataInstancePrivateDnsNameOptionsEl {
    pub fn build(self) -> DataInstancePrivateDnsNameOptionsEl {
        DataInstancePrivateDnsNameOptionsEl {
            enable_resource_name_dns_a_record: core::default::Default::default(),
            enable_resource_name_dns_aaaa_record: core::default::Default::default(),
            hostname_type: core::default::Default::default(),
        }
    }
}

pub struct DataInstancePrivateDnsNameOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstancePrivateDnsNameOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstancePrivateDnsNameOptionsElRef {
        DataInstancePrivateDnsNameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstancePrivateDnsNameOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_resource_name_dns_a_record", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_aaaa_record` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_resource_name_dns_aaaa_record", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname_type` after provisioning.\n"]
    pub fn hostname_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataInstanceRootBlockDeviceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataInstanceRootBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_id`.\n"]
    pub fn set_volume_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataInstanceRootBlockDeviceEl {
    type O = terrars::BlockAssignable<DataInstanceRootBlockDeviceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceRootBlockDeviceEl {}

impl BuildDataInstanceRootBlockDeviceEl {
    pub fn build(self) -> DataInstanceRootBlockDeviceEl {
        DataInstanceRootBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            device_name: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_id: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct DataInstanceRootBlockDeviceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceRootBlockDeviceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstanceRootBlockDeviceElRef {
        DataInstanceRootBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceRootBlockDeviceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataInstanceFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataInstanceFilterEl { }

impl terrars::ToListMappable for DataInstanceFilterEl {
    type O = terrars::BlockAssignable<DataInstanceFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataInstanceFilterEl {
    pub fn build(self) -> DataInstanceFilterEl {
        DataInstanceFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataInstanceFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstanceFilterElRef {
        DataInstanceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceFilterElRef {
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

#[derive(Serialize)]
pub struct DataInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataInstanceTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<DataInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstanceTimeoutsEl {}

impl BuildDataInstanceTimeoutsEl {
    pub fn build(self) -> DataInstanceTimeoutsEl {
        DataInstanceTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataInstanceTimeoutsElRef {
        DataInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstanceTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataInstanceDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataInstanceFilterEl>>,
}

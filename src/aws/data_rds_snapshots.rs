#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRdsSnapshotsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_instance_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_public: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_shared: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataRdsSnapshots_Filter>>,
    dynamic: DataRdsSnapshotsDynamic,
}

struct DataRdsSnapshots_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRdsSnapshotsData_>,
}

#[derive(Clone)]
pub struct DataRdsSnapshots(std::rc::Rc<DataRdsSnapshots_>);

impl DataRdsSnapshots {
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

    #[doc = "Set the field `db_instance_identifier`.\n"]
    pub fn set_db_instance_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().db_instance_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_snapshot_identifier`.\n"]
    pub fn set_db_snapshot_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().db_snapshot_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_public`.\n"]
    pub fn set_include_public(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_public = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_shared`.\n"]
    pub fn set_include_shared(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_shared = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_type`.\n"]
    pub fn set_snapshot_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().snapshot_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataRdsSnapshots_Filter>>) -> Self {
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

    #[doc = "Get a reference to the value of field `db_instance_identifier` after provisioning.\n"]
    pub fn db_instance_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_instance_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_snapshot_identifier` after provisioning.\n"]
    pub fn db_snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_public` after provisioning.\n"]
    pub fn include_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_public", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_shared` after provisioning.\n"]
    pub fn include_shared(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_shared", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_type` after provisioning.\n"]
    pub fn snapshot_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshots` after provisioning.\n"]
    pub fn snapshots(&self) -> terrars::ListRef<DataRdsSnapshots_SnapshotsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshots", self.extract_ref()))
    }
}

impl terrars::Referable for DataRdsSnapshots {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRdsSnapshots { }

impl terrars::ToListMappable for DataRdsSnapshots {
    type O = terrars::ListRef<DataRdsSnapshotsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRdsSnapshots_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_rds_snapshots".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRdsSnapshots {
    pub tf_id: std::string::String,
}

impl BuildDataRdsSnapshots {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRdsSnapshots {
        let out = DataRdsSnapshots(std::rc::Rc::new(DataRdsSnapshots_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRdsSnapshotsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                db_instance_identifier: core::default::Default::default(),
                db_snapshot_identifier: core::default::Default::default(),
                include_public: core::default::Default::default(),
                include_shared: core::default::Default::default(),
                region: core::default::Default::default(),
                snapshot_type: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRdsSnapshotsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRdsSnapshotsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRdsSnapshotsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `db_instance_identifier` after provisioning.\n"]
    pub fn db_instance_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_instance_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_snapshot_identifier` after provisioning.\n"]
    pub fn db_snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_snapshot_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_public` after provisioning.\n"]
    pub fn include_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_public", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_shared` after provisioning.\n"]
    pub fn include_shared(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_shared", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_type` after provisioning.\n"]
    pub fn snapshot_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshots` after provisioning.\n"]
    pub fn snapshots(&self) -> terrars::ListRef<DataRdsSnapshots_SnapshotsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshots", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataRdsSnapshotsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataRdsSnapshots_Filter>>,
}

#[derive(Serialize)]
pub struct DataRdsSnapshots_Snapshots_TagList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRdsSnapshots_Snapshots_TagList {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRdsSnapshots_Snapshots_TagList {
    type O = terrars::BlockAssignable<DataRdsSnapshots_Snapshots_TagList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRdsSnapshots_Snapshots_TagList {}

impl BuildDataRdsSnapshots_Snapshots_TagList {
    pub fn build(self) -> DataRdsSnapshots_Snapshots_TagList {
        DataRdsSnapshots_Snapshots_TagList {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRdsSnapshots_Snapshots_TagListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRdsSnapshots_Snapshots_TagListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRdsSnapshots_Snapshots_TagListRef {
        DataRdsSnapshots_Snapshots_TagListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRdsSnapshots_Snapshots_TagListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRdsSnapshots_Snapshots {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_storage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_instance_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_snapshot_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    option_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    original_snapshot_create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_db_snapshot_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_list: std::option::Option<terrars::ListField<DataRdsSnapshots_Snapshots_TagList>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataRdsSnapshots_Snapshots {
    #[doc = "Set the field `allocated_storage`.\n"]
    pub fn set_allocated_storage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.allocated_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_instance_identifier`.\n"]
    pub fn set_db_instance_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.db_instance_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_snapshot_arn`.\n"]
    pub fn set_db_snapshot_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_snapshot_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_snapshot_identifier`.\n"]
    pub fn set_db_snapshot_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.db_snapshot_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.engine_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `license_model`.\n"]
    pub fn set_license_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.license_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `option_group_name`.\n"]
    pub fn set_option_group_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.option_group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `original_snapshot_create_time`.\n"]
    pub fn set_original_snapshot_create_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.original_snapshot_create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_create_time`.\n"]
    pub fn set_snapshot_create_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.snapshot_create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_type`.\n"]
    pub fn set_snapshot_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_db_snapshot_identifier`.\n"]
    pub fn set_source_db_snapshot_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_db_snapshot_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_region`.\n"]
    pub fn set_source_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_list`.\n"]
    pub fn set_tag_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataRdsSnapshots_Snapshots_TagList>>,
    ) -> Self {
        self.tag_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataRdsSnapshots_Snapshots {
    type O = terrars::BlockAssignable<DataRdsSnapshots_Snapshots>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRdsSnapshots_Snapshots {}

impl BuildDataRdsSnapshots_Snapshots {
    pub fn build(self) -> DataRdsSnapshots_Snapshots {
        DataRdsSnapshots_Snapshots {
            allocated_storage: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
            db_instance_identifier: core::default::Default::default(),
            db_snapshot_arn: core::default::Default::default(),
            db_snapshot_identifier: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            engine: core::default::Default::default(),
            engine_version: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            license_model: core::default::Default::default(),
            option_group_name: core::default::Default::default(),
            original_snapshot_create_time: core::default::Default::default(),
            port: core::default::Default::default(),
            snapshot_create_time: core::default::Default::default(),
            snapshot_type: core::default::Default::default(),
            source_db_snapshot_identifier: core::default::Default::default(),
            source_region: core::default::Default::default(),
            status: core::default::Default::default(),
            storage_type: core::default::Default::default(),
            tag_list: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataRdsSnapshots_SnapshotsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRdsSnapshots_SnapshotsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRdsSnapshots_SnapshotsRef {
        DataRdsSnapshots_SnapshotsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRdsSnapshots_SnapshotsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_storage` after provisioning.\n"]
    pub fn allocated_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `db_instance_identifier` after provisioning.\n"]
    pub fn db_instance_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_instance_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `db_snapshot_arn` after provisioning.\n"]
    pub fn db_snapshot_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_snapshot_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `db_snapshot_identifier` after provisioning.\n"]
    pub fn db_snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_snapshot_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.base))
    }

    #[doc = "Get a reference to the value of field `option_group_name` after provisioning.\n"]
    pub fn option_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.option_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `original_snapshot_create_time` after provisioning.\n"]
    pub fn original_snapshot_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.original_snapshot_create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_create_time` after provisioning.\n"]
    pub fn snapshot_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_type` after provisioning.\n"]
    pub fn snapshot_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_db_snapshot_identifier` after provisioning.\n"]
    pub fn source_db_snapshot_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_db_snapshot_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `source_region` after provisioning.\n"]
    pub fn source_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_region", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_list` after provisioning.\n"]
    pub fn tag_list(&self) -> terrars::ListRef<DataRdsSnapshots_Snapshots_TagListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_list", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRdsSnapshots_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataRdsSnapshots_Filter { }

impl terrars::ToListMappable for DataRdsSnapshots_Filter {
    type O = terrars::BlockAssignable<DataRdsSnapshots_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRdsSnapshots_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataRdsSnapshots_Filter {
    pub fn build(self) -> DataRdsSnapshots_Filter {
        DataRdsSnapshots_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataRdsSnapshots_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRdsSnapshots_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataRdsSnapshots_FilterRef {
        DataRdsSnapshots_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRdsSnapshots_FilterRef {
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

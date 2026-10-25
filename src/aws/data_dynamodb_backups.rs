#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDynamodbBackupsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_range_lower_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_range_upper_bound: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataDynamodbBackups_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDynamodbBackupsData_>,
}

#[derive(Clone)]
pub struct DataDynamodbBackups(std::rc::Rc<DataDynamodbBackups_>);

impl DataDynamodbBackups {
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

    #[doc = "Set the field `backup_type`.\n"]
    pub fn set_backup_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().backup_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_name`.\n"]
    pub fn set_table_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().table_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_range_lower_bound`.\n"]
    pub fn set_time_range_lower_bound(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().time_range_lower_bound = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_range_upper_bound`.\n"]
    pub fn set_time_range_upper_bound(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().time_range_upper_bound = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `backup_summaries` after provisioning.\n"]
    pub fn backup_summaries(&self) -> terrars::ListRef<DataDynamodbBackups_BackupSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_summaries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_type` after provisioning.\n"]
    pub fn backup_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_range_lower_bound` after provisioning.\n"]
    pub fn time_range_lower_bound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_range_lower_bound", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_range_upper_bound` after provisioning.\n"]
    pub fn time_range_upper_bound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_range_upper_bound", self.extract_ref()))
    }
}

impl terrars::Referable for DataDynamodbBackups {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDynamodbBackups { }

impl terrars::ToListMappable for DataDynamodbBackups {
    type O = terrars::ListRef<DataDynamodbBackupsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDynamodbBackups_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_dynamodb_backups".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDynamodbBackups {
    pub tf_id: std::string::String,
}

impl BuildDataDynamodbBackups {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDynamodbBackups {
        let out = DataDynamodbBackups(std::rc::Rc::new(DataDynamodbBackups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDynamodbBackupsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                backup_type: core::default::Default::default(),
                region: core::default::Default::default(),
                table_name: core::default::Default::default(),
                time_range_lower_bound: core::default::Default::default(),
                time_range_upper_bound: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDynamodbBackupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbBackupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDynamodbBackupsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `backup_summaries` after provisioning.\n"]
    pub fn backup_summaries(&self) -> terrars::ListRef<DataDynamodbBackups_BackupSummariesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_summaries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_type` after provisioning.\n"]
    pub fn backup_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_range_lower_bound` after provisioning.\n"]
    pub fn time_range_lower_bound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_range_lower_bound", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_range_upper_bound` after provisioning.\n"]
    pub fn time_range_upper_bound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_range_upper_bound", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDynamodbBackups_BackupSummaries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_creation_date_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_expiry_date_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_size_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDynamodbBackups_BackupSummaries {
    #[doc = "Set the field `backup_arn`.\n"]
    pub fn set_backup_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_creation_date_time`.\n"]
    pub fn set_backup_creation_date_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.backup_creation_date_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_expiry_date_time`.\n"]
    pub fn set_backup_expiry_date_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.backup_expiry_date_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_name`.\n"]
    pub fn set_backup_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_size_bytes`.\n"]
    pub fn set_backup_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.backup_size_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_status`.\n"]
    pub fn set_backup_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_type`.\n"]
    pub fn set_backup_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_arn`.\n"]
    pub fn set_table_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_id`.\n"]
    pub fn set_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_name`.\n"]
    pub fn set_table_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDynamodbBackups_BackupSummaries {
    type O = terrars::BlockAssignable<DataDynamodbBackups_BackupSummaries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDynamodbBackups_BackupSummaries {}

impl BuildDataDynamodbBackups_BackupSummaries {
    pub fn build(self) -> DataDynamodbBackups_BackupSummaries {
        DataDynamodbBackups_BackupSummaries {
            backup_arn: core::default::Default::default(),
            backup_creation_date_time: core::default::Default::default(),
            backup_expiry_date_time: core::default::Default::default(),
            backup_name: core::default::Default::default(),
            backup_size_bytes: core::default::Default::default(),
            backup_status: core::default::Default::default(),
            backup_type: core::default::Default::default(),
            table_arn: core::default::Default::default(),
            table_id: core::default::Default::default(),
            table_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDynamodbBackups_BackupSummariesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDynamodbBackups_BackupSummariesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDynamodbBackups_BackupSummariesRef {
        DataDynamodbBackups_BackupSummariesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDynamodbBackups_BackupSummariesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_arn` after provisioning.\n"]
    pub fn backup_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_creation_date_time` after provisioning.\n"]
    pub fn backup_creation_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_creation_date_time", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_expiry_date_time` after provisioning.\n"]
    pub fn backup_expiry_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_expiry_date_time", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_name` after provisioning.\n"]
    pub fn backup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_name", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_size_bytes` after provisioning.\n"]
    pub fn backup_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_size_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_status` after provisioning.\n"]
    pub fn backup_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_status", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_type` after provisioning.\n"]
    pub fn backup_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_type", self.base))
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_id` after provisioning.\n"]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

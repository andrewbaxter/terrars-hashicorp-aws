#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DmsS3EndpointData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_column_name: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_trailing_padding_character: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_folder: std::option::Option<terrars::PrimField<std::string::String>>,
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canned_acl_for_objects: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_inserts_and_updates: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_inserts_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_max_batch_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_min_file_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_no_sup_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_null_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_row_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_page_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_partition_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_partition_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_partition_sequence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_partition_timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detach_target_on_lob_lookup_failure_parquet: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dict_page_size_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_statistics: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    endpoint_id: terrars::PrimField<std::string::String>,
    endpoint_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_table_definition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glue_catalog_generation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_header_rows: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_op_for_full_load: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_file_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_timestamp_in_millisecond: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_transactions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rfc_4180: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_group_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    service_access_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_column_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_csv_no_sup_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_task_start_time_for_full_load_timestamp: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DmsS3Endpoint_Timeouts>,
}

struct DmsS3Endpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DmsS3EndpointData_>,
}

#[derive(Clone)]
pub struct DmsS3Endpoint(std::rc::Rc<DmsS3Endpoint_>);

impl DmsS3Endpoint {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `add_column_name`.\n"]
    pub fn set_add_column_name(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().add_column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `add_trailing_padding_character`.\n"]
    pub fn set_add_trailing_padding_character(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().add_trailing_padding_character = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_folder`.\n"]
    pub fn set_bucket_folder(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bucket_folder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canned_acl_for_objects`.\n"]
    pub fn set_canned_acl_for_objects(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().canned_acl_for_objects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_inserts_and_updates`.\n"]
    pub fn set_cdc_inserts_and_updates(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cdc_inserts_and_updates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_inserts_only`.\n"]
    pub fn set_cdc_inserts_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cdc_inserts_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_max_batch_interval`.\n"]
    pub fn set_cdc_max_batch_interval(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cdc_max_batch_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_min_file_size`.\n"]
    pub fn set_cdc_min_file_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cdc_min_file_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_path`.\n"]
    pub fn set_cdc_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cdc_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_delimiter`.\n"]
    pub fn set_csv_delimiter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().csv_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_no_sup_value`.\n"]
    pub fn set_csv_no_sup_value(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().csv_no_sup_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_null_value`.\n"]
    pub fn set_csv_null_value(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().csv_null_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_row_delimiter`.\n"]
    pub fn set_csv_row_delimiter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().csv_row_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_format`.\n"]
    pub fn set_data_format(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_page_size`.\n"]
    pub fn set_data_page_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().data_page_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_delimiter`.\n"]
    pub fn set_date_partition_delimiter(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().date_partition_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_enabled`.\n"]
    pub fn set_date_partition_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().date_partition_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_sequence`.\n"]
    pub fn set_date_partition_sequence(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().date_partition_sequence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_timezone`.\n"]
    pub fn set_date_partition_timezone(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().date_partition_timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detach_target_on_lob_lookup_failure_parquet`.\n"]
    pub fn set_detach_target_on_lob_lookup_failure_parquet(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().detach_target_on_lob_lookup_failure_parquet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dict_page_size_limit`.\n"]
    pub fn set_dict_page_size_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().dict_page_size_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_statistics`.\n"]
    pub fn set_enable_statistics(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_statistics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encoding_type`.\n"]
    pub fn set_encoding_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().encoding_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().encryption_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_table_definition`.\n"]
    pub fn set_external_table_definition(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().external_table_definition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `glue_catalog_generation`.\n"]
    pub fn set_glue_catalog_generation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().glue_catalog_generation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_header_rows`.\n"]
    pub fn set_ignore_header_rows(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ignore_header_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_op_for_full_load`.\n"]
    pub fn set_include_op_for_full_load(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_op_for_full_load = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_file_size`.\n"]
    pub fn set_max_file_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_file_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parquet_timestamp_in_millisecond`.\n"]
    pub fn set_parquet_timestamp_in_millisecond(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().parquet_timestamp_in_millisecond = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parquet_version`.\n"]
    pub fn set_parquet_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parquet_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_transactions`.\n"]
    pub fn set_preserve_transactions(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().preserve_transactions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rfc_4180`.\n"]
    pub fn set_rfc_4180(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().rfc_4180 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_group_length`.\n"]
    pub fn set_row_group_length(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().row_group_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_encryption_kms_key_id`.\n"]
    pub fn set_server_side_encryption_kms_key_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().server_side_encryption_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ssl_mode = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp_column_name`.\n"]
    pub fn set_timestamp_column_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timestamp_column_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_csv_no_sup_value`.\n"]
    pub fn set_use_csv_no_sup_value(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_csv_no_sup_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_task_start_time_for_full_load_timestamp`.\n"]
    pub fn set_use_task_start_time_for_full_load_timestamp(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().use_task_start_time_for_full_load_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DmsS3Endpoint_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `add_column_name` after provisioning.\n"]
    pub fn add_column_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.add_column_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `add_trailing_padding_character` after provisioning.\n"]
    pub fn add_trailing_padding_character(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.add_trailing_padding_character", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bucket_folder` after provisioning.\n"]
    pub fn bucket_folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `canned_acl_for_objects` after provisioning.\n"]
    pub fn canned_acl_for_objects(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl_for_objects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_and_updates` after provisioning.\n"]
    pub fn cdc_inserts_and_updates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_and_updates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_only` after provisioning.\n"]
    pub fn cdc_inserts_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_max_batch_interval` after provisioning.\n"]
    pub fn cdc_max_batch_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_max_batch_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_min_file_size` after provisioning.\n"]
    pub fn cdc_min_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_min_file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_path` after provisioning.\n"]
    pub fn cdc_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_delimiter` after provisioning.\n"]
    pub fn csv_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_no_sup_value` after provisioning.\n"]
    pub fn csv_no_sup_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_no_sup_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_null_value` after provisioning.\n"]
    pub fn csv_null_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_null_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_row_delimiter` after provisioning.\n"]
    pub fn csv_row_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_row_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_page_size` after provisioning.\n"]
    pub fn data_page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_page_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_delimiter` after provisioning.\n"]
    pub fn date_partition_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_enabled` after provisioning.\n"]
    pub fn date_partition_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_sequence` after provisioning.\n"]
    pub fn date_partition_sequence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_sequence", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_timezone` after provisioning.\n"]
    pub fn date_partition_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detach_target_on_lob_lookup_failure_parquet` after provisioning.\n"]
    pub fn detach_target_on_lob_lookup_failure_parquet(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.detach_target_on_lob_lookup_failure_parquet", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dict_page_size_limit` after provisioning.\n"]
    pub fn dict_page_size_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dict_page_size_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_statistics` after provisioning.\n"]
    pub fn enable_statistics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encoding_type` after provisioning.\n"]
    pub fn encoding_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_display_name` after provisioning.\n"]
    pub fn engine_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_table_definition` after provisioning.\n"]
    pub fn external_table_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_table_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_catalog_generation` after provisioning.\n"]
    pub fn glue_catalog_generation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.glue_catalog_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_header_rows` after provisioning.\n"]
    pub fn ignore_header_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_header_rows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_op_for_full_load` after provisioning.\n"]
    pub fn include_op_for_full_load(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_op_for_full_load", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parquet_timestamp_in_millisecond` after provisioning.\n"]
    pub fn parquet_timestamp_in_millisecond(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.parquet_timestamp_in_millisecond", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parquet_version` after provisioning.\n"]
    pub fn parquet_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parquet_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_transactions` after provisioning.\n"]
    pub fn preserve_transactions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_transactions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rfc_4180` after provisioning.\n"]
    pub fn rfc_4180(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rfc_4180", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_group_length` after provisioning.\n"]
    pub fn row_group_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_group_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_kms_key_id` after provisioning.\n"]
    pub fn server_side_encryption_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timestamp_column_name` after provisioning.\n"]
    pub fn timestamp_column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp_column_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_csv_no_sup_value` after provisioning.\n"]
    pub fn use_csv_no_sup_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_csv_no_sup_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_task_start_time_for_full_load_timestamp` after provisioning.\n"]
    pub fn use_task_start_time_for_full_load_timestamp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_task_start_time_for_full_load_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsS3Endpoint_TimeoutsRef {
        DmsS3Endpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DmsS3Endpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DmsS3Endpoint { }

impl terrars::ToListMappable for DmsS3Endpoint {
    type O = terrars::ListRef<DmsS3EndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DmsS3Endpoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dms_s3_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDmsS3Endpoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub endpoint_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub endpoint_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_access_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildDmsS3Endpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> DmsS3Endpoint {
        let out = DmsS3Endpoint(std::rc::Rc::new(DmsS3Endpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DmsS3EndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                add_column_name: core::default::Default::default(),
                add_trailing_padding_character: core::default::Default::default(),
                bucket_folder: core::default::Default::default(),
                bucket_name: self.bucket_name,
                canned_acl_for_objects: core::default::Default::default(),
                cdc_inserts_and_updates: core::default::Default::default(),
                cdc_inserts_only: core::default::Default::default(),
                cdc_max_batch_interval: core::default::Default::default(),
                cdc_min_file_size: core::default::Default::default(),
                cdc_path: core::default::Default::default(),
                certificate_arn: core::default::Default::default(),
                compression_type: core::default::Default::default(),
                csv_delimiter: core::default::Default::default(),
                csv_no_sup_value: core::default::Default::default(),
                csv_null_value: core::default::Default::default(),
                csv_row_delimiter: core::default::Default::default(),
                data_format: core::default::Default::default(),
                data_page_size: core::default::Default::default(),
                date_partition_delimiter: core::default::Default::default(),
                date_partition_enabled: core::default::Default::default(),
                date_partition_sequence: core::default::Default::default(),
                date_partition_timezone: core::default::Default::default(),
                detach_target_on_lob_lookup_failure_parquet: core::default::Default::default(),
                dict_page_size_limit: core::default::Default::default(),
                enable_statistics: core::default::Default::default(),
                encoding_type: core::default::Default::default(),
                encryption_mode: core::default::Default::default(),
                endpoint_id: self.endpoint_id,
                endpoint_type: self.endpoint_type,
                expected_bucket_owner: core::default::Default::default(),
                external_table_definition: core::default::Default::default(),
                glue_catalog_generation: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_header_rows: core::default::Default::default(),
                include_op_for_full_load: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                max_file_size: core::default::Default::default(),
                parquet_timestamp_in_millisecond: core::default::Default::default(),
                parquet_version: core::default::Default::default(),
                preserve_transactions: core::default::Default::default(),
                region: core::default::Default::default(),
                rfc_4180: core::default::Default::default(),
                row_group_length: core::default::Default::default(),
                server_side_encryption_kms_key_id: core::default::Default::default(),
                service_access_role_arn: self.service_access_role_arn,
                ssl_mode: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timestamp_column_name: core::default::Default::default(),
                use_csv_no_sup_value: core::default::Default::default(),
                use_task_start_time_for_full_load_timestamp: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DmsS3EndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsS3EndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DmsS3EndpointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add_column_name` after provisioning.\n"]
    pub fn add_column_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.add_column_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `add_trailing_padding_character` after provisioning.\n"]
    pub fn add_trailing_padding_character(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.add_trailing_padding_character", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bucket_folder` after provisioning.\n"]
    pub fn bucket_folder(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `canned_acl_for_objects` after provisioning.\n"]
    pub fn canned_acl_for_objects(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_acl_for_objects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_and_updates` after provisioning.\n"]
    pub fn cdc_inserts_and_updates(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_and_updates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_only` after provisioning.\n"]
    pub fn cdc_inserts_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_max_batch_interval` after provisioning.\n"]
    pub fn cdc_max_batch_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_max_batch_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_min_file_size` after provisioning.\n"]
    pub fn cdc_min_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_min_file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_path` after provisioning.\n"]
    pub fn cdc_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_delimiter` after provisioning.\n"]
    pub fn csv_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_no_sup_value` after provisioning.\n"]
    pub fn csv_no_sup_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_no_sup_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_null_value` after provisioning.\n"]
    pub fn csv_null_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_null_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_row_delimiter` after provisioning.\n"]
    pub fn csv_row_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.csv_row_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_page_size` after provisioning.\n"]
    pub fn data_page_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_page_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_delimiter` after provisioning.\n"]
    pub fn date_partition_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_enabled` after provisioning.\n"]
    pub fn date_partition_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_sequence` after provisioning.\n"]
    pub fn date_partition_sequence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_sequence", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_timezone` after provisioning.\n"]
    pub fn date_partition_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_partition_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detach_target_on_lob_lookup_failure_parquet` after provisioning.\n"]
    pub fn detach_target_on_lob_lookup_failure_parquet(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.detach_target_on_lob_lookup_failure_parquet", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dict_page_size_limit` after provisioning.\n"]
    pub fn dict_page_size_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dict_page_size_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_statistics` after provisioning.\n"]
    pub fn enable_statistics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encoding_type` after provisioning.\n"]
    pub fn encoding_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_display_name` after provisioning.\n"]
    pub fn engine_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_table_definition` after provisioning.\n"]
    pub fn external_table_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_table_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_catalog_generation` after provisioning.\n"]
    pub fn glue_catalog_generation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.glue_catalog_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_header_rows` after provisioning.\n"]
    pub fn ignore_header_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_header_rows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_op_for_full_load` after provisioning.\n"]
    pub fn include_op_for_full_load(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_op_for_full_load", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parquet_timestamp_in_millisecond` after provisioning.\n"]
    pub fn parquet_timestamp_in_millisecond(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.parquet_timestamp_in_millisecond", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parquet_version` after provisioning.\n"]
    pub fn parquet_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parquet_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_transactions` after provisioning.\n"]
    pub fn preserve_transactions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_transactions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rfc_4180` after provisioning.\n"]
    pub fn rfc_4180(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rfc_4180", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_group_length` after provisioning.\n"]
    pub fn row_group_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_group_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_kms_key_id` after provisioning.\n"]
    pub fn server_side_encryption_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_side_encryption_kms_key_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timestamp_column_name` after provisioning.\n"]
    pub fn timestamp_column_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp_column_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_csv_no_sup_value` after provisioning.\n"]
    pub fn use_csv_no_sup_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_csv_no_sup_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_task_start_time_for_full_load_timestamp` after provisioning.\n"]
    pub fn use_task_start_time_for_full_load_timestamp(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_task_start_time_for_full_load_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsS3Endpoint_TimeoutsRef {
        DmsS3Endpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DmsS3Endpoint_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsS3Endpoint_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsS3Endpoint_Timeouts {
    type O = terrars::BlockAssignable<DmsS3Endpoint_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsS3Endpoint_Timeouts {}

impl BuildDmsS3Endpoint_Timeouts {
    pub fn build(self) -> DmsS3Endpoint_Timeouts {
        DmsS3Endpoint_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsS3Endpoint_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsS3Endpoint_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsS3Endpoint_TimeoutsRef {
        DmsS3Endpoint_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsS3Endpoint_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

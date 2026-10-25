#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TimestreaminfluxdbDbClusterData_ {
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
    allocated_storage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    db_instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_parameter_group_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publicly_accessible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    vpc_security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    vpc_subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_delivery_configuration: std::option::Option<std::vec::Vec<TimestreaminfluxdbDbCluster_LogDeliveryConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_schedule: std::option::Option<std::vec::Vec<TimestreaminfluxdbDbCluster_MaintenanceSchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<TimestreaminfluxdbDbCluster_Timeouts>,
    dynamic: TimestreaminfluxdbDbClusterDynamic,
}

struct TimestreaminfluxdbDbCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TimestreaminfluxdbDbClusterData_>,
}

#[derive(Clone)]
pub struct TimestreaminfluxdbDbCluster(std::rc::Rc<TimestreaminfluxdbDbCluster_>);

impl TimestreaminfluxdbDbCluster {
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

    #[doc =
        "Set the field `allocated_storage`.\nThe amount of storage to allocate for your DB storage type in GiB (gibibytes).\n\t\t\t\t\tThis field is forbidden for InfluxDB V3 clusters (when using an InfluxDB V3 db parameter group)."]
    pub fn set_allocated_storage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().allocated_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bucket`.\nName of the initial InfluxDB bucket. All InfluxDB data is stored in a bucket.\n\t\t\t\t\tA bucket combines the concept of a database and a retention period (the duration of time\n\t\t\t\t\tthat each data point persists). A bucket belongs to an organization. Along with organization,\n\t\t\t\t\tusername, and password, this argument will be stored in the secret referred to by the\n\t\t\t\t\tinflux_auth_parameters_secret_arn attribute. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group)."]
    pub fn set_bucket(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `db_parameter_group_identifier`.\nThe ID of the DB parameter group to assign to your DB cluster. \n\t\t\t\t\tDB parameter groups specify how the database is configured. For example, DB parameter groups \n\t\t\t\t\tcan specify the limit for query concurrency."]
    pub fn set_db_parameter_group_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().db_parameter_group_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `db_storage_type`.\nThe Timestream for InfluxDB DB storage type to read and write InfluxDB data. \n\t\t\t\t\tYou can choose between 3 different types of provisioned Influx IOPS included storage according \n\t\t\t\t\tto your workloads requirements: Influx IO Included 3000 IOPS, Influx IO Included 12000 IOPS, \n\t\t\t\t\tInflux IO Included 16000 IOPS."]
    pub fn set_db_storage_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().db_storage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deployment_type`.\nSpecifies the type of cluster to create. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group)."]
    pub fn set_deployment_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `failover_mode`.\nSpecifies the behavior of failure recovery when the primary node of the cluster\n\t\t\t\t\tfails."]
    pub fn set_failover_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().failover_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_type`.\nSpecifies whether the networkType of the Timestream for InfluxDB cluster is \n\t\t\t\t\tIPV4, which can communicate over IPv4 protocol only, or DUAL, which can communicate \n\t\t\t\t\tover both IPv4 and IPv6 protocols."]
    pub fn set_network_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `organization`.\nName of the initial organization for the initial admin user in InfluxDB. An\n\t\t\t\t\tInfluxDB organization is a workspace for a group of users. Along with bucket, username,\n\t\t\t\t\tand password, this argument will be stored in the secret referred to by the\n\t\t\t\t\tinflux_auth_parameters_secret_arn attribute. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group)."]
    pub fn set_organization(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().organization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nPassword of the initial admin user created in InfluxDB. This password will\n\t\t\t\t\tallow you to access the InfluxDB UI to perform various administrative tasks and\n\t\t\t\t\talso use the InfluxDB CLI to create an operator token. Along with bucket, username,\n\t\t\t\t\tand organization, this argument will be stored in the secret referred to by the\n\t\t\t\t\tinflux_auth_parameters_secret_arn attribute. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group) as the AWS API rejects it."]
    pub fn set_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe port number on which InfluxDB accepts connections."]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `publicly_accessible`.\nConfigures the Timestream for InfluxDB cluster with a public IP to facilitate access."]
    pub fn set_publicly_accessible(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `username`.\nUsername of the initial admin user created in InfluxDB. Must start with a letter\n\t\t\t\t\tand can't end with a hyphen or contain two consecutive hyphens. This username will allow\n\t\t\t\t\tyou to access the InfluxDB UI to perform various administrative tasks and also use the\n\t\t\t\t\tInfluxDB CLI to create an operator token. Along with bucket, organization, and password,\n\t\t\t\t\tthis argument will be stored in the secret referred to by the influx_auth_parameters_secret_arn\n\t\t\t\t\tattribute. This field is forbidden for InfluxDB V3 clusters (when using an InfluxDB V3 db parameter group)."]
    pub fn set_username(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_delivery_configuration`.\n"]
    pub fn set_log_delivery_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreaminfluxdbDbCluster_LogDeliveryConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_delivery_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_delivery_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_schedule`.\n"]
    pub fn set_maintenance_schedule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TimestreaminfluxdbDbCluster_MaintenanceSchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<TimestreaminfluxdbDbCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allocated_storage` after provisioning.\nThe amount of storage to allocate for your DB storage type in GiB (gibibytes).\n\t\t\t\t\tThis field is forbidden for InfluxDB V3 clusters (when using an InfluxDB V3 db parameter group)."]
    pub fn allocated_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nName of the initial InfluxDB bucket. All InfluxDB data is stored in a bucket.\n\t\t\t\t\tA bucket combines the concept of a database and a retention period (the duration of time\n\t\t\t\t\tthat each data point persists). A bucket belongs to an organization. Along with organization,\n\t\t\t\t\tusername, and password, this argument will be stored in the secret referred to by the\n\t\t\t\t\tinflux_auth_parameters_secret_arn attribute. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group)."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_instance_type` after provisioning.\nThe Timestream for InfluxDB DB instance type to run InfluxDB on."]
    pub fn db_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_parameter_group_identifier` after provisioning.\nThe ID of the DB parameter group to assign to your DB cluster. \n\t\t\t\t\tDB parameter groups specify how the database is configured. For example, DB parameter groups \n\t\t\t\t\tcan specify the limit for query concurrency."]
    pub fn db_parameter_group_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_parameter_group_identifier", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `db_storage_type` after provisioning.\nThe Timestream for InfluxDB DB storage type to read and write InfluxDB data. \n\t\t\t\t\tYou can choose between 3 different types of provisioned Influx IOPS included storage according \n\t\t\t\t\tto your workloads requirements: Influx IO Included 3000 IOPS, Influx IO Included 12000 IOPS, \n\t\t\t\t\tInflux IO Included 16000 IOPS."]
    pub fn db_storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_storage_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_type` after provisioning.\nSpecifies the type of cluster to create. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group)."]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint used to connect to InfluxDB. The default InfluxDB port is 8086."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_type` after provisioning.\nThe database engine type of the DB cluster."]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failover_mode` after provisioning.\nSpecifies the behavior of failure recovery when the primary node of the cluster\n\t\t\t\t\tfails."]
    pub fn failover_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `influx_auth_parameters_secret_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the AWS Secrets Manager secret containing the \n\t\t\t\t\tinitial InfluxDB authorization parameters. For InfluxDB V2 clusters, the secret value is a JSON\n\t\t\t\t\tformatted key-value pair holding InfluxDB authorization values: organization, bucket,\n\t\t\t\t\tusername, and password. For InfluxDB V3 clusters, the secret contains the InfluxDB admin token."]
    pub fn influx_auth_parameters_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.influx_auth_parameters_secret_arn", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name that uniquely identifies the DB cluster when interacting with the \n\t\t\t\t\tAmazon Timestream for InfluxDB API and CLI commands. This name will also be a \n\t\t\t\t\tprefix included in the endpoint. DB cluster names must be unique per customer \n\t\t\t\t\tand per region."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_type` after provisioning.\nSpecifies whether the networkType of the Timestream for InfluxDB cluster is \n\t\t\t\t\tIPV4, which can communicate over IPv4 protocol only, or DUAL, which can communicate \n\t\t\t\t\tover both IPv4 and IPv6 protocols."]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nName of the initial organization for the initial admin user in InfluxDB. An\n\t\t\t\t\tInfluxDB organization is a workspace for a group of users. Along with bucket, username,\n\t\t\t\t\tand password, this argument will be stored in the secret referred to by the\n\t\t\t\t\tinflux_auth_parameters_secret_arn attribute. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group)."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword of the initial admin user created in InfluxDB. This password will\n\t\t\t\t\tallow you to access the InfluxDB UI to perform various administrative tasks and\n\t\t\t\t\talso use the InfluxDB CLI to create an operator token. Along with bucket, username,\n\t\t\t\t\tand organization, this argument will be stored in the secret referred to by the\n\t\t\t\t\tinflux_auth_parameters_secret_arn attribute. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group) as the AWS API rejects it."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number on which InfluxDB accepts connections."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `publicly_accessible` after provisioning.\nConfigures the Timestream for InfluxDB cluster with a public IP to facilitate access."]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reader_endpoint` after provisioning.\nThe endpoint used to connect to the Timestream for InfluxDB cluster for \n\t\t\t\t\tread-only operations."]
    pub fn reader_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nUsername of the initial admin user created in InfluxDB. Must start with a letter\n\t\t\t\t\tand can't end with a hyphen or contain two consecutive hyphens. This username will allow\n\t\t\t\t\tyou to access the InfluxDB UI to perform various administrative tasks and also use the\n\t\t\t\t\tInfluxDB CLI to create an operator token. Along with bucket, organization, and password,\n\t\t\t\t\tthis argument will be stored in the secret referred to by the influx_auth_parameters_secret_arn\n\t\t\t\t\tattribute. This field is forbidden for InfluxDB V3 clusters (when using an InfluxDB V3 db parameter group)."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\nA list of VPC security group IDs to associate with the Timestream for InfluxDB cluster."]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_subnet_ids` after provisioning.\nA list of VPC subnet IDs to associate with the DB cluster. Provide at least \n\t\t\t\t\ttwo VPC subnet IDs in different availability zones when deploying with a Multi-AZ standby."]
    pub fn vpc_subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_delivery_configuration` after provisioning.\n"]
    pub fn log_delivery_configuration(
        &self,
    ) -> terrars::ListRef<TimestreaminfluxdbDbCluster_LogDeliveryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_delivery_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\n"]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<TimestreaminfluxdbDbCluster_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TimestreaminfluxdbDbCluster_TimeoutsRef {
        TimestreaminfluxdbDbCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for TimestreaminfluxdbDbCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TimestreaminfluxdbDbCluster { }

impl terrars::ToListMappable for TimestreaminfluxdbDbCluster {
    type O = terrars::ListRef<TimestreaminfluxdbDbClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TimestreaminfluxdbDbCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_timestreaminfluxdb_db_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTimestreaminfluxdbDbCluster {
    pub tf_id: std::string::String,
    #[doc = "The Timestream for InfluxDB DB instance type to run InfluxDB on."]
    pub db_instance_type: terrars::PrimField<std::string::String>,
    #[doc =
        "The name that uniquely identifies the DB cluster when interacting with the \n\t\t\t\t\tAmazon Timestream for InfluxDB API and CLI commands. This name will also be a \n\t\t\t\t\tprefix included in the endpoint. DB cluster names must be unique per customer \n\t\t\t\t\tand per region."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "A list of VPC security group IDs to associate with the Timestream for InfluxDB cluster."]
    pub vpc_security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc =
        "A list of VPC subnet IDs to associate with the DB cluster. Provide at least \n\t\t\t\t\ttwo VPC subnet IDs in different availability zones when deploying with a Multi-AZ standby."]
    pub vpc_subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildTimestreaminfluxdbDbCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> TimestreaminfluxdbDbCluster {
        let out = TimestreaminfluxdbDbCluster(std::rc::Rc::new(TimestreaminfluxdbDbCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TimestreaminfluxdbDbClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allocated_storage: core::default::Default::default(),
                bucket: core::default::Default::default(),
                db_instance_type: self.db_instance_type,
                db_parameter_group_identifier: core::default::Default::default(),
                db_storage_type: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                failover_mode: core::default::Default::default(),
                name: self.name,
                network_type: core::default::Default::default(),
                organization: core::default::Default::default(),
                password: core::default::Default::default(),
                port: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                username: core::default::Default::default(),
                vpc_security_group_ids: self.vpc_security_group_ids,
                vpc_subnet_ids: self.vpc_subnet_ids,
                log_delivery_configuration: core::default::Default::default(),
                maintenance_schedule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TimestreaminfluxdbDbClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreaminfluxdbDbClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TimestreaminfluxdbDbClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allocated_storage` after provisioning.\nThe amount of storage to allocate for your DB storage type in GiB (gibibytes).\n\t\t\t\t\tThis field is forbidden for InfluxDB V3 clusters (when using an InfluxDB V3 db parameter group)."]
    pub fn allocated_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nName of the initial InfluxDB bucket. All InfluxDB data is stored in a bucket.\n\t\t\t\t\tA bucket combines the concept of a database and a retention period (the duration of time\n\t\t\t\t\tthat each data point persists). A bucket belongs to an organization. Along with organization,\n\t\t\t\t\tusername, and password, this argument will be stored in the secret referred to by the\n\t\t\t\t\tinflux_auth_parameters_secret_arn attribute. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group)."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_instance_type` after provisioning.\nThe Timestream for InfluxDB DB instance type to run InfluxDB on."]
    pub fn db_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `db_parameter_group_identifier` after provisioning.\nThe ID of the DB parameter group to assign to your DB cluster. \n\t\t\t\t\tDB parameter groups specify how the database is configured. For example, DB parameter groups \n\t\t\t\t\tcan specify the limit for query concurrency."]
    pub fn db_parameter_group_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.db_parameter_group_identifier", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `db_storage_type` after provisioning.\nThe Timestream for InfluxDB DB storage type to read and write InfluxDB data. \n\t\t\t\t\tYou can choose between 3 different types of provisioned Influx IOPS included storage according \n\t\t\t\t\tto your workloads requirements: Influx IO Included 3000 IOPS, Influx IO Included 12000 IOPS, \n\t\t\t\t\tInflux IO Included 16000 IOPS."]
    pub fn db_storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_storage_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_type` after provisioning.\nSpecifies the type of cluster to create. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group)."]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint used to connect to InfluxDB. The default InfluxDB port is 8086."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `engine_type` after provisioning.\nThe database engine type of the DB cluster."]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failover_mode` after provisioning.\nSpecifies the behavior of failure recovery when the primary node of the cluster\n\t\t\t\t\tfails."]
    pub fn failover_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `influx_auth_parameters_secret_arn` after provisioning.\nThe Amazon Resource Name (ARN) of the AWS Secrets Manager secret containing the \n\t\t\t\t\tinitial InfluxDB authorization parameters. For InfluxDB V2 clusters, the secret value is a JSON\n\t\t\t\t\tformatted key-value pair holding InfluxDB authorization values: organization, bucket,\n\t\t\t\t\tusername, and password. For InfluxDB V3 clusters, the secret contains the InfluxDB admin token."]
    pub fn influx_auth_parameters_secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.influx_auth_parameters_secret_arn", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name that uniquely identifies the DB cluster when interacting with the \n\t\t\t\t\tAmazon Timestream for InfluxDB API and CLI commands. This name will also be a \n\t\t\t\t\tprefix included in the endpoint. DB cluster names must be unique per customer \n\t\t\t\t\tand per region."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_type` after provisioning.\nSpecifies whether the networkType of the Timestream for InfluxDB cluster is \n\t\t\t\t\tIPV4, which can communicate over IPv4 protocol only, or DUAL, which can communicate \n\t\t\t\t\tover both IPv4 and IPv6 protocols."]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nName of the initial organization for the initial admin user in InfluxDB. An\n\t\t\t\t\tInfluxDB organization is a workspace for a group of users. Along with bucket, username,\n\t\t\t\t\tand password, this argument will be stored in the secret referred to by the\n\t\t\t\t\tinflux_auth_parameters_secret_arn attribute. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group)."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword of the initial admin user created in InfluxDB. This password will\n\t\t\t\t\tallow you to access the InfluxDB UI to perform various administrative tasks and\n\t\t\t\t\talso use the InfluxDB CLI to create an operator token. Along with bucket, username,\n\t\t\t\t\tand organization, this argument will be stored in the secret referred to by the\n\t\t\t\t\tinflux_auth_parameters_secret_arn attribute. This field is forbidden for InfluxDB V3 clusters\n\t\t\t\t\t(when using an InfluxDB V3 db parameter group) as the AWS API rejects it."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number on which InfluxDB accepts connections."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `publicly_accessible` after provisioning.\nConfigures the Timestream for InfluxDB cluster with a public IP to facilitate access."]
    pub fn publicly_accessible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reader_endpoint` after provisioning.\nThe endpoint used to connect to the Timestream for InfluxDB cluster for \n\t\t\t\t\tread-only operations."]
    pub fn reader_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reader_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nUsername of the initial admin user created in InfluxDB. Must start with a letter\n\t\t\t\t\tand can't end with a hyphen or contain two consecutive hyphens. This username will allow\n\t\t\t\t\tyou to access the InfluxDB UI to perform various administrative tasks and also use the\n\t\t\t\t\tInfluxDB CLI to create an operator token. Along with bucket, organization, and password,\n\t\t\t\t\tthis argument will be stored in the secret referred to by the influx_auth_parameters_secret_arn\n\t\t\t\t\tattribute. This field is forbidden for InfluxDB V3 clusters (when using an InfluxDB V3 db parameter group)."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\nA list of VPC security group IDs to associate with the Timestream for InfluxDB cluster."]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_subnet_ids` after provisioning.\nA list of VPC subnet IDs to associate with the DB cluster. Provide at least \n\t\t\t\t\ttwo VPC subnet IDs in different availability zones when deploying with a Multi-AZ standby."]
    pub fn vpc_subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_delivery_configuration` after provisioning.\n"]
    pub fn log_delivery_configuration(
        &self,
    ) -> terrars::ListRef<TimestreaminfluxdbDbCluster_LogDeliveryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_delivery_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\n"]
    pub fn maintenance_schedule(&self) -> terrars::ListRef<TimestreaminfluxdbDbCluster_MaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TimestreaminfluxdbDbCluster_TimeoutsRef {
        TimestreaminfluxdbDbCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct TimestreaminfluxdbDbClusterDynamic {
    log_delivery_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreaminfluxdbDbCluster_LogDeliveryConfiguration>>,
    maintenance_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreaminfluxdbDbCluster_MaintenanceSchedule>>,
}

#[derive(Serialize)]
pub struct TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration {
    bucket_name: terrars::PrimField<std::string::String>,
    enabled: terrars::PrimField<bool>,
}

impl TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration { }

impl terrars::ToListMappable for TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration {
    type O = terrars::BlockAssignable<TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration {
    #[doc = "The name of the S3 bucket to deliver logs to."]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = "Indicates whether log delivery to the S3 bucket is enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildTimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration {
    pub fn build(self) -> TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration {
        TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration {
            bucket_name: self.bucket_name,
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3ConfigurationRef {
        TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the S3 bucket to deliver logs to."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicates whether log delivery to the S3 bucket is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct TimestreaminfluxdbDbCluster_LogDeliveryConfigurationDynamic {
    s3_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration>>,
}

#[derive(Serialize)]
pub struct TimestreaminfluxdbDbCluster_LogDeliveryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_configuration: std
    ::option
    ::Option<std::vec::Vec<TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration>>,
    dynamic: TimestreaminfluxdbDbCluster_LogDeliveryConfigurationDynamic,
}

impl TimestreaminfluxdbDbCluster_LogDeliveryConfiguration {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3Configuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TimestreaminfluxdbDbCluster_LogDeliveryConfiguration {
    type O = terrars::BlockAssignable<TimestreaminfluxdbDbCluster_LogDeliveryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreaminfluxdbDbCluster_LogDeliveryConfiguration {}

impl BuildTimestreaminfluxdbDbCluster_LogDeliveryConfiguration {
    pub fn build(self) -> TimestreaminfluxdbDbCluster_LogDeliveryConfiguration {
        TimestreaminfluxdbDbCluster_LogDeliveryConfiguration {
            s3_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreaminfluxdbDbCluster_LogDeliveryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreaminfluxdbDbCluster_LogDeliveryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreaminfluxdbDbCluster_LogDeliveryConfigurationRef {
        TimestreaminfluxdbDbCluster_LogDeliveryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreaminfluxdbDbCluster_LogDeliveryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> terrars::ListRef<TimestreaminfluxdbDbCluster_LogDeliveryConfiguration_S3ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreaminfluxdbDbCluster_MaintenanceSchedule {
    preferred_maintenance_window: terrars::PrimField<std::string::String>,
    timezone: terrars::PrimField<std::string::String>,
}

impl TimestreaminfluxdbDbCluster_MaintenanceSchedule { }

impl terrars::ToListMappable for TimestreaminfluxdbDbCluster_MaintenanceSchedule {
    type O = terrars::BlockAssignable<TimestreaminfluxdbDbCluster_MaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreaminfluxdbDbCluster_MaintenanceSchedule {
    #[doc =
        "The preferred maintenance window in the format ddd:HH:MM-ddd:HH:MM.\n\t\t\t\t\t\t\t\tDay must be one of Mon, Tue, Wed, Thu, Fri, Sat, or Sun. Provide an empty\n\t\t\t\t\t\t\t\tstring to let the system choose a window."]
    pub preferred_maintenance_window: terrars::PrimField<std::string::String>,
    #[doc =
        "The IANA timezone identifier for the maintenance window. For\n\t\t\t\t\t\t\t\texample, America/New_York or UTC."]
    pub timezone: terrars::PrimField<std::string::String>,
}

impl BuildTimestreaminfluxdbDbCluster_MaintenanceSchedule {
    pub fn build(self) -> TimestreaminfluxdbDbCluster_MaintenanceSchedule {
        TimestreaminfluxdbDbCluster_MaintenanceSchedule {
            preferred_maintenance_window: self.preferred_maintenance_window,
            timezone: self.timezone,
        }
    }
}

#[allow(dead_code)]
pub struct TimestreaminfluxdbDbCluster_MaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreaminfluxdbDbCluster_MaintenanceScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TimestreaminfluxdbDbCluster_MaintenanceScheduleRef {
        TimestreaminfluxdbDbCluster_MaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreaminfluxdbDbCluster_MaintenanceScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\nThe preferred maintenance window in the format ddd:HH:MM-ddd:HH:MM.\n\t\t\t\t\t\t\t\tDay must be one of Mon, Tue, Wed, Thu, Fri, Sat, or Sun. Provide an empty\n\t\t\t\t\t\t\t\tstring to let the system choose a window."]
    pub fn preferred_maintenance_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timezone` after provisioning.\nThe IANA timezone identifier for the maintenance window. For\n\t\t\t\t\t\t\t\texample, America/New_York or UTC."]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.base))
    }
}

#[derive(Serialize)]
pub struct TimestreaminfluxdbDbCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TimestreaminfluxdbDbCluster_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TimestreaminfluxdbDbCluster_Timeouts {
    type O = terrars::BlockAssignable<TimestreaminfluxdbDbCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTimestreaminfluxdbDbCluster_Timeouts {}

impl BuildTimestreaminfluxdbDbCluster_Timeouts {
    pub fn build(self) -> TimestreaminfluxdbDbCluster_Timeouts {
        TimestreaminfluxdbDbCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TimestreaminfluxdbDbCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TimestreaminfluxdbDbCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TimestreaminfluxdbDbCluster_TimeoutsRef {
        TimestreaminfluxdbDbCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TimestreaminfluxdbDbCluster_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRdsOrderableDbInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_group: Option<PrimField<String>>,
    engine: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_engine_versions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_instance_classes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_enhanced_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_global_databases: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_iam_database_authentication: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_iops: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_kerberos_authentication: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_performance_insights: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_storage_autoscaling: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_storage_encryption: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<PrimField<bool>>,
}

struct DataRdsOrderableDbInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRdsOrderableDbInstanceData>,
}

#[derive(Clone)]
pub struct DataRdsOrderableDbInstance(Rc<DataRdsOrderableDbInstance_>);

impl DataRdsOrderableDbInstance {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `availability_zone_group`.\n"]
    pub fn set_availability_zone_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone_group = Some(v.into());
        self
    }

    #[doc= "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_version = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_class`.\n"]
    pub fn set_instance_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_class = Some(v.into());
        self
    }

    #[doc= "Set the field `license_model`.\n"]
    pub fn set_license_model(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().license_model = Some(v.into());
        self
    }

    #[doc= "Set the field `preferred_engine_versions`.\n"]
    pub fn set_preferred_engine_versions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().preferred_engine_versions = Some(v.into());
        self
    }

    #[doc= "Set the field `preferred_instance_classes`.\n"]
    pub fn set_preferred_instance_classes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().preferred_instance_classes = Some(v.into());
        self
    }

    #[doc= "Set the field `storage_type`.\n"]
    pub fn set_storage_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_type = Some(v.into());
        self
    }

    #[doc= "Set the field `supports_enhanced_monitoring`.\n"]
    pub fn set_supports_enhanced_monitoring(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().supports_enhanced_monitoring = Some(v.into());
        self
    }

    #[doc= "Set the field `supports_global_databases`.\n"]
    pub fn set_supports_global_databases(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().supports_global_databases = Some(v.into());
        self
    }

    #[doc= "Set the field `supports_iam_database_authentication`.\n"]
    pub fn set_supports_iam_database_authentication(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().supports_iam_database_authentication = Some(v.into());
        self
    }

    #[doc= "Set the field `supports_iops`.\n"]
    pub fn set_supports_iops(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().supports_iops = Some(v.into());
        self
    }

    #[doc= "Set the field `supports_kerberos_authentication`.\n"]
    pub fn set_supports_kerberos_authentication(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().supports_kerberos_authentication = Some(v.into());
        self
    }

    #[doc= "Set the field `supports_performance_insights`.\n"]
    pub fn set_supports_performance_insights(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().supports_performance_insights = Some(v.into());
        self
    }

    #[doc= "Set the field `supports_storage_autoscaling`.\n"]
    pub fn set_supports_storage_autoscaling(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().supports_storage_autoscaling = Some(v.into());
        self
    }

    #[doc= "Set the field `supports_storage_encryption`.\n"]
    pub fn set_supports_storage_encryption(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().supports_storage_encryption = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc`.\n"]
    pub fn set_vpc(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().vpc = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `availability_zone_group` after provisioning.\n"]
    pub fn availability_zone_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_class` after provisioning.\n"]
    pub fn instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_iops_per_db_instance` after provisioning.\n"]
    pub fn max_iops_per_db_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_db_instance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_iops_per_gib` after provisioning.\n"]
    pub fn max_iops_per_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_gib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_storage_size` after provisioning.\n"]
    pub fn max_storage_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_storage_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_iops_per_db_instance` after provisioning.\n"]
    pub fn min_iops_per_db_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_iops_per_db_instance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_iops_per_gib` after provisioning.\n"]
    pub fn min_iops_per_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_iops_per_gib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_storage_size` after provisioning.\n"]
    pub fn min_storage_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_storage_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `multi_az_capable` after provisioning.\n"]
    pub fn multi_az_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_az_capable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `outpost_capable` after provisioning.\n"]
    pub fn outpost_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.outpost_capable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preferred_engine_versions` after provisioning.\n"]
    pub fn preferred_engine_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_engine_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preferred_instance_classes` after provisioning.\n"]
    pub fn preferred_instance_classes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_instance_classes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `read_replica_capable` after provisioning.\n"]
    pub fn read_replica_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_replica_capable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_engine_modes` after provisioning.\n"]
    pub fn supported_engine_modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_engine_modes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_network_types` after provisioning.\n"]
    pub fn supported_network_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_network_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_enhanced_monitoring` after provisioning.\n"]
    pub fn supports_enhanced_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_enhanced_monitoring", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_global_databases` after provisioning.\n"]
    pub fn supports_global_databases(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_global_databases", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_iam_database_authentication` after provisioning.\n"]
    pub fn supports_iam_database_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_iam_database_authentication", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_iops` after provisioning.\n"]
    pub fn supports_iops(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_iops", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_kerberos_authentication` after provisioning.\n"]
    pub fn supports_kerberos_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_kerberos_authentication", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_performance_insights` after provisioning.\n"]
    pub fn supports_performance_insights(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_performance_insights", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_storage_autoscaling` after provisioning.\n"]
    pub fn supports_storage_autoscaling(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_storage_autoscaling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_storage_encryption` after provisioning.\n"]
    pub fn supports_storage_encryption(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_storage_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}

impl Datasource for DataRdsOrderableDbInstance {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataRdsOrderableDbInstance {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataRdsOrderableDbInstance {
    type O = ListRef<DataRdsOrderableDbInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataRdsOrderableDbInstance_ {
    fn extract_datasource_type(&self) -> String {
        "aws_rds_orderable_db_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRdsOrderableDbInstance {
    pub tf_id: String,
    #[doc= ""]
    pub engine: PrimField<String>,
}

impl BuildDataRdsOrderableDbInstance {
    pub fn build(self, stack: &mut Stack) -> DataRdsOrderableDbInstance {
        let out = DataRdsOrderableDbInstance(Rc::new(DataRdsOrderableDbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRdsOrderableDbInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                availability_zone_group: core::default::Default::default(),
                engine: self.engine,
                engine_version: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_class: core::default::Default::default(),
                license_model: core::default::Default::default(),
                preferred_engine_versions: core::default::Default::default(),
                preferred_instance_classes: core::default::Default::default(),
                storage_type: core::default::Default::default(),
                supports_enhanced_monitoring: core::default::Default::default(),
                supports_global_databases: core::default::Default::default(),
                supports_iam_database_authentication: core::default::Default::default(),
                supports_iops: core::default::Default::default(),
                supports_kerberos_authentication: core::default::Default::default(),
                supports_performance_insights: core::default::Default::default(),
                supports_storage_autoscaling: core::default::Default::default(),
                supports_storage_encryption: core::default::Default::default(),
                vpc: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRdsOrderableDbInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRdsOrderableDbInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRdsOrderableDbInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `availability_zone_group` after provisioning.\n"]
    pub fn availability_zone_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_group", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_class` after provisioning.\n"]
    pub fn instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_iops_per_db_instance` after provisioning.\n"]
    pub fn max_iops_per_db_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_db_instance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_iops_per_gib` after provisioning.\n"]
    pub fn max_iops_per_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_iops_per_gib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_storage_size` after provisioning.\n"]
    pub fn max_storage_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_storage_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_iops_per_db_instance` after provisioning.\n"]
    pub fn min_iops_per_db_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_iops_per_db_instance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_iops_per_gib` after provisioning.\n"]
    pub fn min_iops_per_gib(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_iops_per_gib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_storage_size` after provisioning.\n"]
    pub fn min_storage_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_storage_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `multi_az_capable` after provisioning.\n"]
    pub fn multi_az_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_az_capable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `outpost_capable` after provisioning.\n"]
    pub fn outpost_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.outpost_capable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preferred_engine_versions` after provisioning.\n"]
    pub fn preferred_engine_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_engine_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `preferred_instance_classes` after provisioning.\n"]
    pub fn preferred_instance_classes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.preferred_instance_classes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `read_replica_capable` after provisioning.\n"]
    pub fn read_replica_capable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.read_replica_capable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_engine_modes` after provisioning.\n"]
    pub fn supported_engine_modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_engine_modes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_network_types` after provisioning.\n"]
    pub fn supported_network_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_network_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_enhanced_monitoring` after provisioning.\n"]
    pub fn supports_enhanced_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_enhanced_monitoring", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_global_databases` after provisioning.\n"]
    pub fn supports_global_databases(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_global_databases", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_iam_database_authentication` after provisioning.\n"]
    pub fn supports_iam_database_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_iam_database_authentication", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_iops` after provisioning.\n"]
    pub fn supports_iops(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_iops", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_kerberos_authentication` after provisioning.\n"]
    pub fn supports_kerberos_authentication(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_kerberos_authentication", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_performance_insights` after provisioning.\n"]
    pub fn supports_performance_insights(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_performance_insights", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_storage_autoscaling` after provisioning.\n"]
    pub fn supports_storage_autoscaling(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_storage_autoscaling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supports_storage_encryption` after provisioning.\n"]
    pub fn supports_storage_encryption(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.supports_storage_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.extract_ref()))
    }
}

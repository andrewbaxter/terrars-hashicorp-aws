use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDirectoryServiceDirectoryData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    directory_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataDirectoryServiceDirectory_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDirectoryServiceDirectoryData>,
}

#[derive(Clone)]
pub struct DataDirectoryServiceDirectory(std::rc::Rc<DataDirectoryServiceDirectory_>);

impl DataDirectoryServiceDirectory {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `access_url` after provisioning.\n"]
    pub fn access_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_settings` after provisioning.\n"]
    pub fn connect_settings(&self) -> terrars::ListRef<DataDirectoryServiceDirectoryConnectSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connect_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_sso` after provisioning.\n"]
    pub fn enable_sso(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sso", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_settings` after provisioning.\n"]
    pub fn radius_settings(&self) -> terrars::ListRef<DataDirectoryServiceDirectoryRadiusSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.radius_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> terrars::ListRef<DataDirectoryServiceDirectoryVpcSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_settings", self.extract_ref()))
    }
}

impl terrars::Referable for DataDirectoryServiceDirectory {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDirectoryServiceDirectory { }

impl terrars::ToListMappable for DataDirectoryServiceDirectory {
    type O = terrars::ListRef<DataDirectoryServiceDirectoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDirectoryServiceDirectory_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_directory_service_directory".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDirectoryServiceDirectory {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub directory_id: terrars::PrimField<std::string::String>,
}

impl BuildDataDirectoryServiceDirectory {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDirectoryServiceDirectory {
        let out = DataDirectoryServiceDirectory(std::rc::Rc::new(DataDirectoryServiceDirectory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDirectoryServiceDirectoryData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                directory_id: self.directory_id,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDirectoryServiceDirectoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDirectoryServiceDirectoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDirectoryServiceDirectoryRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_url` after provisioning.\n"]
    pub fn access_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_settings` after provisioning.\n"]
    pub fn connect_settings(&self) -> terrars::ListRef<DataDirectoryServiceDirectoryConnectSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connect_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_sso` after provisioning.\n"]
    pub fn enable_sso(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_sso", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_settings` after provisioning.\n"]
    pub fn radius_settings(&self) -> terrars::ListRef<DataDirectoryServiceDirectoryRadiusSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.radius_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> terrars::ListRef<DataDirectoryServiceDirectoryVpcSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDirectoryServiceDirectoryConnectSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_dns_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDirectoryServiceDirectoryConnectSettingsEl {
    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.availability_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connect_ips`.\n"]
    pub fn set_connect_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.connect_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_dns_ips`.\n"]
    pub fn set_customer_dns_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.customer_dns_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_username`.\n"]
    pub fn set_customer_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.customer_username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDirectoryServiceDirectoryConnectSettingsEl {
    type O = terrars::BlockAssignable<DataDirectoryServiceDirectoryConnectSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDirectoryServiceDirectoryConnectSettingsEl {}

impl BuildDataDirectoryServiceDirectoryConnectSettingsEl {
    pub fn build(self) -> DataDirectoryServiceDirectoryConnectSettingsEl {
        DataDirectoryServiceDirectoryConnectSettingsEl {
            availability_zones: core::default::Default::default(),
            connect_ips: core::default::Default::default(),
            customer_dns_ips: core::default::Default::default(),
            customer_username: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct DataDirectoryServiceDirectoryConnectSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDirectoryServiceDirectoryConnectSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDirectoryServiceDirectoryConnectSettingsElRef {
        DataDirectoryServiceDirectoryConnectSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDirectoryServiceDirectoryConnectSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `connect_ips` after provisioning.\n"]
    pub fn connect_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.connect_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_dns_ips` after provisioning.\n"]
    pub fn customer_dns_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.customer_dns_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_username` after provisioning.\n"]
    pub fn customer_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_username", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDirectoryServiceDirectoryRadiusSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    radius_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    radius_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    radius_servers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    radius_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_same_username: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDirectoryServiceDirectoryRadiusSettingsEl {
    #[doc = "Set the field `authentication_protocol`.\n"]
    pub fn set_authentication_protocol(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_label`.\n"]
    pub fn set_display_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `radius_port`.\n"]
    pub fn set_radius_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.radius_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `radius_retries`.\n"]
    pub fn set_radius_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.radius_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `radius_servers`.\n"]
    pub fn set_radius_servers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.radius_servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `radius_timeout`.\n"]
    pub fn set_radius_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.radius_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_same_username`.\n"]
    pub fn set_use_same_username(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_same_username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDirectoryServiceDirectoryRadiusSettingsEl {
    type O = terrars::BlockAssignable<DataDirectoryServiceDirectoryRadiusSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDirectoryServiceDirectoryRadiusSettingsEl {}

impl BuildDataDirectoryServiceDirectoryRadiusSettingsEl {
    pub fn build(self) -> DataDirectoryServiceDirectoryRadiusSettingsEl {
        DataDirectoryServiceDirectoryRadiusSettingsEl {
            authentication_protocol: core::default::Default::default(),
            display_label: core::default::Default::default(),
            radius_port: core::default::Default::default(),
            radius_retries: core::default::Default::default(),
            radius_servers: core::default::Default::default(),
            radius_timeout: core::default::Default::default(),
            use_same_username: core::default::Default::default(),
        }
    }
}

pub struct DataDirectoryServiceDirectoryRadiusSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDirectoryServiceDirectoryRadiusSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDirectoryServiceDirectoryRadiusSettingsElRef {
        DataDirectoryServiceDirectoryRadiusSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDirectoryServiceDirectoryRadiusSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_protocol` after provisioning.\n"]
    pub fn authentication_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `display_label` after provisioning.\n"]
    pub fn display_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_label", self.base))
    }

    #[doc = "Get a reference to the value of field `radius_port` after provisioning.\n"]
    pub fn radius_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.radius_port", self.base))
    }

    #[doc = "Get a reference to the value of field `radius_retries` after provisioning.\n"]
    pub fn radius_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.radius_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `radius_servers` after provisioning.\n"]
    pub fn radius_servers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.radius_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `radius_timeout` after provisioning.\n"]
    pub fn radius_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.radius_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `use_same_username` after provisioning.\n"]
    pub fn use_same_username(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_same_username", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDirectoryServiceDirectoryVpcSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDirectoryServiceDirectoryVpcSettingsEl {
    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.availability_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDirectoryServiceDirectoryVpcSettingsEl {
    type O = terrars::BlockAssignable<DataDirectoryServiceDirectoryVpcSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDirectoryServiceDirectoryVpcSettingsEl {}

impl BuildDataDirectoryServiceDirectoryVpcSettingsEl {
    pub fn build(self) -> DataDirectoryServiceDirectoryVpcSettingsEl {
        DataDirectoryServiceDirectoryVpcSettingsEl {
            availability_zones: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct DataDirectoryServiceDirectoryVpcSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDirectoryServiceDirectoryVpcSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDirectoryServiceDirectoryVpcSettingsElRef {
        DataDirectoryServiceDirectoryVpcSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDirectoryServiceDirectoryVpcSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

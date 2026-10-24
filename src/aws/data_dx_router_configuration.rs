use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDxRouterConfigurationData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    router_type_identifier: terrars::PrimField<std::string::String>,
    virtual_interface_id: terrars::PrimField<std::string::String>,
}

struct DataDxRouterConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDxRouterConfigurationData>,
}

#[derive(Clone)]
pub struct DataDxRouterConfiguration(std::rc::Rc<DataDxRouterConfiguration_>);

impl DataDxRouterConfiguration {
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

    #[doc = "Get a reference to the value of field `customer_router_config` after provisioning.\n"]
    pub fn customer_router_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_router_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router` after provisioning.\n"]
    pub fn router(&self) -> terrars::ListRef<DataDxRouterConfigurationRouterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router_type_identifier` after provisioning.\n"]
    pub fn router_type_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router_type_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_interface_id` after provisioning.\n"]
    pub fn virtual_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_interface_name` after provisioning.\n"]
    pub fn virtual_interface_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_interface_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataDxRouterConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDxRouterConfiguration { }

impl terrars::ToListMappable for DataDxRouterConfiguration {
    type O = terrars::ListRef<DataDxRouterConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDxRouterConfiguration_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_dx_router_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDxRouterConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub router_type_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub virtual_interface_id: terrars::PrimField<std::string::String>,
}

impl BuildDataDxRouterConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDxRouterConfiguration {
        let out = DataDxRouterConfiguration(std::rc::Rc::new(DataDxRouterConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDxRouterConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                router_type_identifier: self.router_type_identifier,
                virtual_interface_id: self.virtual_interface_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDxRouterConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDxRouterConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDxRouterConfigurationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `customer_router_config` after provisioning.\n"]
    pub fn customer_router_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_router_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router` after provisioning.\n"]
    pub fn router(&self) -> terrars::ListRef<DataDxRouterConfigurationRouterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router_type_identifier` after provisioning.\n"]
    pub fn router_type_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router_type_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_interface_id` after provisioning.\n"]
    pub fn virtual_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_interface_name` after provisioning.\n"]
    pub fn virtual_interface_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_interface_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDxRouterConfigurationRouterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    router_type_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vendor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xslt_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xslt_template_name_for_mac_sec: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDxRouterConfigurationRouterEl {
    #[doc = "Set the field `platform`.\n"]
    pub fn set_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.platform = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `router_type_identifier`.\n"]
    pub fn set_router_type_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.router_type_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `software`.\n"]
    pub fn set_software(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.software = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vendor`.\n"]
    pub fn set_vendor(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vendor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xslt_template_name`.\n"]
    pub fn set_xslt_template_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.xslt_template_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xslt_template_name_for_mac_sec`.\n"]
    pub fn set_xslt_template_name_for_mac_sec(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.xslt_template_name_for_mac_sec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDxRouterConfigurationRouterEl {
    type O = terrars::BlockAssignable<DataDxRouterConfigurationRouterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDxRouterConfigurationRouterEl {}

impl BuildDataDxRouterConfigurationRouterEl {
    pub fn build(self) -> DataDxRouterConfigurationRouterEl {
        DataDxRouterConfigurationRouterEl {
            platform: core::default::Default::default(),
            router_type_identifier: core::default::Default::default(),
            software: core::default::Default::default(),
            vendor: core::default::Default::default(),
            xslt_template_name: core::default::Default::default(),
            xslt_template_name_for_mac_sec: core::default::Default::default(),
        }
    }
}

pub struct DataDxRouterConfigurationRouterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDxRouterConfigurationRouterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDxRouterConfigurationRouterElRef {
        DataDxRouterConfigurationRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDxRouterConfigurationRouterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }

    #[doc = "Get a reference to the value of field `router_type_identifier` after provisioning.\n"]
    pub fn router_type_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.router_type_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `software` after provisioning.\n"]
    pub fn software(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.software", self.base))
    }

    #[doc = "Get a reference to the value of field `vendor` after provisioning.\n"]
    pub fn vendor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vendor", self.base))
    }

    #[doc = "Get a reference to the value of field `xslt_template_name` after provisioning.\n"]
    pub fn xslt_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xslt_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `xslt_template_name_for_mac_sec` after provisioning.\n"]
    pub fn xslt_template_name_for_mac_sec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xslt_template_name_for_mac_sec", self.base))
    }
}

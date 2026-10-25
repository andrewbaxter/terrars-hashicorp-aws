#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectQuickConnectData_ {
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
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quick_connect_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataConnectQuickConnect_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataConnectQuickConnectData_>,
}

#[derive(Clone)]
pub struct DataConnectQuickConnect(std::rc::Rc<DataConnectQuickConnect_>);

impl DataConnectQuickConnect {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quick_connect_id`.\n"]
    pub fn set_quick_connect_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quick_connect_id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> terrars::ListRef<DataConnectQuickConnect_QuickConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quick_connect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_id` after provisioning.\n"]
    pub fn quick_connect_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_id", self.extract_ref()))
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
}

impl terrars::Referable for DataConnectQuickConnect {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataConnectQuickConnect { }

impl terrars::ToListMappable for DataConnectQuickConnect {
    type O = terrars::ListRef<DataConnectQuickConnectRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataConnectQuickConnect_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_connect_quick_connect".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectQuickConnect {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildDataConnectQuickConnect {
    pub fn build(self, stack: &mut terrars::Stack) -> DataConnectQuickConnect {
        let out = DataConnectQuickConnect(std::rc::Rc::new(DataConnectQuickConnect_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataConnectQuickConnectData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: core::default::Default::default(),
                quick_connect_id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectQuickConnectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectQuickConnectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataConnectQuickConnectRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> terrars::ListRef<DataConnectQuickConnect_QuickConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quick_connect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_id` after provisioning.\n"]
    pub fn quick_connect_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_id", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataConnectQuickConnect_QuickConnectConfig_PhoneConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectQuickConnect_QuickConnectConfig_PhoneConfig {
    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectQuickConnect_QuickConnectConfig_PhoneConfig {
    type O = terrars::BlockAssignable<DataConnectQuickConnect_QuickConnectConfig_PhoneConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectQuickConnect_QuickConnectConfig_PhoneConfig {}

impl BuildDataConnectQuickConnect_QuickConnectConfig_PhoneConfig {
    pub fn build(self) -> DataConnectQuickConnect_QuickConnectConfig_PhoneConfig {
        DataConnectQuickConnect_QuickConnectConfig_PhoneConfig { phone_number: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
        DataConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectQuickConnect_QuickConnectConfig_PhoneConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectQuickConnect_QuickConnectConfig_QueueConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_flow_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectQuickConnect_QuickConnectConfig_QueueConfig {
    #[doc = "Set the field `contact_flow_id`.\n"]
    pub fn set_contact_flow_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contact_flow_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_id`.\n"]
    pub fn set_queue_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.queue_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectQuickConnect_QuickConnectConfig_QueueConfig {
    type O = terrars::BlockAssignable<DataConnectQuickConnect_QuickConnectConfig_QueueConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectQuickConnect_QuickConnectConfig_QueueConfig {}

impl BuildDataConnectQuickConnect_QuickConnectConfig_QueueConfig {
    pub fn build(self) -> DataConnectQuickConnect_QuickConnectConfig_QueueConfig {
        DataConnectQuickConnect_QuickConnectConfig_QueueConfig {
            contact_flow_id: core::default::Default::default(),
            queue_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
        DataConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectQuickConnect_QuickConnectConfig_QueueConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_flow_id` after provisioning.\n"]
    pub fn contact_flow_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_id", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\n"]
    pub fn queue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectQuickConnect_QuickConnectConfig_UserConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_flow_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataConnectQuickConnect_QuickConnectConfig_UserConfig {
    #[doc = "Set the field `contact_flow_id`.\n"]
    pub fn set_contact_flow_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contact_flow_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectQuickConnect_QuickConnectConfig_UserConfig {
    type O = terrars::BlockAssignable<DataConnectQuickConnect_QuickConnectConfig_UserConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectQuickConnect_QuickConnectConfig_UserConfig {}

impl BuildDataConnectQuickConnect_QuickConnectConfig_UserConfig {
    pub fn build(self) -> DataConnectQuickConnect_QuickConnectConfig_UserConfig {
        DataConnectQuickConnect_QuickConnectConfig_UserConfig {
            contact_flow_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectQuickConnect_QuickConnectConfig_UserConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectQuickConnect_QuickConnectConfig_UserConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataConnectQuickConnect_QuickConnectConfig_UserConfigRef {
        DataConnectQuickConnect_QuickConnectConfig_UserConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectQuickConnect_QuickConnectConfig_UserConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_flow_id` after provisioning.\n"]
    pub fn contact_flow_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectQuickConnect_QuickConnectConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_config: std::option::Option<terrars::ListField<DataConnectQuickConnect_QuickConnectConfig_PhoneConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_config: std::option::Option<terrars::ListField<DataConnectQuickConnect_QuickConnectConfig_QueueConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quick_connect_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_config: std::option::Option<terrars::ListField<DataConnectQuickConnect_QuickConnectConfig_UserConfig>>,
}

impl DataConnectQuickConnect_QuickConnectConfig {
    #[doc = "Set the field `phone_config`.\n"]
    pub fn set_phone_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectQuickConnect_QuickConnectConfig_PhoneConfig>>,
    ) -> Self {
        self.phone_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_config`.\n"]
    pub fn set_queue_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectQuickConnect_QuickConnectConfig_QueueConfig>>,
    ) -> Self {
        self.queue_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quick_connect_type`.\n"]
    pub fn set_quick_connect_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.quick_connect_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_config`.\n"]
    pub fn set_user_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataConnectQuickConnect_QuickConnectConfig_UserConfig>>,
    ) -> Self {
        self.user_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataConnectQuickConnect_QuickConnectConfig {
    type O = terrars::BlockAssignable<DataConnectQuickConnect_QuickConnectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectQuickConnect_QuickConnectConfig {}

impl BuildDataConnectQuickConnect_QuickConnectConfig {
    pub fn build(self) -> DataConnectQuickConnect_QuickConnectConfig {
        DataConnectQuickConnect_QuickConnectConfig {
            phone_config: core::default::Default::default(),
            queue_config: core::default::Default::default(),
            quick_connect_type: core::default::Default::default(),
            user_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataConnectQuickConnect_QuickConnectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataConnectQuickConnect_QuickConnectConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataConnectQuickConnect_QuickConnectConfigRef {
        DataConnectQuickConnect_QuickConnectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectQuickConnect_QuickConnectConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> terrars::ListRef<DataConnectQuickConnect_QuickConnectConfig_PhoneConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_config", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_config` after provisioning.\n"]
    pub fn queue_config(&self) -> terrars::ListRef<DataConnectQuickConnect_QuickConnectConfig_QueueConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queue_config", self.base))
    }

    #[doc = "Get a reference to the value of field `quick_connect_type` after provisioning.\n"]
    pub fn quick_connect_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_type", self.base))
    }

    #[doc = "Get a reference to the value of field `user_config` after provisioning.\n"]
    pub fn user_config(&self) -> terrars::ListRef<DataConnectQuickConnect_QuickConnectConfig_UserConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_config", self.base))
    }
}

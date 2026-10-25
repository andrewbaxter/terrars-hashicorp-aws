#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DevopsguruNotificationChannelData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<std::vec::Vec<DevopsguruNotificationChannel_Filters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns: std::option::Option<std::vec::Vec<DevopsguruNotificationChannel_Sns>>,
    dynamic: DevopsguruNotificationChannelDynamic,
}

struct DevopsguruNotificationChannel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DevopsguruNotificationChannelData_>,
}

#[derive(Clone)]
pub struct DevopsguruNotificationChannel(std::rc::Rc<DevopsguruNotificationChannel_>);

impl DevopsguruNotificationChannel {
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
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DevopsguruNotificationChannel_Filters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sns`.\n"]
    pub fn set_sns(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DevopsguruNotificationChannel_Sns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<DevopsguruNotificationChannel_FiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> terrars::ListRef<DevopsguruNotificationChannel_SnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns", self.extract_ref()))
    }
}

impl terrars::Referable for DevopsguruNotificationChannel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DevopsguruNotificationChannel { }

impl terrars::ToListMappable for DevopsguruNotificationChannel {
    type O = terrars::ListRef<DevopsguruNotificationChannelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DevopsguruNotificationChannel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_devopsguru_notification_channel".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevopsguruNotificationChannel {
    pub tf_id: std::string::String,
}

impl BuildDevopsguruNotificationChannel {
    pub fn build(self, stack: &mut terrars::Stack) -> DevopsguruNotificationChannel {
        let out = DevopsguruNotificationChannel(std::rc::Rc::new(DevopsguruNotificationChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DevopsguruNotificationChannelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                filters: core::default::Default::default(),
                sns: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevopsguruNotificationChannelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruNotificationChannelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DevopsguruNotificationChannelRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<DevopsguruNotificationChannel_FiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> terrars::ListRef<DevopsguruNotificationChannel_SnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DevopsguruNotificationChannelDynamic {
    filters: std::option::Option<terrars::DynamicBlock<DevopsguruNotificationChannel_Filters>>,
    sns: std::option::Option<terrars::DynamicBlock<DevopsguruNotificationChannel_Sns>>,
}

#[derive(Serialize)]
pub struct DevopsguruNotificationChannel_Filters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DevopsguruNotificationChannel_Filters {
    #[doc = "Set the field `message_types`.\n"]
    pub fn set_message_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.message_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severities`.\n"]
    pub fn set_severities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.severities = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DevopsguruNotificationChannel_Filters {
    type O = terrars::BlockAssignable<DevopsguruNotificationChannel_Filters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruNotificationChannel_Filters {}

impl BuildDevopsguruNotificationChannel_Filters {
    pub fn build(self) -> DevopsguruNotificationChannel_Filters {
        DevopsguruNotificationChannel_Filters {
            message_types: core::default::Default::default(),
            severities: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DevopsguruNotificationChannel_FiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruNotificationChannel_FiltersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DevopsguruNotificationChannel_FiltersRef {
        DevopsguruNotificationChannel_FiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruNotificationChannel_FiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_types` after provisioning.\n"]
    pub fn message_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.message_types", self.base))
    }

    #[doc = "Get a reference to the value of field `severities` after provisioning.\n"]
    pub fn severities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.severities", self.base))
    }
}

#[derive(Serialize)]
pub struct DevopsguruNotificationChannel_Sns {
    topic_arn: terrars::PrimField<std::string::String>,
}

impl DevopsguruNotificationChannel_Sns { }

impl terrars::ToListMappable for DevopsguruNotificationChannel_Sns {
    type O = terrars::BlockAssignable<DevopsguruNotificationChannel_Sns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruNotificationChannel_Sns {
    #[doc = ""]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildDevopsguruNotificationChannel_Sns {
    pub fn build(self) -> DevopsguruNotificationChannel_Sns {
        DevopsguruNotificationChannel_Sns { topic_arn: self.topic_arn }
    }
}

#[allow(dead_code)]
pub struct DevopsguruNotificationChannel_SnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruNotificationChannel_SnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DevopsguruNotificationChannel_SnsRef {
        DevopsguruNotificationChannel_SnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruNotificationChannel_SnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

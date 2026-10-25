#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDevopsguruNotificationChannelData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<std::vec::Vec<DataDevopsguruNotificationChannel_Filters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns: std::option::Option<std::vec::Vec<DataDevopsguruNotificationChannel_Sns>>,
    dynamic: DataDevopsguruNotificationChannelDynamic,
}

struct DataDevopsguruNotificationChannel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDevopsguruNotificationChannelData_>,
}

#[derive(Clone)]
pub struct DataDevopsguruNotificationChannel(std::rc::Rc<DataDevopsguruNotificationChannel_>);

impl DataDevopsguruNotificationChannel {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataDevopsguruNotificationChannel_Filters>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataDevopsguruNotificationChannel_Sns>>,
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
    pub fn filters(&self) -> terrars::ListRef<DataDevopsguruNotificationChannel_FiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> terrars::ListRef<DataDevopsguruNotificationChannel_SnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns", self.extract_ref()))
    }
}

impl terrars::Referable for DataDevopsguruNotificationChannel {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDevopsguruNotificationChannel { }

impl terrars::ToListMappable for DataDevopsguruNotificationChannel {
    type O = terrars::ListRef<DataDevopsguruNotificationChannelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDevopsguruNotificationChannel_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_devopsguru_notification_channel".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDevopsguruNotificationChannel {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataDevopsguruNotificationChannel {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDevopsguruNotificationChannel {
        let out = DataDevopsguruNotificationChannel(std::rc::Rc::new(DataDevopsguruNotificationChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDevopsguruNotificationChannelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                region: core::default::Default::default(),
                filters: core::default::Default::default(),
                sns: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDevopsguruNotificationChannelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDevopsguruNotificationChannelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDevopsguruNotificationChannelRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
    pub fn filters(&self) -> terrars::ListRef<DataDevopsguruNotificationChannel_FiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> terrars::ListRef<DataDevopsguruNotificationChannel_SnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sns", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataDevopsguruNotificationChannelDynamic {
    filters: std::option::Option<terrars::DynamicBlock<DataDevopsguruNotificationChannel_Filters>>,
    sns: std::option::Option<terrars::DynamicBlock<DataDevopsguruNotificationChannel_Sns>>,
}

#[derive(Serialize)]
pub struct DataDevopsguruNotificationChannel_Filters {}

impl DataDevopsguruNotificationChannel_Filters { }

impl terrars::ToListMappable for DataDevopsguruNotificationChannel_Filters {
    type O = terrars::BlockAssignable<DataDevopsguruNotificationChannel_Filters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDevopsguruNotificationChannel_Filters {}

impl BuildDataDevopsguruNotificationChannel_Filters {
    pub fn build(self) -> DataDevopsguruNotificationChannel_Filters {
        DataDevopsguruNotificationChannel_Filters {}
    }
}

#[allow(dead_code)]
pub struct DataDevopsguruNotificationChannel_FiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDevopsguruNotificationChannel_FiltersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDevopsguruNotificationChannel_FiltersRef {
        DataDevopsguruNotificationChannel_FiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDevopsguruNotificationChannel_FiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_types` after provisioning.\n"]
    pub fn message_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_types", self.base))
    }

    #[doc = "Get a reference to the value of field `severities` after provisioning.\n"]
    pub fn severities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.severities", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDevopsguruNotificationChannel_Sns {}

impl DataDevopsguruNotificationChannel_Sns { }

impl terrars::ToListMappable for DataDevopsguruNotificationChannel_Sns {
    type O = terrars::BlockAssignable<DataDevopsguruNotificationChannel_Sns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDevopsguruNotificationChannel_Sns {}

impl BuildDataDevopsguruNotificationChannel_Sns {
    pub fn build(self) -> DataDevopsguruNotificationChannel_Sns {
        DataDevopsguruNotificationChannel_Sns {}
    }
}

#[allow(dead_code)]
pub struct DataDevopsguruNotificationChannel_SnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDevopsguruNotificationChannel_SnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDevopsguruNotificationChannel_SnsRef {
        DataDevopsguruNotificationChannel_SnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDevopsguruNotificationChannel_SnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

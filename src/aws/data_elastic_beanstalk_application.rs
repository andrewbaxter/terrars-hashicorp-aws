use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataElasticBeanstalkApplicationData {
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
    name: terrars::PrimField<std::string::String>,
}

struct DataElasticBeanstalkApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataElasticBeanstalkApplicationData>,
}

#[derive(Clone)]
pub struct DataElasticBeanstalkApplication(std::rc::Rc<DataElasticBeanstalkApplication_>);

impl DataElasticBeanstalkApplication {
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

    #[doc = "Get a reference to the value of field `appversion_lifecycle` after provisioning.\n"]
    pub fn appversion_lifecycle(&self) -> terrars::ListRef<DataElasticBeanstalkApplicationAppversionLifecycleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.appversion_lifecycle", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl terrars::Referable for DataElasticBeanstalkApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataElasticBeanstalkApplication { }

impl terrars::ToListMappable for DataElasticBeanstalkApplication {
    type O = terrars::ListRef<DataElasticBeanstalkApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataElasticBeanstalkApplication_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_elastic_beanstalk_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataElasticBeanstalkApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataElasticBeanstalkApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> DataElasticBeanstalkApplication {
        let out = DataElasticBeanstalkApplication(std::rc::Rc::new(DataElasticBeanstalkApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataElasticBeanstalkApplicationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataElasticBeanstalkApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticBeanstalkApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataElasticBeanstalkApplicationRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `appversion_lifecycle` after provisioning.\n"]
    pub fn appversion_lifecycle(&self) -> terrars::ListRef<DataElasticBeanstalkApplicationAppversionLifecycleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.appversion_lifecycle", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataElasticBeanstalkApplicationAppversionLifecycleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_source_from_s3: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataElasticBeanstalkApplicationAppversionLifecycleEl {
    #[doc = "Set the field `delete_source_from_s3`.\n"]
    pub fn set_delete_source_from_s3(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_source_from_s3 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age_in_days`.\n"]
    pub fn set_max_age_in_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_count`.\n"]
    pub fn set_max_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_role`.\n"]
    pub fn set_service_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElasticBeanstalkApplicationAppversionLifecycleEl {
    type O = terrars::BlockAssignable<DataElasticBeanstalkApplicationAppversionLifecycleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElasticBeanstalkApplicationAppversionLifecycleEl {}

impl BuildDataElasticBeanstalkApplicationAppversionLifecycleEl {
    pub fn build(self) -> DataElasticBeanstalkApplicationAppversionLifecycleEl {
        DataElasticBeanstalkApplicationAppversionLifecycleEl {
            delete_source_from_s3: core::default::Default::default(),
            max_age_in_days: core::default::Default::default(),
            max_count: core::default::Default::default(),
            service_role: core::default::Default::default(),
        }
    }
}

pub struct DataElasticBeanstalkApplicationAppversionLifecycleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElasticBeanstalkApplicationAppversionLifecycleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataElasticBeanstalkApplicationAppversionLifecycleElRef {
        DataElasticBeanstalkApplicationAppversionLifecycleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElasticBeanstalkApplicationAppversionLifecycleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_source_from_s3` after provisioning.\n"]
    pub fn delete_source_from_s3(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_source_from_s3", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age_in_days` after provisioning.\n"]
    pub fn max_age_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age_in_days", self.base))
    }

    #[doc = "Get a reference to the value of field `max_count` after provisioning.\n"]
    pub fn max_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_count", self.base))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.base))
    }
}

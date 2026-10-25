#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataArcregionswitchRoute53HealthChecksData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    plan_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataArcregionswitchRoute53HealthChecks_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataArcregionswitchRoute53HealthChecksData_>,
}

#[derive(Clone)]
pub struct DataArcregionswitchRoute53HealthChecks(std::rc::Rc<DataArcregionswitchRoute53HealthChecks_>);

impl DataArcregionswitchRoute53HealthChecks {
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

    #[doc = "Get a reference to the value of field `health_checks` after provisioning.\n"]
    pub fn health_checks(&self) -> terrars::ListRef<DataArcregionswitchRoute53HealthChecks_HealthChecksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_checks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan_arn` after provisioning.\n"]
    pub fn plan_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plan_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataArcregionswitchRoute53HealthChecks {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataArcregionswitchRoute53HealthChecks { }

impl terrars::ToListMappable for DataArcregionswitchRoute53HealthChecks {
    type O = terrars::ListRef<DataArcregionswitchRoute53HealthChecksRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataArcregionswitchRoute53HealthChecks_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_arcregionswitch_route53_health_checks".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataArcregionswitchRoute53HealthChecks {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub plan_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataArcregionswitchRoute53HealthChecks {
    pub fn build(self, stack: &mut terrars::Stack) -> DataArcregionswitchRoute53HealthChecks {
        let out = DataArcregionswitchRoute53HealthChecks(std::rc::Rc::new(DataArcregionswitchRoute53HealthChecks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataArcregionswitchRoute53HealthChecksData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                plan_arn: self.plan_arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataArcregionswitchRoute53HealthChecksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArcregionswitchRoute53HealthChecksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataArcregionswitchRoute53HealthChecksRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `health_checks` after provisioning.\n"]
    pub fn health_checks(&self) -> terrars::ListRef<DataArcregionswitchRoute53HealthChecks_HealthChecksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_checks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan_arn` after provisioning.\n"]
    pub fn plan_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plan_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataArcregionswitchRoute53HealthChecks_HealthChecks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosted_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataArcregionswitchRoute53HealthChecks_HealthChecks {
    #[doc = "Set the field `health_check_id`.\n"]
    pub fn set_health_check_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hosted_zone_id`.\n"]
    pub fn set_hosted_zone_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hosted_zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_name`.\n"]
    pub fn set_record_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataArcregionswitchRoute53HealthChecks_HealthChecks {
    type O = terrars::BlockAssignable<DataArcregionswitchRoute53HealthChecks_HealthChecks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataArcregionswitchRoute53HealthChecks_HealthChecks {}

impl BuildDataArcregionswitchRoute53HealthChecks_HealthChecks {
    pub fn build(self) -> DataArcregionswitchRoute53HealthChecks_HealthChecks {
        DataArcregionswitchRoute53HealthChecks_HealthChecks {
            health_check_id: core::default::Default::default(),
            hosted_zone_id: core::default::Default::default(),
            record_name: core::default::Default::default(),
            region: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataArcregionswitchRoute53HealthChecks_HealthChecksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataArcregionswitchRoute53HealthChecks_HealthChecksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataArcregionswitchRoute53HealthChecks_HealthChecksRef {
        DataArcregionswitchRoute53HealthChecks_HealthChecksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataArcregionswitchRoute53HealthChecks_HealthChecksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `health_check_id` after provisioning.\n"]
    pub fn health_check_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_id", self.base))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.base))
    }

    #[doc = "Get a reference to the value of field `record_name` after provisioning.\n"]
    pub fn record_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

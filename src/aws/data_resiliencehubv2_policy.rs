#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataResiliencehubv2PolicyData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataResiliencehubv2Policy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataResiliencehubv2PolicyData_>,
}

#[derive(Clone)]
pub struct DataResiliencehubv2Policy(std::rc::Rc<DataResiliencehubv2Policy_>);

impl DataResiliencehubv2Policy {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_slo` after provisioning.\n"]
    pub fn availability_slo(&self) -> terrars::ListRef<DataResiliencehubv2Policy_AvailabilitySloRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_slo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_recovery` after provisioning.\n"]
    pub fn data_recovery(&self) -> terrars::ListRef<DataResiliencehubv2Policy_DataRecoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> terrars::ListRef<DataResiliencehubv2Policy_MultiAzRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region` after provisioning.\n"]
    pub fn multi_region(&self) -> terrars::ListRef<DataResiliencehubv2Policy_MultiRegionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sharing_enabled` after provisioning.\n"]
    pub fn sharing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sharing_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataResiliencehubv2Policy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataResiliencehubv2Policy { }

impl terrars::ToListMappable for DataResiliencehubv2Policy {
    type O = terrars::ListRef<DataResiliencehubv2PolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataResiliencehubv2Policy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_resiliencehubv2_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataResiliencehubv2Policy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataResiliencehubv2Policy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataResiliencehubv2Policy {
        let out = DataResiliencehubv2Policy(std::rc::Rc::new(DataResiliencehubv2Policy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataResiliencehubv2PolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataResiliencehubv2PolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResiliencehubv2PolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataResiliencehubv2PolicyRef {
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

    #[doc = "Get a reference to the value of field `availability_slo` after provisioning.\n"]
    pub fn availability_slo(&self) -> terrars::ListRef<DataResiliencehubv2Policy_AvailabilitySloRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_slo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_recovery` after provisioning.\n"]
    pub fn data_recovery(&self) -> terrars::ListRef<DataResiliencehubv2Policy_DataRecoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> terrars::ListRef<DataResiliencehubv2Policy_MultiAzRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_region` after provisioning.\n"]
    pub fn multi_region(&self) -> terrars::ListRef<DataResiliencehubv2Policy_MultiRegionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sharing_enabled` after provisioning.\n"]
    pub fn sharing_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sharing_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataResiliencehubv2Policy_AvailabilitySlo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<f64>>,
}

impl DataResiliencehubv2Policy_AvailabilitySlo {
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResiliencehubv2Policy_AvailabilitySlo {
    type O = terrars::BlockAssignable<DataResiliencehubv2Policy_AvailabilitySlo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResiliencehubv2Policy_AvailabilitySlo {}

impl BuildDataResiliencehubv2Policy_AvailabilitySlo {
    pub fn build(self) -> DataResiliencehubv2Policy_AvailabilitySlo {
        DataResiliencehubv2Policy_AvailabilitySlo { target: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataResiliencehubv2Policy_AvailabilitySloRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResiliencehubv2Policy_AvailabilitySloRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResiliencehubv2Policy_AvailabilitySloRef {
        DataResiliencehubv2Policy_AvailabilitySloRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResiliencehubv2Policy_AvailabilitySloRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResiliencehubv2Policy_DataRecovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_between_backups_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataResiliencehubv2Policy_DataRecovery {
    #[doc = "Set the field `time_between_backups_in_minutes`.\n"]
    pub fn set_time_between_backups_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.time_between_backups_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResiliencehubv2Policy_DataRecovery {
    type O = terrars::BlockAssignable<DataResiliencehubv2Policy_DataRecovery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResiliencehubv2Policy_DataRecovery {}

impl BuildDataResiliencehubv2Policy_DataRecovery {
    pub fn build(self) -> DataResiliencehubv2Policy_DataRecovery {
        DataResiliencehubv2Policy_DataRecovery { time_between_backups_in_minutes: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataResiliencehubv2Policy_DataRecoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResiliencehubv2Policy_DataRecoveryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResiliencehubv2Policy_DataRecoveryRef {
        DataResiliencehubv2Policy_DataRecoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResiliencehubv2Policy_DataRecoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `time_between_backups_in_minutes` after provisioning.\n"]
    pub fn time_between_backups_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_between_backups_in_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResiliencehubv2Policy_MultiAz {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disaster_recovery_approach: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rpo_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rto_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataResiliencehubv2Policy_MultiAz {
    #[doc = "Set the field `disaster_recovery_approach`.\n"]
    pub fn set_disaster_recovery_approach(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disaster_recovery_approach = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rpo_in_minutes`.\n"]
    pub fn set_rpo_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rpo_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rto_in_minutes`.\n"]
    pub fn set_rto_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rto_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResiliencehubv2Policy_MultiAz {
    type O = terrars::BlockAssignable<DataResiliencehubv2Policy_MultiAz>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResiliencehubv2Policy_MultiAz {}

impl BuildDataResiliencehubv2Policy_MultiAz {
    pub fn build(self) -> DataResiliencehubv2Policy_MultiAz {
        DataResiliencehubv2Policy_MultiAz {
            disaster_recovery_approach: core::default::Default::default(),
            rpo_in_minutes: core::default::Default::default(),
            rto_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataResiliencehubv2Policy_MultiAzRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResiliencehubv2Policy_MultiAzRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResiliencehubv2Policy_MultiAzRef {
        DataResiliencehubv2Policy_MultiAzRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResiliencehubv2Policy_MultiAzRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disaster_recovery_approach` after provisioning.\n"]
    pub fn disaster_recovery_approach(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disaster_recovery_approach", self.base))
    }

    #[doc = "Get a reference to the value of field `rpo_in_minutes` after provisioning.\n"]
    pub fn rpo_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpo_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `rto_in_minutes` after provisioning.\n"]
    pub fn rto_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rto_in_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResiliencehubv2Policy_MultiRegion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disaster_recovery_approach: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rpo_in_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rto_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataResiliencehubv2Policy_MultiRegion {
    #[doc = "Set the field `disaster_recovery_approach`.\n"]
    pub fn set_disaster_recovery_approach(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disaster_recovery_approach = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rpo_in_minutes`.\n"]
    pub fn set_rpo_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rpo_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rto_in_minutes`.\n"]
    pub fn set_rto_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rto_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResiliencehubv2Policy_MultiRegion {
    type O = terrars::BlockAssignable<DataResiliencehubv2Policy_MultiRegion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResiliencehubv2Policy_MultiRegion {}

impl BuildDataResiliencehubv2Policy_MultiRegion {
    pub fn build(self) -> DataResiliencehubv2Policy_MultiRegion {
        DataResiliencehubv2Policy_MultiRegion {
            disaster_recovery_approach: core::default::Default::default(),
            rpo_in_minutes: core::default::Default::default(),
            rto_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataResiliencehubv2Policy_MultiRegionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResiliencehubv2Policy_MultiRegionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResiliencehubv2Policy_MultiRegionRef {
        DataResiliencehubv2Policy_MultiRegionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResiliencehubv2Policy_MultiRegionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disaster_recovery_approach` after provisioning.\n"]
    pub fn disaster_recovery_approach(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disaster_recovery_approach", self.base))
    }

    #[doc = "Get a reference to the value of field `rpo_in_minutes` after provisioning.\n"]
    pub fn rpo_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rpo_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `rto_in_minutes` after provisioning.\n"]
    pub fn rto_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rto_in_minutes", self.base))
    }
}

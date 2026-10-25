#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmPatchBaselinesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_baselines: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataSsmPatchBaselines_Filter>>,
    dynamic: DataSsmPatchBaselinesDynamic,
}

struct DataSsmPatchBaselines_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSsmPatchBaselinesData_>,
}

#[derive(Clone)]
pub struct DataSsmPatchBaselines(std::rc::Rc<DataSsmPatchBaselines_>);

impl DataSsmPatchBaselines {
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

    #[doc = "Set the field `default_baselines`.\n"]
    pub fn set_default_baselines(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_baselines = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataSsmPatchBaselines_Filter>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `baseline_identities` after provisioning.\n"]
    pub fn baseline_identities(&self) -> terrars::ListRef<DataSsmPatchBaselines_BaselineIdentitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_identities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_baselines` after provisioning.\n"]
    pub fn default_baselines(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_baselines", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataSsmPatchBaselines_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl terrars::Referable for DataSsmPatchBaselines {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSsmPatchBaselines { }

impl terrars::ToListMappable for DataSsmPatchBaselines {
    type O = terrars::ListRef<DataSsmPatchBaselinesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSsmPatchBaselines_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ssm_patch_baselines".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmPatchBaselines {
    pub tf_id: std::string::String,
}

impl BuildDataSsmPatchBaselines {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSsmPatchBaselines {
        let out = DataSsmPatchBaselines(std::rc::Rc::new(DataSsmPatchBaselines_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSsmPatchBaselinesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                default_baselines: core::default::Default::default(),
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmPatchBaselinesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmPatchBaselinesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSsmPatchBaselinesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `baseline_identities` after provisioning.\n"]
    pub fn baseline_identities(&self) -> terrars::ListRef<DataSsmPatchBaselines_BaselineIdentitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_identities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_baselines` after provisioning.\n"]
    pub fn default_baselines(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_baselines", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataSsmPatchBaselines_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataSsmPatchBaselinesDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataSsmPatchBaselines_Filter>>,
}

#[derive(Serialize)]
pub struct DataSsmPatchBaselines_BaselineIdentities {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_baseline: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSsmPatchBaselines_BaselineIdentities {
    #[doc = "Set the field `baseline_description`.\n"]
    pub fn set_baseline_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.baseline_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `baseline_id`.\n"]
    pub fn set_baseline_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.baseline_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `baseline_name`.\n"]
    pub fn set_baseline_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.baseline_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_baseline`.\n"]
    pub fn set_default_baseline(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_baseline = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system`.\n"]
    pub fn set_operating_system(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operating_system = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSsmPatchBaselines_BaselineIdentities {
    type O = terrars::BlockAssignable<DataSsmPatchBaselines_BaselineIdentities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmPatchBaselines_BaselineIdentities {}

impl BuildDataSsmPatchBaselines_BaselineIdentities {
    pub fn build(self) -> DataSsmPatchBaselines_BaselineIdentities {
        DataSsmPatchBaselines_BaselineIdentities {
            baseline_description: core::default::Default::default(),
            baseline_id: core::default::Default::default(),
            baseline_name: core::default::Default::default(),
            default_baseline: core::default::Default::default(),
            operating_system: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmPatchBaselines_BaselineIdentitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmPatchBaselines_BaselineIdentitiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmPatchBaselines_BaselineIdentitiesRef {
        DataSsmPatchBaselines_BaselineIdentitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmPatchBaselines_BaselineIdentitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `baseline_description` after provisioning.\n"]
    pub fn baseline_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.baseline_description", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_id` after provisioning.\n"]
    pub fn baseline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.baseline_id", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_name` after provisioning.\n"]
    pub fn baseline_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.baseline_name", self.base))
    }

    #[doc = "Get a reference to the value of field `default_baseline` after provisioning.\n"]
    pub fn default_baseline(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_baseline", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmPatchBaselines_Filter {
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataSsmPatchBaselines_Filter { }

impl terrars::ToListMappable for DataSsmPatchBaselines_Filter {
    type O = terrars::BlockAssignable<DataSsmPatchBaselines_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmPatchBaselines_Filter {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataSsmPatchBaselines_Filter {
    pub fn build(self) -> DataSsmPatchBaselines_Filter {
        DataSsmPatchBaselines_Filter {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataSsmPatchBaselines_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSsmPatchBaselines_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSsmPatchBaselines_FilterRef {
        DataSsmPatchBaselines_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmPatchBaselines_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEmrSupportedInstanceTypesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    release_label: terrars::PrimField<std::string::String>,
}

struct DataEmrSupportedInstanceTypes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmrSupportedInstanceTypesData_>,
}

#[derive(Clone)]
pub struct DataEmrSupportedInstanceTypes(std::rc::Rc<DataEmrSupportedInstanceTypes_>);

impl DataEmrSupportedInstanceTypes {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_instance_types` after provisioning.\n"]
    pub fn supported_instance_types(&self) -> terrars::ListRef<DataEmrSupportedInstanceTypes_SupportedInstanceTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_instance_types", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmrSupportedInstanceTypes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmrSupportedInstanceTypes { }

impl terrars::ToListMappable for DataEmrSupportedInstanceTypes {
    type O = terrars::ListRef<DataEmrSupportedInstanceTypesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmrSupportedInstanceTypes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_emr_supported_instance_types".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmrSupportedInstanceTypes {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub release_label: terrars::PrimField<std::string::String>,
}

impl BuildDataEmrSupportedInstanceTypes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmrSupportedInstanceTypes {
        let out = DataEmrSupportedInstanceTypes(std::rc::Rc::new(DataEmrSupportedInstanceTypes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmrSupportedInstanceTypesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                release_label: self.release_label,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmrSupportedInstanceTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmrSupportedInstanceTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmrSupportedInstanceTypesRef {
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

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_instance_types` after provisioning.\n"]
    pub fn supported_instance_types(&self) -> terrars::ListRef<DataEmrSupportedInstanceTypes_SupportedInstanceTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_instance_types", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmrSupportedInstanceTypes_SupportedInstanceTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_optimized_available: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_optimized_by_default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_storage_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_family_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_64_bits_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_disks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEmrSupportedInstanceTypes_SupportedInstanceTypes {
    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_optimized_available`.\n"]
    pub fn set_ebs_optimized_available(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ebs_optimized_available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_optimized_by_default`.\n"]
    pub fn set_ebs_optimized_by_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ebs_optimized_by_default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_storage_only`.\n"]
    pub fn set_ebs_storage_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ebs_storage_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_family_id`.\n"]
    pub fn set_instance_family_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_family_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_64_bits_only`.\n"]
    pub fn set_is_64_bits_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_64_bits_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\n"]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_disks`.\n"]
    pub fn set_number_of_disks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_disks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_gb`.\n"]
    pub fn set_storage_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vcpu`.\n"]
    pub fn set_vcpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vcpu = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEmrSupportedInstanceTypes_SupportedInstanceTypes {
    type O = terrars::BlockAssignable<DataEmrSupportedInstanceTypes_SupportedInstanceTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrSupportedInstanceTypes_SupportedInstanceTypes {}

impl BuildDataEmrSupportedInstanceTypes_SupportedInstanceTypes {
    pub fn build(self) -> DataEmrSupportedInstanceTypes_SupportedInstanceTypes {
        DataEmrSupportedInstanceTypes_SupportedInstanceTypes {
            architecture: core::default::Default::default(),
            ebs_optimized_available: core::default::Default::default(),
            ebs_optimized_by_default: core::default::Default::default(),
            ebs_storage_only: core::default::Default::default(),
            instance_family_id: core::default::Default::default(),
            is_64_bits_only: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            number_of_disks: core::default::Default::default(),
            storage_gb: core::default::Default::default(),
            type_: core::default::Default::default(),
            vcpu: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEmrSupportedInstanceTypes_SupportedInstanceTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmrSupportedInstanceTypes_SupportedInstanceTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEmrSupportedInstanceTypes_SupportedInstanceTypesRef {
        DataEmrSupportedInstanceTypes_SupportedInstanceTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrSupportedInstanceTypes_SupportedInstanceTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized_available` after provisioning.\n"]
    pub fn ebs_optimized_available(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized_available", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized_by_default` after provisioning.\n"]
    pub fn ebs_optimized_by_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized_by_default", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs_storage_only` after provisioning.\n"]
    pub fn ebs_storage_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_storage_only", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_family_id` after provisioning.\n"]
    pub fn instance_family_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_family_id", self.base))
    }

    #[doc = "Get a reference to the value of field `is_64_bits_only` after provisioning.\n"]
    pub fn is_64_bits_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_64_bits_only", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\n"]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_disks` after provisioning.\n"]
    pub fn number_of_disks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_disks", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_gb` after provisioning.\n"]
    pub fn storage_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu` after provisioning.\n"]
    pub fn vcpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcpu", self.base))
    }
}

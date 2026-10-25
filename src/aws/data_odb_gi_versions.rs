#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbGiVersionsData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shape: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOdbGiVersions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOdbGiVersionsData_>,
}

#[derive(Clone)]
pub struct DataOdbGiVersions(std::rc::Rc<DataOdbGiVersions_>);

impl DataOdbGiVersions {
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

    #[doc = "Set the field `shape`.\nThe system shape."]
    pub fn set_shape(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().shape = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `gi_versions` after provisioning.\nInformation about a specific version of Oracle Grid Infrastructure (GI) software that can be installed on a VM cluster."]
    pub fn gi_versions(&self) -> terrars::ListRef<DataOdbGiVersions_GiVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gi_versions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\nThe system shape."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }
}

impl terrars::Referable for DataOdbGiVersions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOdbGiVersions { }

impl terrars::ToListMappable for DataOdbGiVersions {
    type O = terrars::ListRef<DataOdbGiVersionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOdbGiVersions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_odb_gi_versions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbGiVersions {
    pub tf_id: std::string::String,
}

impl BuildDataOdbGiVersions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOdbGiVersions {
        let out = DataOdbGiVersions(std::rc::Rc::new(DataOdbGiVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOdbGiVersionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                shape: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbGiVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbGiVersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOdbGiVersionsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `gi_versions` after provisioning.\nInformation about a specific version of Oracle Grid Infrastructure (GI) software that can be installed on a VM cluster."]
    pub fn gi_versions(&self) -> terrars::ListRef<DataOdbGiVersions_GiVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gi_versions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\nThe system shape."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbGiVersions_GiVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbGiVersions_GiVersions {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbGiVersions_GiVersions {
    type O = terrars::BlockAssignable<DataOdbGiVersions_GiVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbGiVersions_GiVersions {}

impl BuildDataOdbGiVersions_GiVersions {
    pub fn build(self) -> DataOdbGiVersions_GiVersions {
        DataOdbGiVersions_GiVersions { version: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataOdbGiVersions_GiVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbGiVersions_GiVersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataOdbGiVersions_GiVersionsRef {
        DataOdbGiVersions_GiVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbGiVersions_GiVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

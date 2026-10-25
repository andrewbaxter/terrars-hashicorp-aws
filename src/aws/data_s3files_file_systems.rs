#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataS3filesFileSystemsData_ {
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
    file_systems: std::option::Option<std::vec::Vec<DataS3filesFileSystems_FileSystems>>,
    dynamic: DataS3filesFileSystemsDynamic,
}

struct DataS3filesFileSystems_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataS3filesFileSystemsData_>,
}

#[derive(Clone)]
pub struct DataS3filesFileSystems(std::rc::Rc<DataS3filesFileSystems_>);

impl DataS3filesFileSystems {
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

    #[doc = "Set the field `file_systems`.\n"]
    pub fn set_file_systems(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataS3filesFileSystems_FileSystems>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_systems = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_systems = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_systems` after provisioning.\n"]
    pub fn file_systems(&self) -> terrars::ListRef<DataS3filesFileSystems_FileSystemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_systems", self.extract_ref()))
    }
}

impl terrars::Referable for DataS3filesFileSystems {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataS3filesFileSystems { }

impl terrars::ToListMappable for DataS3filesFileSystems {
    type O = terrars::ListRef<DataS3filesFileSystemsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataS3filesFileSystems_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_s3files_file_systems".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataS3filesFileSystems {
    pub tf_id: std::string::String,
}

impl BuildDataS3filesFileSystems {
    pub fn build(self, stack: &mut terrars::Stack) -> DataS3filesFileSystems {
        let out = DataS3filesFileSystems(std::rc::Rc::new(DataS3filesFileSystems_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataS3filesFileSystemsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                file_systems: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataS3filesFileSystemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3filesFileSystemsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataS3filesFileSystemsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_systems` after provisioning.\n"]
    pub fn file_systems(&self) -> terrars::ListRef<DataS3filesFileSystems_FileSystemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_systems", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataS3filesFileSystemsDynamic {
    file_systems: std::option::Option<terrars::DynamicBlock<DataS3filesFileSystems_FileSystems>>,
}

#[derive(Serialize)]
pub struct DataS3filesFileSystems_FileSystems {}

impl DataS3filesFileSystems_FileSystems { }

impl terrars::ToListMappable for DataS3filesFileSystems_FileSystems {
    type O = terrars::BlockAssignable<DataS3filesFileSystems_FileSystems>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3filesFileSystems_FileSystems {}

impl BuildDataS3filesFileSystems_FileSystems {
    pub fn build(self) -> DataS3filesFileSystems_FileSystems {
        DataS3filesFileSystems_FileSystems {}
    }
}

#[allow(dead_code)]
pub struct DataS3filesFileSystems_FileSystemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3filesFileSystems_FileSystemsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataS3filesFileSystems_FileSystemsRef {
        DataS3filesFileSystems_FileSystemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3filesFileSystems_FileSystemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\nFile system ARN"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nS3 bucket ARN"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nCreation time"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nFile system ID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\nKMS key ID for encryption"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nFile system name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\nAWS account ID of the owner"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\nIAM role ARN"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nFile system status"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\nStatus message"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TransferWorkflowData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_exception_steps: std::option::Option<std::vec::Vec<TransferWorkflow_OnExceptionSteps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std::option::Option<std::vec::Vec<TransferWorkflow_Steps>>,
    dynamic: TransferWorkflowDynamic,
}

struct TransferWorkflow_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TransferWorkflowData_>,
}

#[derive(Clone)]
pub struct TransferWorkflow(std::rc::Rc<TransferWorkflow_>);

impl TransferWorkflow {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_exception_steps`.\n"]
    pub fn set_on_exception_steps(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_OnExceptionSteps>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_exception_steps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_exception_steps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(self, v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().steps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.steps = std::option::Option::Some(d);
            },
        }
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_exception_steps` after provisioning.\n"]
    pub fn on_exception_steps(&self) -> terrars::ListRef<TransferWorkflow_OnExceptionStepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_exception_steps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> terrars::ListRef<TransferWorkflow_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.extract_ref()))
    }
}

impl terrars::Referable for TransferWorkflow {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TransferWorkflow { }

impl terrars::ToListMappable for TransferWorkflow {
    type O = terrars::ListRef<TransferWorkflowRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TransferWorkflow_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_transfer_workflow".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTransferWorkflow {
    pub tf_id: std::string::String,
}

impl BuildTransferWorkflow {
    pub fn build(self, stack: &mut terrars::Stack) -> TransferWorkflow {
        let out = TransferWorkflow(std::rc::Rc::new(TransferWorkflow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TransferWorkflowData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                on_exception_steps: core::default::Default::default(),
                steps: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TransferWorkflowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TransferWorkflowRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_exception_steps` after provisioning.\n"]
    pub fn on_exception_steps(&self) -> terrars::ListRef<TransferWorkflow_OnExceptionStepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_exception_steps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> terrars::ListRef<TransferWorkflow_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowDynamic {
    on_exception_steps: std::option::Option<terrars::DynamicBlock<TransferWorkflow_OnExceptionSteps>>,
    steps: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps>>,
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation {
    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_system_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation {
    type O =
        terrars::BlockAssignable<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation {}

impl BuildTransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation {
    pub fn build(self) -> TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation {
        TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation {
            file_system_id: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocationRef {
        TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation {
    type O = terrars::BlockAssignable<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation {}

impl BuildTransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation {
    pub fn build(self) -> TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation {
        TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocationRef {
        TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocationDynamic {
    efs_file_location: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation>>,
    s3_file_location: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation>>,
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_file_location: std
    ::option
    ::Option<std::vec::Vec<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_file_location: std
    ::option
    ::Option<std::vec::Vec<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation>>,
    dynamic: TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocationDynamic,
}

impl TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation {
    #[doc = "Set the field `efs_file_location`.\n"]
    pub fn set_efs_file_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.efs_file_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_file_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_file_location`.\n"]
    pub fn set_s3_file_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_file_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_file_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation {
    type O = terrars::BlockAssignable<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation {}

impl BuildTransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation {
    pub fn build(self) -> TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation {
        TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation {
            efs_file_location: core::default::Default::default(),
            s3_file_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocationRef {
        TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_location` after provisioning.\n"]
    pub fn efs_file_location(
        &self,
    ) -> terrars::ListRef<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_EfsFileLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_file_location", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_file_location` after provisioning.\n"]
    pub fn s3_file_location(
        &self,
    ) -> terrars::ListRef<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation_S3FileLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_file_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflow_Steps_CopyStepDetailsDynamic {
    destination_file_location: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation>>,
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps_CopyStepDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_existing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_file_location: std
    ::option
    ::Option<std::vec::Vec<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation>>,
    dynamic: TransferWorkflow_Steps_CopyStepDetailsDynamic,
}

impl TransferWorkflow_Steps_CopyStepDetails {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overwrite_existing`.\n"]
    pub fn set_overwrite_existing(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.overwrite_existing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_file_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_file_location`.\n"]
    pub fn set_destination_file_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_file_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_file_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_Steps_CopyStepDetails {
    type O = terrars::BlockAssignable<TransferWorkflow_Steps_CopyStepDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps_CopyStepDetails {}

impl BuildTransferWorkflow_Steps_CopyStepDetails {
    pub fn build(self) -> TransferWorkflow_Steps_CopyStepDetails {
        TransferWorkflow_Steps_CopyStepDetails {
            name: core::default::Default::default(),
            overwrite_existing: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            destination_file_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_Steps_CopyStepDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_Steps_CopyStepDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflow_Steps_CopyStepDetailsRef {
        TransferWorkflow_Steps_CopyStepDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_Steps_CopyStepDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_existing` after provisioning.\n"]
    pub fn overwrite_existing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overwrite_existing", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_location", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_file_location` after provisioning.\n"]
    pub fn destination_file_location(
        &self,
    ) -> terrars::ListRef<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_file_location", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps_CustomStepDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl TransferWorkflow_Steps_CustomStepDetails {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_file_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_Steps_CustomStepDetails {
    type O = terrars::BlockAssignable<TransferWorkflow_Steps_CustomStepDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps_CustomStepDetails {}

impl BuildTransferWorkflow_Steps_CustomStepDetails {
    pub fn build(self) -> TransferWorkflow_Steps_CustomStepDetails {
        TransferWorkflow_Steps_CustomStepDetails {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            target: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_Steps_CustomStepDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_Steps_CustomStepDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflow_Steps_CustomStepDetailsRef {
        TransferWorkflow_Steps_CustomStepDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_Steps_CustomStepDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_location", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflow_Steps_DecryptStepDetailsDynamic {
    destination_file_location: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation>>,
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps_DecryptStepDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_existing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_file_location: std
    ::option
    ::Option<std::vec::Vec<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation>>,
    dynamic: TransferWorkflow_Steps_DecryptStepDetailsDynamic,
}

impl TransferWorkflow_Steps_DecryptStepDetails {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overwrite_existing`.\n"]
    pub fn set_overwrite_existing(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.overwrite_existing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_file_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_file_location`.\n"]
    pub fn set_destination_file_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_file_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_file_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_Steps_DecryptStepDetails {
    type O = terrars::BlockAssignable<TransferWorkflow_Steps_DecryptStepDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps_DecryptStepDetails {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildTransferWorkflow_Steps_DecryptStepDetails {
    pub fn build(self) -> TransferWorkflow_Steps_DecryptStepDetails {
        TransferWorkflow_Steps_DecryptStepDetails {
            name: core::default::Default::default(),
            overwrite_existing: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            type_: self.type_,
            destination_file_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_Steps_DecryptStepDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_Steps_DecryptStepDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflow_Steps_DecryptStepDetailsRef {
        TransferWorkflow_Steps_DecryptStepDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_Steps_DecryptStepDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_existing` after provisioning.\n"]
    pub fn overwrite_existing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overwrite_existing", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_location", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_file_location` after provisioning.\n"]
    pub fn destination_file_location(
        &self,
    ) -> terrars::ListRef<TransferWorkflow_Steps_CopyStepDetails_DestinationFileLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_file_location", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps_DeleteStepDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWorkflow_Steps_DeleteStepDetails {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_file_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_Steps_DeleteStepDetails {
    type O = terrars::BlockAssignable<TransferWorkflow_Steps_DeleteStepDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps_DeleteStepDetails {}

impl BuildTransferWorkflow_Steps_DeleteStepDetails {
    pub fn build(self) -> TransferWorkflow_Steps_DeleteStepDetails {
        TransferWorkflow_Steps_DeleteStepDetails {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_Steps_DeleteStepDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_Steps_DeleteStepDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflow_Steps_DeleteStepDetailsRef {
        TransferWorkflow_Steps_DeleteStepDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_Steps_DeleteStepDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_location", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps_TagStepDetails_Tags {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl TransferWorkflow_Steps_TagStepDetails_Tags { }

impl terrars::ToListMappable for TransferWorkflow_Steps_TagStepDetails_Tags {
    type O = terrars::BlockAssignable<TransferWorkflow_Steps_TagStepDetails_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps_TagStepDetails_Tags {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildTransferWorkflow_Steps_TagStepDetails_Tags {
    pub fn build(self) -> TransferWorkflow_Steps_TagStepDetails_Tags {
        TransferWorkflow_Steps_TagStepDetails_Tags {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_Steps_TagStepDetails_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_Steps_TagStepDetails_TagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflow_Steps_TagStepDetails_TagsRef {
        TransferWorkflow_Steps_TagStepDetails_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_Steps_TagStepDetails_TagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflow_Steps_TagStepDetailsDynamic {
    tags: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_TagStepDetails_Tags>>,
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps_TagStepDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_TagStepDetails_Tags>>,
    dynamic: TransferWorkflow_Steps_TagStepDetailsDynamic,
}

impl TransferWorkflow_Steps_TagStepDetails {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_file_location`.\n"]
    pub fn set_source_file_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_file_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_TagStepDetails_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_Steps_TagStepDetails {
    type O = terrars::BlockAssignable<TransferWorkflow_Steps_TagStepDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps_TagStepDetails {}

impl BuildTransferWorkflow_Steps_TagStepDetails {
    pub fn build(self) -> TransferWorkflow_Steps_TagStepDetails {
        TransferWorkflow_Steps_TagStepDetails {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_Steps_TagStepDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_Steps_TagStepDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflow_Steps_TagStepDetailsRef {
        TransferWorkflow_Steps_TagStepDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_Steps_TagStepDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_file_location` after provisioning.\n"]
    pub fn source_file_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_location", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<TransferWorkflow_Steps_TagStepDetails_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflow_OnExceptionStepsDynamic {
    copy_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_CopyStepDetails>>,
    custom_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_CustomStepDetails>>,
    decrypt_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_DecryptStepDetails>>,
    delete_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_DeleteStepDetails>>,
    tag_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_TagStepDetails>>,
}

#[derive(Serialize)]
pub struct TransferWorkflow_OnExceptionSteps {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_CopyStepDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_CustomStepDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decrypt_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_DecryptStepDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_DeleteStepDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_TagStepDetails>>,
    dynamic: TransferWorkflow_OnExceptionStepsDynamic,
}

impl TransferWorkflow_OnExceptionSteps {
    #[doc = "Set the field `copy_step_details`.\n"]
    pub fn set_copy_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_CopyStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.copy_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.copy_step_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_step_details`.\n"]
    pub fn set_custom_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_CustomStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_step_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `decrypt_step_details`.\n"]
    pub fn set_decrypt_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_DecryptStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.decrypt_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.decrypt_step_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delete_step_details`.\n"]
    pub fn set_delete_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_DeleteStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delete_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delete_step_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_step_details`.\n"]
    pub fn set_tag_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_TagStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_step_details = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_OnExceptionSteps {
    type O = terrars::BlockAssignable<TransferWorkflow_OnExceptionSteps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_OnExceptionSteps {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildTransferWorkflow_OnExceptionSteps {
    pub fn build(self) -> TransferWorkflow_OnExceptionSteps {
        TransferWorkflow_OnExceptionSteps {
            type_: self.type_,
            copy_step_details: core::default::Default::default(),
            custom_step_details: core::default::Default::default(),
            decrypt_step_details: core::default::Default::default(),
            delete_step_details: core::default::Default::default(),
            tag_step_details: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_OnExceptionStepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_OnExceptionStepsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflow_OnExceptionStepsRef {
        TransferWorkflow_OnExceptionStepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_OnExceptionStepsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_step_details` after provisioning.\n"]
    pub fn copy_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_CopyStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.copy_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_step_details` after provisioning.\n"]
    pub fn custom_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_CustomStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `decrypt_step_details` after provisioning.\n"]
    pub fn decrypt_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_DecryptStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.decrypt_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_step_details` after provisioning.\n"]
    pub fn delete_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_DeleteStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_step_details` after provisioning.\n"]
    pub fn tag_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_TagStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_step_details", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflow_StepsDynamic {
    copy_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_CopyStepDetails>>,
    custom_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_CustomStepDetails>>,
    decrypt_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_DecryptStepDetails>>,
    delete_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_DeleteStepDetails>>,
    tag_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflow_Steps_TagStepDetails>>,
}

#[derive(Serialize)]
pub struct TransferWorkflow_Steps {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_CopyStepDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_CustomStepDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decrypt_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_DecryptStepDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_DeleteStepDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_step_details: std::option::Option<std::vec::Vec<TransferWorkflow_Steps_TagStepDetails>>,
    dynamic: TransferWorkflow_StepsDynamic,
}

impl TransferWorkflow_Steps {
    #[doc = "Set the field `copy_step_details`.\n"]
    pub fn set_copy_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_CopyStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.copy_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.copy_step_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_step_details`.\n"]
    pub fn set_custom_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_CustomStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_step_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `decrypt_step_details`.\n"]
    pub fn set_decrypt_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_DecryptStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.decrypt_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.decrypt_step_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delete_step_details`.\n"]
    pub fn set_delete_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_DeleteStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delete_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delete_step_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_step_details`.\n"]
    pub fn set_tag_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflow_Steps_TagStepDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_step_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_step_details = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferWorkflow_Steps {
    type O = terrars::BlockAssignable<TransferWorkflow_Steps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflow_Steps {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildTransferWorkflow_Steps {
    pub fn build(self) -> TransferWorkflow_Steps {
        TransferWorkflow_Steps {
            type_: self.type_,
            copy_step_details: core::default::Default::default(),
            custom_step_details: core::default::Default::default(),
            decrypt_step_details: core::default::Default::default(),
            delete_step_details: core::default::Default::default(),
            tag_step_details: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferWorkflow_StepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflow_StepsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflow_StepsRef {
        TransferWorkflow_StepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflow_StepsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_step_details` after provisioning.\n"]
    pub fn copy_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_CopyStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.copy_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_step_details` after provisioning.\n"]
    pub fn custom_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_CustomStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `decrypt_step_details` after provisioning.\n"]
    pub fn decrypt_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_DecryptStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.decrypt_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_step_details` after provisioning.\n"]
    pub fn delete_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_DeleteStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_step_details` after provisioning.\n"]
    pub fn tag_step_details(&self) -> terrars::ListRef<TransferWorkflow_Steps_TagStepDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_step_details", self.base))
    }
}

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TransferWorkflowData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_exception_steps: std::option::Option<std::vec::Vec<TransferWorkflowOnExceptionStepsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std::option::Option<std::vec::Vec<TransferWorkflowStepsEl>>,
    dynamic: TransferWorkflowDynamic,
}

struct TransferWorkflow_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TransferWorkflowData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowOnExceptionStepsEl>>,
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
    pub fn set_steps(self, v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowStepsEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_exception_steps` after provisioning.\n"]
    pub fn on_exception_steps(&self) -> terrars::ListRef<TransferWorkflowOnExceptionStepsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_exception_steps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> terrars::ListRef<TransferWorkflowStepsElRef> {
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
            data: std::cell::RefCell::new(TransferWorkflowData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_exception_steps` after provisioning.\n"]
    pub fn on_exception_steps(&self) -> terrars::ListRef<TransferWorkflowOnExceptionStepsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_exception_steps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> terrars::ListRef<TransferWorkflowStepsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
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

impl terrars
::ToListMappable for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    type O =
        terrars
        ::BlockAssignable<
            TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {}

impl BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
            file_system_id: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
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
pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
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

impl terrars
::ToListMappable for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    type O =
        terrars
        ::BlockAssignable<
            TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {}

impl BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    pub fn build(
        self,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
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
struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElDynamic {
    efs_file_location: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl>,
    >,
    s3_file_location: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl>,
    >,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_file_location: std
    ::option
    ::Option<
        std::vec::Vec<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_file_location: std
    ::option
    ::Option<
        std::vec::Vec<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl>,
    >,
    dynamic: TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElDynamic,
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
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
                            TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl,
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
                            TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl,
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

impl terrars::ToListMappable for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
    type O = terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {}

impl BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl {
            efs_file_location: core::default::Default::default(),
            s3_file_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_location` after provisioning.\n"]
    pub fn efs_file_location(
        &self,
    ) -> terrars
    ::ListRef<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_file_location", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_file_location` after provisioning.\n"]
    pub fn s3_file_location(
        &self,
    ) -> terrars
    ::ListRef<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_file_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElDynamic {
    destination_file_location: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_existing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_file_location: std
    ::option
    ::Option<std::vec::Vec<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl>>,
    dynamic: TransferWorkflowOnExceptionStepsElCopyStepDetailsElDynamic,
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
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
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationEl,
                        >,
                    >,
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

impl terrars::ToListMappable for TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
    type O = terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElCopyStepDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsEl {}

impl BuildTransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsEl {
            name: core::default::Default::default(),
            overwrite_existing: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            destination_file_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
        TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef {
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
    ) -> terrars::ListRef<TransferWorkflowOnExceptionStepsElCopyStepDetailsElDestinationFileLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_file_location", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
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

impl terrars::ToListMappable for TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
    type O = terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElCustomStepDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElCustomStepDetailsEl {}

impl BuildTransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
        TransferWorkflowOnExceptionStepsElCustomStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            target: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
        TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef {
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

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
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

impl terrars::ToListMappable for TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
    type O = terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {}

impl BuildTransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
        TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
        TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef {
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
pub struct TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl { }

impl terrars::ToListMappable for TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
    type O = terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildTransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
        TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
        TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef {
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
struct TransferWorkflowOnExceptionStepsElTagStepDetailsElDynamic {
    tags: std::option::Option<terrars::DynamicBlock<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl>>,
    dynamic: TransferWorkflowOnExceptionStepsElTagStepDetailsElDynamic,
}

impl TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsEl>>,
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

impl terrars::ToListMappable for TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
    type O = terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElTagStepDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsElTagStepDetailsEl {}

impl BuildTransferWorkflowOnExceptionStepsElTagStepDetailsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
        TransferWorkflowOnExceptionStepsElTagStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
        TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElTagStepDetailsElRef {
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
    pub fn tags(&self) -> terrars::ListRef<TransferWorkflowOnExceptionStepsElTagStepDetailsElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowOnExceptionStepsElDynamic {
    copy_step_details: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflowOnExceptionStepsElCopyStepDetailsEl>>,
    custom_step_details: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflowOnExceptionStepsElCustomStepDetailsEl>>,
    delete_step_details: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl>>,
    tag_step_details: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflowOnExceptionStepsElTagStepDetailsEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowOnExceptionStepsEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_step_details: std::option::Option<std::vec::Vec<TransferWorkflowOnExceptionStepsElCopyStepDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_step_details: std::option::Option<std::vec::Vec<TransferWorkflowOnExceptionStepsElCustomStepDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_step_details: std::option::Option<std::vec::Vec<TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_step_details: std::option::Option<std::vec::Vec<TransferWorkflowOnExceptionStepsElTagStepDetailsEl>>,
    dynamic: TransferWorkflowOnExceptionStepsElDynamic,
}

impl TransferWorkflowOnExceptionStepsEl {
    #[doc = "Set the field `copy_step_details`.\n"]
    pub fn set_copy_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElCopyStepDetailsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElCustomStepDetailsEl>>,
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

    #[doc = "Set the field `delete_step_details`.\n"]
    pub fn set_delete_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElDeleteStepDetailsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowOnExceptionStepsElTagStepDetailsEl>>,
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

impl terrars::ToListMappable for TransferWorkflowOnExceptionStepsEl {
    type O = terrars::BlockAssignable<TransferWorkflowOnExceptionStepsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowOnExceptionStepsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildTransferWorkflowOnExceptionStepsEl {
    pub fn build(self) -> TransferWorkflowOnExceptionStepsEl {
        TransferWorkflowOnExceptionStepsEl {
            type_: self.type_,
            copy_step_details: core::default::Default::default(),
            custom_step_details: core::default::Default::default(),
            delete_step_details: core::default::Default::default(),
            tag_step_details: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowOnExceptionStepsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowOnExceptionStepsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflowOnExceptionStepsElRef {
        TransferWorkflowOnExceptionStepsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowOnExceptionStepsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_step_details` after provisioning.\n"]
    pub fn copy_step_details(&self) -> terrars::ListRef<TransferWorkflowOnExceptionStepsElCopyStepDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.copy_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_step_details` after provisioning.\n"]
    pub fn custom_step_details(&self) -> terrars::ListRef<TransferWorkflowOnExceptionStepsElCustomStepDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_step_details` after provisioning.\n"]
    pub fn delete_step_details(&self) -> terrars::ListRef<TransferWorkflowOnExceptionStepsElDeleteStepDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_step_details` after provisioning.\n"]
    pub fn tag_step_details(&self) -> terrars::ListRef<TransferWorkflowOnExceptionStepsElTagStepDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_step_details", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
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

impl terrars::ToListMappable for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    type O =
        terrars::BlockAssignable<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {}

impl BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
    pub fn build(self) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl {
            file_system_id: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef {
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
pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
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

impl terrars::ToListMappable for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    type O =
        terrars::BlockAssignable<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {}

impl BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
    pub fn build(self) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef {
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
struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElDynamic {
    efs_file_location: std
    ::option
    ::Option<
        terrars::DynamicBlock<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl>,
    >,
    s3_file_location: std
    ::option
    ::Option<
        terrars::DynamicBlock<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl>,
    >,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_file_location: std
    ::option
    ::Option<std::vec::Vec<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_file_location: std
    ::option
    ::Option<std::vec::Vec<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl>>,
    dynamic: TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElDynamic,
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
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
                            TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationEl,
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
                            TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationEl,
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

impl terrars::ToListMappable for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
    type O = terrars::BlockAssignable<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {}

impl BuildTransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
    pub fn build(self) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl {
            efs_file_location: core::default::Default::default(),
            s3_file_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
        TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `efs_file_location` after provisioning.\n"]
    pub fn efs_file_location(
        &self,
    ) -> terrars::ListRef<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElEfsFileLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_file_location", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_file_location` after provisioning.\n"]
    pub fn s3_file_location(
        &self,
    ) -> terrars::ListRef<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElS3FileLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_file_location", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowStepsElCopyStepDetailsElDynamic {
    destination_file_location: std
    ::option
    ::Option<terrars::DynamicBlock<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElCopyStepDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_existing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_file_location: std
    ::option
    ::Option<std::vec::Vec<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl>>,
    dynamic: TransferWorkflowStepsElCopyStepDetailsElDynamic,
}

impl TransferWorkflowStepsElCopyStepDetailsEl {
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
                    ::Into<
                        terrars::BlockAssignable<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationEl>,
                    >,
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

impl terrars::ToListMappable for TransferWorkflowStepsElCopyStepDetailsEl {
    type O = terrars::BlockAssignable<TransferWorkflowStepsElCopyStepDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCopyStepDetailsEl {}

impl BuildTransferWorkflowStepsElCopyStepDetailsEl {
    pub fn build(self) -> TransferWorkflowStepsElCopyStepDetailsEl {
        TransferWorkflowStepsElCopyStepDetailsEl {
            name: core::default::Default::default(),
            overwrite_existing: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            destination_file_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCopyStepDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowStepsElCopyStepDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflowStepsElCopyStepDetailsElRef {
        TransferWorkflowStepsElCopyStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCopyStepDetailsElRef {
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
    ) -> terrars::ListRef<TransferWorkflowStepsElCopyStepDetailsElDestinationFileLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_file_location", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElCustomStepDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl TransferWorkflowStepsElCustomStepDetailsEl {
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

impl terrars::ToListMappable for TransferWorkflowStepsElCustomStepDetailsEl {
    type O = terrars::BlockAssignable<TransferWorkflowStepsElCustomStepDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElCustomStepDetailsEl {}

impl BuildTransferWorkflowStepsElCustomStepDetailsEl {
    pub fn build(self) -> TransferWorkflowStepsElCustomStepDetailsEl {
        TransferWorkflowStepsElCustomStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            target: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElCustomStepDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowStepsElCustomStepDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflowStepsElCustomStepDetailsElRef {
        TransferWorkflowStepsElCustomStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElCustomStepDetailsElRef {
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

#[derive(Serialize)]
pub struct TransferWorkflowStepsElDeleteStepDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferWorkflowStepsElDeleteStepDetailsEl {
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

impl terrars::ToListMappable for TransferWorkflowStepsElDeleteStepDetailsEl {
    type O = terrars::BlockAssignable<TransferWorkflowStepsElDeleteStepDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElDeleteStepDetailsEl {}

impl BuildTransferWorkflowStepsElDeleteStepDetailsEl {
    pub fn build(self) -> TransferWorkflowStepsElDeleteStepDetailsEl {
        TransferWorkflowStepsElDeleteStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElDeleteStepDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowStepsElDeleteStepDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflowStepsElDeleteStepDetailsElRef {
        TransferWorkflowStepsElDeleteStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElDeleteStepDetailsElRef {
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
pub struct TransferWorkflowStepsElTagStepDetailsElTagsEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl TransferWorkflowStepsElTagStepDetailsElTagsEl { }

impl terrars::ToListMappable for TransferWorkflowStepsElTagStepDetailsElTagsEl {
    type O = terrars::BlockAssignable<TransferWorkflowStepsElTagStepDetailsElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElTagStepDetailsElTagsEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildTransferWorkflowStepsElTagStepDetailsElTagsEl {
    pub fn build(self) -> TransferWorkflowStepsElTagStepDetailsElTagsEl {
        TransferWorkflowStepsElTagStepDetailsElTagsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct TransferWorkflowStepsElTagStepDetailsElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowStepsElTagStepDetailsElTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TransferWorkflowStepsElTagStepDetailsElTagsElRef {
        TransferWorkflowStepsElTagStepDetailsElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElTagStepDetailsElTagsElRef {
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
struct TransferWorkflowStepsElTagStepDetailsElDynamic {
    tags: std::option::Option<terrars::DynamicBlock<TransferWorkflowStepsElTagStepDetailsElTagsEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsElTagStepDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_file_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<TransferWorkflowStepsElTagStepDetailsElTagsEl>>,
    dynamic: TransferWorkflowStepsElTagStepDetailsElDynamic,
}

impl TransferWorkflowStepsElTagStepDetailsEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowStepsElTagStepDetailsElTagsEl>>,
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

impl terrars::ToListMappable for TransferWorkflowStepsElTagStepDetailsEl {
    type O = terrars::BlockAssignable<TransferWorkflowStepsElTagStepDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsElTagStepDetailsEl {}

impl BuildTransferWorkflowStepsElTagStepDetailsEl {
    pub fn build(self) -> TransferWorkflowStepsElTagStepDetailsEl {
        TransferWorkflowStepsElTagStepDetailsEl {
            name: core::default::Default::default(),
            source_file_location: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElTagStepDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowStepsElTagStepDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflowStepsElTagStepDetailsElRef {
        TransferWorkflowStepsElTagStepDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElTagStepDetailsElRef {
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
    pub fn tags(&self) -> terrars::ListRef<TransferWorkflowStepsElTagStepDetailsElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowStepsElDynamic {
    copy_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflowStepsElCopyStepDetailsEl>>,
    custom_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflowStepsElCustomStepDetailsEl>>,
    delete_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflowStepsElDeleteStepDetailsEl>>,
    tag_step_details: std::option::Option<terrars::DynamicBlock<TransferWorkflowStepsElTagStepDetailsEl>>,
}

#[derive(Serialize)]
pub struct TransferWorkflowStepsEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_step_details: std::option::Option<std::vec::Vec<TransferWorkflowStepsElCopyStepDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_step_details: std::option::Option<std::vec::Vec<TransferWorkflowStepsElCustomStepDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_step_details: std::option::Option<std::vec::Vec<TransferWorkflowStepsElDeleteStepDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_step_details: std::option::Option<std::vec::Vec<TransferWorkflowStepsElTagStepDetailsEl>>,
    dynamic: TransferWorkflowStepsElDynamic,
}

impl TransferWorkflowStepsEl {
    #[doc = "Set the field `copy_step_details`.\n"]
    pub fn set_copy_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowStepsElCopyStepDetailsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowStepsElCustomStepDetailsEl>>,
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

    #[doc = "Set the field `delete_step_details`.\n"]
    pub fn set_delete_step_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowStepsElDeleteStepDetailsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<TransferWorkflowStepsElTagStepDetailsEl>>,
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

impl terrars::ToListMappable for TransferWorkflowStepsEl {
    type O = terrars::BlockAssignable<TransferWorkflowStepsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferWorkflowStepsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildTransferWorkflowStepsEl {
    pub fn build(self) -> TransferWorkflowStepsEl {
        TransferWorkflowStepsEl {
            type_: self.type_,
            copy_step_details: core::default::Default::default(),
            custom_step_details: core::default::Default::default(),
            delete_step_details: core::default::Default::default(),
            tag_step_details: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TransferWorkflowStepsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferWorkflowStepsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferWorkflowStepsElRef {
        TransferWorkflowStepsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferWorkflowStepsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_step_details` after provisioning.\n"]
    pub fn copy_step_details(&self) -> terrars::ListRef<TransferWorkflowStepsElCopyStepDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.copy_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_step_details` after provisioning.\n"]
    pub fn custom_step_details(&self) -> terrars::ListRef<TransferWorkflowStepsElCustomStepDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_step_details` after provisioning.\n"]
    pub fn delete_step_details(&self) -> terrars::ListRef<TransferWorkflowStepsElDeleteStepDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delete_step_details", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_step_details` after provisioning.\n"]
    pub fn tag_step_details(&self) -> terrars::ListRef<TransferWorkflowStepsElTagStepDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_step_details", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferWorkflowDynamic {
    on_exception_steps: std::option::Option<terrars::DynamicBlock<TransferWorkflowOnExceptionStepsEl>>,
    steps: std::option::Option<terrars::DynamicBlock<TransferWorkflowStepsEl>>,
}

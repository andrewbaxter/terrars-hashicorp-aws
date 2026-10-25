#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataexchangeEventActionData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<DataexchangeEventAction_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<std::vec::Vec<DataexchangeEventAction_Event>>,
    dynamic: DataexchangeEventActionDynamic,
}

struct DataexchangeEventAction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataexchangeEventActionData_>,
}

#[derive(Clone)]
pub struct DataexchangeEventAction(std::rc::Rc<DataexchangeEventAction_>);

impl DataexchangeEventAction {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataexchangeEventAction_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event`.\n"]
    pub fn set_event(self, v: impl std::convert::Into<terrars::BlockAssignable<DataexchangeEventAction_Event>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataexchangeEventAction_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> terrars::ListRef<DataexchangeEventAction_EventRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event", self.extract_ref()))
    }
}

impl terrars::Referable for DataexchangeEventAction {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataexchangeEventAction { }

impl terrars::ToListMappable for DataexchangeEventAction {
    type O = terrars::ListRef<DataexchangeEventActionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataexchangeEventAction_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dataexchange_event_action".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataexchangeEventAction {
    pub tf_id: std::string::String,
}

impl BuildDataexchangeEventAction {
    pub fn build(self, stack: &mut terrars::Stack) -> DataexchangeEventAction {
        let out = DataexchangeEventAction(std::rc::Rc::new(DataexchangeEventAction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataexchangeEventActionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                action: core::default::Default::default(),
                event: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataexchangeEventActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeEventActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataexchangeEventActionRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataexchangeEventAction_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> terrars::ListRef<DataexchangeEventAction_EventRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeEventActionDynamic {
    action: std::option::Option<terrars::DynamicBlock<DataexchangeEventAction_Action>>,
    event: std::option::Option<terrars::DynamicBlock<DataexchangeEventAction_Event>>,
}

#[derive(Serialize)]
pub struct DataexchangeEventAction_Action_ExportRevisionToS3_Encryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataexchangeEventAction_Action_ExportRevisionToS3_Encryption {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataexchangeEventAction_Action_ExportRevisionToS3_Encryption {
    type O = terrars::BlockAssignable<DataexchangeEventAction_Action_ExportRevisionToS3_Encryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventAction_Action_ExportRevisionToS3_Encryption {}

impl BuildDataexchangeEventAction_Action_ExportRevisionToS3_Encryption {
    pub fn build(self) -> DataexchangeEventAction_Action_ExportRevisionToS3_Encryption {
        DataexchangeEventAction_Action_ExportRevisionToS3_Encryption {
            kms_key_arn: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeEventAction_Action_ExportRevisionToS3_EncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeEventAction_Action_ExportRevisionToS3_EncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeEventAction_Action_ExportRevisionToS3_EncryptionRef {
        DataexchangeEventAction_Action_ExportRevisionToS3_EncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventAction_Action_ExportRevisionToS3_EncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_pattern: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination {
    #[doc = "Set the field `key_pattern`.\n"]
    pub fn set_key_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_pattern = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination {
    type O = terrars::BlockAssignable<DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildDataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination {
    pub fn build(self) -> DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination {
        DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination {
            bucket: self.bucket,
            key_pattern: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestinationRef {
        DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key_pattern` after provisioning.\n"]
    pub fn key_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeEventAction_Action_ExportRevisionToS3Dynamic {
    encryption: std
    ::option
    ::Option<terrars::DynamicBlock<DataexchangeEventAction_Action_ExportRevisionToS3_Encryption>>,
    revision_destination: std
    ::option
    ::Option<terrars::DynamicBlock<DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination>>,
}

#[derive(Serialize)]
pub struct DataexchangeEventAction_Action_ExportRevisionToS3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption: std::option::Option<std::vec::Vec<DataexchangeEventAction_Action_ExportRevisionToS3_Encryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision_destination: std
    ::option
    ::Option<std::vec::Vec<DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination>>,
    dynamic: DataexchangeEventAction_Action_ExportRevisionToS3Dynamic,
}

impl DataexchangeEventAction_Action_ExportRevisionToS3 {
    #[doc = "Set the field `encryption`.\n"]
    pub fn set_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataexchangeEventAction_Action_ExportRevisionToS3_Encryption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `revision_destination`.\n"]
    pub fn set_revision_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestination>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.revision_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.revision_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataexchangeEventAction_Action_ExportRevisionToS3 {
    type O = terrars::BlockAssignable<DataexchangeEventAction_Action_ExportRevisionToS3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventAction_Action_ExportRevisionToS3 {}

impl BuildDataexchangeEventAction_Action_ExportRevisionToS3 {
    pub fn build(self) -> DataexchangeEventAction_Action_ExportRevisionToS3 {
        DataexchangeEventAction_Action_ExportRevisionToS3 {
            encryption: core::default::Default::default(),
            revision_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeEventAction_Action_ExportRevisionToS3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeEventAction_Action_ExportRevisionToS3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeEventAction_Action_ExportRevisionToS3Ref {
        DataexchangeEventAction_Action_ExportRevisionToS3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventAction_Action_ExportRevisionToS3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> terrars::ListRef<DataexchangeEventAction_Action_ExportRevisionToS3_EncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `revision_destination` after provisioning.\n"]
    pub fn revision_destination(
        &self,
    ) -> terrars::ListRef<DataexchangeEventAction_Action_ExportRevisionToS3_RevisionDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revision_destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeEventAction_ActionDynamic {
    export_revision_to_s3: std
    ::option
    ::Option<terrars::DynamicBlock<DataexchangeEventAction_Action_ExportRevisionToS3>>,
}

#[derive(Serialize)]
pub struct DataexchangeEventAction_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_revision_to_s3: std::option::Option<std::vec::Vec<DataexchangeEventAction_Action_ExportRevisionToS3>>,
    dynamic: DataexchangeEventAction_ActionDynamic,
}

impl DataexchangeEventAction_Action {
    #[doc = "Set the field `export_revision_to_s3`.\n"]
    pub fn set_export_revision_to_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataexchangeEventAction_Action_ExportRevisionToS3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.export_revision_to_s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.export_revision_to_s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataexchangeEventAction_Action {
    type O = terrars::BlockAssignable<DataexchangeEventAction_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventAction_Action {}

impl BuildDataexchangeEventAction_Action {
    pub fn build(self) -> DataexchangeEventAction_Action {
        DataexchangeEventAction_Action {
            export_revision_to_s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeEventAction_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeEventAction_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataexchangeEventAction_ActionRef {
        DataexchangeEventAction_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventAction_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `export_revision_to_s3` after provisioning.\n"]
    pub fn export_revision_to_s3(&self) -> terrars::ListRef<DataexchangeEventAction_Action_ExportRevisionToS3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export_revision_to_s3", self.base))
    }
}

#[derive(Serialize)]
pub struct DataexchangeEventAction_Event_RevisionPublished {
    data_set_id: terrars::PrimField<std::string::String>,
}

impl DataexchangeEventAction_Event_RevisionPublished { }

impl terrars::ToListMappable for DataexchangeEventAction_Event_RevisionPublished {
    type O = terrars::BlockAssignable<DataexchangeEventAction_Event_RevisionPublished>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventAction_Event_RevisionPublished {
    #[doc = ""]
    pub data_set_id: terrars::PrimField<std::string::String>,
}

impl BuildDataexchangeEventAction_Event_RevisionPublished {
    pub fn build(self) -> DataexchangeEventAction_Event_RevisionPublished {
        DataexchangeEventAction_Event_RevisionPublished { data_set_id: self.data_set_id }
    }
}

#[allow(dead_code)]
pub struct DataexchangeEventAction_Event_RevisionPublishedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeEventAction_Event_RevisionPublishedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeEventAction_Event_RevisionPublishedRef {
        DataexchangeEventAction_Event_RevisionPublishedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventAction_Event_RevisionPublishedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeEventAction_EventDynamic {
    revision_published: std::option::Option<terrars::DynamicBlock<DataexchangeEventAction_Event_RevisionPublished>>,
}

#[derive(Serialize)]
pub struct DataexchangeEventAction_Event {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision_published: std::option::Option<std::vec::Vec<DataexchangeEventAction_Event_RevisionPublished>>,
    dynamic: DataexchangeEventAction_EventDynamic,
}

impl DataexchangeEventAction_Event {
    #[doc = "Set the field `revision_published`.\n"]
    pub fn set_revision_published(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataexchangeEventAction_Event_RevisionPublished>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.revision_published = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.revision_published = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataexchangeEventAction_Event {
    type O = terrars::BlockAssignable<DataexchangeEventAction_Event>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventAction_Event {}

impl BuildDataexchangeEventAction_Event {
    pub fn build(self) -> DataexchangeEventAction_Event {
        DataexchangeEventAction_Event {
            revision_published: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeEventAction_EventRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeEventAction_EventRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataexchangeEventAction_EventRef {
        DataexchangeEventAction_EventRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventAction_EventRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `revision_published` after provisioning.\n"]
    pub fn revision_published(&self) -> terrars::ListRef<DataexchangeEventAction_Event_RevisionPublishedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revision_published", self.base))
    }
}

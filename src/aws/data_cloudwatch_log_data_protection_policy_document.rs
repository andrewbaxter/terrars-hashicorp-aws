#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudwatchLogDataProtectionPolicyDocumentData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std::option::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Statement>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentDynamic,
}

struct DataCloudwatchLogDataProtectionPolicyDocument_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudwatchLogDataProtectionPolicyDocumentData_>,
}

#[derive(Clone)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument(
    std::rc::Rc<DataCloudwatchLogDataProtectionPolicyDocument_>,
);

impl DataCloudwatchLogDataProtectionPolicyDocument {
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

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Statement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudwatchLogDataProtectionPolicyDocument {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudwatchLogDataProtectionPolicyDocument { }

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument {
    type O = terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocumentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudwatchLogDataProtectionPolicyDocument_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudwatch_log_data_protection_policy_document".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudwatchLogDataProtectionPolicyDocument {
        let out =
            DataCloudwatchLogDataProtectionPolicyDocument(
                std::rc::Rc::new(DataCloudwatchLogDataProtectionPolicyDocument_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataCloudwatchLogDataProtectionPolicyDocumentData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        description: core::default::Default::default(),
                        id: core::default::Default::default(),
                        name: self.name,
                        version: core::default::Default::default(),
                        configuration: core::default::Default::default(),
                        statement: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocumentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentDynamic {
    configuration: std
    ::option
    ::Option<terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocument_Configuration>>,
    statement: std::option::Option<terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocument_Statement>>,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier {
    name: terrars::PrimField<std::string::String>,
    regex: terrars::PrimField<std::string::String>,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier { }

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier {
    type O =
        terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub regex: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier {
        DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier {
            name: self.name,
            regex: self.regex,
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifierRef {
        DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocument_ConfigurationDynamic {
    custom_data_identifier: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier>,
    >,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_data_identifier: std
    ::option
    ::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocument_ConfigurationDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Configuration {
    #[doc = "Set the field `custom_data_identifier`.\n"]
    pub fn set_custom_data_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_data_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_data_identifier = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Configuration {
    type O = terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Configuration {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Configuration {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocument_Configuration {
        DataCloudwatchLogDataProtectionPolicyDocument_Configuration {
            custom_data_identifier: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocument_ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_ConfigurationRef {
        DataCloudwatchLogDataProtectionPolicyDocument_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_data_identifier` after provisioning.\n"]
    pub fn custom_data_identifier(
        &self,
    ) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_Configuration_CustomDataIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_data_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs {
    log_group: terrars::PrimField<std::string::String>,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs { }

impl terrars
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs {
    #[doc = ""]
    pub log_group: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs {
            log_group: self.log_group,
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogsRef {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose {
    delivery_stream: terrars::PrimField<std::string::String>,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose { }

impl terrars
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose {
    #[doc = ""]
    pub delivery_stream: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose {
            delivery_stream: self.delivery_stream,
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_FirehoseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_FirehoseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_FirehoseRef {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_FirehoseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_FirehoseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"]
    pub fn delivery_stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3 {
    bucket: terrars::PrimField<std::string::String>,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3 { }

impl terrars
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3 {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3 {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3 {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3 {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3 {
            bucket: self.bucket,
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3Ref {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestinationDynamic {
    cloudwatch_logs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs,
        >,
    >,
    firehose: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose,
        >,
    >,
    s3: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3>,
    >,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std
    ::option
    ::Option<
        std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3>,
    >,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestinationDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_Firehose,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.firehose = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestinationRef {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> terrars
    ::ListRef<
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_CloudwatchLogsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(
        &self,
    ) -> terrars
    ::ListRef<
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_FirehoseRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> terrars
    ::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_AuditDynamic {
    findings_destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination>,
    >,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    findings_destination: std
    ::option
    ::Option<
        std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination>,
    >,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_AuditDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit {
    #[doc = "Set the field `findings_destination`.\n"]
    pub fn set_findings_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.findings_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.findings_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit {
    type O = terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit {
            findings_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_AuditRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_AuditRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_AuditRef {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_AuditRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_AuditRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `findings_destination` after provisioning.\n"]
    pub fn findings_destination(
        &self,
    ) -> terrars
    ::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit_FindingsDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.findings_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig {}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig { }

impl terrars
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig {
    type O =
        terrars
        ::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig {}
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfigRef {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_DeidentifyDynamic {
    mask_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig>,
    >,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mask_config: std
    ::option
    ::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_DeidentifyDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify {
    #[doc = "Set the field `mask_config`.\n"]
    pub fn set_mask_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mask_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mask_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify {
    type O = terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify {
            mask_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_DeidentifyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_DeidentifyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_DeidentifyRef {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_DeidentifyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_DeidentifyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mask_config` after provisioning.\n"]
    pub fn mask_config(
        &self,
    ) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify_MaskConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mask_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_OperationDynamic {
    audit: std
    ::option
    ::Option<terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit>>,
    deidentify: std
    ::option
    ::Option<terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify>>,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit: std::option::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify: std
    ::option
    ::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocument_Statement_OperationDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation {
    #[doc = "Set the field `audit`.\n"]
    pub fn set_audit(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Audit>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audit = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deidentify`.\n"]
    pub fn set_deidentify(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_Deidentify,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deidentify = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deidentify = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation {
    type O = terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation {
            audit: core::default::Default::default(),
            deidentify: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement_OperationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocument_Statement_OperationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement_OperationRef {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement_OperationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement_OperationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit` after provisioning.\n"]
    pub fn audit(&self) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_AuditRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audit", self.base))
    }

    #[doc = "Get a reference to the value of field `deidentify` after provisioning.\n"]
    pub fn deidentify(
        &self,
    ) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation_DeidentifyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deidentify", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocument_StatementDynamic {
    operation: std
    ::option
    ::Option<terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation>>,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_Statement {
    data_identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocument_StatementDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocument_Statement {
    #[doc = "Set the field `sid`.\n"]
    pub fn set_sid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Statement_Operation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocument_Statement {
    type O = terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocument_Statement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement {
    #[doc = ""]
    pub data_identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocument_Statement {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocument_Statement {
        DataCloudwatchLogDataProtectionPolicyDocument_Statement {
            data_identifiers: self.data_identifiers,
            sid: core::default::Default::default(),
            operation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchLogDataProtectionPolicyDocument_StatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocument_StatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocument_StatementRef {
        DataCloudwatchLogDataProtectionPolicyDocument_StatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocument_StatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_identifiers` after provisioning.\n"]
    pub fn data_identifiers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.data_identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `sid` after provisioning.\n"]
    pub fn sid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sid", self.base))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocument_Statement_OperationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

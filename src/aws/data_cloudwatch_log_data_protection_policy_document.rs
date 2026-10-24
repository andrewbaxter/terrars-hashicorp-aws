use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudwatchLogDataProtectionPolicyDocumentData {
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
    statement: std::option::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementEl>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentDynamic,
}

struct DataCloudwatchLogDataProtectionPolicyDocument_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudwatchLogDataProtectionPolicyDocumentData>,
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

    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementEl>>,
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

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef> {
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
                    data: std::cell::RefCell::new(DataCloudwatchLogDataProtectionPolicyDocumentData {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        description: core::default::Default::default(),
                        id: core::default::Default::default(),
                        name: self.name,
                        version: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl {
    log_group: terrars::PrimField<std::string::String>,
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl {

}

impl terrars
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl {
    #[doc = ""]
    pub log_group: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl {
            log_group: self.log_group,
        }
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl {
    delivery_stream: terrars::PrimField<std::string::String>,
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl { }

impl terrars
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl {
    #[doc = ""]
    pub delivery_stream: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl {
            delivery_stream: self.delivery_stream,
        }
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"]
    pub fn delivery_stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_stream", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El {
    bucket: terrars::PrimField<std::string::String>,
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El { }

impl terrars
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El {
            bucket: self.bucket,
        }
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElDynamic {
    cloudwatch_logs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl,
        >,
    >,
    firehose: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl,
        >,
    >,
    s3: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firehose: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El>,
    >,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl {
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
                            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsEl,
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
                            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseEl,
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
                            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3El,
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
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> terrars
    ::ListRef<
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElCloudwatchLogsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }

    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(
        &self,
    ) -> terrars
    ::ListRef<
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElFirehoseElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(
        &self,
    ) -> terrars
    ::ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElDynamic {
    findings_destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    findings_destination: std
    ::option
    ::Option<
        std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl>,
    >,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
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
                            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationEl,
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

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
    type O = terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl {
            findings_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `findings_destination` after provisioning.\n"]
    pub fn findings_destination(
        &self,
    ) -> terrars
    ::ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElFindingsDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.findings_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl {}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl { }

impl terrars
::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl {
    type O =
        terrars
        ::BlockAssignable<
            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl {
    pub fn build(
        self,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl {}
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElDynamic {
    mask_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mask_config: std
    ::option
    ::Option<
        std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl>,
    >,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
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
                            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigEl,
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

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
    type O =
        terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl {
            mask_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mask_config` after provisioning.\n"]
    pub fn mask_config(
        &self,
    ) -> terrars
    ::ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElMaskConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mask_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDynamic {
    audit: std
    ::option
    ::Option<terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl>>,
    deidentify: std
    ::option
    ::Option<terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl>>,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit: std::option::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify: std
    ::option
    ::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
    #[doc = "Set the field `audit`.\n"]
    pub fn set_audit(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditEl>,
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
                            DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyEl,
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

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
    type O = terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {}

impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl {
            audit: core::default::Default::default(),
            deidentify: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit` after provisioning.\n"]
    pub fn audit(
        &self,
    ) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElAuditElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audit", self.base))
    }

    #[doc = "Get a reference to the value of field `deidentify` after provisioning.\n"]
    pub fn deidentify(
        &self,
    ) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElDeidentifyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deidentify", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElDynamic {
    operation: std
    ::option
    ::Option<terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl>>,
}

#[derive(Serialize)]
pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
    data_identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<std::vec::Vec<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl>>,
    dynamic: DataCloudwatchLogDataProtectionPolicyDocumentStatementElDynamic,
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
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
                        terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationEl>,
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

impl terrars::ToListMappable for DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
    type O = terrars::BlockAssignable<DataCloudwatchLogDataProtectionPolicyDocumentStatementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
    #[doc = ""]
    pub data_identifiers: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
    pub fn build(self) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementEl {
            data_identifiers: self.data_identifiers,
            sid: core::default::Default::default(),
            operation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
        DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchLogDataProtectionPolicyDocumentStatementElRef {
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
    pub fn operation(&self) -> terrars::ListRef<DataCloudwatchLogDataProtectionPolicyDocumentStatementElOperationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCloudwatchLogDataProtectionPolicyDocumentDynamic {
    statement: std::option::Option<terrars::DynamicBlock<DataCloudwatchLogDataProtectionPolicyDocumentStatementEl>>,
}

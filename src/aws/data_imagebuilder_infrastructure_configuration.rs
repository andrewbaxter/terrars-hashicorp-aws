use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderInfrastructureConfigurationData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataImagebuilderInfrastructureConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImagebuilderInfrastructureConfigurationData>,
}

#[derive(Clone)]
pub struct DataImagebuilderInfrastructureConfiguration(std::rc::Rc<DataImagebuilderInfrastructureConfiguration_>);

impl DataImagebuilderInfrastructureConfiguration {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_tags = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_metadata_options` after provisioning.\n"]
    pub fn instance_metadata_options(
        &self,
    ) -> terrars::ListRef<DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_profile_name` after provisioning.\n"]
    pub fn instance_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_pair` after provisioning.\n"]
    pub fn key_pair(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_pair", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<DataImagebuilderInfrastructureConfigurationLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminate_instance_on_failure` after provisioning.\n"]
    pub fn terminate_instance_on_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instance_on_failure", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataImagebuilderInfrastructureConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataImagebuilderInfrastructureConfiguration { }

impl terrars::ToListMappable for DataImagebuilderInfrastructureConfiguration {
    type O = terrars::ListRef<DataImagebuilderInfrastructureConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataImagebuilderInfrastructureConfiguration_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_imagebuilder_infrastructure_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderInfrastructureConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataImagebuilderInfrastructureConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> DataImagebuilderInfrastructureConfiguration {
        let out =
            DataImagebuilderInfrastructureConfiguration(
                std::rc::Rc::new(DataImagebuilderInfrastructureConfiguration_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataImagebuilderInfrastructureConfigurationData {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        arn: self.arn,
                        id: core::default::Default::default(),
                        resource_tags: core::default::Default::default(),
                        tags: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderInfrastructureConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderInfrastructureConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImagebuilderInfrastructureConfigurationRef {
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

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_metadata_options` after provisioning.\n"]
    pub fn instance_metadata_options(
        &self,
    ) -> terrars::ListRef<DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_profile_name` after provisioning.\n"]
    pub fn instance_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_pair` after provisioning.\n"]
    pub fn key_pair(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_pair", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<DataImagebuilderInfrastructureConfigurationLoggingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sns_topic_arn` after provisioning.\n"]
    pub fn sns_topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sns_topic_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `terminate_instance_on_failure` after provisioning.\n"]
    pub fn terminate_instance_on_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.terminate_instance_on_failure", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_put_response_hop_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_tokens: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    #[doc = "Set the field `http_put_response_hop_limit`.\n"]
    pub fn set_http_put_response_hop_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.http_put_response_hop_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_tokens`.\n"]
    pub fn set_http_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_tokens = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    type O = terrars::BlockAssignable<DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {}

impl BuildDataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
    pub fn build(self) -> DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
        DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsEl {
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
        DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderInfrastructureConfigurationInstanceMetadataOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_put_response_hop_limit` after provisioning.\n"]
    pub fn http_put_response_hop_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_put_response_hop_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `http_tokens` after provisioning.\n"]
    pub fn http_tokens(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_tokens", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    type O = terrars::BlockAssignable<DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {}

impl BuildDataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
    pub fn build(self) -> DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
        DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl {
            s3_bucket_name: core::default::Default::default(),
            s3_key_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
        DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderInfrastructureConfigurationLoggingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_logs: std::option::Option<terrars::ListField<DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>>,
}

impl DataImagebuilderInfrastructureConfigurationLoggingEl {
    #[doc = "Set the field `s3_logs`.\n"]
    pub fn set_s3_logs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataImagebuilderInfrastructureConfigurationLoggingElS3LogsEl>>,
    ) -> Self {
        self.s3_logs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderInfrastructureConfigurationLoggingEl {
    type O = terrars::BlockAssignable<DataImagebuilderInfrastructureConfigurationLoggingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderInfrastructureConfigurationLoggingEl {}

impl BuildDataImagebuilderInfrastructureConfigurationLoggingEl {
    pub fn build(self) -> DataImagebuilderInfrastructureConfigurationLoggingEl {
        DataImagebuilderInfrastructureConfigurationLoggingEl { s3_logs: core::default::Default::default() }
    }
}

pub struct DataImagebuilderInfrastructureConfigurationLoggingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderInfrastructureConfigurationLoggingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderInfrastructureConfigurationLoggingElRef {
        DataImagebuilderInfrastructureConfigurationLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderInfrastructureConfigurationLoggingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(&self) -> terrars::ListRef<DataImagebuilderInfrastructureConfigurationLoggingElS3LogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}

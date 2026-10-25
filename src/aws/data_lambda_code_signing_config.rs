#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLambdaCodeSigningConfigData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLambdaCodeSigningConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLambdaCodeSigningConfigData_>,
}

#[derive(Clone)]
pub struct DataLambdaCodeSigningConfig(std::rc::Rc<DataLambdaCodeSigningConfig_>);

impl DataLambdaCodeSigningConfig {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allowed_publishers` after provisioning.\n"]
    pub fn allowed_publishers(&self) -> terrars::ListRef<DataLambdaCodeSigningConfig_AllowedPublishersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_publishers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> terrars::ListRef<DataLambdaCodeSigningConfig_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataLambdaCodeSigningConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLambdaCodeSigningConfig { }

impl terrars::ToListMappable for DataLambdaCodeSigningConfig {
    type O = terrars::ListRef<DataLambdaCodeSigningConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLambdaCodeSigningConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_lambda_code_signing_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLambdaCodeSigningConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataLambdaCodeSigningConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLambdaCodeSigningConfig {
        let out = DataLambdaCodeSigningConfig(std::rc::Rc::new(DataLambdaCodeSigningConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLambdaCodeSigningConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLambdaCodeSigningConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaCodeSigningConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLambdaCodeSigningConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `allowed_publishers` after provisioning.\n"]
    pub fn allowed_publishers(&self) -> terrars::ListRef<DataLambdaCodeSigningConfig_AllowedPublishersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_publishers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> terrars::ListRef<DataLambdaCodeSigningConfig_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLambdaCodeSigningConfig_AllowedPublishers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signing_profile_version_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataLambdaCodeSigningConfig_AllowedPublishers {
    #[doc = "Set the field `signing_profile_version_arns`.\n"]
    pub fn set_signing_profile_version_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.signing_profile_version_arns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaCodeSigningConfig_AllowedPublishers {
    type O = terrars::BlockAssignable<DataLambdaCodeSigningConfig_AllowedPublishers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaCodeSigningConfig_AllowedPublishers {}

impl BuildDataLambdaCodeSigningConfig_AllowedPublishers {
    pub fn build(self) -> DataLambdaCodeSigningConfig_AllowedPublishers {
        DataLambdaCodeSigningConfig_AllowedPublishers {
            signing_profile_version_arns: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLambdaCodeSigningConfig_AllowedPublishersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaCodeSigningConfig_AllowedPublishersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLambdaCodeSigningConfig_AllowedPublishersRef {
        DataLambdaCodeSigningConfig_AllowedPublishersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaCodeSigningConfig_AllowedPublishersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `signing_profile_version_arns` after provisioning.\n"]
    pub fn signing_profile_version_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.signing_profile_version_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLambdaCodeSigningConfig_Policies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    untrusted_artifact_on_deployment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLambdaCodeSigningConfig_Policies {
    #[doc = "Set the field `untrusted_artifact_on_deployment`.\n"]
    pub fn set_untrusted_artifact_on_deployment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.untrusted_artifact_on_deployment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLambdaCodeSigningConfig_Policies {
    type O = terrars::BlockAssignable<DataLambdaCodeSigningConfig_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLambdaCodeSigningConfig_Policies {}

impl BuildDataLambdaCodeSigningConfig_Policies {
    pub fn build(self) -> DataLambdaCodeSigningConfig_Policies {
        DataLambdaCodeSigningConfig_Policies { untrusted_artifact_on_deployment: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLambdaCodeSigningConfig_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLambdaCodeSigningConfig_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLambdaCodeSigningConfig_PoliciesRef {
        DataLambdaCodeSigningConfig_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLambdaCodeSigningConfig_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `untrusted_artifact_on_deployment` after provisioning.\n"]
    pub fn untrusted_artifact_on_deployment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.untrusted_artifact_on_deployment", self.base))
    }
}

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaCodeSigningConfigData_ {
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
    allowed_publishers: std::option::Option<std::vec::Vec<LambdaCodeSigningConfig_AllowedPublishers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policies: std::option::Option<std::vec::Vec<LambdaCodeSigningConfig_Policies>>,
    dynamic: LambdaCodeSigningConfigDynamic,
}

struct LambdaCodeSigningConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaCodeSigningConfigData_>,
}

#[derive(Clone)]
pub struct LambdaCodeSigningConfig(std::rc::Rc<LambdaCodeSigningConfig_>);

impl LambdaCodeSigningConfig {
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

    #[doc = "Set the field `allowed_publishers`.\n"]
    pub fn set_allowed_publishers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaCodeSigningConfig_AllowedPublishers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_publishers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_publishers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policies`.\n"]
    pub fn set_policies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaCodeSigningConfig_Policies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policies = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `allowed_publishers` after provisioning.\n"]
    pub fn allowed_publishers(&self) -> terrars::ListRef<LambdaCodeSigningConfig_AllowedPublishersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_publishers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> terrars::ListRef<LambdaCodeSigningConfig_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }
}

impl terrars::Referable for LambdaCodeSigningConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LambdaCodeSigningConfig { }

impl terrars::ToListMappable for LambdaCodeSigningConfig {
    type O = terrars::ListRef<LambdaCodeSigningConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LambdaCodeSigningConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lambda_code_signing_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaCodeSigningConfig {
    pub tf_id: std::string::String,
}

impl BuildLambdaCodeSigningConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> LambdaCodeSigningConfig {
        let out = LambdaCodeSigningConfig(std::rc::Rc::new(LambdaCodeSigningConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LambdaCodeSigningConfigData_ {
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
                allowed_publishers: core::default::Default::default(),
                policies: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaCodeSigningConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCodeSigningConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaCodeSigningConfigRef {
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

    #[doc = "Get a reference to the value of field `allowed_publishers` after provisioning.\n"]
    pub fn allowed_publishers(&self) -> terrars::ListRef<LambdaCodeSigningConfig_AllowedPublishersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_publishers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> terrars::ListRef<LambdaCodeSigningConfig_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LambdaCodeSigningConfigDynamic {
    allowed_publishers: std::option::Option<terrars::DynamicBlock<LambdaCodeSigningConfig_AllowedPublishers>>,
    policies: std::option::Option<terrars::DynamicBlock<LambdaCodeSigningConfig_Policies>>,
}

#[derive(Serialize)]
pub struct LambdaCodeSigningConfig_AllowedPublishers {
    signing_profile_version_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LambdaCodeSigningConfig_AllowedPublishers { }

impl terrars::ToListMappable for LambdaCodeSigningConfig_AllowedPublishers {
    type O = terrars::BlockAssignable<LambdaCodeSigningConfig_AllowedPublishers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCodeSigningConfig_AllowedPublishers {
    #[doc = ""]
    pub signing_profile_version_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLambdaCodeSigningConfig_AllowedPublishers {
    pub fn build(self) -> LambdaCodeSigningConfig_AllowedPublishers {
        LambdaCodeSigningConfig_AllowedPublishers { signing_profile_version_arns: self.signing_profile_version_arns }
    }
}

#[allow(dead_code)]
pub struct LambdaCodeSigningConfig_AllowedPublishersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCodeSigningConfig_AllowedPublishersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaCodeSigningConfig_AllowedPublishersRef {
        LambdaCodeSigningConfig_AllowedPublishersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCodeSigningConfig_AllowedPublishersRef {
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
pub struct LambdaCodeSigningConfig_Policies {
    untrusted_artifact_on_deployment: terrars::PrimField<std::string::String>,
}

impl LambdaCodeSigningConfig_Policies { }

impl terrars::ToListMappable for LambdaCodeSigningConfig_Policies {
    type O = terrars::BlockAssignable<LambdaCodeSigningConfig_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCodeSigningConfig_Policies {
    #[doc = ""]
    pub untrusted_artifact_on_deployment: terrars::PrimField<std::string::String>,
}

impl BuildLambdaCodeSigningConfig_Policies {
    pub fn build(self) -> LambdaCodeSigningConfig_Policies {
        LambdaCodeSigningConfig_Policies { untrusted_artifact_on_deployment: self.untrusted_artifact_on_deployment }
    }
}

#[allow(dead_code)]
pub struct LambdaCodeSigningConfig_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCodeSigningConfig_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaCodeSigningConfig_PoliciesRef {
        LambdaCodeSigningConfig_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCodeSigningConfig_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `untrusted_artifact_on_deployment` after provisioning.\n"]
    pub fn untrusted_artifact_on_deployment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.untrusted_artifact_on_deployment", self.base))
    }
}

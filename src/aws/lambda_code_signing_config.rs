use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaCodeSigningConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_publishers: Option<Vec<LambdaCodeSigningConfigAllowedPublishersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policies: Option<Vec<LambdaCodeSigningConfigPoliciesEl>>,
    dynamic: LambdaCodeSigningConfigDynamic,
}

struct LambdaCodeSigningConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LambdaCodeSigningConfigData>,
}

#[derive(Clone)]
pub struct LambdaCodeSigningConfig(Rc<LambdaCodeSigningConfig_>);

impl LambdaCodeSigningConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `allowed_publishers`.\n"]
    pub fn set_allowed_publishers(
        self,
        v: impl Into<BlockAssignable<LambdaCodeSigningConfigAllowedPublishersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_publishers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_publishers = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `policies`.\n"]
    pub fn set_policies(self, v: impl Into<BlockAssignable<LambdaCodeSigningConfigPoliciesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policies = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policies = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed_publishers` after provisioning.\n"]
    pub fn allowed_publishers(&self) -> ListRef<LambdaCodeSigningConfigAllowedPublishersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_publishers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> ListRef<LambdaCodeSigningConfigPoliciesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }
}

impl Resource for LambdaCodeSigningConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for LambdaCodeSigningConfig {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for LambdaCodeSigningConfig {
    type O = ListRef<LambdaCodeSigningConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for LambdaCodeSigningConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_lambda_code_signing_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaCodeSigningConfig {
    pub tf_id: String,
}

impl BuildLambdaCodeSigningConfig {
    pub fn build(self, stack: &mut Stack) -> LambdaCodeSigningConfig {
        let out = LambdaCodeSigningConfig(Rc::new(LambdaCodeSigningConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LambdaCodeSigningConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                allowed_publishers: core::default::Default::default(),
                policies: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaCodeSigningConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaCodeSigningConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaCodeSigningConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed_publishers` after provisioning.\n"]
    pub fn allowed_publishers(&self) -> ListRef<LambdaCodeSigningConfigAllowedPublishersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_publishers", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> ListRef<LambdaCodeSigningConfigPoliciesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policies", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LambdaCodeSigningConfigAllowedPublishersEl {
    signing_profile_version_arns: SetField<PrimField<String>>,
}

impl LambdaCodeSigningConfigAllowedPublishersEl { }

impl ToListMappable for LambdaCodeSigningConfigAllowedPublishersEl {
    type O = BlockAssignable<LambdaCodeSigningConfigAllowedPublishersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCodeSigningConfigAllowedPublishersEl {
    #[doc= ""]
    pub signing_profile_version_arns: SetField<PrimField<String>>,
}

impl BuildLambdaCodeSigningConfigAllowedPublishersEl {
    pub fn build(self) -> LambdaCodeSigningConfigAllowedPublishersEl {
        LambdaCodeSigningConfigAllowedPublishersEl {
            signing_profile_version_arns: self.signing_profile_version_arns,
        }
    }
}

pub struct LambdaCodeSigningConfigAllowedPublishersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaCodeSigningConfigAllowedPublishersElRef {
    fn new(shared: StackShared, base: String) -> LambdaCodeSigningConfigAllowedPublishersElRef {
        LambdaCodeSigningConfigAllowedPublishersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCodeSigningConfigAllowedPublishersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `signing_profile_version_arns` after provisioning.\n"]
    pub fn signing_profile_version_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.signing_profile_version_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaCodeSigningConfigPoliciesEl {
    untrusted_artifact_on_deployment: PrimField<String>,
}

impl LambdaCodeSigningConfigPoliciesEl { }

impl ToListMappable for LambdaCodeSigningConfigPoliciesEl {
    type O = BlockAssignable<LambdaCodeSigningConfigPoliciesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCodeSigningConfigPoliciesEl {
    #[doc= ""]
    pub untrusted_artifact_on_deployment: PrimField<String>,
}

impl BuildLambdaCodeSigningConfigPoliciesEl {
    pub fn build(self) -> LambdaCodeSigningConfigPoliciesEl {
        LambdaCodeSigningConfigPoliciesEl { untrusted_artifact_on_deployment: self.untrusted_artifact_on_deployment }
    }
}

pub struct LambdaCodeSigningConfigPoliciesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaCodeSigningConfigPoliciesElRef {
    fn new(shared: StackShared, base: String) -> LambdaCodeSigningConfigPoliciesElRef {
        LambdaCodeSigningConfigPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCodeSigningConfigPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `untrusted_artifact_on_deployment` after provisioning.\n"]
    pub fn untrusted_artifact_on_deployment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.untrusted_artifact_on_deployment", self.base))
    }
}

#[derive(Serialize, Default)]
struct LambdaCodeSigningConfigDynamic {
    allowed_publishers: Option<DynamicBlock<LambdaCodeSigningConfigAllowedPublishersEl>>,
    policies: Option<DynamicBlock<LambdaCodeSigningConfigPoliciesEl>>,
}

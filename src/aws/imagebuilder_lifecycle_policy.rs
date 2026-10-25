#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderLifecyclePolicyData_ {
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
    execution_role: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_detail: std::option::Option<std::vec::Vec<ImagebuilderLifecyclePolicy_PolicyDetail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_selection: std::option::Option<std::vec::Vec<ImagebuilderLifecyclePolicy_ResourceSelection>>,
    dynamic: ImagebuilderLifecyclePolicyDynamic,
}

struct ImagebuilderLifecyclePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ImagebuilderLifecyclePolicyData_>,
}

#[derive(Clone)]
pub struct ImagebuilderLifecyclePolicy(std::rc::Rc<ImagebuilderLifecyclePolicy_>);

impl ImagebuilderLifecyclePolicy {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `policy_detail`.\n"]
    pub fn set_policy_detail(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy_detail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy_detail = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_selection`.\n"]
    pub fn set_resource_selection(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderLifecyclePolicy_ResourceSelection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_selection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_selection = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_selection` after provisioning.\n"]
    pub fn resource_selection(&self) -> terrars::ListRef<ImagebuilderLifecyclePolicy_ResourceSelectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_selection", self.extract_ref()))
    }
}

impl terrars::Referable for ImagebuilderLifecyclePolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ImagebuilderLifecyclePolicy { }

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy {
    type O = terrars::ListRef<ImagebuilderLifecyclePolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ImagebuilderLifecyclePolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_imagebuilder_lifecycle_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderLifecyclePolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub execution_role: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderLifecyclePolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ImagebuilderLifecyclePolicy {
        let out = ImagebuilderLifecyclePolicy(std::rc::Rc::new(ImagebuilderLifecyclePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ImagebuilderLifecyclePolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                execution_role: self.execution_role,
                name: self.name,
                region: core::default::Default::default(),
                resource_type: self.resource_type,
                status: core::default::Default::default(),
                tags: core::default::Default::default(),
                policy_detail: core::default::Default::default(),
                resource_selection: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderLifecyclePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImagebuilderLifecyclePolicyRef {
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

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_selection` after provisioning.\n"]
    pub fn resource_selection(&self) -> terrars::ListRef<ImagebuilderLifecyclePolicy_ResourceSelectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_selection", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicyDynamic {
    policy_detail: std::option::Option<terrars::DynamicBlock<ImagebuilderLifecyclePolicy_PolicyDetail>>,
    resource_selection: std::option::Option<terrars::DynamicBlock<ImagebuilderLifecyclePolicy_ResourceSelection>>,
}

#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amis: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshots: std::option::Option<terrars::PrimField<bool>>,
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources {
    #[doc = "Set the field `amis`.\n"]
    pub fn set_amis(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.amis = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.containers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshots`.\n"]
    pub fn set_snapshots(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.snapshots = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources {
    type O = terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources {}

impl BuildImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources {
    pub fn build(self) -> ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources {
        ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources {
            amis: core::default::Default::default(),
            containers: core::default::Default::default(),
            snapshots: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResourcesRef {
        ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amis` after provisioning.\n"]
    pub fn amis(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amis", self.base))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.containers", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshots` after provisioning.\n"]
    pub fn snapshots(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshots", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicy_PolicyDetail_ActionDynamic {
    include_resources: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources>>,
}

#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_Action {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_resources: std
    ::option
    ::Option<std::vec::Vec<ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources>>,
    dynamic: ImagebuilderLifecyclePolicy_PolicyDetail_ActionDynamic,
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_Action {
    #[doc = "Set the field `include_resources`.\n"]
    pub fn set_include_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResources>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy_PolicyDetail_Action {
    type O = terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderLifecyclePolicy_PolicyDetail_Action {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderLifecyclePolicy_PolicyDetail_Action {
    pub fn build(self) -> ImagebuilderLifecyclePolicy_PolicyDetail_Action {
        ImagebuilderLifecyclePolicy_PolicyDetail_Action {
            type_: self.type_,
            include_resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicy_PolicyDetail_ActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderLifecyclePolicy_PolicyDetail_ActionRef {
        ImagebuilderLifecyclePolicy_PolicyDetail_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `include_resources` after provisioning.\n"]
    pub fn include_resources(
        &self,
    ) -> terrars::ListRef<ImagebuilderLifecyclePolicy_PolicyDetail_Action_IncludeResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_resources", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched { }

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched {
    type O = terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched {
    pub fn build(self) -> ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched {
        ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunchedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunchedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunchedRef {
        ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunchedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunchedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_AmisDynamic {
    last_launched: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched>>,
}

#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_public: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_accounts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_map: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_launched: std
    ::option
    ::Option<std::vec::Vec<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched>>,
    dynamic: ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_AmisDynamic,
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis {
    #[doc = "Set the field `is_public`.\n"]
    pub fn set_is_public(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_public = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shared_accounts`.\n"]
    pub fn set_shared_accounts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.shared_accounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_map`.\n"]
    pub fn set_tag_map(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag_map = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_launched`.\n"]
    pub fn set_last_launched(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunched>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.last_launched = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.last_launched = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis {
    type O = terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis {}

impl BuildImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis {
    pub fn build(self) -> ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis {
        ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis {
            is_public: core::default::Default::default(),
            regions: core::default::Default::default(),
            shared_accounts: core::default::Default::default(),
            tag_map: core::default::Default::default(),
            last_launched: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_AmisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_AmisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_AmisRef {
        ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_AmisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_AmisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_public` after provisioning.\n"]
    pub fn is_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `shared_accounts` after provisioning.\n"]
    pub fn shared_accounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shared_accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_map` after provisioning.\n"]
    pub fn tag_map(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tag_map", self.base))
    }

    #[doc = "Get a reference to the value of field `last_launched` after provisioning.\n"]
    pub fn last_launched(
        &self,
    ) -> terrars::ListRef<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis_LastLaunchedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_launched", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRulesDynamic {
    amis: std::option::Option<terrars::DynamicBlock<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis>>,
}

#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_map: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amis: std::option::Option<std::vec::Vec<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis>>,
    dynamic: ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRulesDynamic,
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules {
    #[doc = "Set the field `tag_map`.\n"]
    pub fn set_tag_map(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag_map = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `amis`.\n"]
    pub fn set_amis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_Amis>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amis = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules {
    type O = terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules {}

impl BuildImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules {
    pub fn build(self) -> ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules {
        ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules {
            tag_map: core::default::Default::default(),
            amis: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRulesRef {
        ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tag_map` after provisioning.\n"]
    pub fn tag_map(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tag_map", self.base))
    }

    #[doc = "Get a reference to the value of field `amis` after provisioning.\n"]
    pub fn amis(&self) -> terrars::ListRef<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules_AmisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amis", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_at_least: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<f64>,
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_Filter {
    #[doc = "Set the field `retain_at_least`.\n"]
    pub fn set_retain_at_least(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retain_at_least = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy_PolicyDetail_Filter {
    type O = terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderLifecyclePolicy_PolicyDetail_Filter {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildImagebuilderLifecyclePolicy_PolicyDetail_Filter {
    pub fn build(self) -> ImagebuilderLifecyclePolicy_PolicyDetail_Filter {
        ImagebuilderLifecyclePolicy_PolicyDetail_Filter {
            retain_at_least: core::default::Default::default(),
            type_: self.type_,
            unit: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicy_PolicyDetail_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderLifecyclePolicy_PolicyDetail_FilterRef {
        ImagebuilderLifecyclePolicy_PolicyDetail_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderLifecyclePolicy_PolicyDetail_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retain_at_least` after provisioning.\n"]
    pub fn retain_at_least(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_at_least", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicy_PolicyDetailDynamic {
    action: std::option::Option<terrars::DynamicBlock<ImagebuilderLifecyclePolicy_PolicyDetail_Action>>,
    exclusion_rules: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules>>,
    filter: std::option::Option<terrars::DynamicBlock<ImagebuilderLifecyclePolicy_PolicyDetail_Filter>>,
}

#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetail {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<ImagebuilderLifecyclePolicy_PolicyDetail_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_rules: std::option::Option<std::vec::Vec<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<ImagebuilderLifecyclePolicy_PolicyDetail_Filter>>,
    dynamic: ImagebuilderLifecyclePolicy_PolicyDetailDynamic,
}

impl ImagebuilderLifecyclePolicy_PolicyDetail {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclusion_rules`.\n"]
    pub fn set_exclusion_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy_PolicyDetail {
    type O = terrars::BlockAssignable<ImagebuilderLifecyclePolicy_PolicyDetail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderLifecyclePolicy_PolicyDetail {}

impl BuildImagebuilderLifecyclePolicy_PolicyDetail {
    pub fn build(self) -> ImagebuilderLifecyclePolicy_PolicyDetail {
        ImagebuilderLifecyclePolicy_PolicyDetail {
            action: core::default::Default::default(),
            exclusion_rules: core::default::Default::default(),
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderLifecyclePolicy_PolicyDetailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicy_PolicyDetailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ImagebuilderLifecyclePolicy_PolicyDetailRef {
        ImagebuilderLifecyclePolicy_PolicyDetailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderLifecyclePolicy_PolicyDetailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<ImagebuilderLifecyclePolicy_PolicyDetail_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusion_rules` after provisioning.\n"]
    pub fn exclusion_rules(&self) -> terrars::ListRef<ImagebuilderLifecyclePolicy_PolicyDetail_ExclusionRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<ImagebuilderLifecyclePolicy_PolicyDetail_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicy_ResourceSelection_Recipe {
    name: terrars::PrimField<std::string::String>,
    semantic_version: terrars::PrimField<std::string::String>,
}

impl ImagebuilderLifecyclePolicy_ResourceSelection_Recipe { }

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy_ResourceSelection_Recipe {
    type O = terrars::BlockAssignable<ImagebuilderLifecyclePolicy_ResourceSelection_Recipe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderLifecyclePolicy_ResourceSelection_Recipe {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub semantic_version: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderLifecyclePolicy_ResourceSelection_Recipe {
    pub fn build(self) -> ImagebuilderLifecyclePolicy_ResourceSelection_Recipe {
        ImagebuilderLifecyclePolicy_ResourceSelection_Recipe {
            name: self.name,
            semantic_version: self.semantic_version,
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderLifecyclePolicy_ResourceSelection_RecipeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicy_ResourceSelection_RecipeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderLifecyclePolicy_ResourceSelection_RecipeRef {
        ImagebuilderLifecyclePolicy_ResourceSelection_RecipeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderLifecyclePolicy_ResourceSelection_RecipeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `semantic_version` after provisioning.\n"]
    pub fn semantic_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.semantic_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderLifecyclePolicy_ResourceSelectionDynamic {
    recipe: std::option::Option<terrars::DynamicBlock<ImagebuilderLifecyclePolicy_ResourceSelection_Recipe>>,
}

#[derive(Serialize)]
pub struct ImagebuilderLifecyclePolicy_ResourceSelection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_map: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recipe: std::option::Option<std::vec::Vec<ImagebuilderLifecyclePolicy_ResourceSelection_Recipe>>,
    dynamic: ImagebuilderLifecyclePolicy_ResourceSelectionDynamic,
}

impl ImagebuilderLifecyclePolicy_ResourceSelection {
    #[doc = "Set the field `tag_map`.\n"]
    pub fn set_tag_map(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag_map = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recipe`.\n"]
    pub fn set_recipe(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderLifecyclePolicy_ResourceSelection_Recipe>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recipe = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recipe = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderLifecyclePolicy_ResourceSelection {
    type O = terrars::BlockAssignable<ImagebuilderLifecyclePolicy_ResourceSelection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderLifecyclePolicy_ResourceSelection {}

impl BuildImagebuilderLifecyclePolicy_ResourceSelection {
    pub fn build(self) -> ImagebuilderLifecyclePolicy_ResourceSelection {
        ImagebuilderLifecyclePolicy_ResourceSelection {
            tag_map: core::default::Default::default(),
            recipe: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ImagebuilderLifecyclePolicy_ResourceSelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderLifecyclePolicy_ResourceSelectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderLifecyclePolicy_ResourceSelectionRef {
        ImagebuilderLifecyclePolicy_ResourceSelectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderLifecyclePolicy_ResourceSelectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tag_map` after provisioning.\n"]
    pub fn tag_map(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tag_map", self.base))
    }
}

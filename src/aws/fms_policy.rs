#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FmsPolicyData_ {
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
    delete_all_policy_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_unused_fm_managed_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    exclude_resource_tags: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remediation_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_set_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tag_logical_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_map: std::option::Option<std::vec::Vec<FmsPolicy_ExcludeMap>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_map: std::option::Option<std::vec::Vec<FmsPolicy_IncludeMap>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_service_policy_data: std::option::Option<std::vec::Vec<FmsPolicy_SecurityServicePolicyData>>,
    dynamic: FmsPolicyDynamic,
}

struct FmsPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FmsPolicyData_>,
}

#[derive(Clone)]
pub struct FmsPolicy(std::rc::Rc<FmsPolicy_>);

impl FmsPolicy {
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

    #[doc = "Set the field `delete_all_policy_resources`.\n"]
    pub fn set_delete_all_policy_resources(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_all_policy_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_unused_fm_managed_resources`.\n"]
    pub fn set_delete_unused_fm_managed_resources(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_unused_fm_managed_resources = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `remediation_enabled`.\n"]
    pub fn set_remediation_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().remediation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_set_ids`.\n"]
    pub fn set_resource_set_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_set_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_tag_logical_operator`.\n"]
    pub fn set_resource_tag_logical_operator(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_tag_logical_operator = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().resource_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type_list`.\n"]
    pub fn set_resource_type_list(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_type_list = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `exclude_map`.\n"]
    pub fn set_exclude_map(self, v: impl std::convert::Into<terrars::BlockAssignable<FmsPolicy_ExcludeMap>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_map = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_map = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_map`.\n"]
    pub fn set_include_map(self, v: impl std::convert::Into<terrars::BlockAssignable<FmsPolicy_IncludeMap>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().include_map = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.include_map = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_service_policy_data`.\n"]
    pub fn set_security_service_policy_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FmsPolicy_SecurityServicePolicyData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_service_policy_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_service_policy_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_all_policy_resources` after provisioning.\n"]
    pub fn delete_all_policy_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_all_policy_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_unused_fm_managed_resources` after provisioning.\n"]
    pub fn delete_unused_fm_managed_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_unused_fm_managed_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_resource_tags` after provisioning.\n"]
    pub fn exclude_resource_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_update_token` after provisioning.\n"]
    pub fn policy_update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_update_token", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remediation_enabled` after provisioning.\n"]
    pub fn remediation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_set_ids` after provisioning.\n"]
    pub fn resource_set_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_set_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tag_logical_operator` after provisioning.\n"]
    pub fn resource_tag_logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_tag_logical_operator", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type_list` after provisioning.\n"]
    pub fn resource_type_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_type_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_map` after provisioning.\n"]
    pub fn exclude_map(&self) -> terrars::ListRef<FmsPolicy_ExcludeMapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_map` after provisioning.\n"]
    pub fn include_map(&self) -> terrars::ListRef<FmsPolicy_IncludeMapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_service_policy_data` after provisioning.\n"]
    pub fn security_service_policy_data(&self) -> terrars::ListRef<FmsPolicy_SecurityServicePolicyDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_service_policy_data", self.extract_ref()))
    }
}

impl terrars::Referable for FmsPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FmsPolicy { }

impl terrars::ToListMappable for FmsPolicy {
    type O = terrars::ListRef<FmsPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FmsPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fms_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFmsPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub exclude_resource_tags: terrars::PrimField<bool>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildFmsPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> FmsPolicy {
        let out = FmsPolicy(std::rc::Rc::new(FmsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FmsPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                delete_all_policy_resources: core::default::Default::default(),
                delete_unused_fm_managed_resources: core::default::Default::default(),
                description: core::default::Default::default(),
                exclude_resource_tags: self.exclude_resource_tags,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                remediation_enabled: core::default::Default::default(),
                resource_set_ids: core::default::Default::default(),
                resource_tag_logical_operator: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
                resource_type: core::default::Default::default(),
                resource_type_list: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                exclude_map: core::default::Default::default(),
                include_map: core::default::Default::default(),
                security_service_policy_data: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FmsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FmsPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FmsPolicyRef {
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

    #[doc = "Get a reference to the value of field `delete_all_policy_resources` after provisioning.\n"]
    pub fn delete_all_policy_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_all_policy_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_unused_fm_managed_resources` after provisioning.\n"]
    pub fn delete_unused_fm_managed_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_unused_fm_managed_resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_resource_tags` after provisioning.\n"]
    pub fn exclude_resource_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_update_token` after provisioning.\n"]
    pub fn policy_update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_update_token", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remediation_enabled` after provisioning.\n"]
    pub fn remediation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remediation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_set_ids` after provisioning.\n"]
    pub fn resource_set_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_set_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tag_logical_operator` after provisioning.\n"]
    pub fn resource_tag_logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_tag_logical_operator", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\n"]
    pub fn resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type_list` after provisioning.\n"]
    pub fn resource_type_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_type_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `exclude_map` after provisioning.\n"]
    pub fn exclude_map(&self) -> terrars::ListRef<FmsPolicy_ExcludeMapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_map` after provisioning.\n"]
    pub fn include_map(&self) -> terrars::ListRef<FmsPolicy_IncludeMapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_map", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_service_policy_data` after provisioning.\n"]
    pub fn security_service_policy_data(&self) -> terrars::ListRef<FmsPolicy_SecurityServicePolicyDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_service_policy_data", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicyDynamic {
    exclude_map: std::option::Option<terrars::DynamicBlock<FmsPolicy_ExcludeMap>>,
    include_map: std::option::Option<terrars::DynamicBlock<FmsPolicy_IncludeMap>>,
    security_service_policy_data: std::option::Option<terrars::DynamicBlock<FmsPolicy_SecurityServicePolicyData>>,
}

#[derive(Serialize)]
pub struct FmsPolicy_ExcludeMap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orgunit: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FmsPolicy_ExcludeMap {
    #[doc = "Set the field `account`.\n"]
    pub fn set_account(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `orgunit`.\n"]
    pub fn set_orgunit(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.orgunit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FmsPolicy_ExcludeMap {
    type O = terrars::BlockAssignable<FmsPolicy_ExcludeMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_ExcludeMap {}

impl BuildFmsPolicy_ExcludeMap {
    pub fn build(self) -> FmsPolicy_ExcludeMap {
        FmsPolicy_ExcludeMap {
            account: core::default::Default::default(),
            orgunit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_ExcludeMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FmsPolicy_ExcludeMapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FmsPolicy_ExcludeMapRef {
        FmsPolicy_ExcludeMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_ExcludeMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc = "Get a reference to the value of field `orgunit` after provisioning.\n"]
    pub fn orgunit(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.orgunit", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicy_IncludeMap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orgunit: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FmsPolicy_IncludeMap {
    #[doc = "Set the field `account`.\n"]
    pub fn set_account(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `orgunit`.\n"]
    pub fn set_orgunit(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.orgunit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FmsPolicy_IncludeMap {
    type O = terrars::BlockAssignable<FmsPolicy_IncludeMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_IncludeMap {}

impl BuildFmsPolicy_IncludeMap {
    pub fn build(self) -> FmsPolicy_IncludeMap {
        FmsPolicy_IncludeMap {
            account: core::default::Default::default(),
            orgunit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_IncludeMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FmsPolicy_IncludeMapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FmsPolicy_IncludeMapRef {
        FmsPolicy_IncludeMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_IncludeMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc = "Get a reference to the value of field `orgunit` after provisioning.\n"]
    pub fn orgunit(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.orgunit", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<f64>>,
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode {
    type O =
        terrars
        ::BlockAssignable<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode {}

impl BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode {
    pub fn build(
        self,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode {
            code: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCodeRef {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange {
    type O =
        terrars
        ::BlockAssignable<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange {}

impl BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange {
    pub fn build(
        self,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRangeRef {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntryDynamic {
    icmp_type_code: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode,
        >,
    >,
    port_range: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange,
        >,
    >,
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    egress: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    protocol: terrars::PrimField<std::string::String>,
    rule_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icmp_type_code: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange,
        >,
    >,
    dynamic: FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntryDynamic,
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_type_code`.\n"]
    pub fn set_icmp_type_code(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.icmp_type_code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.icmp_type_code = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry {
    type O =
        terrars
        ::BlockAssignable<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry {
    #[doc = ""]
    pub egress: terrars::PrimField<bool>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_action: terrars::PrimField<std::string::String>,
}

impl BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry {
    pub fn build(
        self,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry {
            cidr_block: core::default::Default::default(),
            egress: self.egress,
            ipv6_cidr_block: core::default::Default::default(),
            protocol: self.protocol,
            rule_action: self.rule_action,
            icmp_type_code: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntryRef {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type_code` after provisioning.\n"]
    pub fn icmp_type_code(
        &self,
    ) -> terrars
    ::ListRef<
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCodeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.icmp_type_code", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(
        &self,
    ) -> terrars
    ::ListRef<
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRangeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntryDynamic {
    icmp_type_code: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode,
        >,
    >,
    port_range: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange,
        >,
    >,
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    egress: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    protocol: terrars::PrimField<std::string::String>,
    rule_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icmp_type_code: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange,
        >,
    >,
    dynamic: FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntryDynamic,
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_type_code`.\n"]
    pub fn set_icmp_type_code(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCode,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.icmp_type_code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.icmp_type_code = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRange,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry {
    type O =
        terrars
        ::BlockAssignable<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry {
    #[doc = ""]
    pub egress: terrars::PrimField<bool>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_action: terrars::PrimField<std::string::String>,
}

impl BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry {
    pub fn build(
        self,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry {
            cidr_block: core::default::Default::default(),
            egress: self.egress,
            ipv6_cidr_block: core::default::Default::default(),
            protocol: self.protocol,
            rule_action: self.rule_action,
            icmp_type_code: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntryRef {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type_code` after provisioning.\n"]
    pub fn icmp_type_code(
        &self,
    ) -> terrars
    ::ListRef<
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_IcmpTypeCodeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.icmp_type_code", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(
        &self,
    ) -> terrars
    ::ListRef<
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry_PortRangeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySetDynamic {
    first_entry: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry,
        >,
    >,
    last_entry: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry,
        >,
    >,
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet {
    force_remediate_for_first_entries: terrars::PrimField<bool>,
    force_remediate_for_last_entries: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_entry: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_entry: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry>,
    >,
    dynamic: FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySetDynamic,
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet {
    #[doc = "Set the field `first_entry`.\n"]
    pub fn set_first_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_FirstEntry,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.first_entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.first_entry = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `last_entry`.\n"]
    pub fn set_last_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet_LastEntry,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.last_entry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.last_entry = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet {
    type O =
        terrars
        ::BlockAssignable<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet {
    #[doc = ""]
    pub force_remediate_for_first_entries: terrars::PrimField<bool>,
    #[doc = ""]
    pub force_remediate_for_last_entries: terrars::PrimField<bool>,
}

impl BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet {
    pub fn build(self) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet {
            force_remediate_for_first_entries: self.force_remediate_for_first_entries,
            force_remediate_for_last_entries: self.force_remediate_for_last_entries,
            first_entry: core::default::Default::default(),
            last_entry: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySetRef {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `force_remediate_for_first_entries` after provisioning.\n"]
    pub fn force_remediate_for_first_entries(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_remediate_for_first_entries", self.base))
    }

    #[doc = "Get a reference to the value of field `force_remediate_for_last_entries` after provisioning.\n"]
    pub fn force_remediate_for_last_entries(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_remediate_for_last_entries", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicyDynamic {
    network_acl_entry_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet>,
    >,
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_acl_entry_set: std
    ::option
    ::Option<
        std::vec::Vec<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet>,
    >,
    dynamic: FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicyDynamic,
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy {
    #[doc = "Set the field `network_acl_entry_set`.\n"]
    pub fn set_network_acl_entry_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_acl_entry_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_acl_entry_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy {
    type O = terrars::BlockAssignable<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy {}

impl BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy {
    pub fn build(self) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy {
            network_acl_entry_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicyRef {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_acl_entry_set` after provisioning.\n"]
    pub fn network_acl_entry_set(
        &self,
    ) -> terrars
    ::ListRef<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy_NetworkAclEntrySetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_acl_entry_set", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firewall_deployment_model: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy {
    #[doc = "Set the field `firewall_deployment_model`.\n"]
    pub fn set_firewall_deployment_model(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.firewall_deployment_model = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy {
    type O = terrars::BlockAssignable<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy {}

impl BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy {
    pub fn build(self) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy {
            firewall_deployment_model: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicyRef {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `firewall_deployment_model` after provisioning.\n"]
    pub fn firewall_deployment_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_deployment_model", self.base))
    }
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firewall_deployment_model: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy {
    #[doc = "Set the field `firewall_deployment_model`.\n"]
    pub fn set_firewall_deployment_model(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.firewall_deployment_model = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy {
    type O = terrars::BlockAssignable<FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy {}

impl BuildFmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy {
    pub fn build(self) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy {
            firewall_deployment_model: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicyRef {
        FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `firewall_deployment_model` after provisioning.\n"]
    pub fn firewall_deployment_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_deployment_model", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicy_SecurityServicePolicyData_PolicyOptionDynamic {
    network_acl_common_policy: std
    ::option
    ::Option<terrars::DynamicBlock<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy>>,
    network_firewall_policy: std
    ::option
    ::Option<terrars::DynamicBlock<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy>>,
    third_party_firewall_policy: std
    ::option
    ::Option<terrars::DynamicBlock<FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy>>,
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_acl_common_policy: std
    ::option
    ::Option<std::vec::Vec<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_firewall_policy: std
    ::option
    ::Option<std::vec::Vec<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    third_party_firewall_policy: std
    ::option
    ::Option<std::vec::Vec<FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy>>,
    dynamic: FmsPolicy_SecurityServicePolicyData_PolicyOptionDynamic,
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOption {
    #[doc = "Set the field `network_acl_common_policy`.\n"]
    pub fn set_network_acl_common_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_acl_common_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_acl_common_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_firewall_policy`.\n"]
    pub fn set_network_firewall_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_firewall_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_firewall_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `third_party_firewall_policy`.\n"]
    pub fn set_third_party_firewall_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.third_party_firewall_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.third_party_firewall_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FmsPolicy_SecurityServicePolicyData_PolicyOption {
    type O = terrars::BlockAssignable<FmsPolicy_SecurityServicePolicyData_PolicyOption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData_PolicyOption {}

impl BuildFmsPolicy_SecurityServicePolicyData_PolicyOption {
    pub fn build(self) -> FmsPolicy_SecurityServicePolicyData_PolicyOption {
        FmsPolicy_SecurityServicePolicyData_PolicyOption {
            network_acl_common_policy: core::default::Default::default(),
            network_firewall_policy: core::default::Default::default(),
            third_party_firewall_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyData_PolicyOptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FmsPolicy_SecurityServicePolicyData_PolicyOptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FmsPolicy_SecurityServicePolicyData_PolicyOptionRef {
        FmsPolicy_SecurityServicePolicyData_PolicyOptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyData_PolicyOptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_acl_common_policy` after provisioning.\n"]
    pub fn network_acl_common_policy(
        &self,
    ) -> terrars::ListRef<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkAclCommonPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_acl_common_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `network_firewall_policy` after provisioning.\n"]
    pub fn network_firewall_policy(
        &self,
    ) -> terrars::ListRef<FmsPolicy_SecurityServicePolicyData_PolicyOption_NetworkFirewallPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_firewall_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `third_party_firewall_policy` after provisioning.\n"]
    pub fn third_party_firewall_policy(
        &self,
    ) -> terrars::ListRef<FmsPolicy_SecurityServicePolicyData_PolicyOption_ThirdPartyFirewallPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.third_party_firewall_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct FmsPolicy_SecurityServicePolicyDataDynamic {
    policy_option: std::option::Option<terrars::DynamicBlock<FmsPolicy_SecurityServicePolicyData_PolicyOption>>,
}

#[derive(Serialize)]
pub struct FmsPolicy_SecurityServicePolicyData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_service_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_option: std::option::Option<std::vec::Vec<FmsPolicy_SecurityServicePolicyData_PolicyOption>>,
    dynamic: FmsPolicy_SecurityServicePolicyDataDynamic,
}

impl FmsPolicy_SecurityServicePolicyData {
    #[doc = "Set the field `managed_service_data`.\n"]
    pub fn set_managed_service_data(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.managed_service_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_option`.\n"]
    pub fn set_policy_option(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FmsPolicy_SecurityServicePolicyData_PolicyOption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_option = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_option = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FmsPolicy_SecurityServicePolicyData {
    type O = terrars::BlockAssignable<FmsPolicy_SecurityServicePolicyData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFmsPolicy_SecurityServicePolicyData {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildFmsPolicy_SecurityServicePolicyData {
    pub fn build(self) -> FmsPolicy_SecurityServicePolicyData {
        FmsPolicy_SecurityServicePolicyData {
            managed_service_data: core::default::Default::default(),
            type_: self.type_,
            policy_option: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FmsPolicy_SecurityServicePolicyDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FmsPolicy_SecurityServicePolicyDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FmsPolicy_SecurityServicePolicyDataRef {
        FmsPolicy_SecurityServicePolicyDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FmsPolicy_SecurityServicePolicyDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_service_data` after provisioning.\n"]
    pub fn managed_service_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_service_data", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_option` after provisioning.\n"]
    pub fn policy_option(&self) -> terrars::ListRef<FmsPolicy_SecurityServicePolicyData_PolicyOptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_option", self.base))
    }
}

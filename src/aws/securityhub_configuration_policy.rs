#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecurityhubConfigurationPolicyData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_policy: std::option::Option<std::vec::Vec<SecurityhubConfigurationPolicy_ConfigurationPolicy>>,
    dynamic: SecurityhubConfigurationPolicyDynamic,
}

struct SecurityhubConfigurationPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecurityhubConfigurationPolicyData_>,
}

#[derive(Clone)]
pub struct SecurityhubConfigurationPolicy(std::rc::Rc<SecurityhubConfigurationPolicy_>);

impl SecurityhubConfigurationPolicy {
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

    #[doc = "Set the field `configuration_policy`.\n"]
    pub fn set_configuration_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubConfigurationPolicy_ConfigurationPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration_policy = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_policy` after provisioning.\n"]
    pub fn configuration_policy(&self) -> terrars::ListRef<SecurityhubConfigurationPolicy_ConfigurationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_policy", self.extract_ref()))
    }
}

impl terrars::Referable for SecurityhubConfigurationPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecurityhubConfigurationPolicy { }

impl terrars::ToListMappable for SecurityhubConfigurationPolicy {
    type O = terrars::ListRef<SecurityhubConfigurationPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecurityhubConfigurationPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securityhub_configuration_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityhubConfigurationPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubConfigurationPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> SecurityhubConfigurationPolicy {
        let out = SecurityhubConfigurationPolicy(std::rc::Rc::new(SecurityhubConfigurationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecurityhubConfigurationPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                configuration_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityhubConfigurationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubConfigurationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecurityhubConfigurationPolicyRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_policy` after provisioning.\n"]
    pub fn configuration_policy(&self) -> terrars::ListRef<SecurityhubConfigurationPolicy_ConfigurationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_policy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicyDynamic {
    configuration_policy: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubConfigurationPolicy_ConfigurationPolicy>>,
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool {
    value: terrars::PrimField<bool>,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool {

}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool {
    #[doc = ""]
    pub value: terrars::PrimField<bool>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_BoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_BoolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_BoolRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_BoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_BoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double {
    value: terrars::PrimField<f64>,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double {

}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double {
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_DoubleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_DoubleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_DoubleRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_DoubleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_DoubleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum {
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum {

}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList {
    value: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList {

}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList {
    #[doc = ""]
    pub value: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumListRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int {
    value: terrars::PrimField<f64>,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int {

}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int {
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList {
    value: terrars::ListField<terrars::PrimField<f64>>,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList {

}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList {
    #[doc = ""]
    pub value: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntListRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String {
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String {

}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList {
    value: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList {

}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList {
    #[doc = ""]
    pub value: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList {
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringListRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_ParameterDynamic {
    bool: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool,
        >,
    >,
    double: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double,
        >,
    >,
    enum_: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum,
        >,
    >,
    enum_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList,
        >,
    >,
    int: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int,
        >,
    >,
    int_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList,
        >,
    >,
    string: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String,
        >,
    >,
    string_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList,
        >,
    >,
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter {
    name: terrars::PrimField<std::string::String>,
    value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    double: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double,
        >,
    >,
    #[serde(rename = "enum", skip_serializing_if = "std::option::Option::is_none")]
    enum_: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_list: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    int: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    int_list: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_list: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList,
        >,
    >,
    dynamic: SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_ParameterDynamic,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter {
    #[doc = "Set the field `bool`.\n"]
    pub fn set_bool(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Bool,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `double`.\n"]
    pub fn set_double(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Double,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.double = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.double = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enum_`.\n"]
    pub fn set_enum(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Enum,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enum_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enum_list`.\n"]
    pub fn set_enum_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enum_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `int`.\n"]
    pub fn set_int(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_Int,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.int = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.int = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `int_list`.\n"]
    pub fn set_int_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.int_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.int_list = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string`.\n"]
    pub fn set_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_String,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_list`.\n"]
    pub fn set_string_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_type: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter {
            name: self.name,
            value_type: self.value_type,
            bool: core::default::Default::default(),
            double: core::default::Default::default(),
            enum_: core::default::Default::default(),
            enum_list: core::default::Default::default(),
            int: core::default::Default::default(),
            int_list: core::default::Default::default(),
            string: core::default::Default::default(),
            string_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_ParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_ParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_ParameterRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_ParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_ParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value_type` after provisioning.\n"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `bool` after provisioning.\n"]
    pub fn bool(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_BoolRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bool", self.base))
    }

    #[doc = "Get a reference to the value of field `double` after provisioning.\n"]
    pub fn double(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_DoubleRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.double", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_` after provisioning.\n"]
    pub fn enum_(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_list` after provisioning.\n"]
    pub fn enum_list(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_EnumListRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum_list", self.base))
    }

    #[doc = "Get a reference to the value of field `int` after provisioning.\n"]
    pub fn int(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.int", self.base))
    }

    #[doc = "Get a reference to the value of field `int_list` after provisioning.\n"]
    pub fn int_list(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_IntListRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.int_list", self.base))
    }

    #[doc = "Get a reference to the value of field `string` after provisioning.\n"]
    pub fn string(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string", self.base))
    }

    #[doc = "Get a reference to the value of field `string_list` after provisioning.\n"]
    pub fn string_list(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter_StringListRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameterDynamic {
    parameter: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter,
        >,
    >,
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter {
    security_control_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter,
        >,
    >,
    dynamic: SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameterDynamic,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter {
    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter_Parameter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter {
    type O =
        terrars
        ::BlockAssignable<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter {
    #[doc = ""]
    pub security_control_id: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter {
    pub fn build(
        self,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter {
            security_control_id: self.security_control_id,
            parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameterRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_control_id` after provisioning.\n"]
    pub fn security_control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_control_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfigurationDynamic {
    security_control_custom_parameter: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter,
        >,
    >,
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled_control_identifiers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_control_identifiers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_control_custom_parameter: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter,
        >,
    >,
    dynamic: SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfigurationDynamic,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration {
    #[doc = "Set the field `disabled_control_identifiers`.\n"]
    pub fn set_disabled_control_identifiers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.disabled_control_identifiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_control_identifiers`.\n"]
    pub fn set_enabled_control_identifiers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enabled_control_identifiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_control_custom_parameter`.\n"]
    pub fn set_security_control_custom_parameter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameter,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_control_custom_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_control_custom_parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration {
    type O =
        terrars::BlockAssignable<SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration {}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration {
    pub fn build(self) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration {
            disabled_control_identifiers: core::default::Default::default(),
            enabled_control_identifiers: core::default::Default::default(),
            security_control_custom_parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfigurationRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled_control_identifiers` after provisioning.\n"]
    pub fn disabled_control_identifiers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.disabled_control_identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled_control_identifiers` after provisioning.\n"]
    pub fn enabled_control_identifiers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_control_identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `security_control_custom_parameter` after provisioning.\n"]
    pub fn security_control_custom_parameter(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration_SecurityControlCustomParameterRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_control_custom_parameter", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubConfigurationPolicy_ConfigurationPolicyDynamic {
    security_controls_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_standard_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    service_enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_controls_configuration: std
    ::option
    ::Option<std::vec::Vec<SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration>>,
    dynamic: SecurityhubConfigurationPolicy_ConfigurationPolicyDynamic,
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicy {
    #[doc = "Set the field `enabled_standard_arns`.\n"]
    pub fn set_enabled_standard_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enabled_standard_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_controls_configuration`.\n"]
    pub fn set_security_controls_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_controls_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_controls_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubConfigurationPolicy_ConfigurationPolicy {
    type O = terrars::BlockAssignable<SecurityhubConfigurationPolicy_ConfigurationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConfigurationPolicy_ConfigurationPolicy {
    #[doc = ""]
    pub service_enabled: terrars::PrimField<bool>,
}

impl BuildSecurityhubConfigurationPolicy_ConfigurationPolicy {
    pub fn build(self) -> SecurityhubConfigurationPolicy_ConfigurationPolicy {
        SecurityhubConfigurationPolicy_ConfigurationPolicy {
            enabled_standard_arns: core::default::Default::default(),
            service_enabled: self.service_enabled,
            security_controls_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConfigurationPolicy_ConfigurationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubConfigurationPolicy_ConfigurationPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConfigurationPolicy_ConfigurationPolicyRef {
        SecurityhubConfigurationPolicy_ConfigurationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConfigurationPolicy_ConfigurationPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_standard_arns` after provisioning.\n"]
    pub fn enabled_standard_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_standard_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `service_enabled` after provisioning.\n"]
    pub fn service_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `security_controls_configuration` after provisioning.\n"]
    pub fn security_controls_configuration(
        &self,
    ) -> terrars::ListRef<SecurityhubConfigurationPolicy_ConfigurationPolicy_SecurityControlsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_controls_configuration", self.base))
    }
}

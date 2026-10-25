#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecurityhubInsightData_ {
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
    group_by_attribute: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters>>,
    dynamic: SecurityhubInsightDynamic,
}

struct SecurityhubInsight_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecurityhubInsightData_>,
}

#[derive(Clone)]
pub struct SecurityhubInsight(std::rc::Rc<SecurityhubInsight_>);

impl SecurityhubInsight {
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

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(self, v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_by_attribute` after provisioning.\n"]
    pub fn group_by_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_by_attribute", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<SecurityhubInsight_FiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

impl terrars::Referable for SecurityhubInsight {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecurityhubInsight { }

impl terrars::ToListMappable for SecurityhubInsight {
    type O = terrars::ListRef<SecurityhubInsightRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecurityhubInsight_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securityhub_insight".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityhubInsight {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub group_by_attribute: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight {
    pub fn build(self, stack: &mut terrars::Stack) -> SecurityhubInsight {
        let out = SecurityhubInsight(std::rc::Rc::new(SecurityhubInsight_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecurityhubInsightData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                group_by_attribute: self.group_by_attribute,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                filters: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityhubInsightRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecurityhubInsightRef {
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

    #[doc = "Get a reference to the value of field `group_by_attribute` after provisioning.\n"]
    pub fn group_by_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_by_attribute", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<SecurityhubInsight_FiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightDynamic {
    filters: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_AwsAccountId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_AwsAccountId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_AwsAccountId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_AwsAccountId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_AwsAccountId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_AwsAccountId {
    pub fn build(self) -> SecurityhubInsight_Filters_AwsAccountId {
        SecurityhubInsight_Filters_AwsAccountId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_AwsAccountIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_AwsAccountIdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_AwsAccountIdRef {
        SecurityhubInsight_Filters_AwsAccountIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_AwsAccountIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_AwsAccountName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_AwsAccountName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_AwsAccountName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_AwsAccountName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_AwsAccountName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_AwsAccountName {
    pub fn build(self) -> SecurityhubInsight_Filters_AwsAccountName {
        SecurityhubInsight_Filters_AwsAccountName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_AwsAccountNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_AwsAccountNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_AwsAccountNameRef {
        SecurityhubInsight_Filters_AwsAccountNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_AwsAccountNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_CompanyName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_CompanyName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_CompanyName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_CompanyName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_CompanyName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_CompanyName {
    pub fn build(self) -> SecurityhubInsight_Filters_CompanyName {
        SecurityhubInsight_Filters_CompanyName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_CompanyNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_CompanyNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_CompanyNameRef {
        SecurityhubInsight_Filters_CompanyNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_CompanyNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ComplianceAssociatedStandardsId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ComplianceAssociatedStandardsId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ComplianceAssociatedStandardsId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceAssociatedStandardsId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ComplianceAssociatedStandardsId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ComplianceAssociatedStandardsId {
    pub fn build(self) -> SecurityhubInsight_Filters_ComplianceAssociatedStandardsId {
        SecurityhubInsight_Filters_ComplianceAssociatedStandardsId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ComplianceAssociatedStandardsIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ComplianceAssociatedStandardsIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ComplianceAssociatedStandardsIdRef {
        SecurityhubInsight_Filters_ComplianceAssociatedStandardsIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ComplianceAssociatedStandardsIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ComplianceSecurityControlId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ComplianceSecurityControlId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ComplianceSecurityControlId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceSecurityControlId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ComplianceSecurityControlId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ComplianceSecurityControlId {
    pub fn build(self) -> SecurityhubInsight_Filters_ComplianceSecurityControlId {
        SecurityhubInsight_Filters_ComplianceSecurityControlId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ComplianceSecurityControlIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ComplianceSecurityControlIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ComplianceSecurityControlIdRef {
        SecurityhubInsight_Filters_ComplianceSecurityControlIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ComplianceSecurityControlIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ComplianceSecurityControlParametersName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ComplianceSecurityControlParametersName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ComplianceSecurityControlParametersName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceSecurityControlParametersName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ComplianceSecurityControlParametersName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ComplianceSecurityControlParametersName {
    pub fn build(self) -> SecurityhubInsight_Filters_ComplianceSecurityControlParametersName {
        SecurityhubInsight_Filters_ComplianceSecurityControlParametersName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ComplianceSecurityControlParametersNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ComplianceSecurityControlParametersNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ComplianceSecurityControlParametersNameRef {
        SecurityhubInsight_Filters_ComplianceSecurityControlParametersNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ComplianceSecurityControlParametersNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ComplianceSecurityControlParametersValue {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ComplianceSecurityControlParametersValue { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ComplianceSecurityControlParametersValue {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceSecurityControlParametersValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ComplianceSecurityControlParametersValue {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ComplianceSecurityControlParametersValue {
    pub fn build(self) -> SecurityhubInsight_Filters_ComplianceSecurityControlParametersValue {
        SecurityhubInsight_Filters_ComplianceSecurityControlParametersValue {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ComplianceSecurityControlParametersValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ComplianceSecurityControlParametersValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ComplianceSecurityControlParametersValueRef {
        SecurityhubInsight_Filters_ComplianceSecurityControlParametersValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ComplianceSecurityControlParametersValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ComplianceStatus {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ComplianceStatus { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ComplianceStatus {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ComplianceStatus {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ComplianceStatus {
    pub fn build(self) -> SecurityhubInsight_Filters_ComplianceStatus {
        SecurityhubInsight_Filters_ComplianceStatus {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ComplianceStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ComplianceStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ComplianceStatusRef {
        SecurityhubInsight_Filters_ComplianceStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ComplianceStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_Confidence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsight_Filters_Confidence {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_Confidence {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_Confidence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_Confidence {}

impl BuildSecurityhubInsight_Filters_Confidence {
    pub fn build(self) -> SecurityhubInsight_Filters_Confidence {
        SecurityhubInsight_Filters_Confidence {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ConfidenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ConfidenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ConfidenceRef {
        SecurityhubInsight_Filters_ConfidenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ConfidenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_CreatedAt_DateRange {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsight_Filters_CreatedAt_DateRange { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_CreatedAt_DateRange {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_CreatedAt_DateRange {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsight_Filters_CreatedAt_DateRange {
    pub fn build(self) -> SecurityhubInsight_Filters_CreatedAt_DateRange {
        SecurityhubInsight_Filters_CreatedAt_DateRange {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_CreatedAt_DateRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_CreatedAt_DateRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_CreatedAt_DateRangeRef {
        SecurityhubInsight_Filters_CreatedAt_DateRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_CreatedAt_DateRangeRef {
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
struct SecurityhubInsight_Filters_CreatedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_CreatedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_CreatedAtDynamic,
}

impl SecurityhubInsight_Filters_CreatedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_CreatedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_CreatedAt {}

impl BuildSecurityhubInsight_Filters_CreatedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_CreatedAt {
        SecurityhubInsight_Filters_CreatedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_CreatedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_CreatedAtRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_CreatedAtRef {
        SecurityhubInsight_Filters_CreatedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_CreatedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_Criticality {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsight_Filters_Criticality {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_Criticality {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_Criticality>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_Criticality {}

impl BuildSecurityhubInsight_Filters_Criticality {
    pub fn build(self) -> SecurityhubInsight_Filters_Criticality {
        SecurityhubInsight_Filters_Criticality {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_CriticalityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_CriticalityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_CriticalityRef {
        SecurityhubInsight_Filters_CriticalityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_CriticalityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_Description {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_Description { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_Description {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_Description>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_Description {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_Description {
    pub fn build(self) -> SecurityhubInsight_Filters_Description {
        SecurityhubInsight_Filters_Description {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_DescriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_DescriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_DescriptionRef {
        SecurityhubInsight_Filters_DescriptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_DescriptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsConfidence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsight_Filters_FindingProviderFieldsConfidence {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_FindingProviderFieldsConfidence {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsConfidence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_FindingProviderFieldsConfidence {}

impl BuildSecurityhubInsight_Filters_FindingProviderFieldsConfidence {
    pub fn build(self) -> SecurityhubInsight_Filters_FindingProviderFieldsConfidence {
        SecurityhubInsight_Filters_FindingProviderFieldsConfidence {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsConfidenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_FindingProviderFieldsConfidenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_FindingProviderFieldsConfidenceRef {
        SecurityhubInsight_Filters_FindingProviderFieldsConfidenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_FindingProviderFieldsConfidenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsCriticality {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsight_Filters_FindingProviderFieldsCriticality {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_FindingProviderFieldsCriticality {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsCriticality>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_FindingProviderFieldsCriticality {}

impl BuildSecurityhubInsight_Filters_FindingProviderFieldsCriticality {
    pub fn build(self) -> SecurityhubInsight_Filters_FindingProviderFieldsCriticality {
        SecurityhubInsight_Filters_FindingProviderFieldsCriticality {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsCriticalityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_FindingProviderFieldsCriticalityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_FindingProviderFieldsCriticalityRef {
        SecurityhubInsight_Filters_FindingProviderFieldsCriticalityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_FindingProviderFieldsCriticalityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId {
    pub fn build(self) -> SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId {
        SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsIdRef {
        SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn {
    pub fn build(self) -> SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn {
        SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArnRef {
        SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel {
    pub fn build(self) -> SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel {
        SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabelRef {
        SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal {
    pub fn build(self) -> SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal {
        SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginalRef {
        SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsTypes {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_FindingProviderFieldsTypes { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_FindingProviderFieldsTypes {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_FindingProviderFieldsTypes {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_FindingProviderFieldsTypes {
    pub fn build(self) -> SecurityhubInsight_Filters_FindingProviderFieldsTypes {
        SecurityhubInsight_Filters_FindingProviderFieldsTypes {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_FindingProviderFieldsTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_FindingProviderFieldsTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_FindingProviderFieldsTypesRef {
        SecurityhubInsight_Filters_FindingProviderFieldsTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_FindingProviderFieldsTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_FirstObservedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_FirstObservedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_FirstObservedAtDynamic,
}

impl SecurityhubInsight_Filters_FirstObservedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_FirstObservedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_FirstObservedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_FirstObservedAt {}

impl BuildSecurityhubInsight_Filters_FirstObservedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_FirstObservedAt {
        SecurityhubInsight_Filters_FirstObservedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_FirstObservedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_FirstObservedAtRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_FirstObservedAtRef {
        SecurityhubInsight_Filters_FirstObservedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_FirstObservedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_GeneratorId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_GeneratorId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_GeneratorId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_GeneratorId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_GeneratorId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_GeneratorId {
    pub fn build(self) -> SecurityhubInsight_Filters_GeneratorId {
        SecurityhubInsight_Filters_GeneratorId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_GeneratorIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_GeneratorIdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_GeneratorIdRef {
        SecurityhubInsight_Filters_GeneratorIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_GeneratorIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_Id {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_Id { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_Id {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_Id>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_Id {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_Id {
    pub fn build(self) -> SecurityhubInsight_Filters_Id {
        SecurityhubInsight_Filters_Id {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_IdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_IdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_IdRef {
        SecurityhubInsight_Filters_IdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_IdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_Keyword {
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_Keyword { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_Keyword {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_Keyword>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_Keyword {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_Keyword {
    pub fn build(self) -> SecurityhubInsight_Filters_Keyword {
        SecurityhubInsight_Filters_Keyword { value: self.value }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_KeywordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_KeywordRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_KeywordRef {
        SecurityhubInsight_Filters_KeywordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_KeywordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_LastObservedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_LastObservedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_LastObservedAtDynamic,
}

impl SecurityhubInsight_Filters_LastObservedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_LastObservedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_LastObservedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_LastObservedAt {}

impl BuildSecurityhubInsight_Filters_LastObservedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_LastObservedAt {
        SecurityhubInsight_Filters_LastObservedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_LastObservedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_LastObservedAtRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_LastObservedAtRef {
        SecurityhubInsight_Filters_LastObservedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_LastObservedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_MalwareName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_MalwareName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_MalwareName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_MalwareName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_MalwareName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_MalwareName {
    pub fn build(self) -> SecurityhubInsight_Filters_MalwareName {
        SecurityhubInsight_Filters_MalwareName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_MalwareNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_MalwareNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_MalwareNameRef {
        SecurityhubInsight_Filters_MalwareNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_MalwareNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_MalwarePath {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_MalwarePath { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_MalwarePath {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_MalwarePath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_MalwarePath {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_MalwarePath {
    pub fn build(self) -> SecurityhubInsight_Filters_MalwarePath {
        SecurityhubInsight_Filters_MalwarePath {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_MalwarePathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_MalwarePathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_MalwarePathRef {
        SecurityhubInsight_Filters_MalwarePathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_MalwarePathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_MalwareState {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_MalwareState { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_MalwareState {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_MalwareState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_MalwareState {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_MalwareState {
    pub fn build(self) -> SecurityhubInsight_Filters_MalwareState {
        SecurityhubInsight_Filters_MalwareState {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_MalwareStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_MalwareStateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_MalwareStateRef {
        SecurityhubInsight_Filters_MalwareStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_MalwareStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_MalwareType {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_MalwareType { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_MalwareType {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_MalwareType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_MalwareType {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_MalwareType {
    pub fn build(self) -> SecurityhubInsight_Filters_MalwareType {
        SecurityhubInsight_Filters_MalwareType {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_MalwareTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_MalwareTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_MalwareTypeRef {
        SecurityhubInsight_Filters_MalwareTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_MalwareTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkDestinationDomain {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NetworkDestinationDomain { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkDestinationDomain {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDestinationDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkDestinationDomain {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NetworkDestinationDomain {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkDestinationDomain {
        SecurityhubInsight_Filters_NetworkDestinationDomain {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkDestinationDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkDestinationDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkDestinationDomainRef {
        SecurityhubInsight_Filters_NetworkDestinationDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkDestinationDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkDestinationIpv4 {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NetworkDestinationIpv4 { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkDestinationIpv4 {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDestinationIpv4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkDestinationIpv4 {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NetworkDestinationIpv4 {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkDestinationIpv4 {
        SecurityhubInsight_Filters_NetworkDestinationIpv4 { cidr: self.cidr }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkDestinationIpv4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkDestinationIpv4Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkDestinationIpv4Ref {
        SecurityhubInsight_Filters_NetworkDestinationIpv4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkDestinationIpv4Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkDestinationIpv6 {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NetworkDestinationIpv6 { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkDestinationIpv6 {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDestinationIpv6>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkDestinationIpv6 {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NetworkDestinationIpv6 {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkDestinationIpv6 {
        SecurityhubInsight_Filters_NetworkDestinationIpv6 { cidr: self.cidr }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkDestinationIpv6Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkDestinationIpv6Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkDestinationIpv6Ref {
        SecurityhubInsight_Filters_NetworkDestinationIpv6Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkDestinationIpv6Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkDestinationPort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsight_Filters_NetworkDestinationPort {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkDestinationPort {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDestinationPort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkDestinationPort {}

impl BuildSecurityhubInsight_Filters_NetworkDestinationPort {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkDestinationPort {
        SecurityhubInsight_Filters_NetworkDestinationPort {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkDestinationPortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkDestinationPortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkDestinationPortRef {
        SecurityhubInsight_Filters_NetworkDestinationPortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkDestinationPortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkDirection {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NetworkDirection { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkDirection {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDirection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkDirection {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NetworkDirection {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkDirection {
        SecurityhubInsight_Filters_NetworkDirection {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkDirectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkDirectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkDirectionRef {
        SecurityhubInsight_Filters_NetworkDirectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkDirectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkProtocol {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NetworkProtocol { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkProtocol {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkProtocol>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkProtocol {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NetworkProtocol {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkProtocol {
        SecurityhubInsight_Filters_NetworkProtocol {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkProtocolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkProtocolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_NetworkProtocolRef {
        SecurityhubInsight_Filters_NetworkProtocolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkProtocolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkSourceDomain {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NetworkSourceDomain { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkSourceDomain {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourceDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkSourceDomain {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NetworkSourceDomain {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkSourceDomain {
        SecurityhubInsight_Filters_NetworkSourceDomain {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkSourceDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkSourceDomainRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkSourceDomainRef {
        SecurityhubInsight_Filters_NetworkSourceDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkSourceDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkSourceIpv4 {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NetworkSourceIpv4 { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkSourceIpv4 {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourceIpv4>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkSourceIpv4 {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NetworkSourceIpv4 {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkSourceIpv4 {
        SecurityhubInsight_Filters_NetworkSourceIpv4 { cidr: self.cidr }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkSourceIpv4Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkSourceIpv4Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkSourceIpv4Ref {
        SecurityhubInsight_Filters_NetworkSourceIpv4Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkSourceIpv4Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkSourceIpv6 {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NetworkSourceIpv6 { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkSourceIpv6 {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourceIpv6>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkSourceIpv6 {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NetworkSourceIpv6 {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkSourceIpv6 {
        SecurityhubInsight_Filters_NetworkSourceIpv6 { cidr: self.cidr }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkSourceIpv6Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkSourceIpv6Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkSourceIpv6Ref {
        SecurityhubInsight_Filters_NetworkSourceIpv6Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkSourceIpv6Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkSourceMac {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NetworkSourceMac { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkSourceMac {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourceMac>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkSourceMac {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NetworkSourceMac {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkSourceMac {
        SecurityhubInsight_Filters_NetworkSourceMac {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkSourceMacRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkSourceMacRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkSourceMacRef {
        SecurityhubInsight_Filters_NetworkSourceMacRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkSourceMacRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NetworkSourcePort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsight_Filters_NetworkSourcePort {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_NetworkSourcePort {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourcePort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NetworkSourcePort {}

impl BuildSecurityhubInsight_Filters_NetworkSourcePort {
    pub fn build(self) -> SecurityhubInsight_Filters_NetworkSourcePort {
        SecurityhubInsight_Filters_NetworkSourcePort {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NetworkSourcePortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NetworkSourcePortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_NetworkSourcePortRef {
        SecurityhubInsight_Filters_NetworkSourcePortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NetworkSourcePortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NoteText {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NoteText { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NoteText {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NoteText>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NoteText {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NoteText {
    pub fn build(self) -> SecurityhubInsight_Filters_NoteText {
        SecurityhubInsight_Filters_NoteText {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NoteTextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NoteTextRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_NoteTextRef {
        SecurityhubInsight_Filters_NoteTextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NoteTextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_NoteUpdatedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NoteUpdatedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_NoteUpdatedAtDynamic,
}

impl SecurityhubInsight_Filters_NoteUpdatedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_NoteUpdatedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NoteUpdatedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NoteUpdatedAt {}

impl BuildSecurityhubInsight_Filters_NoteUpdatedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_NoteUpdatedAt {
        SecurityhubInsight_Filters_NoteUpdatedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NoteUpdatedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NoteUpdatedAtRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_NoteUpdatedAtRef {
        SecurityhubInsight_Filters_NoteUpdatedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NoteUpdatedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_NoteUpdatedBy {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_NoteUpdatedBy { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_NoteUpdatedBy {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_NoteUpdatedBy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_NoteUpdatedBy {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_NoteUpdatedBy {
    pub fn build(self) -> SecurityhubInsight_Filters_NoteUpdatedBy {
        SecurityhubInsight_Filters_NoteUpdatedBy {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_NoteUpdatedByRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_NoteUpdatedByRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_NoteUpdatedByRef {
        SecurityhubInsight_Filters_NoteUpdatedByRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_NoteUpdatedByRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_ProcessLaunchedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ProcessLaunchedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_ProcessLaunchedAtDynamic,
}

impl SecurityhubInsight_Filters_ProcessLaunchedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_ProcessLaunchedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessLaunchedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ProcessLaunchedAt {}

impl BuildSecurityhubInsight_Filters_ProcessLaunchedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_ProcessLaunchedAt {
        SecurityhubInsight_Filters_ProcessLaunchedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ProcessLaunchedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ProcessLaunchedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ProcessLaunchedAtRef {
        SecurityhubInsight_Filters_ProcessLaunchedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ProcessLaunchedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ProcessName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ProcessName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ProcessName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ProcessName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ProcessName {
    pub fn build(self) -> SecurityhubInsight_Filters_ProcessName {
        SecurityhubInsight_Filters_ProcessName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ProcessNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ProcessNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ProcessNameRef {
        SecurityhubInsight_Filters_ProcessNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ProcessNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ProcessParentPid {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsight_Filters_ProcessParentPid {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_ProcessParentPid {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessParentPid>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ProcessParentPid {}

impl BuildSecurityhubInsight_Filters_ProcessParentPid {
    pub fn build(self) -> SecurityhubInsight_Filters_ProcessParentPid {
        SecurityhubInsight_Filters_ProcessParentPid {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ProcessParentPidRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ProcessParentPidRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ProcessParentPidRef {
        SecurityhubInsight_Filters_ProcessParentPidRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ProcessParentPidRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ProcessPath {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ProcessPath { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ProcessPath {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessPath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ProcessPath {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ProcessPath {
    pub fn build(self) -> SecurityhubInsight_Filters_ProcessPath {
        SecurityhubInsight_Filters_ProcessPath {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ProcessPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ProcessPathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ProcessPathRef {
        SecurityhubInsight_Filters_ProcessPathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ProcessPathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ProcessPid {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsight_Filters_ProcessPid {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_ProcessPid {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessPid>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ProcessPid {}

impl BuildSecurityhubInsight_Filters_ProcessPid {
    pub fn build(self) -> SecurityhubInsight_Filters_ProcessPid {
        SecurityhubInsight_Filters_ProcessPid {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ProcessPidRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ProcessPidRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ProcessPidRef {
        SecurityhubInsight_Filters_ProcessPidRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ProcessPidRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_ProcessTerminatedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ProcessTerminatedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_ProcessTerminatedAtDynamic,
}

impl SecurityhubInsight_Filters_ProcessTerminatedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_ProcessTerminatedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessTerminatedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ProcessTerminatedAt {}

impl BuildSecurityhubInsight_Filters_ProcessTerminatedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_ProcessTerminatedAt {
        SecurityhubInsight_Filters_ProcessTerminatedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ProcessTerminatedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ProcessTerminatedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ProcessTerminatedAtRef {
        SecurityhubInsight_Filters_ProcessTerminatedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ProcessTerminatedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ProductArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ProductArn { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ProductArn {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ProductArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ProductArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ProductArn {
    pub fn build(self) -> SecurityhubInsight_Filters_ProductArn {
        SecurityhubInsight_Filters_ProductArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ProductArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ProductArnRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ProductArnRef {
        SecurityhubInsight_Filters_ProductArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ProductArnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ProductFields {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ProductFields { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ProductFields {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ProductFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ProductFields {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ProductFields {
    pub fn build(self) -> SecurityhubInsight_Filters_ProductFields {
        SecurityhubInsight_Filters_ProductFields {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ProductFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ProductFieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ProductFieldsRef {
        SecurityhubInsight_Filters_ProductFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ProductFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ProductName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ProductName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ProductName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ProductName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ProductName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ProductName {
    pub fn build(self) -> SecurityhubInsight_Filters_ProductName {
        SecurityhubInsight_Filters_ProductName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ProductNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ProductNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ProductNameRef {
        SecurityhubInsight_Filters_ProductNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ProductNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_RecommendationText {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_RecommendationText { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_RecommendationText {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_RecommendationText>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_RecommendationText {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_RecommendationText {
    pub fn build(self) -> SecurityhubInsight_Filters_RecommendationText {
        SecurityhubInsight_Filters_RecommendationText {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_RecommendationTextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_RecommendationTextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_RecommendationTextRef {
        SecurityhubInsight_Filters_RecommendationTextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_RecommendationTextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_RecordState {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_RecordState { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_RecordState {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_RecordState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_RecordState {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_RecordState {
    pub fn build(self) -> SecurityhubInsight_Filters_RecordState {
        SecurityhubInsight_Filters_RecordState {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_RecordStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_RecordStateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_RecordStateRef {
        SecurityhubInsight_Filters_RecordStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_RecordStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_RelatedFindingsId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_RelatedFindingsId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_RelatedFindingsId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_RelatedFindingsId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_RelatedFindingsId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_RelatedFindingsId {
    pub fn build(self) -> SecurityhubInsight_Filters_RelatedFindingsId {
        SecurityhubInsight_Filters_RelatedFindingsId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_RelatedFindingsIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_RelatedFindingsIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_RelatedFindingsIdRef {
        SecurityhubInsight_Filters_RelatedFindingsIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_RelatedFindingsIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_RelatedFindingsProductArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_RelatedFindingsProductArn { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_RelatedFindingsProductArn {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_RelatedFindingsProductArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_RelatedFindingsProductArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_RelatedFindingsProductArn {
    pub fn build(self) -> SecurityhubInsight_Filters_RelatedFindingsProductArn {
        SecurityhubInsight_Filters_RelatedFindingsProductArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_RelatedFindingsProductArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_RelatedFindingsProductArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_RelatedFindingsProductArnRef {
        SecurityhubInsight_Filters_RelatedFindingsProductArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_RelatedFindingsProductArnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArnRef {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageIdRef {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses { cidr: self.cidr }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4AddressesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4AddressesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4AddressesRef {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4AddressesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4AddressesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses { cidr: self.cidr }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6AddressesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6AddressesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6AddressesRef {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6AddressesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6AddressesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyNameRef {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAtDynamic,
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt {}

impl BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAtRef {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetIdRef {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceType {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceType { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsEc2InstanceType {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceType {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceType {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceType {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceType {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsEc2InstanceTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceTypeRef {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcIdRef {
        SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAtDynamic,
}

impl SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt {}

impl BuildSecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt {
        SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAtRef {
        SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus {
        SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatusRef {
        SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName {
        SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserNameRef {
        SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId {
        SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerIdRef {
        SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName {
        SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerNameRef {
        SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceContainerImageId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceContainerImageId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceContainerImageId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceContainerImageId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceContainerImageId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceContainerImageId {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceContainerImageId {
        SecurityhubInsight_Filters_ResourceContainerImageId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceContainerImageIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceContainerImageIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceContainerImageIdRef {
        SecurityhubInsight_Filters_ResourceContainerImageIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceContainerImageIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceContainerImageName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceContainerImageName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceContainerImageName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceContainerImageName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceContainerImageName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceContainerImageName {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceContainerImageName {
        SecurityhubInsight_Filters_ResourceContainerImageName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceContainerImageNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceContainerImageNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceContainerImageNameRef {
        SecurityhubInsight_Filters_ResourceContainerImageNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceContainerImageNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_ResourceContainerLaunchedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceContainerLaunchedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_ResourceContainerLaunchedAtDynamic,
}

impl SecurityhubInsight_Filters_ResourceContainerLaunchedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceContainerLaunchedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceContainerLaunchedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceContainerLaunchedAt {}

impl BuildSecurityhubInsight_Filters_ResourceContainerLaunchedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceContainerLaunchedAt {
        SecurityhubInsight_Filters_ResourceContainerLaunchedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceContainerLaunchedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceContainerLaunchedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceContainerLaunchedAtRef {
        SecurityhubInsight_Filters_ResourceContainerLaunchedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceContainerLaunchedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceContainerName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceContainerName { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceContainerName {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceContainerName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceContainerName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceContainerName {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceContainerName {
        SecurityhubInsight_Filters_ResourceContainerName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceContainerNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceContainerNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceContainerNameRef {
        SecurityhubInsight_Filters_ResourceContainerNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceContainerNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceDetailsOther {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceDetailsOther { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceDetailsOther {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceDetailsOther>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceDetailsOther {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceDetailsOther {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceDetailsOther {
        SecurityhubInsight_Filters_ResourceDetailsOther {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceDetailsOtherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceDetailsOtherRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourceDetailsOtherRef {
        SecurityhubInsight_Filters_ResourceDetailsOtherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceDetailsOtherRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceId { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceId {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceId {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceId {
        SecurityhubInsight_Filters_ResourceId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceIdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ResourceIdRef {
        SecurityhubInsight_Filters_ResourceIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourcePartition {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourcePartition { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourcePartition {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourcePartition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourcePartition {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourcePartition {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourcePartition {
        SecurityhubInsight_Filters_ResourcePartition {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourcePartitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourcePartitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ResourcePartitionRef {
        SecurityhubInsight_Filters_ResourcePartitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourcePartitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceRegion {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceRegion { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceRegion {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceRegion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceRegion {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceRegion {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceRegion {
        SecurityhubInsight_Filters_ResourceRegion {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceRegionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceRegionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ResourceRegionRef {
        SecurityhubInsight_Filters_ResourceRegionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceRegionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceTags {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceTags { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceTags {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceTags {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceTags {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceTags {
        SecurityhubInsight_Filters_ResourceTags {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceTagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ResourceTagsRef {
        SecurityhubInsight_Filters_ResourceTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceTagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ResourceType {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ResourceType { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ResourceType {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ResourceType {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ResourceType {
    pub fn build(self) -> SecurityhubInsight_Filters_ResourceType {
        SecurityhubInsight_Filters_ResourceType {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ResourceTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ResourceTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_ResourceTypeRef {
        SecurityhubInsight_Filters_ResourceTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ResourceTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_SeverityLabel {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_SeverityLabel { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_SeverityLabel {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_SeverityLabel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_SeverityLabel {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_SeverityLabel {
    pub fn build(self) -> SecurityhubInsight_Filters_SeverityLabel {
        SecurityhubInsight_Filters_SeverityLabel {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_SeverityLabelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_SeverityLabelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_SeverityLabelRef {
        SecurityhubInsight_Filters_SeverityLabelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_SeverityLabelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_SourceUrl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_SourceUrl { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_SourceUrl {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_SourceUrl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_SourceUrl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_SourceUrl {
    pub fn build(self) -> SecurityhubInsight_Filters_SourceUrl {
        SecurityhubInsight_Filters_SourceUrl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_SourceUrlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_SourceUrlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_SourceUrlRef {
        SecurityhubInsight_Filters_SourceUrlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_SourceUrlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorCategory {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorCategory { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ThreatIntelIndicatorCategory {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorCategory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ThreatIntelIndicatorCategory {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ThreatIntelIndicatorCategory {
    pub fn build(self) -> SecurityhubInsight_Filters_ThreatIntelIndicatorCategory {
        SecurityhubInsight_Filters_ThreatIntelIndicatorCategory {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorCategoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ThreatIntelIndicatorCategoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ThreatIntelIndicatorCategoryRef {
        SecurityhubInsight_Filters_ThreatIntelIndicatorCategoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorCategoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAtDynamic,
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt {}

impl BuildSecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt {
        SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAtRef {
        SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorSource {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorSource { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ThreatIntelIndicatorSource {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ThreatIntelIndicatorSource {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ThreatIntelIndicatorSource {
    pub fn build(self) -> SecurityhubInsight_Filters_ThreatIntelIndicatorSource {
        SecurityhubInsight_Filters_ThreatIntelIndicatorSource {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ThreatIntelIndicatorSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ThreatIntelIndicatorSourceRef {
        SecurityhubInsight_Filters_ThreatIntelIndicatorSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl {
    pub fn build(self) -> SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl {
        SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrlRef {
        SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorType {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorType { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ThreatIntelIndicatorType {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ThreatIntelIndicatorType {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ThreatIntelIndicatorType {
    pub fn build(self) -> SecurityhubInsight_Filters_ThreatIntelIndicatorType {
        SecurityhubInsight_Filters_ThreatIntelIndicatorType {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ThreatIntelIndicatorTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ThreatIntelIndicatorTypeRef {
        SecurityhubInsight_Filters_ThreatIntelIndicatorTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorValue {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorValue { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_ThreatIntelIndicatorValue {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_ThreatIntelIndicatorValue {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_ThreatIntelIndicatorValue {
    pub fn build(self) -> SecurityhubInsight_Filters_ThreatIntelIndicatorValue {
        SecurityhubInsight_Filters_ThreatIntelIndicatorValue {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_ThreatIntelIndicatorValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_ThreatIntelIndicatorValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_ThreatIntelIndicatorValueRef {
        SecurityhubInsight_Filters_ThreatIntelIndicatorValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_ThreatIntelIndicatorValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_Title {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_Title { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_Title {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_Title>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_Title {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_Title {
    pub fn build(self) -> SecurityhubInsight_Filters_Title {
        SecurityhubInsight_Filters_Title {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_TitleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_TitleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_TitleRef {
        SecurityhubInsight_Filters_TitleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_TitleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_Type {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_Type { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_Type {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_Type>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_Type {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_Type {
    pub fn build(self) -> SecurityhubInsight_Filters_Type {
        SecurityhubInsight_Filters_Type {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_TypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_TypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_TypeRef {
        SecurityhubInsight_Filters_TypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_TypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_Filters_UpdatedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_UpdatedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    dynamic: SecurityhubInsight_Filters_UpdatedAtDynamic,
}

impl SecurityhubInsight_Filters_UpdatedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters_UpdatedAt {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_UpdatedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_UpdatedAt {}

impl BuildSecurityhubInsight_Filters_UpdatedAt {
    pub fn build(self) -> SecurityhubInsight_Filters_UpdatedAt {
        SecurityhubInsight_Filters_UpdatedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_UpdatedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_UpdatedAtRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_UpdatedAtRef {
        SecurityhubInsight_Filters_UpdatedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_UpdatedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsight_Filters_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_UserDefinedValues {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_UserDefinedValues { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_UserDefinedValues {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_UserDefinedValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_UserDefinedValues {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_UserDefinedValues {
    pub fn build(self) -> SecurityhubInsight_Filters_UserDefinedValues {
        SecurityhubInsight_Filters_UserDefinedValues {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_UserDefinedValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_UserDefinedValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_UserDefinedValuesRef {
        SecurityhubInsight_Filters_UserDefinedValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_UserDefinedValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_VerificationState {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_VerificationState { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_VerificationState {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_VerificationState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_VerificationState {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_VerificationState {
    pub fn build(self) -> SecurityhubInsight_Filters_VerificationState {
        SecurityhubInsight_Filters_VerificationState {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_VerificationStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_VerificationStateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsight_Filters_VerificationStateRef {
        SecurityhubInsight_Filters_VerificationStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_VerificationStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters_WorkflowStatus {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsight_Filters_WorkflowStatus { }

impl terrars::ToListMappable for SecurityhubInsight_Filters_WorkflowStatus {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters_WorkflowStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters_WorkflowStatus {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight_Filters_WorkflowStatus {
    pub fn build(self) -> SecurityhubInsight_Filters_WorkflowStatus {
        SecurityhubInsight_Filters_WorkflowStatus {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_Filters_WorkflowStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_Filters_WorkflowStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_Filters_WorkflowStatusRef {
        SecurityhubInsight_Filters_WorkflowStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_Filters_WorkflowStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsight_FiltersDynamic {
    aws_account_id: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_AwsAccountId>>,
    aws_account_name: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_AwsAccountName>>,
    company_name: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CompanyName>>,
    compliance_associated_standards_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ComplianceAssociatedStandardsId>>,
    compliance_security_control_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ComplianceSecurityControlId>>,
    compliance_security_control_parameters_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ComplianceSecurityControlParametersName>>,
    compliance_security_control_parameters_value: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ComplianceSecurityControlParametersValue>>,
    compliance_status: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ComplianceStatus>>,
    confidence: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_Confidence>>,
    created_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_CreatedAt>>,
    criticality: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_Criticality>>,
    description: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_Description>>,
    finding_provider_fields_confidence: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_FindingProviderFieldsConfidence>>,
    finding_provider_fields_criticality: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_FindingProviderFieldsCriticality>>,
    finding_provider_fields_related_findings_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId>>,
    finding_provider_fields_related_findings_product_arn: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn>>,
    finding_provider_fields_severity_label: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel>>,
    finding_provider_fields_severity_original: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal>>,
    finding_provider_fields_types: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_FindingProviderFieldsTypes>>,
    first_observed_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_FirstObservedAt>>,
    generator_id: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_GeneratorId>>,
    id: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_Id>>,
    keyword: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_Keyword>>,
    last_observed_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_LastObservedAt>>,
    malware_name: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_MalwareName>>,
    malware_path: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_MalwarePath>>,
    malware_state: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_MalwareState>>,
    malware_type: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_MalwareType>>,
    network_destination_domain: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkDestinationDomain>>,
    network_destination_ipv4: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkDestinationIpv4>>,
    network_destination_ipv6: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkDestinationIpv6>>,
    network_destination_port: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkDestinationPort>>,
    network_direction: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkDirection>>,
    network_protocol: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkProtocol>>,
    network_source_domain: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkSourceDomain>>,
    network_source_ipv4: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkSourceIpv4>>,
    network_source_ipv6: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkSourceIpv6>>,
    network_source_mac: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkSourceMac>>,
    network_source_port: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NetworkSourcePort>>,
    note_text: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NoteText>>,
    note_updated_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NoteUpdatedAt>>,
    note_updated_by: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_NoteUpdatedBy>>,
    process_launched_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ProcessLaunchedAt>>,
    process_name: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ProcessName>>,
    process_parent_pid: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ProcessParentPid>>,
    process_path: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ProcessPath>>,
    process_pid: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ProcessPid>>,
    process_terminated_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ProcessTerminatedAt>>,
    product_arn: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ProductArn>>,
    product_fields: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ProductFields>>,
    product_name: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ProductName>>,
    recommendation_text: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_RecommendationText>>,
    record_state: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_RecordState>>,
    related_findings_id: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_RelatedFindingsId>>,
    related_findings_product_arn: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_RelatedFindingsProductArn>>,
    resource_aws_ec2_instance_iam_instance_profile_arn: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn>>,
    resource_aws_ec2_instance_image_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId>>,
    resource_aws_ec2_instance_ipv4_addresses: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses>>,
    resource_aws_ec2_instance_ipv6_addresses: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses>>,
    resource_aws_ec2_instance_key_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName>>,
    resource_aws_ec2_instance_launched_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt>>,
    resource_aws_ec2_instance_subnet_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId>>,
    resource_aws_ec2_instance_type: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsEc2InstanceType>>,
    resource_aws_ec2_instance_vpc_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId>>,
    resource_aws_iam_access_key_created_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt>>,
    resource_aws_iam_access_key_status: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus>>,
    resource_aws_iam_access_key_user_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName>>,
    resource_aws_s3_bucket_owner_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId>>,
    resource_aws_s3_bucket_owner_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName>>,
    resource_container_image_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceContainerImageId>>,
    resource_container_image_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceContainerImageName>>,
    resource_container_launched_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceContainerLaunchedAt>>,
    resource_container_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceContainerName>>,
    resource_details_other: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceDetailsOther>>,
    resource_id: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceId>>,
    resource_partition: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourcePartition>>,
    resource_region: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceRegion>>,
    resource_tags: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceTags>>,
    resource_type: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ResourceType>>,
    severity_label: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_SeverityLabel>>,
    source_url: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_SourceUrl>>,
    threat_intel_indicator_category: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ThreatIntelIndicatorCategory>>,
    threat_intel_indicator_last_observed_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt>>,
    threat_intel_indicator_source: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ThreatIntelIndicatorSource>>,
    threat_intel_indicator_source_url: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl>>,
    threat_intel_indicator_type: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ThreatIntelIndicatorType>>,
    threat_intel_indicator_value: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_ThreatIntelIndicatorValue>>,
    title: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_Title>>,
    type_: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_Type>>,
    updated_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_UpdatedAt>>,
    user_defined_values: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_UserDefinedValues>>,
    verification_state: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_VerificationState>>,
    workflow_status: std::option::Option<terrars::DynamicBlock<SecurityhubInsight_Filters_WorkflowStatus>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsight_Filters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_id: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_AwsAccountId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_name: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_AwsAccountName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_name: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CompanyName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_associated_standards_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ComplianceAssociatedStandardsId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_security_control_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ComplianceSecurityControlId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_security_control_parameters_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ComplianceSecurityControlParametersName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_security_control_parameters_value: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ComplianceSecurityControlParametersValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_status: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ComplianceStatus>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_Confidence>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_CreatedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criticality: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_Criticality>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_Description>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_confidence: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_FindingProviderFieldsConfidence>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_criticality: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_FindingProviderFieldsCriticality>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_related_findings_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_related_findings_product_arn: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_severity_label: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_severity_original: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_types: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_FindingProviderFieldsTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_observed_at: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_FirstObservedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generator_id: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_GeneratorId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_Id>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyword: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_Keyword>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_observed_at: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_LastObservedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_name: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_MalwareName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_path: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_MalwarePath>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_state: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_MalwareState>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_type: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_MalwareType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_destination_domain: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkDestinationDomain>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_destination_ipv4: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkDestinationIpv4>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_destination_ipv6: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkDestinationIpv6>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_destination_port: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkDestinationPort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_direction: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkDirection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_protocol: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkProtocol>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_domain: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkSourceDomain>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_ipv4: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkSourceIpv4>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_ipv6: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkSourceIpv6>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_mac: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkSourceMac>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_port: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NetworkSourcePort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note_text: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NoteText>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note_updated_at: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NoteUpdatedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note_updated_by: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_NoteUpdatedBy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_launched_at: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ProcessLaunchedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_name: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ProcessName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_parent_pid: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ProcessParentPid>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_path: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ProcessPath>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_pid: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ProcessPid>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_terminated_at: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ProcessTerminatedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_arn: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ProductArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_fields: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ProductFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_name: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ProductName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recommendation_text: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_RecommendationText>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_state: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_RecordState>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_findings_id: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_RelatedFindingsId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_findings_product_arn: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_RelatedFindingsProductArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_iam_instance_profile_arn: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_image_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_ipv4_addresses: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_ipv6_addresses: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_key_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_launched_at: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_subnet_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_type: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsEc2InstanceType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_vpc_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_iam_access_key_created_at: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_iam_access_key_status: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_iam_access_key_user_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_s3_bucket_owner_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_s3_bucket_owner_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_container_image_id: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceContainerImageId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_container_image_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceContainerImageName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_container_launched_at: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceContainerLaunchedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_container_name: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceContainerName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_details_other: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceDetailsOther>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_id: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_partition: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourcePartition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_region: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceRegion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceTags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ResourceType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity_label: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_SeverityLabel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_url: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_SourceUrl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_category: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ThreatIntelIndicatorCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_last_observed_at: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_source: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ThreatIntelIndicatorSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_source_url: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_type: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_ThreatIntelIndicatorType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_value: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsight_Filters_ThreatIntelIndicatorValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_Title>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_Type>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_UpdatedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_defined_values: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_UserDefinedValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_state: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_VerificationState>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow_status: std::option::Option<std::vec::Vec<SecurityhubInsight_Filters_WorkflowStatus>>,
    dynamic: SecurityhubInsight_FiltersDynamic,
}

impl SecurityhubInsight_Filters {
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_AwsAccountId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_account_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_account_name`.\n"]
    pub fn set_aws_account_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_AwsAccountName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_account_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `company_name`.\n"]
    pub fn set_company_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CompanyName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.company_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.company_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_associated_standards_id`.\n"]
    pub fn set_compliance_associated_standards_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceAssociatedStandardsId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_associated_standards_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_associated_standards_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_security_control_id`.\n"]
    pub fn set_compliance_security_control_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceSecurityControlId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_security_control_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_security_control_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_security_control_parameters_name`.\n"]
    pub fn set_compliance_security_control_parameters_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceSecurityControlParametersName>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_security_control_parameters_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_security_control_parameters_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_security_control_parameters_value`.\n"]
    pub fn set_compliance_security_control_parameters_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceSecurityControlParametersValue>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_security_control_parameters_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_security_control_parameters_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_status`.\n"]
    pub fn set_compliance_status(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ComplianceStatus>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confidence`.\n"]
    pub fn set_confidence(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_Confidence>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confidence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confidence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_CreatedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.created_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.created_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_Criticality>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.criticality = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.criticality = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_Description>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.description = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.description = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_confidence`.\n"]
    pub fn set_finding_provider_fields_confidence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsConfidence>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_confidence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_confidence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_criticality`.\n"]
    pub fn set_finding_provider_fields_criticality(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsCriticality>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_criticality = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_criticality = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_related_findings_id`.\n"]
    pub fn set_finding_provider_fields_related_findings_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsId>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_related_findings_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_related_findings_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_related_findings_product_arn`.\n"]
    pub fn set_finding_provider_fields_related_findings_product_arn(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsRelatedFindingsProductArn>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_related_findings_product_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_related_findings_product_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_severity_label`.\n"]
    pub fn set_finding_provider_fields_severity_label(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsSeverityLabel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_severity_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_severity_label = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_severity_original`.\n"]
    pub fn set_finding_provider_fields_severity_original(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsSeverityOriginal>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_severity_original = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_severity_original = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_types`.\n"]
    pub fn set_finding_provider_fields_types(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_FindingProviderFieldsTypes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `first_observed_at`.\n"]
    pub fn set_first_observed_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_FirstObservedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.first_observed_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.first_observed_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generator_id`.\n"]
    pub fn set_generator_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_GeneratorId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generator_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generator_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_Id>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `keyword`.\n"]
    pub fn set_keyword(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_Keyword>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.keyword = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.keyword = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `last_observed_at`.\n"]
    pub fn set_last_observed_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_LastObservedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.last_observed_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.last_observed_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `malware_name`.\n"]
    pub fn set_malware_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_MalwareName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malware_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `malware_path`.\n"]
    pub fn set_malware_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_MalwarePath>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malware_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `malware_state`.\n"]
    pub fn set_malware_state(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_MalwareState>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malware_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `malware_type`.\n"]
    pub fn set_malware_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_MalwareType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malware_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_destination_domain`.\n"]
    pub fn set_network_destination_domain(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDestinationDomain>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_destination_domain = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_domain = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_destination_ipv4`.\n"]
    pub fn set_network_destination_ipv4(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDestinationIpv4>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_destination_ipv4 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_ipv4 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_destination_ipv6`.\n"]
    pub fn set_network_destination_ipv6(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDestinationIpv6>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_destination_ipv6 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_ipv6 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_destination_port`.\n"]
    pub fn set_network_destination_port(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDestinationPort>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_destination_port = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_port = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_direction`.\n"]
    pub fn set_network_direction(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkDirection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_direction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_direction = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_protocol`.\n"]
    pub fn set_network_protocol(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkProtocol>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_protocol = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_protocol = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_domain`.\n"]
    pub fn set_network_source_domain(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourceDomain>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_domain = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_domain = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_ipv4`.\n"]
    pub fn set_network_source_ipv4(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourceIpv4>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_ipv4 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_ipv4 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_ipv6`.\n"]
    pub fn set_network_source_ipv6(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourceIpv6>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_ipv6 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_ipv6 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_mac`.\n"]
    pub fn set_network_source_mac(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourceMac>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_mac = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_mac = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_port`.\n"]
    pub fn set_network_source_port(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NetworkSourcePort>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_port = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_port = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `note_text`.\n"]
    pub fn set_note_text(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NoteText>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `note_updated_at`.\n"]
    pub fn set_note_updated_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NoteUpdatedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note_updated_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `note_updated_by`.\n"]
    pub fn set_note_updated_by(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_NoteUpdatedBy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note_updated_by = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_by = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_launched_at`.\n"]
    pub fn set_process_launched_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessLaunchedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_launched_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_launched_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_name`.\n"]
    pub fn set_process_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_parent_pid`.\n"]
    pub fn set_process_parent_pid(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessParentPid>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_parent_pid = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_parent_pid = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_path`.\n"]
    pub fn set_process_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessPath>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_pid`.\n"]
    pub fn set_process_pid(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessPid>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_pid = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_pid = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_terminated_at`.\n"]
    pub fn set_process_terminated_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ProcessTerminatedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_terminated_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_terminated_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `product_arn`.\n"]
    pub fn set_product_arn(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ProductArn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.product_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.product_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `product_fields`.\n"]
    pub fn set_product_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ProductFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.product_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.product_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `product_name`.\n"]
    pub fn set_product_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ProductName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.product_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.product_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recommendation_text`.\n"]
    pub fn set_recommendation_text(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_RecommendationText>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recommendation_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recommendation_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_state`.\n"]
    pub fn set_record_state(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_RecordState>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `related_findings_id`.\n"]
    pub fn set_related_findings_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_RelatedFindingsId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.related_findings_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `related_findings_product_arn`.\n"]
    pub fn set_related_findings_product_arn(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_RelatedFindingsProductArn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.related_findings_product_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_product_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_iam_instance_profile_arn`.\n"]
    pub fn set_resource_aws_ec2_instance_iam_instance_profile_arn(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIamInstanceProfileArn>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_iam_instance_profile_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_iam_instance_profile_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_image_id`.\n"]
    pub fn set_resource_aws_ec2_instance_image_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceImageId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_image_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_image_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_ipv4_addresses`.\n"]
    pub fn set_resource_aws_ec2_instance_ipv4_addresses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv4Addresses>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_ipv4_addresses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_ipv4_addresses = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_ipv6_addresses`.\n"]
    pub fn set_resource_aws_ec2_instance_ipv6_addresses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceIpv6Addresses>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_ipv6_addresses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_ipv6_addresses = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_key_name`.\n"]
    pub fn set_resource_aws_ec2_instance_key_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceKeyName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_key_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_key_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_launched_at`.\n"]
    pub fn set_resource_aws_ec2_instance_launched_at(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceLaunchedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_launched_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_launched_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_subnet_id`.\n"]
    pub fn set_resource_aws_ec2_instance_subnet_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceSubnetId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_subnet_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_subnet_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_type`.\n"]
    pub fn set_resource_aws_ec2_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_vpc_id`.\n"]
    pub fn set_resource_aws_ec2_instance_vpc_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsEc2InstanceVpcId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_vpc_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_vpc_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_iam_access_key_created_at`.\n"]
    pub fn set_resource_aws_iam_access_key_created_at(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyCreatedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_iam_access_key_created_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_iam_access_key_created_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_iam_access_key_status`.\n"]
    pub fn set_resource_aws_iam_access_key_status(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyStatus>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_iam_access_key_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_iam_access_key_status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_iam_access_key_user_name`.\n"]
    pub fn set_resource_aws_iam_access_key_user_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsIamAccessKeyUserName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_iam_access_key_user_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_iam_access_key_user_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_s3_bucket_owner_id`.\n"]
    pub fn set_resource_aws_s3_bucket_owner_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_s3_bucket_owner_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_s3_bucket_owner_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_s3_bucket_owner_name`.\n"]
    pub fn set_resource_aws_s3_bucket_owner_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceAwsS3BucketOwnerName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_s3_bucket_owner_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_s3_bucket_owner_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_container_image_id`.\n"]
    pub fn set_resource_container_image_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceContainerImageId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_container_image_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_image_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_container_image_name`.\n"]
    pub fn set_resource_container_image_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceContainerImageName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_container_image_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_image_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_container_launched_at`.\n"]
    pub fn set_resource_container_launched_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceContainerLaunchedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_container_launched_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_launched_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_container_name`.\n"]
    pub fn set_resource_container_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceContainerName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_container_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_details_other`.\n"]
    pub fn set_resource_details_other(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceDetailsOther>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_details_other = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_details_other = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_partition`.\n"]
    pub fn set_resource_partition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourcePartition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_partition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_partition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_region`.\n"]
    pub fn set_resource_region(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceRegion>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_region = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_region = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceTags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ResourceType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `severity_label`.\n"]
    pub fn set_severity_label(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_SeverityLabel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.severity_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.severity_label = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_url`.\n"]
    pub fn set_source_url(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_SourceUrl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_url = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_url = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_category`.\n"]
    pub fn set_threat_intel_indicator_category(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorCategory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_last_observed_at`.\n"]
    pub fn set_threat_intel_indicator_last_observed_at(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorLastObservedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_last_observed_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_last_observed_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_source`.\n"]
    pub fn set_threat_intel_indicator_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_source_url`.\n"]
    pub fn set_threat_intel_indicator_source_url(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorSourceUrl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_source_url = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_source_url = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_type`.\n"]
    pub fn set_threat_intel_indicator_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_value`.\n"]
    pub fn set_threat_intel_indicator_value(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_ThreatIntelIndicatorValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_Title>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.title = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.title = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_Type>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.type_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.type_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_UpdatedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.updated_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.updated_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_defined_values`.\n"]
    pub fn set_user_defined_values(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_UserDefinedValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_defined_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verification_state`.\n"]
    pub fn set_verification_state(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_VerificationState>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.verification_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.verification_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workflow_status`.\n"]
    pub fn set_workflow_status(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsight_Filters_WorkflowStatus>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workflow_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workflow_status = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsight_Filters {
    type O = terrars::BlockAssignable<SecurityhubInsight_Filters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsight_Filters {}

impl BuildSecurityhubInsight_Filters {
    pub fn build(self) -> SecurityhubInsight_Filters {
        SecurityhubInsight_Filters {
            aws_account_id: core::default::Default::default(),
            aws_account_name: core::default::Default::default(),
            company_name: core::default::Default::default(),
            compliance_associated_standards_id: core::default::Default::default(),
            compliance_security_control_id: core::default::Default::default(),
            compliance_security_control_parameters_name: core::default::Default::default(),
            compliance_security_control_parameters_value: core::default::Default::default(),
            compliance_status: core::default::Default::default(),
            confidence: core::default::Default::default(),
            created_at: core::default::Default::default(),
            criticality: core::default::Default::default(),
            description: core::default::Default::default(),
            finding_provider_fields_confidence: core::default::Default::default(),
            finding_provider_fields_criticality: core::default::Default::default(),
            finding_provider_fields_related_findings_id: core::default::Default::default(),
            finding_provider_fields_related_findings_product_arn: core::default::Default::default(),
            finding_provider_fields_severity_label: core::default::Default::default(),
            finding_provider_fields_severity_original: core::default::Default::default(),
            finding_provider_fields_types: core::default::Default::default(),
            first_observed_at: core::default::Default::default(),
            generator_id: core::default::Default::default(),
            id: core::default::Default::default(),
            keyword: core::default::Default::default(),
            last_observed_at: core::default::Default::default(),
            malware_name: core::default::Default::default(),
            malware_path: core::default::Default::default(),
            malware_state: core::default::Default::default(),
            malware_type: core::default::Default::default(),
            network_destination_domain: core::default::Default::default(),
            network_destination_ipv4: core::default::Default::default(),
            network_destination_ipv6: core::default::Default::default(),
            network_destination_port: core::default::Default::default(),
            network_direction: core::default::Default::default(),
            network_protocol: core::default::Default::default(),
            network_source_domain: core::default::Default::default(),
            network_source_ipv4: core::default::Default::default(),
            network_source_ipv6: core::default::Default::default(),
            network_source_mac: core::default::Default::default(),
            network_source_port: core::default::Default::default(),
            note_text: core::default::Default::default(),
            note_updated_at: core::default::Default::default(),
            note_updated_by: core::default::Default::default(),
            process_launched_at: core::default::Default::default(),
            process_name: core::default::Default::default(),
            process_parent_pid: core::default::Default::default(),
            process_path: core::default::Default::default(),
            process_pid: core::default::Default::default(),
            process_terminated_at: core::default::Default::default(),
            product_arn: core::default::Default::default(),
            product_fields: core::default::Default::default(),
            product_name: core::default::Default::default(),
            recommendation_text: core::default::Default::default(),
            record_state: core::default::Default::default(),
            related_findings_id: core::default::Default::default(),
            related_findings_product_arn: core::default::Default::default(),
            resource_aws_ec2_instance_iam_instance_profile_arn: core::default::Default::default(),
            resource_aws_ec2_instance_image_id: core::default::Default::default(),
            resource_aws_ec2_instance_ipv4_addresses: core::default::Default::default(),
            resource_aws_ec2_instance_ipv6_addresses: core::default::Default::default(),
            resource_aws_ec2_instance_key_name: core::default::Default::default(),
            resource_aws_ec2_instance_launched_at: core::default::Default::default(),
            resource_aws_ec2_instance_subnet_id: core::default::Default::default(),
            resource_aws_ec2_instance_type: core::default::Default::default(),
            resource_aws_ec2_instance_vpc_id: core::default::Default::default(),
            resource_aws_iam_access_key_created_at: core::default::Default::default(),
            resource_aws_iam_access_key_status: core::default::Default::default(),
            resource_aws_iam_access_key_user_name: core::default::Default::default(),
            resource_aws_s3_bucket_owner_id: core::default::Default::default(),
            resource_aws_s3_bucket_owner_name: core::default::Default::default(),
            resource_container_image_id: core::default::Default::default(),
            resource_container_image_name: core::default::Default::default(),
            resource_container_launched_at: core::default::Default::default(),
            resource_container_name: core::default::Default::default(),
            resource_details_other: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_partition: core::default::Default::default(),
            resource_region: core::default::Default::default(),
            resource_tags: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            severity_label: core::default::Default::default(),
            source_url: core::default::Default::default(),
            threat_intel_indicator_category: core::default::Default::default(),
            threat_intel_indicator_last_observed_at: core::default::Default::default(),
            threat_intel_indicator_source: core::default::Default::default(),
            threat_intel_indicator_source_url: core::default::Default::default(),
            threat_intel_indicator_type: core::default::Default::default(),
            threat_intel_indicator_value: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            user_defined_values: core::default::Default::default(),
            verification_state: core::default::Default::default(),
            workflow_status: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubInsight_FiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsight_FiltersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsight_FiltersRef {
        SecurityhubInsight_FiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsight_FiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

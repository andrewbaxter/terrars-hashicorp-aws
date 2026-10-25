#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatazonePolicyGrantData_ {
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
    domain_identifier: terrars::PrimField<std::string::String>,
    entity_identifier: terrars::PrimField<std::string::String>,
    entity_type: terrars::PrimField<std::string::String>,
    policy_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detail: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Principal>>,
    dynamic: DatazonePolicyGrantDynamic,
}

struct DatazonePolicyGrant_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatazonePolicyGrantData_>,
}

#[derive(Clone)]
pub struct DatazonePolicyGrant(std::rc::Rc<DatazonePolicyGrant_>);

impl DatazonePolicyGrant {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detail`.\n"]
    pub fn set_detail(self, v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().detail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.detail = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Principal>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_identifier` after provisioning.\n"]
    pub fn entity_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_type` after provisioning.\n"]
    pub fn entity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_id` after provisioning.\n"]
    pub fn grant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detail` after provisioning.\n"]
    pub fn detail(&self) -> terrars::ListRef<DatazonePolicyGrant_DetailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.detail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<DatazonePolicyGrant_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }
}

impl terrars::Referable for DatazonePolicyGrant {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatazonePolicyGrant { }

impl terrars::ToListMappable for DatazonePolicyGrant {
    type O = terrars::ListRef<DatazonePolicyGrantRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatazonePolicyGrant_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_datazone_policy_grant".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatazonePolicyGrant {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub entity_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub entity_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub policy_type: terrars::PrimField<std::string::String>,
}

impl BuildDatazonePolicyGrant {
    pub fn build(self, stack: &mut terrars::Stack) -> DatazonePolicyGrant {
        let out = DatazonePolicyGrant(std::rc::Rc::new(DatazonePolicyGrant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatazonePolicyGrantData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain_identifier: self.domain_identifier,
                entity_identifier: self.entity_identifier,
                entity_type: self.entity_type,
                policy_type: self.policy_type,
                region: core::default::Default::default(),
                detail: core::default::Default::default(),
                principal: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatazonePolicyGrantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatazonePolicyGrantRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_identifier` after provisioning.\n"]
    pub fn domain_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_identifier` after provisioning.\n"]
    pub fn entity_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_type` after provisioning.\n"]
    pub fn entity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_id` after provisioning.\n"]
    pub fn grant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detail` after provisioning.\n"]
    pub fn detail(&self) -> terrars::ListRef<DatazonePolicyGrant_DetailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.detail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<DatazonePolicyGrant_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DatazonePolicyGrantDynamic {
    detail: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail>>,
    principal: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Principal>>,
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_AddToProjectMemberPool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
}

impl DatazonePolicyGrant_Detail_AddToProjectMemberPool {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_AddToProjectMemberPool {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_AddToProjectMemberPool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_AddToProjectMemberPool {}

impl BuildDatazonePolicyGrant_Detail_AddToProjectMemberPool {
    pub fn build(self) -> DatazonePolicyGrant_Detail_AddToProjectMemberPool {
        DatazonePolicyGrant_Detail_AddToProjectMemberPool {
            include_child_domain_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_AddToProjectMemberPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_AddToProjectMemberPoolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Detail_AddToProjectMemberPoolRef {
        DatazonePolicyGrant_Detail_AddToProjectMemberPoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_AddToProjectMemberPoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_CreateAssetType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
}

impl DatazonePolicyGrant_Detail_CreateAssetType {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_CreateAssetType {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateAssetType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_CreateAssetType {}

impl BuildDatazonePolicyGrant_Detail_CreateAssetType {
    pub fn build(self) -> DatazonePolicyGrant_Detail_CreateAssetType {
        DatazonePolicyGrant_Detail_CreateAssetType { include_child_domain_units: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_CreateAssetTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_CreateAssetTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_Detail_CreateAssetTypeRef {
        DatazonePolicyGrant_Detail_CreateAssetTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_CreateAssetTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_CreateDomainUnit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
}

impl DatazonePolicyGrant_Detail_CreateDomainUnit {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_CreateDomainUnit {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateDomainUnit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_CreateDomainUnit {}

impl BuildDatazonePolicyGrant_Detail_CreateDomainUnit {
    pub fn build(self) -> DatazonePolicyGrant_Detail_CreateDomainUnit {
        DatazonePolicyGrant_Detail_CreateDomainUnit { include_child_domain_units: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_CreateDomainUnitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_CreateDomainUnitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Detail_CreateDomainUnitRef {
        DatazonePolicyGrant_Detail_CreateDomainUnitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_CreateDomainUnitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_CreateEnvironment {}

impl DatazonePolicyGrant_Detail_CreateEnvironment { }

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_CreateEnvironment {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateEnvironment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_CreateEnvironment {}

impl BuildDatazonePolicyGrant_Detail_CreateEnvironment {
    pub fn build(self) -> DatazonePolicyGrant_Detail_CreateEnvironment {
        DatazonePolicyGrant_Detail_CreateEnvironment {}
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_CreateEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_CreateEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Detail_CreateEnvironmentRef {
        DatazonePolicyGrant_Detail_CreateEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_CreateEnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint {}

impl DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint { }

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint {}

impl BuildDatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint {
    pub fn build(self) -> DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint {
        DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint {}
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprintRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprintRef {
        DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_CreateEnvironmentProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_unit_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatazonePolicyGrant_Detail_CreateEnvironmentProfile {
    #[doc = "Set the field `domain_unit_id`.\n"]
    pub fn set_domain_unit_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_unit_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_CreateEnvironmentProfile {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateEnvironmentProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_CreateEnvironmentProfile {}

impl BuildDatazonePolicyGrant_Detail_CreateEnvironmentProfile {
    pub fn build(self) -> DatazonePolicyGrant_Detail_CreateEnvironmentProfile {
        DatazonePolicyGrant_Detail_CreateEnvironmentProfile { domain_unit_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_CreateEnvironmentProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_CreateEnvironmentProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Detail_CreateEnvironmentProfileRef {
        DatazonePolicyGrant_Detail_CreateEnvironmentProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_CreateEnvironmentProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_unit_id` after provisioning.\n"]
    pub fn domain_unit_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_unit_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_CreateFormType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
}

impl DatazonePolicyGrant_Detail_CreateFormType {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_CreateFormType {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateFormType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_CreateFormType {}

impl BuildDatazonePolicyGrant_Detail_CreateFormType {
    pub fn build(self) -> DatazonePolicyGrant_Detail_CreateFormType {
        DatazonePolicyGrant_Detail_CreateFormType { include_child_domain_units: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_CreateFormTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_CreateFormTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_Detail_CreateFormTypeRef {
        DatazonePolicyGrant_Detail_CreateFormTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_CreateFormTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_CreateGlossary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
}

impl DatazonePolicyGrant_Detail_CreateGlossary {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_CreateGlossary {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateGlossary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_CreateGlossary {}

impl BuildDatazonePolicyGrant_Detail_CreateGlossary {
    pub fn build(self) -> DatazonePolicyGrant_Detail_CreateGlossary {
        DatazonePolicyGrant_Detail_CreateGlossary { include_child_domain_units: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_CreateGlossaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_CreateGlossaryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_Detail_CreateGlossaryRef {
        DatazonePolicyGrant_Detail_CreateGlossaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_CreateGlossaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_CreateProject {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
}

impl DatazonePolicyGrant_Detail_CreateProject {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_CreateProject {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateProject>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_CreateProject {}

impl BuildDatazonePolicyGrant_Detail_CreateProject {
    pub fn build(self) -> DatazonePolicyGrant_Detail_CreateProject {
        DatazonePolicyGrant_Detail_CreateProject { include_child_domain_units: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_CreateProjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_CreateProjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_Detail_CreateProjectRef {
        DatazonePolicyGrant_Detail_CreateProjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_CreateProjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_CreateProjectFromProjectProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_profiles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DatazonePolicyGrant_Detail_CreateProjectFromProjectProfile {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_profiles`.\n"]
    pub fn set_project_profiles(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.project_profiles = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_CreateProjectFromProjectProfile {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateProjectFromProjectProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_CreateProjectFromProjectProfile {}

impl BuildDatazonePolicyGrant_Detail_CreateProjectFromProjectProfile {
    pub fn build(self) -> DatazonePolicyGrant_Detail_CreateProjectFromProjectProfile {
        DatazonePolicyGrant_Detail_CreateProjectFromProjectProfile {
            include_child_domain_units: core::default::Default::default(),
            project_profiles: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_CreateProjectFromProjectProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_CreateProjectFromProjectProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Detail_CreateProjectFromProjectProfileRef {
        DatazonePolicyGrant_Detail_CreateProjectFromProjectProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_CreateProjectFromProjectProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }

    #[doc = "Get a reference to the value of field `project_profiles` after provisioning.\n"]
    pub fn project_profiles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_profiles", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile {}

impl DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile { }

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile {}

impl BuildDatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile {
    pub fn build(self) -> DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile {
        DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile {}
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfileRef {
        DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_OverrideDomainUnitOwners {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
}

impl DatazonePolicyGrant_Detail_OverrideDomainUnitOwners {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_OverrideDomainUnitOwners {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_OverrideDomainUnitOwners>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_OverrideDomainUnitOwners {}

impl BuildDatazonePolicyGrant_Detail_OverrideDomainUnitOwners {
    pub fn build(self) -> DatazonePolicyGrant_Detail_OverrideDomainUnitOwners {
        DatazonePolicyGrant_Detail_OverrideDomainUnitOwners {
            include_child_domain_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_OverrideDomainUnitOwnersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_OverrideDomainUnitOwnersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Detail_OverrideDomainUnitOwnersRef {
        DatazonePolicyGrant_Detail_OverrideDomainUnitOwnersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_OverrideDomainUnitOwnersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_OverrideProjectOwners {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
}

impl DatazonePolicyGrant_Detail_OverrideProjectOwners {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_OverrideProjectOwners {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_OverrideProjectOwners>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_OverrideProjectOwners {}

impl BuildDatazonePolicyGrant_Detail_OverrideProjectOwners {
    pub fn build(self) -> DatazonePolicyGrant_Detail_OverrideProjectOwners {
        DatazonePolicyGrant_Detail_OverrideProjectOwners {
            include_child_domain_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_OverrideProjectOwnersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_OverrideProjectOwnersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Detail_OverrideProjectOwnersRef {
        DatazonePolicyGrant_Detail_OverrideProjectOwnersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_OverrideProjectOwnersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail_UseAssetType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_unit_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatazonePolicyGrant_Detail_UseAssetType {
    #[doc = "Set the field `domain_unit_id`.\n"]
    pub fn set_domain_unit_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_unit_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail_UseAssetType {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail_UseAssetType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail_UseAssetType {}

impl BuildDatazonePolicyGrant_Detail_UseAssetType {
    pub fn build(self) -> DatazonePolicyGrant_Detail_UseAssetType {
        DatazonePolicyGrant_Detail_UseAssetType { domain_unit_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Detail_UseAssetTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Detail_UseAssetTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_Detail_UseAssetTypeRef {
        DatazonePolicyGrant_Detail_UseAssetTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Detail_UseAssetTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_unit_id` after provisioning.\n"]
    pub fn domain_unit_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_unit_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatazonePolicyGrant_DetailDynamic {
    add_to_project_member_pool: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_AddToProjectMemberPool>>,
    create_asset_type: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_CreateAssetType>>,
    create_domain_unit: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_CreateDomainUnit>>,
    create_environment: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_CreateEnvironment>>,
    create_environment_from_blueprint: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint>>,
    create_environment_profile: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_CreateEnvironmentProfile>>,
    create_form_type: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_CreateFormType>>,
    create_glossary: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_CreateGlossary>>,
    create_project: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_CreateProject>>,
    create_project_from_project_profile: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_CreateProjectFromProjectProfile>>,
    delegate_create_environment_profile: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile>>,
    override_domain_unit_owners: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_OverrideDomainUnitOwners>>,
    override_project_owners: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_OverrideProjectOwners>>,
    use_asset_type: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Detail_UseAssetType>>,
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Detail {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_to_project_member_pool: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_AddToProjectMemberPool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_asset_type: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_CreateAssetType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_domain_unit: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_CreateDomainUnit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_environment: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_CreateEnvironment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_environment_from_blueprint: std
    ::option
    ::Option<std::vec::Vec<DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_environment_profile: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_CreateEnvironmentProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_form_type: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_CreateFormType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_glossary: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_CreateGlossary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_project: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_CreateProject>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_project_from_project_profile: std
    ::option
    ::Option<std::vec::Vec<DatazonePolicyGrant_Detail_CreateProjectFromProjectProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delegate_create_environment_profile: std
    ::option
    ::Option<std::vec::Vec<DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_domain_unit_owners: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_OverrideDomainUnitOwners>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_project_owners: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_OverrideProjectOwners>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_asset_type: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Detail_UseAssetType>>,
    dynamic: DatazonePolicyGrant_DetailDynamic,
}

impl DatazonePolicyGrant_Detail {
    #[doc = "Set the field `add_to_project_member_pool`.\n"]
    pub fn set_add_to_project_member_pool(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_AddToProjectMemberPool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.add_to_project_member_pool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.add_to_project_member_pool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_asset_type`.\n"]
    pub fn set_create_asset_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateAssetType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_asset_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_asset_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_domain_unit`.\n"]
    pub fn set_create_domain_unit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateDomainUnit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_domain_unit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_domain_unit = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_environment`.\n"]
    pub fn set_create_environment(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateEnvironment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_environment_from_blueprint`.\n"]
    pub fn set_create_environment_from_blueprint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_environment_from_blueprint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_environment_from_blueprint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_environment_profile`.\n"]
    pub fn set_create_environment_profile(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateEnvironmentProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_environment_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_environment_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_form_type`.\n"]
    pub fn set_create_form_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateFormType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_form_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_form_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_glossary`.\n"]
    pub fn set_create_glossary(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateGlossary>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_glossary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_glossary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_project`.\n"]
    pub fn set_create_project(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateProject>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_project = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_project = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_project_from_project_profile`.\n"]
    pub fn set_create_project_from_project_profile(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_CreateProjectFromProjectProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_project_from_project_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_project_from_project_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delegate_create_environment_profile`.\n"]
    pub fn set_delegate_create_environment_profile(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delegate_create_environment_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delegate_create_environment_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `override_domain_unit_owners`.\n"]
    pub fn set_override_domain_unit_owners(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_OverrideDomainUnitOwners>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.override_domain_unit_owners = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.override_domain_unit_owners = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `override_project_owners`.\n"]
    pub fn set_override_project_owners(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_OverrideProjectOwners>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.override_project_owners = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.override_project_owners = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `use_asset_type`.\n"]
    pub fn set_use_asset_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Detail_UseAssetType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.use_asset_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.use_asset_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Detail {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Detail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Detail {}

impl BuildDatazonePolicyGrant_Detail {
    pub fn build(self) -> DatazonePolicyGrant_Detail {
        DatazonePolicyGrant_Detail {
            add_to_project_member_pool: core::default::Default::default(),
            create_asset_type: core::default::Default::default(),
            create_domain_unit: core::default::Default::default(),
            create_environment: core::default::Default::default(),
            create_environment_from_blueprint: core::default::Default::default(),
            create_environment_profile: core::default::Default::default(),
            create_form_type: core::default::Default::default(),
            create_glossary: core::default::Default::default(),
            create_project: core::default::Default::default(),
            create_project_from_project_profile: core::default::Default::default(),
            delegate_create_environment_profile: core::default::Default::default(),
            override_domain_unit_owners: core::default::Default::default(),
            override_project_owners: core::default::Default::default(),
            use_asset_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_DetailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_DetailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_DetailRef {
        DatazonePolicyGrant_DetailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_DetailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add_to_project_member_pool` after provisioning.\n"]
    pub fn add_to_project_member_pool(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_AddToProjectMemberPoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.add_to_project_member_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `create_asset_type` after provisioning.\n"]
    pub fn create_asset_type(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_CreateAssetTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_asset_type", self.base))
    }

    #[doc = "Get a reference to the value of field `create_domain_unit` after provisioning.\n"]
    pub fn create_domain_unit(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_CreateDomainUnitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_domain_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `create_environment` after provisioning.\n"]
    pub fn create_environment(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_CreateEnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_environment", self.base))
    }

    #[doc = "Get a reference to the value of field `create_environment_from_blueprint` after provisioning.\n"]
    pub fn create_environment_from_blueprint(
        &self,
    ) -> terrars::ListRef<DatazonePolicyGrant_Detail_CreateEnvironmentFromBlueprintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_environment_from_blueprint", self.base))
    }

    #[doc = "Get a reference to the value of field `create_environment_profile` after provisioning.\n"]
    pub fn create_environment_profile(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_CreateEnvironmentProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_environment_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `create_form_type` after provisioning.\n"]
    pub fn create_form_type(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_CreateFormTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_form_type", self.base))
    }

    #[doc = "Get a reference to the value of field `create_glossary` after provisioning.\n"]
    pub fn create_glossary(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_CreateGlossaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_glossary", self.base))
    }

    #[doc = "Get a reference to the value of field `create_project` after provisioning.\n"]
    pub fn create_project(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_CreateProjectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_project", self.base))
    }

    #[doc = "Get a reference to the value of field `create_project_from_project_profile` after provisioning.\n"]
    pub fn create_project_from_project_profile(
        &self,
    ) -> terrars::ListRef<DatazonePolicyGrant_Detail_CreateProjectFromProjectProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_project_from_project_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `delegate_create_environment_profile` after provisioning.\n"]
    pub fn delegate_create_environment_profile(
        &self,
    ) -> terrars::ListRef<DatazonePolicyGrant_Detail_DelegateCreateEnvironmentProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delegate_create_environment_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `override_domain_unit_owners` after provisioning.\n"]
    pub fn override_domain_unit_owners(
        &self,
    ) -> terrars::ListRef<DatazonePolicyGrant_Detail_OverrideDomainUnitOwnersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_domain_unit_owners", self.base))
    }

    #[doc = "Get a reference to the value of field `override_project_owners` after provisioning.\n"]
    pub fn override_project_owners(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_OverrideProjectOwnersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_project_owners", self.base))
    }

    #[doc = "Get a reference to the value of field `use_asset_type` after provisioning.\n"]
    pub fn use_asset_type(&self) -> terrars::ListRef<DatazonePolicyGrant_Detail_UseAssetTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.use_asset_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter {}

impl DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter { }

impl terrars::ToListMappable for DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter {}

impl BuildDatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter {
    pub fn build(self) -> DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter {
        DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter {}
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilterRef {
        DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatazonePolicyGrant_Principal_DomainUnitDynamic {
    all_domain_units_grant_filter: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter>>,
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Principal_DomainUnit {
    domain_unit_designation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_unit_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_domain_units_grant_filter: std
    ::option
    ::Option<std::vec::Vec<DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter>>,
    dynamic: DatazonePolicyGrant_Principal_DomainUnitDynamic,
}

impl DatazonePolicyGrant_Principal_DomainUnit {
    #[doc = "Set the field `domain_unit_identifier`.\n"]
    pub fn set_domain_unit_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.domain_unit_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all_domain_units_grant_filter`.\n"]
    pub fn set_all_domain_units_grant_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilter>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.all_domain_units_grant_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.all_domain_units_grant_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Principal_DomainUnit {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Principal_DomainUnit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Principal_DomainUnit {
    #[doc = ""]
    pub domain_unit_designation: terrars::PrimField<std::string::String>,
}

impl BuildDatazonePolicyGrant_Principal_DomainUnit {
    pub fn build(self) -> DatazonePolicyGrant_Principal_DomainUnit {
        DatazonePolicyGrant_Principal_DomainUnit {
            domain_unit_designation: self.domain_unit_designation,
            domain_unit_identifier: core::default::Default::default(),
            all_domain_units_grant_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Principal_DomainUnitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Principal_DomainUnitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_Principal_DomainUnitRef {
        DatazonePolicyGrant_Principal_DomainUnitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Principal_DomainUnitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_unit_designation` after provisioning.\n"]
    pub fn domain_unit_designation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_unit_designation", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_unit_identifier` after provisioning.\n"]
    pub fn domain_unit_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_unit_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `all_domain_units_grant_filter` after provisioning.\n"]
    pub fn all_domain_units_grant_filter(
        &self,
    ) -> terrars::ListRef<DatazonePolicyGrant_Principal_DomainUnit_AllDomainUnitsGrantFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_domain_units_grant_filter", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Principal_Group {
    group_identifier: terrars::PrimField<std::string::String>,
}

impl DatazonePolicyGrant_Principal_Group { }

impl terrars::ToListMappable for DatazonePolicyGrant_Principal_Group {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Principal_Group>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Principal_Group {
    #[doc = ""]
    pub group_identifier: terrars::PrimField<std::string::String>,
}

impl BuildDatazonePolicyGrant_Principal_Group {
    pub fn build(self) -> DatazonePolicyGrant_Principal_Group {
        DatazonePolicyGrant_Principal_Group { group_identifier: self.group_identifier }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Principal_GroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Principal_GroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_Principal_GroupRef {
        DatazonePolicyGrant_Principal_GroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Principal_GroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_identifier` after provisioning.\n"]
    pub fn group_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Principal_Project_DomainUnitFilter {
    domain_unit: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_child_domain_units: std::option::Option<terrars::PrimField<bool>>,
}

impl DatazonePolicyGrant_Principal_Project_DomainUnitFilter {
    #[doc = "Set the field `include_child_domain_units`.\n"]
    pub fn set_include_child_domain_units(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_child_domain_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Principal_Project_DomainUnitFilter {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Principal_Project_DomainUnitFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Principal_Project_DomainUnitFilter {
    #[doc = ""]
    pub domain_unit: terrars::PrimField<std::string::String>,
}

impl BuildDatazonePolicyGrant_Principal_Project_DomainUnitFilter {
    pub fn build(self) -> DatazonePolicyGrant_Principal_Project_DomainUnitFilter {
        DatazonePolicyGrant_Principal_Project_DomainUnitFilter {
            domain_unit: self.domain_unit,
            include_child_domain_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Principal_Project_DomainUnitFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Principal_Project_DomainUnitFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Principal_Project_DomainUnitFilterRef {
        DatazonePolicyGrant_Principal_Project_DomainUnitFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Principal_Project_DomainUnitFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_unit` after provisioning.\n"]
    pub fn domain_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `include_child_domain_units` after provisioning.\n"]
    pub fn include_child_domain_units(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_child_domain_units", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatazonePolicyGrant_Principal_ProjectDynamic {
    domain_unit_filter: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Principal_Project_DomainUnitFilter>>,
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Principal_Project {
    project_designation: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_unit_filter: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Principal_Project_DomainUnitFilter>>,
    dynamic: DatazonePolicyGrant_Principal_ProjectDynamic,
}

impl DatazonePolicyGrant_Principal_Project {
    #[doc = "Set the field `project_identifier`.\n"]
    pub fn set_project_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.project_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_unit_filter`.\n"]
    pub fn set_domain_unit_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Principal_Project_DomainUnitFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.domain_unit_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.domain_unit_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Principal_Project {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Principal_Project>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Principal_Project {
    #[doc = ""]
    pub project_designation: terrars::PrimField<std::string::String>,
}

impl BuildDatazonePolicyGrant_Principal_Project {
    pub fn build(self) -> DatazonePolicyGrant_Principal_Project {
        DatazonePolicyGrant_Principal_Project {
            project_designation: self.project_designation,
            project_identifier: core::default::Default::default(),
            domain_unit_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Principal_ProjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Principal_ProjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_Principal_ProjectRef {
        DatazonePolicyGrant_Principal_ProjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Principal_ProjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `project_designation` after provisioning.\n"]
    pub fn project_designation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_designation", self.base))
    }

    #[doc = "Get a reference to the value of field `project_identifier` after provisioning.\n"]
    pub fn project_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_unit_filter` after provisioning.\n"]
    pub fn domain_unit_filter(&self) -> terrars::ListRef<DatazonePolicyGrant_Principal_Project_DomainUnitFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_unit_filter", self.base))
    }
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Principal_User_AllUsersGrantFilter {}

impl DatazonePolicyGrant_Principal_User_AllUsersGrantFilter { }

impl terrars::ToListMappable for DatazonePolicyGrant_Principal_User_AllUsersGrantFilter {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Principal_User_AllUsersGrantFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Principal_User_AllUsersGrantFilter {}

impl BuildDatazonePolicyGrant_Principal_User_AllUsersGrantFilter {
    pub fn build(self) -> DatazonePolicyGrant_Principal_User_AllUsersGrantFilter {
        DatazonePolicyGrant_Principal_User_AllUsersGrantFilter {}
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Principal_User_AllUsersGrantFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Principal_User_AllUsersGrantFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatazonePolicyGrant_Principal_User_AllUsersGrantFilterRef {
        DatazonePolicyGrant_Principal_User_AllUsersGrantFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Principal_User_AllUsersGrantFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatazonePolicyGrant_Principal_UserDynamic {
    all_users_grant_filter: std
    ::option
    ::Option<terrars::DynamicBlock<DatazonePolicyGrant_Principal_User_AllUsersGrantFilter>>,
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Principal_User {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_users_grant_filter: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Principal_User_AllUsersGrantFilter>>,
    dynamic: DatazonePolicyGrant_Principal_UserDynamic,
}

impl DatazonePolicyGrant_Principal_User {
    #[doc = "Set the field `user_identifier`.\n"]
    pub fn set_user_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all_users_grant_filter`.\n"]
    pub fn set_all_users_grant_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Principal_User_AllUsersGrantFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.all_users_grant_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.all_users_grant_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Principal_User {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Principal_User>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Principal_User {}

impl BuildDatazonePolicyGrant_Principal_User {
    pub fn build(self) -> DatazonePolicyGrant_Principal_User {
        DatazonePolicyGrant_Principal_User {
            user_identifier: core::default::Default::default(),
            all_users_grant_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_Principal_UserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_Principal_UserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_Principal_UserRef {
        DatazonePolicyGrant_Principal_UserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_Principal_UserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_identifier` after provisioning.\n"]
    pub fn user_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `all_users_grant_filter` after provisioning.\n"]
    pub fn all_users_grant_filter(&self) -> terrars::ListRef<DatazonePolicyGrant_Principal_User_AllUsersGrantFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_users_grant_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatazonePolicyGrant_PrincipalDynamic {
    domain_unit: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Principal_DomainUnit>>,
    group: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Principal_Group>>,
    project: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Principal_Project>>,
    user: std::option::Option<terrars::DynamicBlock<DatazonePolicyGrant_Principal_User>>,
}

#[derive(Serialize)]
pub struct DatazonePolicyGrant_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_unit: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Principal_DomainUnit>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Principal_Group>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Principal_Project>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<std::vec::Vec<DatazonePolicyGrant_Principal_User>>,
    dynamic: DatazonePolicyGrant_PrincipalDynamic,
}

impl DatazonePolicyGrant_Principal {
    #[doc = "Set the field `domain_unit`.\n"]
    pub fn set_domain_unit(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Principal_DomainUnit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.domain_unit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.domain_unit = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Principal_Group>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Principal_Project>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.project = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.project = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatazonePolicyGrant_Principal_User>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatazonePolicyGrant_Principal {
    type O = terrars::BlockAssignable<DatazonePolicyGrant_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatazonePolicyGrant_Principal {}

impl BuildDatazonePolicyGrant_Principal {
    pub fn build(self) -> DatazonePolicyGrant_Principal {
        DatazonePolicyGrant_Principal {
            domain_unit: core::default::Default::default(),
            group: core::default::Default::default(),
            project: core::default::Default::default(),
            user: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatazonePolicyGrant_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatazonePolicyGrant_PrincipalRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatazonePolicyGrant_PrincipalRef {
        DatazonePolicyGrant_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatazonePolicyGrant_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_unit` after provisioning.\n"]
    pub fn domain_unit(&self) -> terrars::ListRef<DatazonePolicyGrant_Principal_DomainUnitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::ListRef<DatazonePolicyGrant_Principal_GroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::ListRef<DatazonePolicyGrant_Principal_ProjectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::ListRef<DatazonePolicyGrant_Principal_UserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

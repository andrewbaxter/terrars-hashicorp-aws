#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AuditmanagerAssessmentData_ {
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
    framework_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assessment_reports_destination: std
    ::option
    ::Option<std::vec::Vec<AuditmanagerAssessment_AssessmentReportsDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<std::vec::Vec<AuditmanagerAssessment_Roles>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<std::vec::Vec<AuditmanagerAssessment_Scope>>,
    dynamic: AuditmanagerAssessmentDynamic,
}

struct AuditmanagerAssessment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AuditmanagerAssessmentData_>,
}

#[derive(Clone)]
pub struct AuditmanagerAssessment(std::rc::Rc<AuditmanagerAssessment_>);

impl AuditmanagerAssessment {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `assessment_reports_destination`.\n"]
    pub fn set_assessment_reports_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerAssessment_AssessmentReportsDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().assessment_reports_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.assessment_reports_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(self, v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerAssessment_Roles>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().roles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.roles = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(self, v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerAssessment_Scope>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `framework_id` after provisioning.\n"]
    pub fn framework_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `roles_all` after provisioning.\n"]
    pub fn roles_all(&self) -> terrars::ListRef<AuditmanagerAssessment_RolesAllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roles_all", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `assessment_reports_destination` after provisioning.\n"]
    pub fn assessment_reports_destination(
        &self,
    ) -> terrars::ListRef<AuditmanagerAssessment_AssessmentReportsDestinationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.assessment_reports_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<AuditmanagerAssessment_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

impl terrars::Referable for AuditmanagerAssessment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AuditmanagerAssessment { }

impl terrars::ToListMappable for AuditmanagerAssessment {
    type O = terrars::ListRef<AuditmanagerAssessmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AuditmanagerAssessment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_auditmanager_assessment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAuditmanagerAssessment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub framework_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerAssessment {
    pub fn build(self, stack: &mut terrars::Stack) -> AuditmanagerAssessment {
        let out = AuditmanagerAssessment(std::rc::Rc::new(AuditmanagerAssessment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AuditmanagerAssessmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                framework_id: self.framework_id,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                assessment_reports_destination: core::default::Default::default(),
                roles: core::default::Default::default(),
                scope: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AuditmanagerAssessmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AuditmanagerAssessmentRef {
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

    #[doc = "Get a reference to the value of field `framework_id` after provisioning.\n"]
    pub fn framework_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `roles_all` after provisioning.\n"]
    pub fn roles_all(&self) -> terrars::ListRef<AuditmanagerAssessment_RolesAllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.roles_all", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `assessment_reports_destination` after provisioning.\n"]
    pub fn assessment_reports_destination(
        &self,
    ) -> terrars::ListRef<AuditmanagerAssessment_AssessmentReportsDestinationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.assessment_reports_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<AuditmanagerAssessment_ScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AuditmanagerAssessmentDynamic {
    assessment_reports_destination: std
    ::option
    ::Option<terrars::DynamicBlock<AuditmanagerAssessment_AssessmentReportsDestination>>,
    roles: std::option::Option<terrars::DynamicBlock<AuditmanagerAssessment_Roles>>,
    scope: std::option::Option<terrars::DynamicBlock<AuditmanagerAssessment_Scope>>,
}

#[derive(Serialize)]
pub struct AuditmanagerAssessment_RolesAll {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AuditmanagerAssessment_RolesAll {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_type`.\n"]
    pub fn set_role_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AuditmanagerAssessment_RolesAll {
    type O = terrars::BlockAssignable<AuditmanagerAssessment_RolesAll>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessment_RolesAll {}

impl BuildAuditmanagerAssessment_RolesAll {
    pub fn build(self) -> AuditmanagerAssessment_RolesAll {
        AuditmanagerAssessment_RolesAll {
            role_arn: core::default::Default::default(),
            role_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AuditmanagerAssessment_RolesAllRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessment_RolesAllRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessment_RolesAllRef {
        AuditmanagerAssessment_RolesAllRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessment_RolesAllRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_type` after provisioning.\n"]
    pub fn role_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AuditmanagerAssessment_AssessmentReportsDestination {
    destination: terrars::PrimField<std::string::String>,
    destination_type: terrars::PrimField<std::string::String>,
}

impl AuditmanagerAssessment_AssessmentReportsDestination { }

impl terrars::ToListMappable for AuditmanagerAssessment_AssessmentReportsDestination {
    type O = terrars::BlockAssignable<AuditmanagerAssessment_AssessmentReportsDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessment_AssessmentReportsDestination {
    #[doc = ""]
    pub destination: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub destination_type: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerAssessment_AssessmentReportsDestination {
    pub fn build(self) -> AuditmanagerAssessment_AssessmentReportsDestination {
        AuditmanagerAssessment_AssessmentReportsDestination {
            destination: self.destination,
            destination_type: self.destination_type,
        }
    }
}

#[allow(dead_code)]
pub struct AuditmanagerAssessment_AssessmentReportsDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessment_AssessmentReportsDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AuditmanagerAssessment_AssessmentReportsDestinationRef {
        AuditmanagerAssessment_AssessmentReportsDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessment_AssessmentReportsDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_type` after provisioning.\n"]
    pub fn destination_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AuditmanagerAssessment_Roles {
    role_arn: terrars::PrimField<std::string::String>,
    role_type: terrars::PrimField<std::string::String>,
}

impl AuditmanagerAssessment_Roles { }

impl terrars::ToListMappable for AuditmanagerAssessment_Roles {
    type O = terrars::BlockAssignable<AuditmanagerAssessment_Roles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessment_Roles {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_type: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerAssessment_Roles {
    pub fn build(self) -> AuditmanagerAssessment_Roles {
        AuditmanagerAssessment_Roles {
            role_arn: self.role_arn,
            role_type: self.role_type,
        }
    }
}

#[allow(dead_code)]
pub struct AuditmanagerAssessment_RolesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessment_RolesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessment_RolesRef {
        AuditmanagerAssessment_RolesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessment_RolesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_type` after provisioning.\n"]
    pub fn role_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AuditmanagerAssessment_Scope_AwsAccounts {
    id: terrars::PrimField<std::string::String>,
}

impl AuditmanagerAssessment_Scope_AwsAccounts { }

impl terrars::ToListMappable for AuditmanagerAssessment_Scope_AwsAccounts {
    type O = terrars::BlockAssignable<AuditmanagerAssessment_Scope_AwsAccounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessment_Scope_AwsAccounts {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerAssessment_Scope_AwsAccounts {
    pub fn build(self) -> AuditmanagerAssessment_Scope_AwsAccounts {
        AuditmanagerAssessment_Scope_AwsAccounts { id: self.id }
    }
}

#[allow(dead_code)]
pub struct AuditmanagerAssessment_Scope_AwsAccountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessment_Scope_AwsAccountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessment_Scope_AwsAccountsRef {
        AuditmanagerAssessment_Scope_AwsAccountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessment_Scope_AwsAccountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct AuditmanagerAssessment_Scope_AwsServices {
    service_name: terrars::PrimField<std::string::String>,
}

impl AuditmanagerAssessment_Scope_AwsServices { }

impl terrars::ToListMappable for AuditmanagerAssessment_Scope_AwsServices {
    type O = terrars::BlockAssignable<AuditmanagerAssessment_Scope_AwsServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessment_Scope_AwsServices {
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerAssessment_Scope_AwsServices {
    pub fn build(self) -> AuditmanagerAssessment_Scope_AwsServices {
        AuditmanagerAssessment_Scope_AwsServices { service_name: self.service_name }
    }
}

#[allow(dead_code)]
pub struct AuditmanagerAssessment_Scope_AwsServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessment_Scope_AwsServicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessment_Scope_AwsServicesRef {
        AuditmanagerAssessment_Scope_AwsServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessment_Scope_AwsServicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AuditmanagerAssessment_ScopeDynamic {
    aws_accounts: std::option::Option<terrars::DynamicBlock<AuditmanagerAssessment_Scope_AwsAccounts>>,
    aws_services: std::option::Option<terrars::DynamicBlock<AuditmanagerAssessment_Scope_AwsServices>>,
}

#[derive(Serialize)]
pub struct AuditmanagerAssessment_Scope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_accounts: std::option::Option<std::vec::Vec<AuditmanagerAssessment_Scope_AwsAccounts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_services: std::option::Option<std::vec::Vec<AuditmanagerAssessment_Scope_AwsServices>>,
    dynamic: AuditmanagerAssessment_ScopeDynamic,
}

impl AuditmanagerAssessment_Scope {
    #[doc = "Set the field `aws_accounts`.\n"]
    pub fn set_aws_accounts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerAssessment_Scope_AwsAccounts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_accounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_accounts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_services`.\n"]
    pub fn set_aws_services(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerAssessment_Scope_AwsServices>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AuditmanagerAssessment_Scope {
    type O = terrars::BlockAssignable<AuditmanagerAssessment_Scope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessment_Scope {}

impl BuildAuditmanagerAssessment_Scope {
    pub fn build(self) -> AuditmanagerAssessment_Scope {
        AuditmanagerAssessment_Scope {
            aws_accounts: core::default::Default::default(),
            aws_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AuditmanagerAssessment_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessment_ScopeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessment_ScopeRef {
        AuditmanagerAssessment_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessment_ScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

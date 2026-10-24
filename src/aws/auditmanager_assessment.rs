use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AuditmanagerAssessmentData {
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
    roles: terrars::SetField<AuditmanagerAssessmentRolesEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assessment_reports_destination: std
    ::option
    ::Option<std::vec::Vec<AuditmanagerAssessmentAssessmentReportsDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<std::vec::Vec<AuditmanagerAssessmentScopeEl>>,
    dynamic: AuditmanagerAssessmentDynamic,
}

struct AuditmanagerAssessment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AuditmanagerAssessmentData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerAssessmentAssessmentReportsDestinationEl>>,
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

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(self, v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerAssessmentScopeEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> terrars::SetRef<AuditmanagerAssessmentRolesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roles_all` after provisioning.\n"]
    pub fn roles_all(&self) -> terrars::SetRef<AuditmanagerAssessmentRolesAllElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles_all", self.extract_ref()))
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
    ) -> terrars::ListRef<AuditmanagerAssessmentAssessmentReportsDestinationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.assessment_reports_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<AuditmanagerAssessmentScopeElRef> {
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
    #[doc = ""]
    pub roles: terrars::SetField<AuditmanagerAssessmentRolesEl>,
}

impl BuildAuditmanagerAssessment {
    pub fn build(self, stack: &mut terrars::Stack) -> AuditmanagerAssessment {
        let out = AuditmanagerAssessment(std::rc::Rc::new(AuditmanagerAssessment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AuditmanagerAssessmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                framework_id: self.framework_id,
                name: self.name,
                roles: self.roles,
                tags: core::default::Default::default(),
                assessment_reports_destination: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> terrars::SetRef<AuditmanagerAssessmentRolesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roles_all` after provisioning.\n"]
    pub fn roles_all(&self) -> terrars::SetRef<AuditmanagerAssessmentRolesAllElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles_all", self.extract_ref()))
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
    ) -> terrars::ListRef<AuditmanagerAssessmentAssessmentReportsDestinationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.assessment_reports_destination", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::ListRef<AuditmanagerAssessmentScopeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AuditmanagerAssessmentRolesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AuditmanagerAssessmentRolesEl {
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

impl terrars::ToListMappable for AuditmanagerAssessmentRolesEl {
    type O = terrars::BlockAssignable<AuditmanagerAssessmentRolesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessmentRolesEl {}

impl BuildAuditmanagerAssessmentRolesEl {
    pub fn build(self) -> AuditmanagerAssessmentRolesEl {
        AuditmanagerAssessmentRolesEl {
            role_arn: core::default::Default::default(),
            role_type: core::default::Default::default(),
        }
    }
}

pub struct AuditmanagerAssessmentRolesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessmentRolesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessmentRolesElRef {
        AuditmanagerAssessmentRolesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessmentRolesElRef {
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
pub struct AuditmanagerAssessmentRolesAllEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AuditmanagerAssessmentRolesAllEl {
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

impl terrars::ToListMappable for AuditmanagerAssessmentRolesAllEl {
    type O = terrars::BlockAssignable<AuditmanagerAssessmentRolesAllEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessmentRolesAllEl {}

impl BuildAuditmanagerAssessmentRolesAllEl {
    pub fn build(self) -> AuditmanagerAssessmentRolesAllEl {
        AuditmanagerAssessmentRolesAllEl {
            role_arn: core::default::Default::default(),
            role_type: core::default::Default::default(),
        }
    }
}

pub struct AuditmanagerAssessmentRolesAllElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessmentRolesAllElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessmentRolesAllElRef {
        AuditmanagerAssessmentRolesAllElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessmentRolesAllElRef {
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
pub struct AuditmanagerAssessmentAssessmentReportsDestinationEl {
    destination: terrars::PrimField<std::string::String>,
    destination_type: terrars::PrimField<std::string::String>,
}

impl AuditmanagerAssessmentAssessmentReportsDestinationEl { }

impl terrars::ToListMappable for AuditmanagerAssessmentAssessmentReportsDestinationEl {
    type O = terrars::BlockAssignable<AuditmanagerAssessmentAssessmentReportsDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessmentAssessmentReportsDestinationEl {
    #[doc = ""]
    pub destination: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub destination_type: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerAssessmentAssessmentReportsDestinationEl {
    pub fn build(self) -> AuditmanagerAssessmentAssessmentReportsDestinationEl {
        AuditmanagerAssessmentAssessmentReportsDestinationEl {
            destination: self.destination,
            destination_type: self.destination_type,
        }
    }
}

pub struct AuditmanagerAssessmentAssessmentReportsDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessmentAssessmentReportsDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AuditmanagerAssessmentAssessmentReportsDestinationElRef {
        AuditmanagerAssessmentAssessmentReportsDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessmentAssessmentReportsDestinationElRef {
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
pub struct AuditmanagerAssessmentScopeElAwsAccountsEl {
    id: terrars::PrimField<std::string::String>,
}

impl AuditmanagerAssessmentScopeElAwsAccountsEl { }

impl terrars::ToListMappable for AuditmanagerAssessmentScopeElAwsAccountsEl {
    type O = terrars::BlockAssignable<AuditmanagerAssessmentScopeElAwsAccountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessmentScopeElAwsAccountsEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerAssessmentScopeElAwsAccountsEl {
    pub fn build(self) -> AuditmanagerAssessmentScopeElAwsAccountsEl {
        AuditmanagerAssessmentScopeElAwsAccountsEl { id: self.id }
    }
}

pub struct AuditmanagerAssessmentScopeElAwsAccountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessmentScopeElAwsAccountsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessmentScopeElAwsAccountsElRef {
        AuditmanagerAssessmentScopeElAwsAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessmentScopeElAwsAccountsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct AuditmanagerAssessmentScopeElAwsServicesEl {
    service_name: terrars::PrimField<std::string::String>,
}

impl AuditmanagerAssessmentScopeElAwsServicesEl { }

impl terrars::ToListMappable for AuditmanagerAssessmentScopeElAwsServicesEl {
    type O = terrars::BlockAssignable<AuditmanagerAssessmentScopeElAwsServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessmentScopeElAwsServicesEl {
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerAssessmentScopeElAwsServicesEl {
    pub fn build(self) -> AuditmanagerAssessmentScopeElAwsServicesEl {
        AuditmanagerAssessmentScopeElAwsServicesEl { service_name: self.service_name }
    }
}

pub struct AuditmanagerAssessmentScopeElAwsServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessmentScopeElAwsServicesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessmentScopeElAwsServicesElRef {
        AuditmanagerAssessmentScopeElAwsServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessmentScopeElAwsServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AuditmanagerAssessmentScopeElDynamic {
    aws_accounts: std::option::Option<terrars::DynamicBlock<AuditmanagerAssessmentScopeElAwsAccountsEl>>,
    aws_services: std::option::Option<terrars::DynamicBlock<AuditmanagerAssessmentScopeElAwsServicesEl>>,
}

#[derive(Serialize)]
pub struct AuditmanagerAssessmentScopeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_accounts: std::option::Option<std::vec::Vec<AuditmanagerAssessmentScopeElAwsAccountsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_services: std::option::Option<std::vec::Vec<AuditmanagerAssessmentScopeElAwsServicesEl>>,
    dynamic: AuditmanagerAssessmentScopeElDynamic,
}

impl AuditmanagerAssessmentScopeEl {
    #[doc = "Set the field `aws_accounts`.\n"]
    pub fn set_aws_accounts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerAssessmentScopeElAwsAccountsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerAssessmentScopeElAwsServicesEl>>,
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

impl terrars::ToListMappable for AuditmanagerAssessmentScopeEl {
    type O = terrars::BlockAssignable<AuditmanagerAssessmentScopeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerAssessmentScopeEl {}

impl BuildAuditmanagerAssessmentScopeEl {
    pub fn build(self) -> AuditmanagerAssessmentScopeEl {
        AuditmanagerAssessmentScopeEl {
            aws_accounts: core::default::Default::default(),
            aws_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AuditmanagerAssessmentScopeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerAssessmentScopeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerAssessmentScopeElRef {
        AuditmanagerAssessmentScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerAssessmentScopeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct AuditmanagerAssessmentDynamic {
    assessment_reports_destination: std
    ::option
    ::Option<terrars::DynamicBlock<AuditmanagerAssessmentAssessmentReportsDestinationEl>>,
    scope: std::option::Option<terrars::DynamicBlock<AuditmanagerAssessmentScopeEl>>,
}

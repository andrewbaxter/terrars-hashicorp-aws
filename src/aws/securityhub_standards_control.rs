use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecurityhubStandardsControlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    control_status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    standards_control_arn: PrimField<String>,
}

struct SecurityhubStandardsControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecurityhubStandardsControlData>,
}

#[derive(Clone)]
pub struct SecurityhubStandardsControl(Rc<SecurityhubStandardsControl_>);

impl SecurityhubStandardsControl {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `disabled_reason`.\n"]
    pub fn set_disabled_reason(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().disabled_reason = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `control_id` after provisioning.\n"]
    pub fn control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `control_status` after provisioning.\n"]
    pub fn control_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `control_status_updated_at` after provisioning.\n"]
    pub fn control_status_updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_status_updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled_reason` after provisioning.\n"]
    pub fn disabled_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `related_requirements` after provisioning.\n"]
    pub fn related_requirements(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.related_requirements", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remediation_url` after provisioning.\n"]
    pub fn remediation_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remediation_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `severity_rating` after provisioning.\n"]
    pub fn severity_rating(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity_rating", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `standards_control_arn` after provisioning.\n"]
    pub fn standards_control_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.standards_control_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}

impl Referable for SecurityhubStandardsControl {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SecurityhubStandardsControl { }

impl ToListMappable for SecurityhubStandardsControl {
    type O = ListRef<SecurityhubStandardsControlRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SecurityhubStandardsControl_ {
    fn extract_resource_type(&self) -> String {
        "aws_securityhub_standards_control".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityhubStandardsControl {
    pub tf_id: String,
    #[doc= ""]
    pub control_status: PrimField<String>,
    #[doc= ""]
    pub standards_control_arn: PrimField<String>,
}

impl BuildSecurityhubStandardsControl {
    pub fn build(self, stack: &mut Stack) -> SecurityhubStandardsControl {
        let out = SecurityhubStandardsControl(Rc::new(SecurityhubStandardsControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecurityhubStandardsControlData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                control_status: self.control_status,
                disabled_reason: core::default::Default::default(),
                id: core::default::Default::default(),
                standards_control_arn: self.standards_control_arn,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityhubStandardsControlRef {
    shared: StackShared,
    base: String,
}

impl Ref for SecurityhubStandardsControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecurityhubStandardsControlRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `control_id` after provisioning.\n"]
    pub fn control_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `control_status` after provisioning.\n"]
    pub fn control_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `control_status_updated_at` after provisioning.\n"]
    pub fn control_status_updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_status_updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled_reason` after provisioning.\n"]
    pub fn disabled_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `related_requirements` after provisioning.\n"]
    pub fn related_requirements(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.related_requirements", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `remediation_url` after provisioning.\n"]
    pub fn remediation_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remediation_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `severity_rating` after provisioning.\n"]
    pub fn severity_rating(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.severity_rating", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `standards_control_arn` after provisioning.\n"]
    pub fn standards_control_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.standards_control_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}

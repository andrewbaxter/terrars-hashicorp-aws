use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LightsailStaticIpAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_name: PrimField<String>,
    static_ip_name: PrimField<String>,
}

struct LightsailStaticIpAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LightsailStaticIpAttachmentData>,
}

#[derive(Clone)]
pub struct LightsailStaticIpAttachment(Rc<LightsailStaticIpAttachment_>);

impl LightsailStaticIpAttachment {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_name` after provisioning.\n"]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `static_ip_name` after provisioning.\n"]
    pub fn static_ip_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.static_ip_name", self.extract_ref()))
    }
}

impl Referable for LightsailStaticIpAttachment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LightsailStaticIpAttachment { }

impl ToListMappable for LightsailStaticIpAttachment {
    type O = ListRef<LightsailStaticIpAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LightsailStaticIpAttachment_ {
    fn extract_resource_type(&self) -> String {
        "aws_lightsail_static_ip_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLightsailStaticIpAttachment {
    pub tf_id: String,
    #[doc= ""]
    pub instance_name: PrimField<String>,
    #[doc= ""]
    pub static_ip_name: PrimField<String>,
}

impl BuildLightsailStaticIpAttachment {
    pub fn build(self, stack: &mut Stack) -> LightsailStaticIpAttachment {
        let out = LightsailStaticIpAttachment(Rc::new(LightsailStaticIpAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LightsailStaticIpAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                instance_name: self.instance_name,
                static_ip_name: self.static_ip_name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LightsailStaticIpAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for LightsailStaticIpAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LightsailStaticIpAttachmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_name` after provisioning.\n"]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `static_ip_name` after provisioning.\n"]
    pub fn static_ip_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.static_ip_name", self.extract_ref()))
    }
}

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KendraExperienceData {
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
    index_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<KendraExperienceConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KendraExperienceTimeoutsEl>,
    dynamic: KendraExperienceDynamic,
}

struct KendraExperience_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KendraExperienceData>,
}

#[derive(Clone)]
pub struct KendraExperience(std::rc::Rc<KendraExperience_>);

impl KendraExperience {
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KendraExperienceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KendraExperienceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::SetRef<KendraExperienceEndpointsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experience_id` after provisioning.\n"]
    pub fn experience_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.experience_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<KendraExperienceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraExperienceTimeoutsElRef {
        KendraExperienceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for KendraExperience {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KendraExperience { }

impl terrars::ToListMappable for KendraExperience {
    type O = terrars::ListRef<KendraExperienceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KendraExperience_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_kendra_experience".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKendraExperience {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub index_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildKendraExperience {
    pub fn build(self, stack: &mut terrars::Stack) -> KendraExperience {
        let out = KendraExperience(std::rc::Rc::new(KendraExperience_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KendraExperienceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                index_id: self.index_id,
                name: self.name,
                role_arn: self.role_arn,
                configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KendraExperienceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraExperienceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KendraExperienceRef {
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

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::SetRef<KendraExperienceEndpointsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `experience_id` after provisioning.\n"]
    pub fn experience_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.experience_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<KendraExperienceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraExperienceTimeoutsElRef {
        KendraExperienceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct KendraExperienceEndpointsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraExperienceEndpointsEl {
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_type`.\n"]
    pub fn set_endpoint_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraExperienceEndpointsEl {
    type O = terrars::BlockAssignable<KendraExperienceEndpointsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceEndpointsEl {}

impl BuildKendraExperienceEndpointsEl {
    pub fn build(self) -> KendraExperienceEndpointsEl {
        KendraExperienceEndpointsEl {
            endpoint: core::default::Default::default(),
            endpoint_type: core::default::Default::default(),
        }
    }
}

pub struct KendraExperienceEndpointsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraExperienceEndpointsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraExperienceEndpointsElRef {
        KendraExperienceEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceEndpointsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraExperienceConfigurationElContentSourceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_put_content: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faq_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl KendraExperienceConfigurationElContentSourceConfigurationEl {
    #[doc = "Set the field `data_source_ids`.\n"]
    pub fn set_data_source_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_source_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direct_put_content`.\n"]
    pub fn set_direct_put_content(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.direct_put_content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `faq_ids`.\n"]
    pub fn set_faq_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.faq_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraExperienceConfigurationElContentSourceConfigurationEl {
    type O = terrars::BlockAssignable<KendraExperienceConfigurationElContentSourceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceConfigurationElContentSourceConfigurationEl {}

impl BuildKendraExperienceConfigurationElContentSourceConfigurationEl {
    pub fn build(self) -> KendraExperienceConfigurationElContentSourceConfigurationEl {
        KendraExperienceConfigurationElContentSourceConfigurationEl {
            data_source_ids: core::default::Default::default(),
            direct_put_content: core::default::Default::default(),
            faq_ids: core::default::Default::default(),
        }
    }
}

pub struct KendraExperienceConfigurationElContentSourceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraExperienceConfigurationElContentSourceConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraExperienceConfigurationElContentSourceConfigurationElRef {
        KendraExperienceConfigurationElContentSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceConfigurationElContentSourceConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_ids` after provisioning.\n"]
    pub fn data_source_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.data_source_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `direct_put_content` after provisioning.\n"]
    pub fn direct_put_content(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direct_put_content", self.base))
    }

    #[doc = "Get a reference to the value of field `faq_ids` after provisioning.\n"]
    pub fn faq_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.faq_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraExperienceConfigurationElUserIdentityConfigurationEl {
    identity_attribute_name: terrars::PrimField<std::string::String>,
}

impl KendraExperienceConfigurationElUserIdentityConfigurationEl { }

impl terrars::ToListMappable for KendraExperienceConfigurationElUserIdentityConfigurationEl {
    type O = terrars::BlockAssignable<KendraExperienceConfigurationElUserIdentityConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceConfigurationElUserIdentityConfigurationEl {
    #[doc = ""]
    pub identity_attribute_name: terrars::PrimField<std::string::String>,
}

impl BuildKendraExperienceConfigurationElUserIdentityConfigurationEl {
    pub fn build(self) -> KendraExperienceConfigurationElUserIdentityConfigurationEl {
        KendraExperienceConfigurationElUserIdentityConfigurationEl {
            identity_attribute_name: self.identity_attribute_name,
        }
    }
}

pub struct KendraExperienceConfigurationElUserIdentityConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraExperienceConfigurationElUserIdentityConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KendraExperienceConfigurationElUserIdentityConfigurationElRef {
        KendraExperienceConfigurationElUserIdentityConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceConfigurationElUserIdentityConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_attribute_name` after provisioning.\n"]
    pub fn identity_attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_attribute_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraExperienceConfigurationElDynamic {
    content_source_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraExperienceConfigurationElContentSourceConfigurationEl>>,
    user_identity_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<KendraExperienceConfigurationElUserIdentityConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KendraExperienceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_source_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraExperienceConfigurationElContentSourceConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_identity_configuration: std
    ::option
    ::Option<std::vec::Vec<KendraExperienceConfigurationElUserIdentityConfigurationEl>>,
    dynamic: KendraExperienceConfigurationElDynamic,
}

impl KendraExperienceConfigurationEl {
    #[doc = "Set the field `content_source_configuration`.\n"]
    pub fn set_content_source_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraExperienceConfigurationElContentSourceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.content_source_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.content_source_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_identity_configuration`.\n"]
    pub fn set_user_identity_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<KendraExperienceConfigurationElUserIdentityConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_identity_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_identity_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for KendraExperienceConfigurationEl {
    type O = terrars::BlockAssignable<KendraExperienceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceConfigurationEl {}

impl BuildKendraExperienceConfigurationEl {
    pub fn build(self) -> KendraExperienceConfigurationEl {
        KendraExperienceConfigurationEl {
            content_source_configuration: core::default::Default::default(),
            user_identity_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct KendraExperienceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraExperienceConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraExperienceConfigurationElRef {
        KendraExperienceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_source_configuration` after provisioning.\n"]
    pub fn content_source_configuration(
        &self,
    ) -> terrars::ListRef<KendraExperienceConfigurationElContentSourceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_source_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `user_identity_configuration` after provisioning.\n"]
    pub fn user_identity_configuration(
        &self,
    ) -> terrars::ListRef<KendraExperienceConfigurationElUserIdentityConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_identity_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraExperienceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KendraExperienceTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for KendraExperienceTimeoutsEl {
    type O = terrars::BlockAssignable<KendraExperienceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraExperienceTimeoutsEl {}

impl BuildKendraExperienceTimeoutsEl {
    pub fn build(self) -> KendraExperienceTimeoutsEl {
        KendraExperienceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct KendraExperienceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KendraExperienceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KendraExperienceTimeoutsElRef {
        KendraExperienceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraExperienceTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraExperienceDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<KendraExperienceConfigurationEl>>,
}

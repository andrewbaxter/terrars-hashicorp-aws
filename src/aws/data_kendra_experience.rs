use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKendraExperienceData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    experience_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    index_id: terrars::PrimField<std::string::String>,
}

struct DataKendraExperience_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataKendraExperienceData>,
}

#[derive(Clone)]
pub struct DataKendraExperience(std::rc::Rc<DataKendraExperience_>);

impl DataKendraExperience {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<DataKendraExperienceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::SetRef<DataKendraExperienceEndpointsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataKendraExperience {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataKendraExperience { }

impl terrars::ToListMappable for DataKendraExperience {
    type O = terrars::ListRef<DataKendraExperienceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataKendraExperience_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_kendra_experience".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKendraExperience {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub experience_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub index_id: terrars::PrimField<std::string::String>,
}

impl BuildDataKendraExperience {
    pub fn build(self, stack: &mut terrars::Stack) -> DataKendraExperience {
        let out = DataKendraExperience(std::rc::Rc::new(DataKendraExperience_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataKendraExperienceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                experience_id: self.experience_id,
                id: core::default::Default::default(),
                index_id: self.index_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKendraExperienceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraExperienceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKendraExperienceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<DataKendraExperienceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::SetRef<DataKendraExperienceEndpointsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKendraExperienceConfigurationElContentSourceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_put_content: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faq_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataKendraExperienceConfigurationElContentSourceConfigurationEl {
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

impl terrars::ToListMappable for DataKendraExperienceConfigurationElContentSourceConfigurationEl {
    type O = terrars::BlockAssignable<DataKendraExperienceConfigurationElContentSourceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraExperienceConfigurationElContentSourceConfigurationEl {}

impl BuildDataKendraExperienceConfigurationElContentSourceConfigurationEl {
    pub fn build(self) -> DataKendraExperienceConfigurationElContentSourceConfigurationEl {
        DataKendraExperienceConfigurationElContentSourceConfigurationEl {
            data_source_ids: core::default::Default::default(),
            direct_put_content: core::default::Default::default(),
            faq_ids: core::default::Default::default(),
        }
    }
}

pub struct DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
        DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraExperienceConfigurationElContentSourceConfigurationElRef {
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
pub struct DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_attribute_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
    #[doc = "Set the field `identity_attribute_name`.\n"]
    pub fn set_identity_attribute_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.identity_attribute_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
    type O = terrars::BlockAssignable<DataKendraExperienceConfigurationElUserIdentityConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraExperienceConfigurationElUserIdentityConfigurationEl {}

impl BuildDataKendraExperienceConfigurationElUserIdentityConfigurationEl {
    pub fn build(self) -> DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
        DataKendraExperienceConfigurationElUserIdentityConfigurationEl {
            identity_attribute_name: core::default::Default::default(),
        }
    }
}

pub struct DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
        DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraExperienceConfigurationElUserIdentityConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_attribute_name` after provisioning.\n"]
    pub fn identity_attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_attribute_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraExperienceConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_source_configuration: std
    ::option
    ::Option<terrars::ListField<DataKendraExperienceConfigurationElContentSourceConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_identity_configuration: std
    ::option
    ::Option<terrars::ListField<DataKendraExperienceConfigurationElUserIdentityConfigurationEl>>,
}

impl DataKendraExperienceConfigurationEl {
    #[doc = "Set the field `content_source_configuration`.\n"]
    pub fn set_content_source_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataKendraExperienceConfigurationElContentSourceConfigurationEl>>,
    ) -> Self {
        self.content_source_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_identity_configuration`.\n"]
    pub fn set_user_identity_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataKendraExperienceConfigurationElUserIdentityConfigurationEl>>,
    ) -> Self {
        self.user_identity_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataKendraExperienceConfigurationEl {
    type O = terrars::BlockAssignable<DataKendraExperienceConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraExperienceConfigurationEl {}

impl BuildDataKendraExperienceConfigurationEl {
    pub fn build(self) -> DataKendraExperienceConfigurationEl {
        DataKendraExperienceConfigurationEl {
            content_source_configuration: core::default::Default::default(),
            user_identity_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataKendraExperienceConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraExperienceConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKendraExperienceConfigurationElRef {
        DataKendraExperienceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraExperienceConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_source_configuration` after provisioning.\n"]
    pub fn content_source_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraExperienceConfigurationElContentSourceConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_source_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `user_identity_configuration` after provisioning.\n"]
    pub fn user_identity_configuration(
        &self,
    ) -> terrars::ListRef<DataKendraExperienceConfigurationElUserIdentityConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_identity_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataKendraExperienceEndpointsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataKendraExperienceEndpointsEl {
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

impl terrars::ToListMappable for DataKendraExperienceEndpointsEl {
    type O = terrars::BlockAssignable<DataKendraExperienceEndpointsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraExperienceEndpointsEl {}

impl BuildDataKendraExperienceEndpointsEl {
    pub fn build(self) -> DataKendraExperienceEndpointsEl {
        DataKendraExperienceEndpointsEl {
            endpoint: core::default::Default::default(),
            endpoint_type: core::default::Default::default(),
        }
    }
}

pub struct DataKendraExperienceEndpointsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataKendraExperienceEndpointsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataKendraExperienceEndpointsElRef {
        DataKendraExperienceEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraExperienceEndpointsElRef {
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

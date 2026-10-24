use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ElasticsearchDomainSamlOptionsData {
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
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    saml_options: std::option::Option<std::vec::Vec<ElasticsearchDomainSamlOptionsSamlOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ElasticsearchDomainSamlOptionsTimeoutsEl>,
    dynamic: ElasticsearchDomainSamlOptionsDynamic,
}

struct ElasticsearchDomainSamlOptions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ElasticsearchDomainSamlOptionsData>,
}

#[derive(Clone)]
pub struct ElasticsearchDomainSamlOptions(std::rc::Rc<ElasticsearchDomainSamlOptions_>);

impl ElasticsearchDomainSamlOptions {
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

    #[doc = "Set the field `saml_options`.\n"]
    pub fn set_saml_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainSamlOptionsSamlOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().saml_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.saml_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ElasticsearchDomainSamlOptionsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> terrars::ListRef<ElasticsearchDomainSamlOptionsSamlOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomainSamlOptionsTimeoutsElRef {
        ElasticsearchDomainSamlOptionsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ElasticsearchDomainSamlOptions {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ElasticsearchDomainSamlOptions { }

impl terrars::ToListMappable for ElasticsearchDomainSamlOptions {
    type O = terrars::ListRef<ElasticsearchDomainSamlOptionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ElasticsearchDomainSamlOptions_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_elasticsearch_domain_saml_options".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElasticsearchDomainSamlOptions {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomainSamlOptions {
    pub fn build(self, stack: &mut terrars::Stack) -> ElasticsearchDomainSamlOptions {
        let out = ElasticsearchDomainSamlOptions(std::rc::Rc::new(ElasticsearchDomainSamlOptions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ElasticsearchDomainSamlOptionsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                saml_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElasticsearchDomainSamlOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainSamlOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ElasticsearchDomainSamlOptionsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `saml_options` after provisioning.\n"]
    pub fn saml_options(&self) -> terrars::ListRef<ElasticsearchDomainSamlOptionsSamlOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.saml_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomainSamlOptionsTimeoutsElRef {
        ElasticsearchDomainSamlOptionsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
    entity_id: terrars::PrimField<std::string::String>,
    metadata_content: terrars::PrimField<std::string::String>,
}

impl ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl { }

impl terrars::ToListMappable for ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
    #[doc = ""]
    pub entity_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub metadata_content: terrars::PrimField<std::string::String>,
}

impl BuildElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
    pub fn build(self) -> ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
        ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl {
            entity_id: self.entity_id,
            metadata_content: self.metadata_content,
        }
    }
}

pub struct ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
        ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\n"]
    pub fn entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_content` after provisioning.\n"]
    pub fn metadata_content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_content", self.base))
    }
}

#[derive(Serialize, Default)]
struct ElasticsearchDomainSamlOptionsSamlOptionsElDynamic {
    idp: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl>>,
}

#[derive(Serialize)]
pub struct ElasticsearchDomainSamlOptionsSamlOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_backend_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_user_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idp: std::option::Option<std::vec::Vec<ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl>>,
    dynamic: ElasticsearchDomainSamlOptionsSamlOptionsElDynamic,
}

impl ElasticsearchDomainSamlOptionsSamlOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_backend_role`.\n"]
    pub fn set_master_backend_role(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.master_backend_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `master_user_name`.\n"]
    pub fn set_master_user_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.master_user_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `roles_key`.\n"]
    pub fn set_roles_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.roles_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_timeout_minutes`.\n"]
    pub fn set_session_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.session_timeout_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_key`.\n"]
    pub fn set_subject_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idp`.\n"]
    pub fn set_idp(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElasticsearchDomainSamlOptionsSamlOptionsElIdpEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.idp = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.idp = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ElasticsearchDomainSamlOptionsSamlOptionsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainSamlOptionsSamlOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainSamlOptionsSamlOptionsEl {}

impl BuildElasticsearchDomainSamlOptionsSamlOptionsEl {
    pub fn build(self) -> ElasticsearchDomainSamlOptionsSamlOptionsEl {
        ElasticsearchDomainSamlOptionsSamlOptionsEl {
            enabled: core::default::Default::default(),
            master_backend_role: core::default::Default::default(),
            master_user_name: core::default::Default::default(),
            roles_key: core::default::Default::default(),
            session_timeout_minutes: core::default::Default::default(),
            subject_key: core::default::Default::default(),
            idp: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainSamlOptionsSamlOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainSamlOptionsSamlOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElasticsearchDomainSamlOptionsSamlOptionsElRef {
        ElasticsearchDomainSamlOptionsSamlOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainSamlOptionsSamlOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `master_backend_role` after provisioning.\n"]
    pub fn master_backend_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_backend_role", self.base))
    }

    #[doc = "Get a reference to the value of field `master_user_name` after provisioning.\n"]
    pub fn master_user_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_user_name", self.base))
    }

    #[doc = "Get a reference to the value of field `roles_key` after provisioning.\n"]
    pub fn roles_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.roles_key", self.base))
    }

    #[doc = "Get a reference to the value of field `session_timeout_minutes` after provisioning.\n"]
    pub fn session_timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_timeout_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_key` after provisioning.\n"]
    pub fn subject_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject_key", self.base))
    }

    #[doc = "Get a reference to the value of field `idp` after provisioning.\n"]
    pub fn idp(&self) -> terrars::ListRef<ElasticsearchDomainSamlOptionsSamlOptionsElIdpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idp", self.base))
    }
}

#[derive(Serialize)]
pub struct ElasticsearchDomainSamlOptionsTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElasticsearchDomainSamlOptionsTimeoutsEl {
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

impl terrars::ToListMappable for ElasticsearchDomainSamlOptionsTimeoutsEl {
    type O = terrars::BlockAssignable<ElasticsearchDomainSamlOptionsTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElasticsearchDomainSamlOptionsTimeoutsEl {}

impl BuildElasticsearchDomainSamlOptionsTimeoutsEl {
    pub fn build(self) -> ElasticsearchDomainSamlOptionsTimeoutsEl {
        ElasticsearchDomainSamlOptionsTimeoutsEl {
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ElasticsearchDomainSamlOptionsTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElasticsearchDomainSamlOptionsTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElasticsearchDomainSamlOptionsTimeoutsElRef {
        ElasticsearchDomainSamlOptionsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElasticsearchDomainSamlOptionsTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
struct ElasticsearchDomainSamlOptionsDynamic {
    saml_options: std::option::Option<terrars::DynamicBlock<ElasticsearchDomainSamlOptionsSamlOptionsEl>>,
}

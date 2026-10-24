use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcrpublicRepositoryData {
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
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    repository_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_data: std::option::Option<std::vec::Vec<EcrpublicRepositoryCatalogDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EcrpublicRepositoryTimeoutsEl>,
    dynamic: EcrpublicRepositoryDynamic,
}

struct EcrpublicRepository_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcrpublicRepositoryData>,
}

#[derive(Clone)]
pub struct EcrpublicRepository(std::rc::Rc<EcrpublicRepository_>);

impl EcrpublicRepository {
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

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `catalog_data`.\n"]
    pub fn set_catalog_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcrpublicRepositoryCatalogDataEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().catalog_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.catalog_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EcrpublicRepositoryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_uri` after provisioning.\n"]
    pub fn repository_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_data` after provisioning.\n"]
    pub fn catalog_data(&self) -> terrars::ListRef<EcrpublicRepositoryCatalogDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcrpublicRepositoryTimeoutsElRef {
        EcrpublicRepositoryTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EcrpublicRepository {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcrpublicRepository { }

impl terrars::ToListMappable for EcrpublicRepository {
    type O = terrars::ListRef<EcrpublicRepositoryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcrpublicRepository_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecrpublic_repository".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcrpublicRepository {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub repository_name: terrars::PrimField<std::string::String>,
}

impl BuildEcrpublicRepository {
    pub fn build(self, stack: &mut terrars::Stack) -> EcrpublicRepository {
        let out = EcrpublicRepository(std::rc::Rc::new(EcrpublicRepository_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcrpublicRepositoryData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                repository_name: self.repository_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                catalog_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcrpublicRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrpublicRepositoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcrpublicRepositoryRef {
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

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `repository_uri` after provisioning.\n"]
    pub fn repository_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_data` after provisioning.\n"]
    pub fn catalog_data(&self) -> terrars::ListRef<EcrpublicRepositoryCatalogDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcrpublicRepositoryTimeoutsElRef {
        EcrpublicRepositoryTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcrpublicRepositoryCatalogDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    about_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architectures: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo_image_blob: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_systems: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcrpublicRepositoryCatalogDataEl {
    #[doc = "Set the field `about_text`.\n"]
    pub fn set_about_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.about_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `architectures`.\n"]
    pub fn set_architectures(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.architectures = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo_image_blob`.\n"]
    pub fn set_logo_image_blob(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logo_image_blob = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_systems`.\n"]
    pub fn set_operating_systems(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.operating_systems = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usage_text`.\n"]
    pub fn set_usage_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.usage_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcrpublicRepositoryCatalogDataEl {
    type O = terrars::BlockAssignable<EcrpublicRepositoryCatalogDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrpublicRepositoryCatalogDataEl {}

impl BuildEcrpublicRepositoryCatalogDataEl {
    pub fn build(self) -> EcrpublicRepositoryCatalogDataEl {
        EcrpublicRepositoryCatalogDataEl {
            about_text: core::default::Default::default(),
            architectures: core::default::Default::default(),
            description: core::default::Default::default(),
            logo_image_blob: core::default::Default::default(),
            operating_systems: core::default::Default::default(),
            usage_text: core::default::Default::default(),
        }
    }
}

pub struct EcrpublicRepositoryCatalogDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrpublicRepositoryCatalogDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcrpublicRepositoryCatalogDataElRef {
        EcrpublicRepositoryCatalogDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrpublicRepositoryCatalogDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `about_text` after provisioning.\n"]
    pub fn about_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.about_text", self.base))
    }

    #[doc = "Get a reference to the value of field `architectures` after provisioning.\n"]
    pub fn architectures(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.architectures", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `logo_image_blob` after provisioning.\n"]
    pub fn logo_image_blob(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo_image_blob", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_systems` after provisioning.\n"]
    pub fn operating_systems(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.operating_systems", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_text` after provisioning.\n"]
    pub fn usage_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_text", self.base))
    }
}

#[derive(Serialize)]
pub struct EcrpublicRepositoryTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcrpublicRepositoryTimeoutsEl {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcrpublicRepositoryTimeoutsEl {
    type O = terrars::BlockAssignable<EcrpublicRepositoryTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrpublicRepositoryTimeoutsEl {}

impl BuildEcrpublicRepositoryTimeoutsEl {
    pub fn build(self) -> EcrpublicRepositoryTimeoutsEl {
        EcrpublicRepositoryTimeoutsEl { delete: core::default::Default::default() }
    }
}

pub struct EcrpublicRepositoryTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrpublicRepositoryTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcrpublicRepositoryTimeoutsElRef {
        EcrpublicRepositoryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrpublicRepositoryTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrpublicRepositoryDynamic {
    catalog_data: std::option::Option<terrars::DynamicBlock<EcrpublicRepositoryCatalogDataEl>>,
}

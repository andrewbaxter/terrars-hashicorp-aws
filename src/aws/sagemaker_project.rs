use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerProjectData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_description: std::option::Option<terrars::PrimField<std::string::String>>,
    project_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_catalog_provisioning_details: std
    ::option
    ::Option<std::vec::Vec<SagemakerProjectServiceCatalogProvisioningDetailsEl>>,
    dynamic: SagemakerProjectDynamic,
}

struct SagemakerProject_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerProjectData>,
}

#[derive(Clone)]
pub struct SagemakerProject(std::rc::Rc<SagemakerProject_>);

impl SagemakerProject {
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

    #[doc = "Set the field `project_description`.\n"]
    pub fn set_project_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project_description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `service_catalog_provisioning_details`.\n"]
    pub fn set_service_catalog_provisioning_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerProjectServiceCatalogProvisioningDetailsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_catalog_provisioning_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_catalog_provisioning_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_description` after provisioning.\n"]
    pub fn project_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_catalog_provisioning_details` after provisioning.\n"]
    pub fn service_catalog_provisioning_details(
        &self,
    ) -> terrars::ListRef<SagemakerProjectServiceCatalogProvisioningDetailsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.service_catalog_provisioning_details", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SagemakerProject {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerProject { }

impl terrars::ToListMappable for SagemakerProject {
    type O = terrars::ListRef<SagemakerProjectRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerProject_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_project".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerProject {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub project_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerProject {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerProject {
        let out = SagemakerProject(std::rc::Rc::new(SagemakerProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerProjectData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                project_description: core::default::Default::default(),
                project_name: self.project_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                service_catalog_provisioning_details: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerProjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerProjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerProjectRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_description` after provisioning.\n"]
    pub fn project_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_catalog_provisioning_details` after provisioning.\n"]
    pub fn service_catalog_provisioning_details(
        &self,
    ) -> terrars::ListRef<SagemakerProjectServiceCatalogProvisioningDetailsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.service_catalog_provisioning_details", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    type O = terrars::BlockAssignable<SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
    pub fn build(self) -> SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
        SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl {
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}

pub struct SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
        SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

#[derive(Serialize, Default)]
struct SagemakerProjectServiceCatalogProvisioningDetailsElDynamic {
    provisioning_parameter: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl>>,
}

#[derive(Serialize)]
pub struct SagemakerProjectServiceCatalogProvisioningDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_id: std::option::Option<terrars::PrimField<std::string::String>>,
    product_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_artifact_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_parameter: std
    ::option
    ::Option<std::vec::Vec<SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl>>,
    dynamic: SagemakerProjectServiceCatalogProvisioningDetailsElDynamic,
}

impl SagemakerProjectServiceCatalogProvisioningDetailsEl {
    #[doc = "Set the field `path_id`.\n"]
    pub fn set_path_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioning_artifact_id`.\n"]
    pub fn set_provisioning_artifact_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.provisioning_artifact_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioning_parameter`.\n"]
    pub fn set_provisioning_parameter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provisioning_parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioning_parameter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerProjectServiceCatalogProvisioningDetailsEl {
    type O = terrars::BlockAssignable<SagemakerProjectServiceCatalogProvisioningDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerProjectServiceCatalogProvisioningDetailsEl {
    #[doc = ""]
    pub product_id: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerProjectServiceCatalogProvisioningDetailsEl {
    pub fn build(self) -> SagemakerProjectServiceCatalogProvisioningDetailsEl {
        SagemakerProjectServiceCatalogProvisioningDetailsEl {
            path_id: core::default::Default::default(),
            product_id: self.product_id,
            provisioning_artifact_id: core::default::Default::default(),
            provisioning_parameter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerProjectServiceCatalogProvisioningDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerProjectServiceCatalogProvisioningDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerProjectServiceCatalogProvisioningDetailsElRef {
        SagemakerProjectServiceCatalogProvisioningDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerProjectServiceCatalogProvisioningDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path_id` after provisioning.\n"]
    pub fn path_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_id", self.base))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioning_artifact_id` after provisioning.\n"]
    pub fn provisioning_artifact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_artifact_id", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioning_parameter` after provisioning.\n"]
    pub fn provisioning_parameter(
        &self,
    ) -> terrars::ListRef<SagemakerProjectServiceCatalogProvisioningDetailsElProvisioningParameterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioning_parameter", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerProjectDynamic {
    service_catalog_provisioning_details: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerProjectServiceCatalogProvisioningDetailsEl>>,
}

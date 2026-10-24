use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ServicecatalogProvisionedProductData {
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
    accept_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_errors: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_arns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_artifact_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_artifact_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_physical_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_parameters: std::option::Option<std::vec::Vec<ServicecatalogProvisionedProductProvisioningParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_set_provisioning_preferences: std
    ::option
    ::Option<std::vec::Vec<ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ServicecatalogProvisionedProductTimeoutsEl>,
    dynamic: ServicecatalogProvisionedProductDynamic,
}

struct ServicecatalogProvisionedProduct_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ServicecatalogProvisionedProductData>,
}

#[derive(Clone)]
pub struct ServicecatalogProvisionedProduct(std::rc::Rc<ServicecatalogProvisionedProduct_>);

impl ServicecatalogProvisionedProduct {
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

    #[doc = "Set the field `accept_language`.\n"]
    pub fn set_accept_language(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().accept_language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_errors`.\n"]
    pub fn set_ignore_errors(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_arns`.\n"]
    pub fn set_notification_arns(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().notification_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path_id`.\n"]
    pub fn set_path_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().path_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path_name`.\n"]
    pub fn set_path_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().path_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_id`.\n"]
    pub fn set_product_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_name`.\n"]
    pub fn set_product_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioning_artifact_id`.\n"]
    pub fn set_provisioning_artifact_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().provisioning_artifact_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioning_artifact_name`.\n"]
    pub fn set_provisioning_artifact_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().provisioning_artifact_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retain_physical_resources`.\n"]
    pub fn set_retain_physical_resources(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retain_physical_resources = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `provisioning_parameters`.\n"]
    pub fn set_provisioning_parameters(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ServicecatalogProvisionedProductProvisioningParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().provisioning_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.provisioning_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stack_set_provisioning_preferences`.\n"]
    pub fn set_stack_set_provisioning_preferences(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stack_set_provisioning_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stack_set_provisioning_preferences = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ServicecatalogProvisionedProductTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_dashboard_names` after provisioning.\n"]
    pub fn cloudwatch_dashboard_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cloudwatch_dashboard_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_errors` after provisioning.\n"]
    pub fn ignore_errors(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_provisioning_record_id` after provisioning.\n"]
    pub fn last_provisioning_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_provisioning_record_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_record_id` after provisioning.\n"]
    pub fn last_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_record_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_successful_provisioning_record_id` after provisioning.\n"]
    pub fn last_successful_provisioning_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_successful_provisioning_record_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_role_arn` after provisioning.\n"]
    pub fn launch_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_arns` after provisioning.\n"]
    pub fn notification_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> terrars::SetRef<ServicecatalogProvisionedProductOutputsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.outputs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_id` after provisioning.\n"]
    pub fn path_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_name` after provisioning.\n"]
    pub fn path_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_name` after provisioning.\n"]
    pub fn product_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_artifact_id` after provisioning.\n"]
    pub fn provisioning_artifact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_artifact_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_artifact_name` after provisioning.\n"]
    pub fn provisioning_artifact_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_artifact_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_physical_resources` after provisioning.\n"]
    pub fn retain_physical_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_physical_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_parameters` after provisioning.\n"]
    pub fn provisioning_parameters(
        &self,
    ) -> terrars::ListRef<ServicecatalogProvisionedProductProvisioningParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioning_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_provisioning_preferences` after provisioning.\n"]
    pub fn stack_set_provisioning_preferences(
        &self,
    ) -> terrars::ListRef<ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.stack_set_provisioning_preferences", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ServicecatalogProvisionedProductTimeoutsElRef {
        ServicecatalogProvisionedProductTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ServicecatalogProvisionedProduct {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ServicecatalogProvisionedProduct { }

impl terrars::ToListMappable for ServicecatalogProvisionedProduct {
    type O = terrars::ListRef<ServicecatalogProvisionedProductRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ServicecatalogProvisionedProduct_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_servicecatalog_provisioned_product".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServicecatalogProvisionedProduct {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildServicecatalogProvisionedProduct {
    pub fn build(self, stack: &mut terrars::Stack) -> ServicecatalogProvisionedProduct {
        let out = ServicecatalogProvisionedProduct(std::rc::Rc::new(ServicecatalogProvisionedProduct_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ServicecatalogProvisionedProductData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                accept_language: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_errors: core::default::Default::default(),
                name: self.name,
                notification_arns: core::default::Default::default(),
                path_id: core::default::Default::default(),
                path_name: core::default::Default::default(),
                product_id: core::default::Default::default(),
                product_name: core::default::Default::default(),
                provisioning_artifact_id: core::default::Default::default(),
                provisioning_artifact_name: core::default::Default::default(),
                retain_physical_resources: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                provisioning_parameters: core::default::Default::default(),
                stack_set_provisioning_preferences: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServicecatalogProvisionedProductRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServicecatalogProvisionedProductRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ServicecatalogProvisionedProductRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_dashboard_names` after provisioning.\n"]
    pub fn cloudwatch_dashboard_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cloudwatch_dashboard_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_errors` after provisioning.\n"]
    pub fn ignore_errors(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_provisioning_record_id` after provisioning.\n"]
    pub fn last_provisioning_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_provisioning_record_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_record_id` after provisioning.\n"]
    pub fn last_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_record_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_successful_provisioning_record_id` after provisioning.\n"]
    pub fn last_successful_provisioning_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_successful_provisioning_record_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_role_arn` after provisioning.\n"]
    pub fn launch_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_arns` after provisioning.\n"]
    pub fn notification_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outputs` after provisioning.\n"]
    pub fn outputs(&self) -> terrars::SetRef<ServicecatalogProvisionedProductOutputsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.outputs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_id` after provisioning.\n"]
    pub fn path_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_name` after provisioning.\n"]
    pub fn path_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_name` after provisioning.\n"]
    pub fn product_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_artifact_id` after provisioning.\n"]
    pub fn provisioning_artifact_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_artifact_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_artifact_name` after provisioning.\n"]
    pub fn provisioning_artifact_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_artifact_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retain_physical_resources` after provisioning.\n"]
    pub fn retain_physical_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_physical_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioning_parameters` after provisioning.\n"]
    pub fn provisioning_parameters(
        &self,
    ) -> terrars::ListRef<ServicecatalogProvisionedProductProvisioningParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioning_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_set_provisioning_preferences` after provisioning.\n"]
    pub fn stack_set_provisioning_preferences(
        &self,
    ) -> terrars::ListRef<ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.stack_set_provisioning_preferences", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ServicecatalogProvisionedProductTimeoutsElRef {
        ServicecatalogProvisionedProductTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ServicecatalogProvisionedProductOutputsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServicecatalogProvisionedProductOutputsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServicecatalogProvisionedProductOutputsEl {
    type O = terrars::BlockAssignable<ServicecatalogProvisionedProductOutputsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicecatalogProvisionedProductOutputsEl {}

impl BuildServicecatalogProvisionedProductOutputsEl {
    pub fn build(self) -> ServicecatalogProvisionedProductOutputsEl {
        ServicecatalogProvisionedProductOutputsEl {
            description: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct ServicecatalogProvisionedProductOutputsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServicecatalogProvisionedProductOutputsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServicecatalogProvisionedProductOutputsElRef {
        ServicecatalogProvisionedProductOutputsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicecatalogProvisionedProductOutputsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
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

#[derive(Serialize)]
pub struct ServicecatalogProvisionedProductProvisioningParametersEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_previous_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServicecatalogProvisionedProductProvisioningParametersEl {
    #[doc = "Set the field `use_previous_value`.\n"]
    pub fn set_use_previous_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_previous_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServicecatalogProvisionedProductProvisioningParametersEl {
    type O = terrars::BlockAssignable<ServicecatalogProvisionedProductProvisioningParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicecatalogProvisionedProductProvisioningParametersEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildServicecatalogProvisionedProductProvisioningParametersEl {
    pub fn build(self) -> ServicecatalogProvisionedProductProvisioningParametersEl {
        ServicecatalogProvisionedProductProvisioningParametersEl {
            key: self.key,
            use_previous_value: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct ServicecatalogProvisionedProductProvisioningParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServicecatalogProvisionedProductProvisioningParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServicecatalogProvisionedProductProvisioningParametersElRef {
        ServicecatalogProvisionedProductProvisioningParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicecatalogProvisionedProductProvisioningParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `use_previous_value` after provisioning.\n"]
    pub fn use_previous_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_previous_value", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accounts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_tolerance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_tolerance_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrency_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrency_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
    #[doc = "Set the field `accounts`.\n"]
    pub fn set_accounts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_tolerance_count`.\n"]
    pub fn set_failure_tolerance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_tolerance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_tolerance_percentage`.\n"]
    pub fn set_failure_tolerance_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_tolerance_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrency_count`.\n"]
    pub fn set_max_concurrency_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrency_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrency_percentage`.\n"]
    pub fn set_max_concurrency_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrency_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
    type O = terrars::BlockAssignable<ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {}

impl BuildServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
    pub fn build(self) -> ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
        ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl {
            accounts: core::default::Default::default(),
            failure_tolerance_count: core::default::Default::default(),
            failure_tolerance_percentage: core::default::Default::default(),
            max_concurrency_count: core::default::Default::default(),
            max_concurrency_percentage: core::default::Default::default(),
            regions: core::default::Default::default(),
        }
    }
}

pub struct ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
        ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicecatalogProvisionedProductStackSetProvisioningPreferencesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accounts` after provisioning.\n"]
    pub fn accounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_count` after provisioning.\n"]
    pub fn failure_tolerance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_tolerance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_tolerance_percentage` after provisioning.\n"]
    pub fn failure_tolerance_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_tolerance_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `max_concurrency_count` after provisioning.\n"]
    pub fn max_concurrency_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_concurrency_percentage` after provisioning.\n"]
    pub fn max_concurrency_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrency_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}

#[derive(Serialize)]
pub struct ServicecatalogProvisionedProductTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ServicecatalogProvisionedProductTimeoutsEl {
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

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ServicecatalogProvisionedProductTimeoutsEl {
    type O = terrars::BlockAssignable<ServicecatalogProvisionedProductTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicecatalogProvisionedProductTimeoutsEl {}

impl BuildServicecatalogProvisionedProductTimeoutsEl {
    pub fn build(self) -> ServicecatalogProvisionedProductTimeoutsEl {
        ServicecatalogProvisionedProductTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ServicecatalogProvisionedProductTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ServicecatalogProvisionedProductTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ServicecatalogProvisionedProductTimeoutsElRef {
        ServicecatalogProvisionedProductTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicecatalogProvisionedProductTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServicecatalogProvisionedProductDynamic {
    provisioning_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<ServicecatalogProvisionedProductProvisioningParametersEl>>,
    stack_set_provisioning_preferences: std
    ::option
    ::Option<terrars::DynamicBlock<ServicecatalogProvisionedProductStackSetProvisioningPreferencesEl>>,
}

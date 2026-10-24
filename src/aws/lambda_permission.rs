use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaPermissionData {
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
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_source_token: std::option::Option<terrars::PrimField<std::string::String>>,
    function_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_url_auth_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    principal: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_org_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement_id_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct LambdaPermission_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaPermissionData>,
}

#[derive(Clone)]
pub struct LambdaPermission(std::rc::Rc<LambdaPermission_>);

impl LambdaPermission {
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

    #[doc = "Set the field `event_source_token`.\n"]
    pub fn set_event_source_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().event_source_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `function_url_auth_type`.\n"]
    pub fn set_function_url_auth_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().function_url_auth_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal_org_id`.\n"]
    pub fn set_principal_org_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().principal_org_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().qualifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_account`.\n"]
    pub fn set_source_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_arn`.\n"]
    pub fn set_source_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statement_id`.\n"]
    pub fn set_statement_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().statement_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statement_id_prefix`.\n"]
    pub fn set_statement_id_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().statement_id_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_source_token` after provisioning.\n"]
    pub fn event_source_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_source_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_url_auth_type` after provisioning.\n"]
    pub fn function_url_auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_url_auth_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_org_id` after provisioning.\n"]
    pub fn principal_org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_org_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_account` after provisioning.\n"]
    pub fn source_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement_id` after provisioning.\n"]
    pub fn statement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement_id_prefix` after provisioning.\n"]
    pub fn statement_id_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_id_prefix", self.extract_ref()))
    }
}

impl terrars::Referable for LambdaPermission {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LambdaPermission { }

impl terrars::ToListMappable for LambdaPermission {
    type O = terrars::ListRef<LambdaPermissionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LambdaPermission_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lambda_permission".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaPermission {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub function_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildLambdaPermission {
    pub fn build(self, stack: &mut terrars::Stack) -> LambdaPermission {
        let out = LambdaPermission(std::rc::Rc::new(LambdaPermission_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LambdaPermissionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                event_source_token: core::default::Default::default(),
                function_name: self.function_name,
                function_url_auth_type: core::default::Default::default(),
                id: core::default::Default::default(),
                principal: self.principal,
                principal_org_id: core::default::Default::default(),
                qualifier: core::default::Default::default(),
                source_account: core::default::Default::default(),
                source_arn: core::default::Default::default(),
                statement_id: core::default::Default::default(),
                statement_id_prefix: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaPermissionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaPermissionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaPermissionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_source_token` after provisioning.\n"]
    pub fn event_source_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_source_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_url_auth_type` after provisioning.\n"]
    pub fn function_url_auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_url_auth_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_org_id` after provisioning.\n"]
    pub fn principal_org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_org_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_account` after provisioning.\n"]
    pub fn source_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement_id` after provisioning.\n"]
    pub fn statement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement_id_prefix` after provisioning.\n"]
    pub fn statement_id_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_id_prefix", self.extract_ref()))
    }
}

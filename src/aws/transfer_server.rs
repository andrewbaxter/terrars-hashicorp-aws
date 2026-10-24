use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TransferServerData {
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
    certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_authentication_login_banner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_authentication_login_banner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_details: std::option::Option<std::vec::Vec<TransferServerEndpointDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow_details: std::option::Option<std::vec::Vec<TransferServerWorkflowDetailsEl>>,
    dynamic: TransferServerDynamic,
}

struct TransferServer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TransferServerData>,
}

#[derive(Clone)]
pub struct TransferServer(std::rc::Rc<TransferServer_>);

impl TransferServer {
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

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `directory_id`.\n"]
    pub fn set_directory_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().directory_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_type`.\n"]
    pub fn set_endpoint_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().endpoint_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `function`.\n"]
    pub fn set_function(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().function = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_key`.\n"]
    pub fn set_host_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().host_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_provider_type`.\n"]
    pub fn set_identity_provider_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().identity_provider_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invocation_role`.\n"]
    pub fn set_invocation_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().invocation_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_role`.\n"]
    pub fn set_logging_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().logging_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_authentication_login_banner`.\n"]
    pub fn set_post_authentication_login_banner(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().post_authentication_login_banner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pre_authentication_login_banner`.\n"]
    pub fn set_pre_authentication_login_banner(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().pre_authentication_login_banner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocols`.\n"]
    pub fn set_protocols(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_policy_name`.\n"]
    pub fn set_security_policy_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().security_policy_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_details`.\n"]
    pub fn set_endpoint_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferServerEndpointDetailsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workflow_details`.\n"]
    pub fn set_workflow_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferServerWorkflowDetailsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workflow_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workflow_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_key` after provisioning.\n"]
    pub fn host_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_key_fingerprint` after provisioning.\n"]
    pub fn host_key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_key_fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_provider_type` after provisioning.\n"]
    pub fn identity_provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invocation_role` after provisioning.\n"]
    pub fn invocation_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_role` after provisioning.\n"]
    pub fn logging_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `post_authentication_login_banner` after provisioning.\n"]
    pub fn post_authentication_login_banner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_authentication_login_banner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pre_authentication_login_banner` after provisioning.\n"]
    pub fn pre_authentication_login_banner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pre_authentication_login_banner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_policy_name` after provisioning.\n"]
    pub fn security_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_policy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_details` after provisioning.\n"]
    pub fn endpoint_details(&self) -> terrars::ListRef<TransferServerEndpointDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow_details` after provisioning.\n"]
    pub fn workflow_details(&self) -> terrars::ListRef<TransferServerWorkflowDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workflow_details", self.extract_ref()))
    }
}

impl terrars::Referable for TransferServer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TransferServer { }

impl terrars::ToListMappable for TransferServer {
    type O = terrars::ListRef<TransferServerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TransferServer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_transfer_server".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTransferServer {
    pub tf_id: std::string::String,
}

impl BuildTransferServer {
    pub fn build(self, stack: &mut terrars::Stack) -> TransferServer {
        let out = TransferServer(std::rc::Rc::new(TransferServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TransferServerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                certificate: core::default::Default::default(),
                directory_id: core::default::Default::default(),
                domain: core::default::Default::default(),
                endpoint_type: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                function: core::default::Default::default(),
                host_key: core::default::Default::default(),
                id: core::default::Default::default(),
                identity_provider_type: core::default::Default::default(),
                invocation_role: core::default::Default::default(),
                logging_role: core::default::Default::default(),
                post_authentication_login_banner: core::default::Default::default(),
                pre_authentication_login_banner: core::default::Default::default(),
                protocols: core::default::Default::default(),
                security_policy_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                url: core::default::Default::default(),
                endpoint_details: core::default::Default::default(),
                workflow_details: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TransferServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TransferServerRef {
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

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_key` after provisioning.\n"]
    pub fn host_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_key_fingerprint` after provisioning.\n"]
    pub fn host_key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_key_fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_provider_type` after provisioning.\n"]
    pub fn identity_provider_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invocation_role` after provisioning.\n"]
    pub fn invocation_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_role` after provisioning.\n"]
    pub fn logging_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `post_authentication_login_banner` after provisioning.\n"]
    pub fn post_authentication_login_banner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_authentication_login_banner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `pre_authentication_login_banner` after provisioning.\n"]
    pub fn pre_authentication_login_banner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pre_authentication_login_banner", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_policy_name` after provisioning.\n"]
    pub fn security_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_policy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_details` after provisioning.\n"]
    pub fn endpoint_details(&self) -> terrars::ListRef<TransferServerEndpointDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workflow_details` after provisioning.\n"]
    pub fn workflow_details(&self) -> terrars::ListRef<TransferServerWorkflowDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workflow_details", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TransferServerEndpointDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_allocation_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferServerEndpointDetailsEl {
    #[doc = "Set the field `address_allocation_ids`.\n"]
    pub fn set_address_allocation_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.address_allocation_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_endpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferServerEndpointDetailsEl {
    type O = terrars::BlockAssignable<TransferServerEndpointDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferServerEndpointDetailsEl {}

impl BuildTransferServerEndpointDetailsEl {
    pub fn build(self) -> TransferServerEndpointDetailsEl {
        TransferServerEndpointDetailsEl {
            address_allocation_ids: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_endpoint_id: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct TransferServerEndpointDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferServerEndpointDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferServerEndpointDetailsElRef {
        TransferServerEndpointDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferServerEndpointDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_allocation_ids` after provisioning.\n"]
    pub fn address_allocation_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.address_allocation_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferServerWorkflowDetailsElOnUploadEl {
    execution_role: terrars::PrimField<std::string::String>,
    workflow_id: terrars::PrimField<std::string::String>,
}

impl TransferServerWorkflowDetailsElOnUploadEl { }

impl terrars::ToListMappable for TransferServerWorkflowDetailsElOnUploadEl {
    type O = terrars::BlockAssignable<TransferServerWorkflowDetailsElOnUploadEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferServerWorkflowDetailsElOnUploadEl {
    #[doc = ""]
    pub execution_role: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub workflow_id: terrars::PrimField<std::string::String>,
}

impl BuildTransferServerWorkflowDetailsElOnUploadEl {
    pub fn build(self) -> TransferServerWorkflowDetailsElOnUploadEl {
        TransferServerWorkflowDetailsElOnUploadEl {
            execution_role: self.execution_role,
            workflow_id: self.workflow_id,
        }
    }
}

pub struct TransferServerWorkflowDetailsElOnUploadElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferServerWorkflowDetailsElOnUploadElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferServerWorkflowDetailsElOnUploadElRef {
        TransferServerWorkflowDetailsElOnUploadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferServerWorkflowDetailsElOnUploadElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc = "Get a reference to the value of field `workflow_id` after provisioning.\n"]
    pub fn workflow_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workflow_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferServerWorkflowDetailsElDynamic {
    on_upload: std::option::Option<terrars::DynamicBlock<TransferServerWorkflowDetailsElOnUploadEl>>,
}

#[derive(Serialize)]
pub struct TransferServerWorkflowDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_upload: std::option::Option<std::vec::Vec<TransferServerWorkflowDetailsElOnUploadEl>>,
    dynamic: TransferServerWorkflowDetailsElDynamic,
}

impl TransferServerWorkflowDetailsEl {
    #[doc = "Set the field `on_upload`.\n"]
    pub fn set_on_upload(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferServerWorkflowDetailsElOnUploadEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_upload = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_upload = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferServerWorkflowDetailsEl {
    type O = terrars::BlockAssignable<TransferServerWorkflowDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferServerWorkflowDetailsEl {}

impl BuildTransferServerWorkflowDetailsEl {
    pub fn build(self) -> TransferServerWorkflowDetailsEl {
        TransferServerWorkflowDetailsEl {
            on_upload: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct TransferServerWorkflowDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferServerWorkflowDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferServerWorkflowDetailsElRef {
        TransferServerWorkflowDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferServerWorkflowDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_upload` after provisioning.\n"]
    pub fn on_upload(&self) -> terrars::ListRef<TransferServerWorkflowDetailsElOnUploadElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_upload", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferServerDynamic {
    endpoint_details: std::option::Option<terrars::DynamicBlock<TransferServerEndpointDetailsEl>>,
    workflow_details: std::option::Option<terrars::DynamicBlock<TransferServerWorkflowDetailsEl>>,
}

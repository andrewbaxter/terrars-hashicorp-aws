use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TransferServerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_provider_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_authentication_login_banner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_authentication_login_banner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocols: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_policy_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_details: Option<Vec<TransferServerEndpointDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow_details: Option<Vec<TransferServerWorkflowDetailsEl>>,
    dynamic: TransferServerDynamic,
}

struct TransferServer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TransferServerData>,
}

#[derive(Clone)]
pub struct TransferServer(Rc<TransferServer_>);

impl TransferServer {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `certificate`.\n"]
    pub fn set_certificate(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate = Some(v.into());
        self
    }

    #[doc= "Set the field `directory_id`.\n"]
    pub fn set_directory_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().directory_id = Some(v.into());
        self
    }

    #[doc= "Set the field `domain`.\n"]
    pub fn set_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().domain = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint_type`.\n"]
    pub fn set_endpoint_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().endpoint_type = Some(v.into());
        self
    }

    #[doc= "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `function`.\n"]
    pub fn set_function(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().function = Some(v.into());
        self
    }

    #[doc= "Set the field `host_key`.\n"]
    pub fn set_host_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().host_key = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_provider_type`.\n"]
    pub fn set_identity_provider_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().identity_provider_type = Some(v.into());
        self
    }

    #[doc= "Set the field `invocation_role`.\n"]
    pub fn set_invocation_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().invocation_role = Some(v.into());
        self
    }

    #[doc= "Set the field `logging_role`.\n"]
    pub fn set_logging_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().logging_role = Some(v.into());
        self
    }

    #[doc= "Set the field `post_authentication_login_banner`.\n"]
    pub fn set_post_authentication_login_banner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().post_authentication_login_banner = Some(v.into());
        self
    }

    #[doc= "Set the field `pre_authentication_login_banner`.\n"]
    pub fn set_pre_authentication_login_banner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().pre_authentication_login_banner = Some(v.into());
        self
    }

    #[doc= "Set the field `protocols`.\n"]
    pub fn set_protocols(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().protocols = Some(v.into());
        self
    }

    #[doc= "Set the field `security_policy_name`.\n"]
    pub fn set_security_policy_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_policy_name = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `url`.\n"]
    pub fn set_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().url = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint_details`.\n"]
    pub fn set_endpoint_details(self, v: impl Into<BlockAssignable<TransferServerEndpointDetailsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_details = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_details = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `workflow_details`.\n"]
    pub fn set_workflow_details(self, v: impl Into<BlockAssignable<TransferServerWorkflowDetailsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workflow_details = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workflow_details = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host_key` after provisioning.\n"]
    pub fn host_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host_key_fingerprint` after provisioning.\n"]
    pub fn host_key_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_key_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity_provider_type` after provisioning.\n"]
    pub fn identity_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `invocation_role` after provisioning.\n"]
    pub fn invocation_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.invocation_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_role` after provisioning.\n"]
    pub fn logging_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `post_authentication_login_banner` after provisioning.\n"]
    pub fn post_authentication_login_banner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.post_authentication_login_banner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pre_authentication_login_banner` after provisioning.\n"]
    pub fn pre_authentication_login_banner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_authentication_login_banner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_policy_name` after provisioning.\n"]
    pub fn security_policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_policy_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_details` after provisioning.\n"]
    pub fn endpoint_details(&self) -> ListRef<TransferServerEndpointDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint_details", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workflow_details` after provisioning.\n"]
    pub fn workflow_details(&self) -> ListRef<TransferServerWorkflowDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workflow_details", self.extract_ref()))
    }
}

impl Resource for TransferServer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for TransferServer {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for TransferServer {
    type O = ListRef<TransferServerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TransferServer_ {
    fn extract_resource_type(&self) -> String {
        "aws_transfer_server".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTransferServer {
    pub tf_id: String,
}

impl BuildTransferServer {
    pub fn build(self, stack: &mut Stack) -> TransferServer {
        let out = TransferServer(Rc::new(TransferServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TransferServerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TransferServerRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferServerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TransferServerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host_key` after provisioning.\n"]
    pub fn host_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host_key_fingerprint` after provisioning.\n"]
    pub fn host_key_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_key_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity_provider_type` after provisioning.\n"]
    pub fn identity_provider_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_provider_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `invocation_role` after provisioning.\n"]
    pub fn invocation_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.invocation_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_role` after provisioning.\n"]
    pub fn logging_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.logging_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `post_authentication_login_banner` after provisioning.\n"]
    pub fn post_authentication_login_banner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.post_authentication_login_banner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pre_authentication_login_banner` after provisioning.\n"]
    pub fn pre_authentication_login_banner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pre_authentication_login_banner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.protocols", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_policy_name` after provisioning.\n"]
    pub fn security_policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_policy_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_details` after provisioning.\n"]
    pub fn endpoint_details(&self) -> ListRef<TransferServerEndpointDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoint_details", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workflow_details` after provisioning.\n"]
    pub fn workflow_details(&self) -> ListRef<TransferServerWorkflowDetailsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.workflow_details", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TransferServerEndpointDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_allocation_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl TransferServerEndpointDetailsEl {
    #[doc= "Set the field `address_allocation_ids`.\n"]
    pub fn set_address_allocation_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.address_allocation_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_endpoint_id`.\n"]
    pub fn set_vpc_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_endpoint_id = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}

impl ToListMappable for TransferServerEndpointDetailsEl {
    type O = BlockAssignable<TransferServerEndpointDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for TransferServerEndpointDetailsElRef {
    fn new(shared: StackShared, base: String) -> TransferServerEndpointDetailsElRef {
        TransferServerEndpointDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferServerEndpointDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address_allocation_ids` after provisioning.\n"]
    pub fn address_allocation_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.address_allocation_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferServerWorkflowDetailsElOnUploadEl {
    execution_role: PrimField<String>,
    workflow_id: PrimField<String>,
}

impl TransferServerWorkflowDetailsElOnUploadEl { }

impl ToListMappable for TransferServerWorkflowDetailsElOnUploadEl {
    type O = BlockAssignable<TransferServerWorkflowDetailsElOnUploadEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferServerWorkflowDetailsElOnUploadEl {
    #[doc= ""]
    pub execution_role: PrimField<String>,
    #[doc= ""]
    pub workflow_id: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for TransferServerWorkflowDetailsElOnUploadElRef {
    fn new(shared: StackShared, base: String) -> TransferServerWorkflowDetailsElOnUploadElRef {
        TransferServerWorkflowDetailsElOnUploadElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferServerWorkflowDetailsElOnUploadElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `execution_role` after provisioning.\n"]
    pub fn execution_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.execution_role", self.base))
    }

    #[doc= "Get a reference to the value of field `workflow_id` after provisioning.\n"]
    pub fn workflow_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workflow_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferServerWorkflowDetailsElDynamic {
    on_upload: Option<DynamicBlock<TransferServerWorkflowDetailsElOnUploadEl>>,
}

#[derive(Serialize)]
pub struct TransferServerWorkflowDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_upload: Option<Vec<TransferServerWorkflowDetailsElOnUploadEl>>,
    dynamic: TransferServerWorkflowDetailsElDynamic,
}

impl TransferServerWorkflowDetailsEl {
    #[doc= "Set the field `on_upload`.\n"]
    pub fn set_on_upload(mut self, v: impl Into<BlockAssignable<TransferServerWorkflowDetailsElOnUploadEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_upload = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_upload = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for TransferServerWorkflowDetailsEl {
    type O = BlockAssignable<TransferServerWorkflowDetailsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct TransferServerWorkflowDetailsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TransferServerWorkflowDetailsElRef {
    fn new(shared: StackShared, base: String) -> TransferServerWorkflowDetailsElRef {
        TransferServerWorkflowDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferServerWorkflowDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `on_upload` after provisioning.\n"]
    pub fn on_upload(&self) -> ListRef<TransferServerWorkflowDetailsElOnUploadElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_upload", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferServerDynamic {
    endpoint_details: Option<DynamicBlock<TransferServerEndpointDetailsEl>>,
    workflow_details: Option<DynamicBlock<TransferServerWorkflowDetailsEl>>,
}

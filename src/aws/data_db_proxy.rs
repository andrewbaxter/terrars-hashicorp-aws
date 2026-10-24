use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDbProxyData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

struct DataDbProxy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDbProxyData>,
}

#[derive(Clone)]
pub struct DataDbProxy(std::rc::Rc<DataDbProxy_>);

impl DataDbProxy {
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

    #[doc = "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> terrars::SetRef<DataDbProxyAuthElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `debug_logging` after provisioning.\n"]
    pub fn debug_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.debug_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_family` after provisioning.\n"]
    pub fn engine_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_client_timeout` after provisioning.\n"]
    pub fn idle_client_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_client_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_tls` after provisioning.\n"]
    pub fn require_tls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_subnet_ids` after provisioning.\n"]
    pub fn vpc_subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_subnet_ids", self.extract_ref()))
    }
}

impl terrars::Referable for DataDbProxy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDbProxy { }

impl terrars::ToListMappable for DataDbProxy {
    type O = terrars::ListRef<DataDbProxyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDbProxy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_db_proxy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDbProxy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataDbProxy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDbProxy {
        let out = DataDbProxy(std::rc::Rc::new(DataDbProxy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDbProxyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDbProxyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDbProxyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDbProxyRef {
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

    #[doc = "Get a reference to the value of field `auth` after provisioning.\n"]
    pub fn auth(&self) -> terrars::SetRef<DataDbProxyAuthElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `debug_logging` after provisioning.\n"]
    pub fn debug_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.debug_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_family` after provisioning.\n"]
    pub fn engine_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_client_timeout` after provisioning.\n"]
    pub fn idle_client_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_client_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `require_tls` after provisioning.\n"]
    pub fn require_tls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_subnet_ids` after provisioning.\n"]
    pub fn vpc_subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_subnet_ids", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDbProxyAuthEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_auth: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDbProxyAuthEl {
    #[doc = "Set the field `auth_scheme`.\n"]
    pub fn set_auth_scheme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_auth`.\n"]
    pub fn set_iam_auth(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.iam_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDbProxyAuthEl {
    type O = terrars::BlockAssignable<DataDbProxyAuthEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDbProxyAuthEl {}

impl BuildDataDbProxyAuthEl {
    pub fn build(self) -> DataDbProxyAuthEl {
        DataDbProxyAuthEl {
            auth_scheme: core::default::Default::default(),
            description: core::default::Default::default(),
            iam_auth: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

pub struct DataDbProxyAuthElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDbProxyAuthElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDbProxyAuthElRef {
        DataDbProxyAuthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDbProxyAuthElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_scheme` after provisioning.\n"]
    pub fn auth_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_scheme", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `iam_auth` after provisioning.\n"]
    pub fn iam_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

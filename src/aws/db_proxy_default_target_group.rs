use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DbProxyDefaultTargetGroupData {
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
    db_proxy_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pool_config: std::option::Option<std::vec::Vec<DbProxyDefaultTargetGroupConnectionPoolConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DbProxyDefaultTargetGroupTimeoutsEl>,
    dynamic: DbProxyDefaultTargetGroupDynamic,
}

struct DbProxyDefaultTargetGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DbProxyDefaultTargetGroupData>,
}

#[derive(Clone)]
pub struct DbProxyDefaultTargetGroup(std::rc::Rc<DbProxyDefaultTargetGroup_>);

impl DbProxyDefaultTargetGroup {
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

    #[doc = "Set the field `connection_pool_config`.\n"]
    pub fn set_connection_pool_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DbProxyDefaultTargetGroupConnectionPoolConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_pool_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_pool_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DbProxyDefaultTargetGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_proxy_name` after provisioning.\n"]
    pub fn db_proxy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_proxy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\n"]
    pub fn connection_pool_config(&self) -> terrars::ListRef<DbProxyDefaultTargetGroupConnectionPoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbProxyDefaultTargetGroupTimeoutsElRef {
        DbProxyDefaultTargetGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DbProxyDefaultTargetGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DbProxyDefaultTargetGroup { }

impl terrars::ToListMappable for DbProxyDefaultTargetGroup {
    type O = terrars::ListRef<DbProxyDefaultTargetGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DbProxyDefaultTargetGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_db_proxy_default_target_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDbProxyDefaultTargetGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub db_proxy_name: terrars::PrimField<std::string::String>,
}

impl BuildDbProxyDefaultTargetGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DbProxyDefaultTargetGroup {
        let out = DbProxyDefaultTargetGroup(std::rc::Rc::new(DbProxyDefaultTargetGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DbProxyDefaultTargetGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                db_proxy_name: self.db_proxy_name,
                id: core::default::Default::default(),
                connection_pool_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DbProxyDefaultTargetGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DbProxyDefaultTargetGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DbProxyDefaultTargetGroupRef {
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

    #[doc = "Get a reference to the value of field `db_proxy_name` after provisioning.\n"]
    pub fn db_proxy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_proxy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\n"]
    pub fn connection_pool_config(&self) -> terrars::ListRef<DbProxyDefaultTargetGroupConnectionPoolConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DbProxyDefaultTargetGroupTimeoutsElRef {
        DbProxyDefaultTargetGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DbProxyDefaultTargetGroupConnectionPoolConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_borrow_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    init_query: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_connections_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_idle_connections_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_pinning_filters: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DbProxyDefaultTargetGroupConnectionPoolConfigEl {
    #[doc = "Set the field `connection_borrow_timeout`.\n"]
    pub fn set_connection_borrow_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.connection_borrow_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `init_query`.\n"]
    pub fn set_init_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.init_query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_connections_percent`.\n"]
    pub fn set_max_connections_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_idle_connections_percent`.\n"]
    pub fn set_max_idle_connections_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_idle_connections_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_pinning_filters`.\n"]
    pub fn set_session_pinning_filters(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.session_pinning_filters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DbProxyDefaultTargetGroupConnectionPoolConfigEl {
    type O = terrars::BlockAssignable<DbProxyDefaultTargetGroupConnectionPoolConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDbProxyDefaultTargetGroupConnectionPoolConfigEl {}

impl BuildDbProxyDefaultTargetGroupConnectionPoolConfigEl {
    pub fn build(self) -> DbProxyDefaultTargetGroupConnectionPoolConfigEl {
        DbProxyDefaultTargetGroupConnectionPoolConfigEl {
            connection_borrow_timeout: core::default::Default::default(),
            init_query: core::default::Default::default(),
            max_connections_percent: core::default::Default::default(),
            max_idle_connections_percent: core::default::Default::default(),
            session_pinning_filters: core::default::Default::default(),
        }
    }
}

pub struct DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
        DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DbProxyDefaultTargetGroupConnectionPoolConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_borrow_timeout` after provisioning.\n"]
    pub fn connection_borrow_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_borrow_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `init_query` after provisioning.\n"]
    pub fn init_query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.init_query", self.base))
    }

    #[doc = "Get a reference to the value of field `max_connections_percent` after provisioning.\n"]
    pub fn max_connections_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_connections_percent` after provisioning.\n"]
    pub fn max_idle_connections_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_idle_connections_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `session_pinning_filters` after provisioning.\n"]
    pub fn session_pinning_filters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.session_pinning_filters", self.base))
    }
}

#[derive(Serialize)]
pub struct DbProxyDefaultTargetGroupTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DbProxyDefaultTargetGroupTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DbProxyDefaultTargetGroupTimeoutsEl {
    type O = terrars::BlockAssignable<DbProxyDefaultTargetGroupTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDbProxyDefaultTargetGroupTimeoutsEl {}

impl BuildDbProxyDefaultTargetGroupTimeoutsEl {
    pub fn build(self) -> DbProxyDefaultTargetGroupTimeoutsEl {
        DbProxyDefaultTargetGroupTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DbProxyDefaultTargetGroupTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DbProxyDefaultTargetGroupTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DbProxyDefaultTargetGroupTimeoutsElRef {
        DbProxyDefaultTargetGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DbProxyDefaultTargetGroupTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct DbProxyDefaultTargetGroupDynamic {
    connection_pool_config: std
    ::option
    ::Option<terrars::DynamicBlock<DbProxyDefaultTargetGroupConnectionPoolConfigEl>>,
}

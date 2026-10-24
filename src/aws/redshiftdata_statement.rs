use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftdataStatementData {
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
    cluster_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    sql: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_event: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workgroup_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<RedshiftdataStatementParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RedshiftdataStatementTimeoutsEl>,
    dynamic: RedshiftdataStatementDynamic,
}

struct RedshiftdataStatement_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RedshiftdataStatementData>,
}

#[derive(Clone)]
pub struct RedshiftdataStatement(std::rc::Rc<RedshiftdataStatement_>);

impl RedshiftdataStatement {
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

    #[doc = "Set the field `cluster_identifier`.\n"]
    pub fn set_cluster_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_user`.\n"]
    pub fn set_db_user(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().db_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().secret_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statement_name`.\n"]
    pub fn set_statement_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().statement_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_event`.\n"]
    pub fn set_with_event(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().with_event = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workgroup_name`.\n"]
    pub fn set_workgroup_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().workgroup_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftdataStatementParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<RedshiftdataStatementTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement_name` after provisioning.\n"]
    pub fn statement_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `with_event` after provisioning.\n"]
    pub fn with_event(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_event", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<RedshiftdataStatementParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedshiftdataStatementTimeoutsElRef {
        RedshiftdataStatementTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for RedshiftdataStatement {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RedshiftdataStatement { }

impl terrars::ToListMappable for RedshiftdataStatement {
    type O = terrars::ListRef<RedshiftdataStatementRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RedshiftdataStatement_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_redshiftdata_statement".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftdataStatement {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sql: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftdataStatement {
    pub fn build(self, stack: &mut terrars::Stack) -> RedshiftdataStatement {
        let out = RedshiftdataStatement(std::rc::Rc::new(RedshiftdataStatement_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RedshiftdataStatementData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_identifier: core::default::Default::default(),
                database: self.database,
                db_user: core::default::Default::default(),
                id: core::default::Default::default(),
                secret_arn: core::default::Default::default(),
                sql: self.sql,
                statement_name: core::default::Default::default(),
                with_event: core::default::Default::default(),
                workgroup_name: core::default::Default::default(),
                parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftdataStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftdataStatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RedshiftdataStatementRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sql` after provisioning.\n"]
    pub fn sql(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `statement_name` after provisioning.\n"]
    pub fn statement_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `with_event` after provisioning.\n"]
    pub fn with_event(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_event", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workgroup_name` after provisioning.\n"]
    pub fn workgroup_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workgroup_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<RedshiftdataStatementParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RedshiftdataStatementTimeoutsElRef {
        RedshiftdataStatementTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedshiftdataStatementParametersEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl RedshiftdataStatementParametersEl { }

impl terrars::ToListMappable for RedshiftdataStatementParametersEl {
    type O = terrars::BlockAssignable<RedshiftdataStatementParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftdataStatementParametersEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftdataStatementParametersEl {
    pub fn build(self) -> RedshiftdataStatementParametersEl {
        RedshiftdataStatementParametersEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct RedshiftdataStatementParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftdataStatementParametersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftdataStatementParametersElRef {
        RedshiftdataStatementParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftdataStatementParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftdataStatementTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RedshiftdataStatementTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftdataStatementTimeoutsEl {
    type O = terrars::BlockAssignable<RedshiftdataStatementTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftdataStatementTimeoutsEl {}

impl BuildRedshiftdataStatementTimeoutsEl {
    pub fn build(self) -> RedshiftdataStatementTimeoutsEl {
        RedshiftdataStatementTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct RedshiftdataStatementTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftdataStatementTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftdataStatementTimeoutsElRef {
        RedshiftdataStatementTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftdataStatementTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftdataStatementDynamic {
    parameters: std::option::Option<terrars::DynamicBlock<RedshiftdataStatementParametersEl>>,
}

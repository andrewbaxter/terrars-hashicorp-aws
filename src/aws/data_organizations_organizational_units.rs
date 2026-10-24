use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOrganizationsOrganizationalUnitsData {
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
    parent_id: terrars::PrimField<std::string::String>,
}

struct DataOrganizationsOrganizationalUnits_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOrganizationsOrganizationalUnitsData>,
}

#[derive(Clone)]
pub struct DataOrganizationsOrganizationalUnits(std::rc::Rc<DataOrganizationsOrganizationalUnits_>);

impl DataOrganizationsOrganizationalUnits {
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

    #[doc = "Get a reference to the value of field `children` after provisioning.\n"]
    pub fn children(&self) -> terrars::ListRef<DataOrganizationsOrganizationalUnitsChildrenElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.children", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataOrganizationsOrganizationalUnits {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOrganizationsOrganizationalUnits { }

impl terrars::ToListMappable for DataOrganizationsOrganizationalUnits {
    type O = terrars::ListRef<DataOrganizationsOrganizationalUnitsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOrganizationsOrganizationalUnits_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_organizations_organizational_units".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationsOrganizationalUnits {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub parent_id: terrars::PrimField<std::string::String>,
}

impl BuildDataOrganizationsOrganizationalUnits {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOrganizationsOrganizationalUnits {
        let out = DataOrganizationsOrganizationalUnits(std::rc::Rc::new(DataOrganizationsOrganizationalUnits_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOrganizationsOrganizationalUnitsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                parent_id: self.parent_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationsOrganizationalUnitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganizationalUnitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationsOrganizationalUnitsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `children` after provisioning.\n"]
    pub fn children(&self) -> terrars::ListRef<DataOrganizationsOrganizationalUnitsChildrenElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.children", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationalUnitsChildrenEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizationsOrganizationalUnitsChildrenEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsOrganizationalUnitsChildrenEl {
    type O = terrars::BlockAssignable<DataOrganizationsOrganizationalUnitsChildrenEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsOrganizationalUnitsChildrenEl {}

impl BuildDataOrganizationsOrganizationalUnitsChildrenEl {
    pub fn build(self) -> DataOrganizationsOrganizationalUnitsChildrenEl {
        DataOrganizationsOrganizationalUnitsChildrenEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsOrganizationalUnitsChildrenElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsOrganizationalUnitsChildrenElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOrganizationsOrganizationalUnitsChildrenElRef {
        DataOrganizationsOrganizationalUnitsChildrenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationalUnitsChildrenElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

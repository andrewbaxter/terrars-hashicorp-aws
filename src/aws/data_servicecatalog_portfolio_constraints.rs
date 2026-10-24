use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServicecatalogPortfolioConstraintsData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accept_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    portfolio_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataServicecatalogPortfolioConstraintsTimeoutsEl>,
}

struct DataServicecatalogPortfolioConstraints_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataServicecatalogPortfolioConstraintsData>,
}

#[derive(Clone)]
pub struct DataServicecatalogPortfolioConstraints(std::rc::Rc<DataServicecatalogPortfolioConstraints_>);

impl DataServicecatalogPortfolioConstraints {
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

    #[doc = "Set the field `product_id`.\n"]
    pub fn set_product_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataServicecatalogPortfolioConstraintsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<DataServicecatalogPortfolioConstraintsDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.portfolio_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogPortfolioConstraintsTimeoutsElRef {
        DataServicecatalogPortfolioConstraintsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataServicecatalogPortfolioConstraints {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataServicecatalogPortfolioConstraints { }

impl terrars::ToListMappable for DataServicecatalogPortfolioConstraints {
    type O = terrars::ListRef<DataServicecatalogPortfolioConstraintsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataServicecatalogPortfolioConstraints_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_servicecatalog_portfolio_constraints".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServicecatalogPortfolioConstraints {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub portfolio_id: terrars::PrimField<std::string::String>,
}

impl BuildDataServicecatalogPortfolioConstraints {
    pub fn build(self, stack: &mut terrars::Stack) -> DataServicecatalogPortfolioConstraints {
        let out = DataServicecatalogPortfolioConstraints(std::rc::Rc::new(DataServicecatalogPortfolioConstraints_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataServicecatalogPortfolioConstraintsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                accept_language: core::default::Default::default(),
                id: core::default::Default::default(),
                portfolio_id: self.portfolio_id,
                product_id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServicecatalogPortfolioConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicecatalogPortfolioConstraintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataServicecatalogPortfolioConstraintsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<DataServicecatalogPortfolioConstraintsDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.portfolio_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogPortfolioConstraintsTimeoutsElRef {
        DataServicecatalogPortfolioConstraintsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataServicecatalogPortfolioConstraintsDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    portfolio_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServicecatalogPortfolioConstraintsDetailsEl {
    #[doc = "Set the field `constraint_id`.\n"]
    pub fn set_constraint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.constraint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `portfolio_id`.\n"]
    pub fn set_portfolio_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.portfolio_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_id`.\n"]
    pub fn set_product_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.product_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServicecatalogPortfolioConstraintsDetailsEl {
    type O = terrars::BlockAssignable<DataServicecatalogPortfolioConstraintsDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicecatalogPortfolioConstraintsDetailsEl {}

impl BuildDataServicecatalogPortfolioConstraintsDetailsEl {
    pub fn build(self) -> DataServicecatalogPortfolioConstraintsDetailsEl {
        DataServicecatalogPortfolioConstraintsDetailsEl {
            constraint_id: core::default::Default::default(),
            description: core::default::Default::default(),
            owner: core::default::Default::default(),
            portfolio_id: core::default::Default::default(),
            product_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataServicecatalogPortfolioConstraintsDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicecatalogPortfolioConstraintsDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServicecatalogPortfolioConstraintsDetailsElRef {
        DataServicecatalogPortfolioConstraintsDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicecatalogPortfolioConstraintsDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraint_id` after provisioning.\n"]
    pub fn constraint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constraint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.base))
    }

    #[doc = "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.portfolio_id", self.base))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServicecatalogPortfolioConstraintsTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServicecatalogPortfolioConstraintsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServicecatalogPortfolioConstraintsTimeoutsEl {
    type O = terrars::BlockAssignable<DataServicecatalogPortfolioConstraintsTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicecatalogPortfolioConstraintsTimeoutsEl {}

impl BuildDataServicecatalogPortfolioConstraintsTimeoutsEl {
    pub fn build(self) -> DataServicecatalogPortfolioConstraintsTimeoutsEl {
        DataServicecatalogPortfolioConstraintsTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataServicecatalogPortfolioConstraintsTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicecatalogPortfolioConstraintsTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServicecatalogPortfolioConstraintsTimeoutsElRef {
        DataServicecatalogPortfolioConstraintsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicecatalogPortfolioConstraintsTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

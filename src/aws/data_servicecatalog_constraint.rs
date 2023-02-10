use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServicecatalogConstraintData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataServicecatalogConstraintTimeoutsEl>,
}

struct DataServicecatalogConstraint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServicecatalogConstraintData>,
}

#[derive(Clone)]
pub struct DataServicecatalogConstraint(Rc<DataServicecatalogConstraint_>);

impl DataServicecatalogConstraint {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `accept_language`.\n"]
    pub fn set_accept_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().accept_language = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataServicecatalogConstraintTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.portfolio_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogConstraintTimeoutsElRef {
        DataServicecatalogConstraintTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Datasource for DataServicecatalogConstraint {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataServicecatalogConstraint {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataServicecatalogConstraint {
    type O = ListRef<DataServicecatalogConstraintRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataServicecatalogConstraint_ {
    fn extract_datasource_type(&self) -> String {
        "aws_servicecatalog_constraint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServicecatalogConstraint {
    pub tf_id: String,
    #[doc= ""]
    pub id: PrimField<String>,
}

impl BuildDataServicecatalogConstraint {
    pub fn build(self, stack: &mut Stack) -> DataServicecatalogConstraint {
        let out = DataServicecatalogConstraint(Rc::new(DataServicecatalogConstraint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataServicecatalogConstraintData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                accept_language: core::default::Default::default(),
                description: core::default::Default::default(),
                id: self.id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServicecatalogConstraintRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicecatalogConstraintRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataServicecatalogConstraintRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.portfolio_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogConstraintTimeoutsElRef {
        DataServicecatalogConstraintTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataServicecatalogConstraintTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataServicecatalogConstraintTimeoutsEl {
    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataServicecatalogConstraintTimeoutsEl {
    type O = BlockAssignable<DataServicecatalogConstraintTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicecatalogConstraintTimeoutsEl {}

impl BuildDataServicecatalogConstraintTimeoutsEl {
    pub fn build(self) -> DataServicecatalogConstraintTimeoutsEl {
        DataServicecatalogConstraintTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataServicecatalogConstraintTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServicecatalogConstraintTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataServicecatalogConstraintTimeoutsElRef {
        DataServicecatalogConstraintTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicecatalogConstraintTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

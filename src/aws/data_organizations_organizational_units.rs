use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOrganizationsOrganizationalUnitsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent_id: PrimField<String>,
}

struct DataOrganizationsOrganizationalUnits_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOrganizationsOrganizationalUnitsData>,
}

#[derive(Clone)]
pub struct DataOrganizationsOrganizationalUnits(Rc<DataOrganizationsOrganizationalUnits_>);

impl DataOrganizationsOrganizationalUnits {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `children` after provisioning.\n"]
    pub fn children(&self) -> ListRef<DataOrganizationsOrganizationalUnitsChildrenElRef> {
        ListRef::new(self.shared().clone(), format!("{}.children", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }
}

impl Referable for DataOrganizationsOrganizationalUnits {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOrganizationsOrganizationalUnits { }

impl ToListMappable for DataOrganizationsOrganizationalUnits {
    type O = ListRef<DataOrganizationsOrganizationalUnitsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOrganizationsOrganizationalUnits_ {
    fn extract_datasource_type(&self) -> String {
        "aws_organizations_organizational_units".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationsOrganizationalUnits {
    pub tf_id: String,
    #[doc= ""]
    pub parent_id: PrimField<String>,
}

impl BuildDataOrganizationsOrganizationalUnits {
    pub fn build(self, stack: &mut Stack) -> DataOrganizationsOrganizationalUnits {
        let out = DataOrganizationsOrganizationalUnits(Rc::new(DataOrganizationsOrganizationalUnits_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOrganizationsOrganizationalUnitsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                parent_id: self.parent_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationsOrganizationalUnitsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsOrganizationalUnitsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationsOrganizationalUnitsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `children` after provisioning.\n"]
    pub fn children(&self) -> ListRef<DataOrganizationsOrganizationalUnitsChildrenElRef> {
        ListRef::new(self.shared().clone(), format!("{}.children", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent_id` after provisioning.\n"]
    pub fn parent_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsOrganizationalUnitsChildrenEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataOrganizationsOrganizationalUnitsChildrenEl {
    #[doc= "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataOrganizationsOrganizationalUnitsChildrenEl {
    type O = BlockAssignable<DataOrganizationsOrganizationalUnitsChildrenEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataOrganizationsOrganizationalUnitsChildrenElRef {
    fn new(shared: StackShared, base: String) -> DataOrganizationsOrganizationalUnitsChildrenElRef {
        DataOrganizationsOrganizationalUnitsChildrenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsOrganizationalUnitsChildrenElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

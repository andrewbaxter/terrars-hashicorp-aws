use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderContainerRecipesData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataImagebuilderContainerRecipesFilterEl>>,
    dynamic: DataImagebuilderContainerRecipesDynamic,
}

struct DataImagebuilderContainerRecipes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataImagebuilderContainerRecipesData>,
}

#[derive(Clone)]
pub struct DataImagebuilderContainerRecipes(Rc<DataImagebuilderContainerRecipes_>);

impl DataImagebuilderContainerRecipes {
    fn shared(&self) -> &StackShared {
        &self.0.shared
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

    #[doc= "Set the field `owner`.\n"]
    pub fn set_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner = Some(v.into());
        self
    }

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataImagebuilderContainerRecipesFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }
}

impl Datasource for DataImagebuilderContainerRecipes {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataImagebuilderContainerRecipes {
    type O = ListRef<DataImagebuilderContainerRecipesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataImagebuilderContainerRecipes_ {
    fn extract_datasource_type(&self) -> String {
        "aws_imagebuilder_container_recipes".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderContainerRecipes {
    pub tf_id: String,
}

impl BuildDataImagebuilderContainerRecipes {
    pub fn build(self, stack: &mut Stack) -> DataImagebuilderContainerRecipes {
        let out = DataImagebuilderContainerRecipes(Rc::new(DataImagebuilderContainerRecipes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataImagebuilderContainerRecipesData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                owner: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderContainerRecipesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderContainerRecipesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImagebuilderContainerRecipesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderContainerRecipesFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataImagebuilderContainerRecipesFilterEl { }

impl ToListMappable for DataImagebuilderContainerRecipesFilterEl {
    type O = BlockAssignable<DataImagebuilderContainerRecipesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderContainerRecipesFilterEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataImagebuilderContainerRecipesFilterEl {
    pub fn build(self) -> DataImagebuilderContainerRecipesFilterEl {
        DataImagebuilderContainerRecipesFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataImagebuilderContainerRecipesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderContainerRecipesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderContainerRecipesFilterElRef {
        DataImagebuilderContainerRecipesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderContainerRecipesFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataImagebuilderContainerRecipesDynamic {
    filter: Option<DynamicBlock<DataImagebuilderContainerRecipesFilterEl>>,
}

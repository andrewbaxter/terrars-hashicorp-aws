use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLocationPlaceIndexData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    index_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataLocationPlaceIndex_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLocationPlaceIndexData>,
}

#[derive(Clone)]
pub struct DataLocationPlaceIndex(Rc<DataLocationPlaceIndex_>);

impl DataLocationPlaceIndex {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source_configuration` after provisioning.\n"]
    pub fn data_source_configuration(&self) -> ListRef<DataLocationPlaceIndexDataSourceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_source_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `index_arn` after provisioning.\n"]
    pub fn index_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl Datasource for DataLocationPlaceIndex {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataLocationPlaceIndex {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataLocationPlaceIndex {
    type O = ListRef<DataLocationPlaceIndexRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataLocationPlaceIndex_ {
    fn extract_datasource_type(&self) -> String {
        "aws_location_place_index".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLocationPlaceIndex {
    pub tf_id: String,
    #[doc= ""]
    pub index_name: PrimField<String>,
}

impl BuildDataLocationPlaceIndex {
    pub fn build(self, stack: &mut Stack) -> DataLocationPlaceIndex {
        let out = DataLocationPlaceIndex(Rc::new(DataLocationPlaceIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLocationPlaceIndexData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                index_name: self.index_name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLocationPlaceIndexRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLocationPlaceIndexRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLocationPlaceIndexRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_source_configuration` after provisioning.\n"]
    pub fn data_source_configuration(&self) -> ListRef<DataLocationPlaceIndexDataSourceConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_source_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `index_arn` after provisioning.\n"]
    pub fn index_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLocationPlaceIndexDataSourceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    intended_use: Option<PrimField<String>>,
}

impl DataLocationPlaceIndexDataSourceConfigurationEl {
    #[doc= "Set the field `intended_use`.\n"]
    pub fn set_intended_use(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.intended_use = Some(v.into());
        self
    }
}

impl ToListMappable for DataLocationPlaceIndexDataSourceConfigurationEl {
    type O = BlockAssignable<DataLocationPlaceIndexDataSourceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLocationPlaceIndexDataSourceConfigurationEl {}

impl BuildDataLocationPlaceIndexDataSourceConfigurationEl {
    pub fn build(self) -> DataLocationPlaceIndexDataSourceConfigurationEl {
        DataLocationPlaceIndexDataSourceConfigurationEl { intended_use: core::default::Default::default() }
    }
}

pub struct DataLocationPlaceIndexDataSourceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLocationPlaceIndexDataSourceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataLocationPlaceIndexDataSourceConfigurationElRef {
        DataLocationPlaceIndexDataSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLocationPlaceIndexDataSourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `intended_use` after provisioning.\n"]
    pub fn intended_use(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.intended_use", self.base))
    }
}

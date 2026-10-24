use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServicecatalogLaunchPathsData {
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
    product_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataServicecatalogLaunchPathsTimeoutsEl>,
}

struct DataServicecatalogLaunchPaths_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataServicecatalogLaunchPathsData>,
}

#[derive(Clone)]
pub struct DataServicecatalogLaunchPaths(std::rc::Rc<DataServicecatalogLaunchPaths_>);

impl DataServicecatalogLaunchPaths {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataServicecatalogLaunchPathsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summaries` after provisioning.\n"]
    pub fn summaries(&self) -> terrars::ListRef<DataServicecatalogLaunchPathsSummariesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summaries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogLaunchPathsTimeoutsElRef {
        DataServicecatalogLaunchPathsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataServicecatalogLaunchPaths {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataServicecatalogLaunchPaths { }

impl terrars::ToListMappable for DataServicecatalogLaunchPaths {
    type O = terrars::ListRef<DataServicecatalogLaunchPathsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataServicecatalogLaunchPaths_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_servicecatalog_launch_paths".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServicecatalogLaunchPaths {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub product_id: terrars::PrimField<std::string::String>,
}

impl BuildDataServicecatalogLaunchPaths {
    pub fn build(self, stack: &mut terrars::Stack) -> DataServicecatalogLaunchPaths {
        let out = DataServicecatalogLaunchPaths(std::rc::Rc::new(DataServicecatalogLaunchPaths_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataServicecatalogLaunchPathsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                accept_language: core::default::Default::default(),
                id: core::default::Default::default(),
                product_id: self.product_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServicecatalogLaunchPathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicecatalogLaunchPathsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataServicecatalogLaunchPathsRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_id` after provisioning.\n"]
    pub fn product_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `summaries` after provisioning.\n"]
    pub fn summaries(&self) -> terrars::ListRef<DataServicecatalogLaunchPathsSummariesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summaries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataServicecatalogLaunchPathsTimeoutsElRef {
        DataServicecatalogLaunchPathsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
    type O = terrars::BlockAssignable<DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {}

impl BuildDataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
    pub fn build(self) -> DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
        DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl {
            description: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
        DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServicecatalogLaunchPathsSummariesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraint_summaries: std
    ::option
    ::Option<terrars::ListField<DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataServicecatalogLaunchPathsSummariesEl {
    #[doc = "Set the field `constraint_summaries`.\n"]
    pub fn set_constraint_summaries(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataServicecatalogLaunchPathsSummariesElConstraintSummariesEl>>,
    ) -> Self {
        self.constraint_summaries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path_id`.\n"]
    pub fn set_path_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServicecatalogLaunchPathsSummariesEl {
    type O = terrars::BlockAssignable<DataServicecatalogLaunchPathsSummariesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicecatalogLaunchPathsSummariesEl {}

impl BuildDataServicecatalogLaunchPathsSummariesEl {
    pub fn build(self) -> DataServicecatalogLaunchPathsSummariesEl {
        DataServicecatalogLaunchPathsSummariesEl {
            constraint_summaries: core::default::Default::default(),
            name: core::default::Default::default(),
            path_id: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataServicecatalogLaunchPathsSummariesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicecatalogLaunchPathsSummariesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataServicecatalogLaunchPathsSummariesElRef {
        DataServicecatalogLaunchPathsSummariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicecatalogLaunchPathsSummariesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constraint_summaries` after provisioning.\n"]
    pub fn constraint_summaries(
        &self,
    ) -> terrars::ListRef<DataServicecatalogLaunchPathsSummariesElConstraintSummariesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraint_summaries", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path_id` after provisioning.\n"]
    pub fn path_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServicecatalogLaunchPathsTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataServicecatalogLaunchPathsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataServicecatalogLaunchPathsTimeoutsEl {
    type O = terrars::BlockAssignable<DataServicecatalogLaunchPathsTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServicecatalogLaunchPathsTimeoutsEl {}

impl BuildDataServicecatalogLaunchPathsTimeoutsEl {
    pub fn build(self) -> DataServicecatalogLaunchPathsTimeoutsEl {
        DataServicecatalogLaunchPathsTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataServicecatalogLaunchPathsTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicecatalogLaunchPathsTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataServicecatalogLaunchPathsTimeoutsElRef {
        DataServicecatalogLaunchPathsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServicecatalogLaunchPathsTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

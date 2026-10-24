use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEmrReleaseLabelsData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<std::vec::Vec<DataEmrReleaseLabelsFiltersEl>>,
    dynamic: DataEmrReleaseLabelsDynamic,
}

struct DataEmrReleaseLabels_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEmrReleaseLabelsData>,
}

#[derive(Clone)]
pub struct DataEmrReleaseLabels(std::rc::Rc<DataEmrReleaseLabels_>);

impl DataEmrReleaseLabels {
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

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEmrReleaseLabelsFiltersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_labels` after provisioning.\n"]
    pub fn release_labels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.release_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<DataEmrReleaseLabelsFiltersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

impl terrars::Referable for DataEmrReleaseLabels {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEmrReleaseLabels { }

impl terrars::ToListMappable for DataEmrReleaseLabels {
    type O = terrars::ListRef<DataEmrReleaseLabelsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEmrReleaseLabels_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_emr_release_labels".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmrReleaseLabels {
    pub tf_id: std::string::String,
}

impl BuildDataEmrReleaseLabels {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEmrReleaseLabels {
        let out = DataEmrReleaseLabels(std::rc::Rc::new(DataEmrReleaseLabels_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEmrReleaseLabelsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                filters: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmrReleaseLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmrReleaseLabelsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEmrReleaseLabelsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_labels` after provisioning.\n"]
    pub fn release_labels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.release_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<DataEmrReleaseLabelsFiltersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEmrReleaseLabelsFiltersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEmrReleaseLabelsFiltersEl {
    #[doc = "Set the field `application`.\n"]
    pub fn set_application(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEmrReleaseLabelsFiltersEl {
    type O = terrars::BlockAssignable<DataEmrReleaseLabelsFiltersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrReleaseLabelsFiltersEl {}

impl BuildDataEmrReleaseLabelsFiltersEl {
    pub fn build(self) -> DataEmrReleaseLabelsFiltersEl {
        DataEmrReleaseLabelsFiltersEl {
            application: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct DataEmrReleaseLabelsFiltersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEmrReleaseLabelsFiltersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEmrReleaseLabelsFiltersElRef {
        DataEmrReleaseLabelsFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrReleaseLabelsFiltersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application` after provisioning.\n"]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEmrReleaseLabelsDynamic {
    filters: std::option::Option<terrars::DynamicBlock<DataEmrReleaseLabelsFiltersEl>>,
}

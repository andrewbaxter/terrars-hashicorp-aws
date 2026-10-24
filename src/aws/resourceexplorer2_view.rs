use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Resourceexplorer2ViewData {
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
    default_view: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<std::vec::Vec<Resourceexplorer2ViewFiltersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_property: std::option::Option<std::vec::Vec<Resourceexplorer2ViewIncludedPropertyEl>>,
    dynamic: Resourceexplorer2ViewDynamic,
}

struct Resourceexplorer2View_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Resourceexplorer2ViewData>,
}

#[derive(Clone)]
pub struct Resourceexplorer2View(std::rc::Rc<Resourceexplorer2View_>);

impl Resourceexplorer2View {
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

    #[doc = "Set the field `default_view`.\n"]
    pub fn set_default_view(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_view = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Resourceexplorer2ViewFiltersEl>>,
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

    #[doc = "Set the field `included_property`.\n"]
    pub fn set_included_property(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Resourceexplorer2ViewIncludedPropertyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().included_property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.included_property = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_view` after provisioning.\n"]
    pub fn default_view(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_view", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<Resourceexplorer2ViewFiltersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `included_property` after provisioning.\n"]
    pub fn included_property(&self) -> terrars::ListRef<Resourceexplorer2ViewIncludedPropertyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_property", self.extract_ref()))
    }
}

impl terrars::Referable for Resourceexplorer2View {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Resourceexplorer2View { }

impl terrars::ToListMappable for Resourceexplorer2View {
    type O = terrars::ListRef<Resourceexplorer2ViewRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Resourceexplorer2View_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_resourceexplorer2_view".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildResourceexplorer2View {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildResourceexplorer2View {
    pub fn build(self, stack: &mut terrars::Stack) -> Resourceexplorer2View {
        let out = Resourceexplorer2View(std::rc::Rc::new(Resourceexplorer2View_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Resourceexplorer2ViewData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_view: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                filters: core::default::Default::default(),
                included_property: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Resourceexplorer2ViewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resourceexplorer2ViewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Resourceexplorer2ViewRef {
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

    #[doc = "Get a reference to the value of field `default_view` after provisioning.\n"]
    pub fn default_view(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_view", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<Resourceexplorer2ViewFiltersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `included_property` after provisioning.\n"]
    pub fn included_property(&self) -> terrars::ListRef<Resourceexplorer2ViewIncludedPropertyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_property", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Resourceexplorer2ViewFiltersEl {
    filter_string: terrars::PrimField<std::string::String>,
}

impl Resourceexplorer2ViewFiltersEl { }

impl terrars::ToListMappable for Resourceexplorer2ViewFiltersEl {
    type O = terrars::BlockAssignable<Resourceexplorer2ViewFiltersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResourceexplorer2ViewFiltersEl {
    #[doc = ""]
    pub filter_string: terrars::PrimField<std::string::String>,
}

impl BuildResourceexplorer2ViewFiltersEl {
    pub fn build(self) -> Resourceexplorer2ViewFiltersEl {
        Resourceexplorer2ViewFiltersEl { filter_string: self.filter_string }
    }
}

pub struct Resourceexplorer2ViewFiltersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resourceexplorer2ViewFiltersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Resourceexplorer2ViewFiltersElRef {
        Resourceexplorer2ViewFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resourceexplorer2ViewFiltersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_string` after provisioning.\n"]
    pub fn filter_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_string", self.base))
    }
}

#[derive(Serialize)]
pub struct Resourceexplorer2ViewIncludedPropertyEl {
    name: terrars::PrimField<std::string::String>,
}

impl Resourceexplorer2ViewIncludedPropertyEl { }

impl terrars::ToListMappable for Resourceexplorer2ViewIncludedPropertyEl {
    type O = terrars::BlockAssignable<Resourceexplorer2ViewIncludedPropertyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildResourceexplorer2ViewIncludedPropertyEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildResourceexplorer2ViewIncludedPropertyEl {
    pub fn build(self) -> Resourceexplorer2ViewIncludedPropertyEl {
        Resourceexplorer2ViewIncludedPropertyEl { name: self.name }
    }
}

pub struct Resourceexplorer2ViewIncludedPropertyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Resourceexplorer2ViewIncludedPropertyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Resourceexplorer2ViewIncludedPropertyElRef {
        Resourceexplorer2ViewIncludedPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Resourceexplorer2ViewIncludedPropertyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct Resourceexplorer2ViewDynamic {
    filters: std::option::Option<terrars::DynamicBlock<Resourceexplorer2ViewFiltersEl>>,
    included_property: std::option::Option<terrars::DynamicBlock<Resourceexplorer2ViewIncludedPropertyEl>>,
}

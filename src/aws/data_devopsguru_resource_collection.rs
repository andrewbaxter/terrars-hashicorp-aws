#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDevopsguruResourceCollectionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

struct DataDevopsguruResourceCollection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDevopsguruResourceCollectionData_>,
}

#[derive(Clone)]
pub struct DataDevopsguruResourceCollection(std::rc::Rc<DataDevopsguruResourceCollection_>);

impl DataDevopsguruResourceCollection {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cloudformation` after provisioning.\n"]
    pub fn cloudformation(&self) -> terrars::ListRef<DataDevopsguruResourceCollection_CloudformationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudformation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataDevopsguruResourceCollection_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataDevopsguruResourceCollection {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDevopsguruResourceCollection { }

impl terrars::ToListMappable for DataDevopsguruResourceCollection {
    type O = terrars::ListRef<DataDevopsguruResourceCollectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDevopsguruResourceCollection_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_devopsguru_resource_collection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDevopsguruResourceCollection {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataDevopsguruResourceCollection {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDevopsguruResourceCollection {
        let out = DataDevopsguruResourceCollection(std::rc::Rc::new(DataDevopsguruResourceCollection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDevopsguruResourceCollectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                type_: self.type_,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDevopsguruResourceCollectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDevopsguruResourceCollectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDevopsguruResourceCollectionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cloudformation` after provisioning.\n"]
    pub fn cloudformation(&self) -> terrars::ListRef<DataDevopsguruResourceCollection_CloudformationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudformation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DataDevopsguruResourceCollection_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDevopsguruResourceCollection_Cloudformation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataDevopsguruResourceCollection_Cloudformation {
    #[doc = "Set the field `stack_names`.\n"]
    pub fn set_stack_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stack_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDevopsguruResourceCollection_Cloudformation {
    type O = terrars::BlockAssignable<DataDevopsguruResourceCollection_Cloudformation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDevopsguruResourceCollection_Cloudformation {}

impl BuildDataDevopsguruResourceCollection_Cloudformation {
    pub fn build(self) -> DataDevopsguruResourceCollection_Cloudformation {
        DataDevopsguruResourceCollection_Cloudformation { stack_names: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDevopsguruResourceCollection_CloudformationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDevopsguruResourceCollection_CloudformationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDevopsguruResourceCollection_CloudformationRef {
        DataDevopsguruResourceCollection_CloudformationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDevopsguruResourceCollection_CloudformationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stack_names` after provisioning.\n"]
    pub fn stack_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stack_names", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDevopsguruResourceCollection_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_boundary_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataDevopsguruResourceCollection_Tags {
    #[doc = "Set the field `app_boundary_key`.\n"]
    pub fn set_app_boundary_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.app_boundary_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_values`.\n"]
    pub fn set_tag_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tag_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDevopsguruResourceCollection_Tags {
    type O = terrars::BlockAssignable<DataDevopsguruResourceCollection_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDevopsguruResourceCollection_Tags {}

impl BuildDataDevopsguruResourceCollection_Tags {
    pub fn build(self) -> DataDevopsguruResourceCollection_Tags {
        DataDevopsguruResourceCollection_Tags {
            app_boundary_key: core::default::Default::default(),
            tag_values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDevopsguruResourceCollection_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDevopsguruResourceCollection_TagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDevopsguruResourceCollection_TagsRef {
        DataDevopsguruResourceCollection_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDevopsguruResourceCollection_TagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_boundary_key` after provisioning.\n"]
    pub fn app_boundary_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_boundary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_values` after provisioning.\n"]
    pub fn tag_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_values", self.base))
    }
}

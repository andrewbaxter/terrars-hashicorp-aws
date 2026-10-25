#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DevopsguruResourceCollectionData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudformation: std::option::Option<std::vec::Vec<DevopsguruResourceCollection_Cloudformation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<DevopsguruResourceCollection_Tags>>,
    dynamic: DevopsguruResourceCollectionDynamic,
}

struct DevopsguruResourceCollection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DevopsguruResourceCollectionData_>,
}

#[derive(Clone)]
pub struct DevopsguruResourceCollection(std::rc::Rc<DevopsguruResourceCollection_>);

impl DevopsguruResourceCollection {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudformation`.\n"]
    pub fn set_cloudformation(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DevopsguruResourceCollection_Cloudformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudformation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DevopsguruResourceCollection_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudformation` after provisioning.\n"]
    pub fn cloudformation(&self) -> terrars::ListRef<DevopsguruResourceCollection_CloudformationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudformation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DevopsguruResourceCollection_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DevopsguruResourceCollection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DevopsguruResourceCollection { }

impl terrars::ToListMappable for DevopsguruResourceCollection {
    type O = terrars::ListRef<DevopsguruResourceCollectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DevopsguruResourceCollection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_devopsguru_resource_collection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevopsguruResourceCollection {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDevopsguruResourceCollection {
    pub fn build(self, stack: &mut terrars::Stack) -> DevopsguruResourceCollection {
        let out = DevopsguruResourceCollection(std::rc::Rc::new(DevopsguruResourceCollection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DevopsguruResourceCollectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: self.type_,
                cloudformation: core::default::Default::default(),
                tags: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevopsguruResourceCollectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruResourceCollectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DevopsguruResourceCollectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudformation` after provisioning.\n"]
    pub fn cloudformation(&self) -> terrars::ListRef<DevopsguruResourceCollection_CloudformationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudformation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<DevopsguruResourceCollection_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DevopsguruResourceCollectionDynamic {
    cloudformation: std::option::Option<terrars::DynamicBlock<DevopsguruResourceCollection_Cloudformation>>,
    tags: std::option::Option<terrars::DynamicBlock<DevopsguruResourceCollection_Tags>>,
}

#[derive(Serialize)]
pub struct DevopsguruResourceCollection_Cloudformation {
    stack_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DevopsguruResourceCollection_Cloudformation { }

impl terrars::ToListMappable for DevopsguruResourceCollection_Cloudformation {
    type O = terrars::BlockAssignable<DevopsguruResourceCollection_Cloudformation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruResourceCollection_Cloudformation {
    #[doc = ""]
    pub stack_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDevopsguruResourceCollection_Cloudformation {
    pub fn build(self) -> DevopsguruResourceCollection_Cloudformation {
        DevopsguruResourceCollection_Cloudformation { stack_names: self.stack_names }
    }
}

#[allow(dead_code)]
pub struct DevopsguruResourceCollection_CloudformationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruResourceCollection_CloudformationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DevopsguruResourceCollection_CloudformationRef {
        DevopsguruResourceCollection_CloudformationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruResourceCollection_CloudformationRef {
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
pub struct DevopsguruResourceCollection_Tags {
    app_boundary_key: terrars::PrimField<std::string::String>,
    tag_values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DevopsguruResourceCollection_Tags { }

impl terrars::ToListMappable for DevopsguruResourceCollection_Tags {
    type O = terrars::BlockAssignable<DevopsguruResourceCollection_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruResourceCollection_Tags {
    #[doc = ""]
    pub app_boundary_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub tag_values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDevopsguruResourceCollection_Tags {
    pub fn build(self) -> DevopsguruResourceCollection_Tags {
        DevopsguruResourceCollection_Tags {
            app_boundary_key: self.app_boundary_key,
            tag_values: self.tag_values,
        }
    }
}

#[allow(dead_code)]
pub struct DevopsguruResourceCollection_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruResourceCollection_TagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DevopsguruResourceCollection_TagsRef {
        DevopsguruResourceCollection_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruResourceCollection_TagsRef {
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

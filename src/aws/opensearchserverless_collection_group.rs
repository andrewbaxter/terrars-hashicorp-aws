#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpensearchserverlessCollectionGroupData_ {
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
    capacity_limits: std::option::Option<terrars::ListField<OpensearchserverlessCollectionGroup_CapacityLimits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    standby_replicas: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct OpensearchserverlessCollectionGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpensearchserverlessCollectionGroupData_>,
}

#[derive(Clone)]
pub struct OpensearchserverlessCollectionGroup(std::rc::Rc<OpensearchserverlessCollectionGroup_>);

impl OpensearchserverlessCollectionGroup {
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

    #[doc = "Set the field `capacity_limits`.\n"]
    pub fn set_capacity_limits(
        self,
        v: impl std::convert::Into<terrars::ListField<OpensearchserverlessCollectionGroup_CapacityLimits>>,
    ) -> Self {
        self.0.data.borrow_mut().capacity_limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the collection group."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generation`.\nGeneration of the collection group."]
    pub fn set_generation(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().generation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_limits` after provisioning.\n"]
    pub fn capacity_limits(&self) -> terrars::ListRef<OpensearchserverlessCollectionGroup_CapacityLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_limits", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_date` after provisioning.\nDate the collection group was created."]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the collection group."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration of the collection group."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the collection group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_replicas` after provisioning.\nIndicates whether standby replicas should be used for collections in this group. One of `ENABLED` or `DISABLED`."]
    pub fn standby_replicas(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standby_replicas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl terrars::Referable for OpensearchserverlessCollectionGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OpensearchserverlessCollectionGroup { }

impl terrars::ToListMappable for OpensearchserverlessCollectionGroup {
    type O = terrars::ListRef<OpensearchserverlessCollectionGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OpensearchserverlessCollectionGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_opensearchserverless_collection_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpensearchserverlessCollectionGroup {
    pub tf_id: std::string::String,
    #[doc = "Name of the collection group."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Indicates whether standby replicas should be used for collections in this group. One of `ENABLED` or `DISABLED`."]
    pub standby_replicas: terrars::PrimField<std::string::String>,
}

impl BuildOpensearchserverlessCollectionGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> OpensearchserverlessCollectionGroup {
        let out = OpensearchserverlessCollectionGroup(std::rc::Rc::new(OpensearchserverlessCollectionGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OpensearchserverlessCollectionGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity_limits: core::default::Default::default(),
                description: core::default::Default::default(),
                generation: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                standby_replicas: self.standby_replicas,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpensearchserverlessCollectionGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchserverlessCollectionGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpensearchserverlessCollectionGroupRef {
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

    #[doc = "Get a reference to the value of field `capacity_limits` after provisioning.\n"]
    pub fn capacity_limits(&self) -> terrars::ListRef<OpensearchserverlessCollectionGroup_CapacityLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_limits", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_date` after provisioning.\nDate the collection group was created."]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the collection group."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration of the collection group."]
    pub fn generation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the collection group."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_replicas` after provisioning.\nIndicates whether standby replicas should be used for collections in this group. One of `ENABLED` or `DISABLED`."]
    pub fn standby_replicas(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standby_replicas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpensearchserverlessCollectionGroup_CapacityLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_indexing_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_search_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_indexing_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_search_capacity_in_ocu: std::option::Option<terrars::PrimField<f64>>,
}

impl OpensearchserverlessCollectionGroup_CapacityLimits {
    #[doc = "Set the field `max_indexing_capacity_in_ocu`.\n"]
    pub fn set_max_indexing_capacity_in_ocu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_indexing_capacity_in_ocu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_search_capacity_in_ocu`.\n"]
    pub fn set_max_search_capacity_in_ocu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_search_capacity_in_ocu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_indexing_capacity_in_ocu`.\n"]
    pub fn set_min_indexing_capacity_in_ocu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_indexing_capacity_in_ocu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_search_capacity_in_ocu`.\n"]
    pub fn set_min_search_capacity_in_ocu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_search_capacity_in_ocu = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpensearchserverlessCollectionGroup_CapacityLimits {
    type O = terrars::BlockAssignable<OpensearchserverlessCollectionGroup_CapacityLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchserverlessCollectionGroup_CapacityLimits {}

impl BuildOpensearchserverlessCollectionGroup_CapacityLimits {
    pub fn build(self) -> OpensearchserverlessCollectionGroup_CapacityLimits {
        OpensearchserverlessCollectionGroup_CapacityLimits {
            max_indexing_capacity_in_ocu: core::default::Default::default(),
            max_search_capacity_in_ocu: core::default::Default::default(),
            min_indexing_capacity_in_ocu: core::default::Default::default(),
            min_search_capacity_in_ocu: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OpensearchserverlessCollectionGroup_CapacityLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpensearchserverlessCollectionGroup_CapacityLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpensearchserverlessCollectionGroup_CapacityLimitsRef {
        OpensearchserverlessCollectionGroup_CapacityLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchserverlessCollectionGroup_CapacityLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_indexing_capacity_in_ocu` after provisioning.\n"]
    pub fn max_indexing_capacity_in_ocu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_indexing_capacity_in_ocu", self.base))
    }

    #[doc = "Get a reference to the value of field `max_search_capacity_in_ocu` after provisioning.\n"]
    pub fn max_search_capacity_in_ocu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_search_capacity_in_ocu", self.base))
    }

    #[doc = "Get a reference to the value of field `min_indexing_capacity_in_ocu` after provisioning.\n"]
    pub fn min_indexing_capacity_in_ocu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_indexing_capacity_in_ocu", self.base))
    }

    #[doc = "Get a reference to the value of field `min_search_capacity_in_ocu` after provisioning.\n"]
    pub fn min_search_capacity_in_ocu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_search_capacity_in_ocu", self.base))
    }
}

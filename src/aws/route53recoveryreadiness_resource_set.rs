#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53recoveryreadinessResourceSetData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_set_name: terrars::PrimField<std::string::String>,
    resource_set_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<Route53recoveryreadinessResourceSet_Resources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Route53recoveryreadinessResourceSet_Timeouts>,
    dynamic: Route53recoveryreadinessResourceSetDynamic,
}

struct Route53recoveryreadinessResourceSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Route53recoveryreadinessResourceSetData_>,
}

#[derive(Clone)]
pub struct Route53recoveryreadinessResourceSet(std::rc::Rc<Route53recoveryreadinessResourceSet_>);

impl Route53recoveryreadinessResourceSet {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Route53recoveryreadinessResourceSet_Resources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Route53recoveryreadinessResourceSet_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_set_name` after provisioning.\n"]
    pub fn resource_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_set_type` after provisioning.\n"]
    pub fn resource_set_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_set_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<Route53recoveryreadinessResourceSet_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53recoveryreadinessResourceSet_TimeoutsRef {
        Route53recoveryreadinessResourceSet_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for Route53recoveryreadinessResourceSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Route53recoveryreadinessResourceSet { }

impl terrars::ToListMappable for Route53recoveryreadinessResourceSet {
    type O = terrars::ListRef<Route53recoveryreadinessResourceSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Route53recoveryreadinessResourceSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_route53recoveryreadiness_resource_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53recoveryreadinessResourceSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub resource_set_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_set_type: terrars::PrimField<std::string::String>,
}

impl BuildRoute53recoveryreadinessResourceSet {
    pub fn build(self, stack: &mut terrars::Stack) -> Route53recoveryreadinessResourceSet {
        let out = Route53recoveryreadinessResourceSet(std::rc::Rc::new(Route53recoveryreadinessResourceSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Route53recoveryreadinessResourceSetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                resource_set_name: self.resource_set_name,
                resource_set_type: self.resource_set_type,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                resources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53recoveryreadinessResourceSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53recoveryreadinessResourceSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Route53recoveryreadinessResourceSetRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_set_name` after provisioning.\n"]
    pub fn resource_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_set_type` after provisioning.\n"]
    pub fn resource_set_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_set_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<Route53recoveryreadinessResourceSet_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53recoveryreadinessResourceSet_TimeoutsRef {
        Route53recoveryreadinessResourceSet_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct Route53recoveryreadinessResourceSetDynamic {
    resources: std::option::Option<terrars::DynamicBlock<Route53recoveryreadinessResourceSet_Resources>>,
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource {
    type O =
        terrars
        ::BlockAssignable<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource {}

impl BuildRoute53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource {
    pub fn build(self) -> Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource {
        Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource {
            arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResourceRef {
        Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_set_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_set_id`.\n"]
    pub fn set_record_set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_set_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource {
    type O =
        terrars
        ::BlockAssignable<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource {}

impl BuildRoute53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource {
    pub fn build(self) -> Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource {
        Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource {
            domain_name: core::default::Default::default(),
            record_set_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53ResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53ResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53ResourceRef {
        Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53ResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53ResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `record_set_id` after provisioning.\n"]
    pub fn record_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_set_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResourceDynamic {
    nlb_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource>,
    >,
    r53_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource>,
    >,
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nlb_resource: std
    ::option
    ::Option<
        std::vec::Vec<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    r53_resource: std
    ::option
    ::Option<
        std::vec::Vec<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource>,
    >,
    dynamic: Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResourceDynamic,
}

impl Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource {
    #[doc = "Set the field `nlb_resource`.\n"]
    pub fn set_nlb_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nlb_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nlb_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `r53_resource`.\n"]
    pub fn set_r53_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53Resource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.r53_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.r53_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource {
    type O =
        terrars::BlockAssignable<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource {}

impl BuildRoute53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource {
    pub fn build(self) -> Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource {
        Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource {
            nlb_resource: core::default::Default::default(),
            r53_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResourceRef {
        Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nlb_resource` after provisioning.\n"]
    pub fn nlb_resource(
        &self,
    ) -> terrars
    ::ListRef<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_NlbResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nlb_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `r53_resource` after provisioning.\n"]
    pub fn r53_resource(
        &self,
    ) -> terrars
    ::ListRef<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource_R53ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.r53_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResourceDynamic {
    target_resource: std
    ::option
    ::Option<terrars::DynamicBlock<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource>>,
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResource {
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosted_zone_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_set_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_resource: std
    ::option
    ::Option<std::vec::Vec<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource>>,
    dynamic: Route53recoveryreadinessResourceSet_Resources_DnsTargetResourceDynamic,
}

impl Route53recoveryreadinessResourceSet_Resources_DnsTargetResource {
    #[doc = "Set the field `hosted_zone_arn`.\n"]
    pub fn set_hosted_zone_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hosted_zone_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_set_id`.\n"]
    pub fn set_record_set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_set_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_type`.\n"]
    pub fn set_record_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_resource`.\n"]
    pub fn set_target_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Route53recoveryreadinessResourceSet_Resources_DnsTargetResource {
    type O = terrars::BlockAssignable<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSet_Resources_DnsTargetResource {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildRoute53recoveryreadinessResourceSet_Resources_DnsTargetResource {
    pub fn build(self) -> Route53recoveryreadinessResourceSet_Resources_DnsTargetResource {
        Route53recoveryreadinessResourceSet_Resources_DnsTargetResource {
            domain_name: self.domain_name,
            hosted_zone_arn: core::default::Default::default(),
            record_set_id: core::default::Default::default(),
            record_type: core::default::Default::default(),
            target_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53recoveryreadinessResourceSet_Resources_DnsTargetResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53recoveryreadinessResourceSet_Resources_DnsTargetResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53recoveryreadinessResourceSet_Resources_DnsTargetResourceRef {
        Route53recoveryreadinessResourceSet_Resources_DnsTargetResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSet_Resources_DnsTargetResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_arn` after provisioning.\n"]
    pub fn hosted_zone_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `record_set_id` after provisioning.\n"]
    pub fn record_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_set_id", self.base))
    }

    #[doc = "Get a reference to the value of field `record_type` after provisioning.\n"]
    pub fn record_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_resource` after provisioning.\n"]
    pub fn target_resource(
        &self,
    ) -> terrars::ListRef<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource_TargetResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53recoveryreadinessResourceSet_ResourcesDynamic {
    dns_target_resource: std
    ::option
    ::Option<terrars::DynamicBlock<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource>>,
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSet_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    readiness_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_target_resource: std
    ::option
    ::Option<std::vec::Vec<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource>>,
    dynamic: Route53recoveryreadinessResourceSet_ResourcesDynamic,
}

impl Route53recoveryreadinessResourceSet_Resources {
    #[doc = "Set the field `readiness_scopes`.\n"]
    pub fn set_readiness_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.readiness_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_target_resource`.\n"]
    pub fn set_dns_target_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Route53recoveryreadinessResourceSet_Resources_DnsTargetResource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_target_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_target_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Route53recoveryreadinessResourceSet_Resources {
    type O = terrars::BlockAssignable<Route53recoveryreadinessResourceSet_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSet_Resources {}

impl BuildRoute53recoveryreadinessResourceSet_Resources {
    pub fn build(self) -> Route53recoveryreadinessResourceSet_Resources {
        Route53recoveryreadinessResourceSet_Resources {
            readiness_scopes: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
            dns_target_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Route53recoveryreadinessResourceSet_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53recoveryreadinessResourceSet_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53recoveryreadinessResourceSet_ResourcesRef {
        Route53recoveryreadinessResourceSet_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSet_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_id` after provisioning.\n"]
    pub fn component_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component_id", self.base))
    }

    #[doc = "Get a reference to the value of field `readiness_scopes` after provisioning.\n"]
    pub fn readiness_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.readiness_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_target_resource` after provisioning.\n"]
    pub fn dns_target_resource(
        &self,
    ) -> terrars::ListRef<Route53recoveryreadinessResourceSet_Resources_DnsTargetResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_target_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct Route53recoveryreadinessResourceSet_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Route53recoveryreadinessResourceSet_Timeouts {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Route53recoveryreadinessResourceSet_Timeouts {
    type O = terrars::BlockAssignable<Route53recoveryreadinessResourceSet_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoveryreadinessResourceSet_Timeouts {}

impl BuildRoute53recoveryreadinessResourceSet_Timeouts {
    pub fn build(self) -> Route53recoveryreadinessResourceSet_Timeouts {
        Route53recoveryreadinessResourceSet_Timeouts { delete: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Route53recoveryreadinessResourceSet_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Route53recoveryreadinessResourceSet_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Route53recoveryreadinessResourceSet_TimeoutsRef {
        Route53recoveryreadinessResourceSet_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoveryreadinessResourceSet_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

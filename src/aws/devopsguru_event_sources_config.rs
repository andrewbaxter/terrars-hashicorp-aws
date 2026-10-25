#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DevopsguruEventSourcesConfigData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_sources: std::option::Option<std::vec::Vec<DevopsguruEventSourcesConfig_EventSources>>,
    dynamic: DevopsguruEventSourcesConfigDynamic,
}

struct DevopsguruEventSourcesConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DevopsguruEventSourcesConfigData_>,
}

#[derive(Clone)]
pub struct DevopsguruEventSourcesConfig(std::rc::Rc<DevopsguruEventSourcesConfig_>);

impl DevopsguruEventSourcesConfig {
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

    #[doc = "Set the field `event_sources`.\n"]
    pub fn set_event_sources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DevopsguruEventSourcesConfig_EventSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_sources = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `event_sources` after provisioning.\n"]
    pub fn event_sources(&self) -> terrars::ListRef<DevopsguruEventSourcesConfig_EventSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_sources", self.extract_ref()))
    }
}

impl terrars::Referable for DevopsguruEventSourcesConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DevopsguruEventSourcesConfig { }

impl terrars::ToListMappable for DevopsguruEventSourcesConfig {
    type O = terrars::ListRef<DevopsguruEventSourcesConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DevopsguruEventSourcesConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_devopsguru_event_sources_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevopsguruEventSourcesConfig {
    pub tf_id: std::string::String,
}

impl BuildDevopsguruEventSourcesConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DevopsguruEventSourcesConfig {
        let out = DevopsguruEventSourcesConfig(std::rc::Rc::new(DevopsguruEventSourcesConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DevopsguruEventSourcesConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                event_sources: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevopsguruEventSourcesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruEventSourcesConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DevopsguruEventSourcesConfigRef {
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

    #[doc = "Get a reference to the value of field `event_sources` after provisioning.\n"]
    pub fn event_sources(&self) -> terrars::ListRef<DevopsguruEventSourcesConfig_EventSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_sources", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DevopsguruEventSourcesConfigDynamic {
    event_sources: std::option::Option<terrars::DynamicBlock<DevopsguruEventSourcesConfig_EventSources>>,
}

#[derive(Serialize)]
pub struct DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler {
    status: terrars::PrimField<std::string::String>,
}

impl DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler { }

impl terrars::ToListMappable for DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler {
    type O = terrars::BlockAssignable<DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler {
    #[doc = ""]
    pub status: terrars::PrimField<std::string::String>,
}

impl BuildDevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler {
    pub fn build(self) -> DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler {
        DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler { status: self.status }
    }
}

#[allow(dead_code)]
pub struct DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfilerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfilerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfilerRef {
        DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfilerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfilerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct DevopsguruEventSourcesConfig_EventSourcesDynamic {
    amazon_code_guru_profiler: std
    ::option
    ::Option<terrars::DynamicBlock<DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler>>,
}

#[derive(Serialize)]
pub struct DevopsguruEventSourcesConfig_EventSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_code_guru_profiler: std
    ::option
    ::Option<std::vec::Vec<DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler>>,
    dynamic: DevopsguruEventSourcesConfig_EventSourcesDynamic,
}

impl DevopsguruEventSourcesConfig_EventSources {
    #[doc = "Set the field `amazon_code_guru_profiler`.\n"]
    pub fn set_amazon_code_guru_profiler(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfiler>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_code_guru_profiler = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_code_guru_profiler = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DevopsguruEventSourcesConfig_EventSources {
    type O = terrars::BlockAssignable<DevopsguruEventSourcesConfig_EventSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruEventSourcesConfig_EventSources {}

impl BuildDevopsguruEventSourcesConfig_EventSources {
    pub fn build(self) -> DevopsguruEventSourcesConfig_EventSources {
        DevopsguruEventSourcesConfig_EventSources {
            amazon_code_guru_profiler: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DevopsguruEventSourcesConfig_EventSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruEventSourcesConfig_EventSourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DevopsguruEventSourcesConfig_EventSourcesRef {
        DevopsguruEventSourcesConfig_EventSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruEventSourcesConfig_EventSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_code_guru_profiler` after provisioning.\n"]
    pub fn amazon_code_guru_profiler(
        &self,
    ) -> terrars::ListRef<DevopsguruEventSourcesConfig_EventSources_AmazonCodeGuruProfilerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_code_guru_profiler", self.base))
    }
}

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontVpcOriginData_ {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudfrontVpcOrigin_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_origin_endpoint_config: std::option::Option<std::vec::Vec<CloudfrontVpcOrigin_VpcOriginEndpointConfig>>,
    dynamic: CloudfrontVpcOriginDynamic,
}

struct CloudfrontVpcOrigin_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontVpcOriginData_>,
}

#[derive(Clone)]
pub struct CloudfrontVpcOrigin(std::rc::Rc<CloudfrontVpcOrigin_>);

impl CloudfrontVpcOrigin {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudfrontVpcOrigin_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_origin_endpoint_config`.\n"]
    pub fn set_vpc_origin_endpoint_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontVpcOrigin_VpcOriginEndpointConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_origin_endpoint_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_origin_endpoint_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfrontVpcOrigin_TimeoutsRef {
        CloudfrontVpcOrigin_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_origin_endpoint_config` after provisioning.\n"]
    pub fn vpc_origin_endpoint_config(&self) -> terrars::ListRef<CloudfrontVpcOrigin_VpcOriginEndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_origin_endpoint_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontVpcOrigin {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontVpcOrigin { }

impl terrars::ToListMappable for CloudfrontVpcOrigin {
    type O = terrars::ListRef<CloudfrontVpcOriginRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontVpcOrigin_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_vpc_origin".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontVpcOrigin {
    pub tf_id: std::string::String,
}

impl BuildCloudfrontVpcOrigin {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontVpcOrigin {
        let out = CloudfrontVpcOrigin(std::rc::Rc::new(CloudfrontVpcOrigin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontVpcOriginData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_origin_endpoint_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontVpcOriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontVpcOriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontVpcOriginRef {
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

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfrontVpcOrigin_TimeoutsRef {
        CloudfrontVpcOrigin_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_origin_endpoint_config` after provisioning.\n"]
    pub fn vpc_origin_endpoint_config(&self) -> terrars::ListRef<CloudfrontVpcOrigin_VpcOriginEndpointConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_origin_endpoint_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontVpcOriginDynamic {
    vpc_origin_endpoint_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontVpcOrigin_VpcOriginEndpointConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontVpcOrigin_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfrontVpcOrigin_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontVpcOrigin_Timeouts {
    type O = terrars::BlockAssignable<CloudfrontVpcOrigin_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontVpcOrigin_Timeouts {}

impl BuildCloudfrontVpcOrigin_Timeouts {
    pub fn build(self) -> CloudfrontVpcOrigin_Timeouts {
        CloudfrontVpcOrigin_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontVpcOrigin_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontVpcOrigin_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfrontVpcOrigin_TimeoutsRef {
        CloudfrontVpcOrigin_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontVpcOrigin_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols {
    items: terrars::SetField<terrars::PrimField<std::string::String>>,
    quantity: terrars::PrimField<f64>,
}

impl CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols { }

impl terrars::ToListMappable for CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols {
    type O = terrars::BlockAssignable<CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols {
    #[doc = ""]
    pub items: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub quantity: terrars::PrimField<f64>,
}

impl BuildCloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols {
    pub fn build(self) -> CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols {
        CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols {
            items: self.items,
            quantity: self.quantity,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocolsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocolsRef {
        CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontVpcOrigin_VpcOriginEndpointConfigDynamic {
    origin_ssl_protocols: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols>>,
}

#[derive(Serialize)]
pub struct CloudfrontVpcOrigin_VpcOriginEndpointConfig {
    arn: terrars::PrimField<std::string::String>,
    http_port: terrars::PrimField<f64>,
    https_port: terrars::PrimField<f64>,
    name: terrars::PrimField<std::string::String>,
    origin_protocol_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_ssl_protocols: std
    ::option
    ::Option<std::vec::Vec<CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols>>,
    dynamic: CloudfrontVpcOrigin_VpcOriginEndpointConfigDynamic,
}

impl CloudfrontVpcOrigin_VpcOriginEndpointConfig {
    #[doc = "Set the field `origin_ssl_protocols`.\n"]
    pub fn set_origin_ssl_protocols(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocols>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.origin_ssl_protocols = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.origin_ssl_protocols = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontVpcOrigin_VpcOriginEndpointConfig {
    type O = terrars::BlockAssignable<CloudfrontVpcOrigin_VpcOriginEndpointConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontVpcOrigin_VpcOriginEndpointConfig {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub http_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub https_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub origin_protocol_policy: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontVpcOrigin_VpcOriginEndpointConfig {
    pub fn build(self) -> CloudfrontVpcOrigin_VpcOriginEndpointConfig {
        CloudfrontVpcOrigin_VpcOriginEndpointConfig {
            arn: self.arn,
            http_port: self.http_port,
            https_port: self.https_port,
            name: self.name,
            origin_protocol_policy: self.origin_protocol_policy,
            origin_ssl_protocols: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontVpcOrigin_VpcOriginEndpointConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontVpcOrigin_VpcOriginEndpointConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontVpcOrigin_VpcOriginEndpointConfigRef {
        CloudfrontVpcOrigin_VpcOriginEndpointConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontVpcOrigin_VpcOriginEndpointConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }

    #[doc = "Get a reference to the value of field `https_port` after provisioning.\n"]
    pub fn https_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_port", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_protocol_policy` after provisioning.\n"]
    pub fn origin_protocol_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_protocol_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_ssl_protocols` after provisioning.\n"]
    pub fn origin_ssl_protocols(
        &self,
    ) -> terrars::ListRef<CloudfrontVpcOrigin_VpcOriginEndpointConfig_OriginSslProtocolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_ssl_protocols", self.base))
    }
}

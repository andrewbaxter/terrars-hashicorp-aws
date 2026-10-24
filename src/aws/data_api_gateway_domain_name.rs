use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataApiGatewayDomainNameData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataApiGatewayDomainName_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataApiGatewayDomainNameData>,
}

#[derive(Clone)]
pub struct DataApiGatewayDomainName(std::rc::Rc<DataApiGatewayDomainName_>);

impl DataApiGatewayDomainName {
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

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_upload_date` after provisioning.\n"]
    pub fn certificate_upload_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_upload_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_domain_name` after provisioning.\n"]
    pub fn cloudfront_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_zone_id` after provisioning.\n"]
    pub fn cloudfront_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_configuration` after provisioning.\n"]
    pub fn endpoint_configuration(&self) -> terrars::ListRef<DataApiGatewayDomainNameEndpointConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_certificate_arn` after provisioning.\n"]
    pub fn regional_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regional_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_certificate_name` after provisioning.\n"]
    pub fn regional_certificate_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regional_certificate_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_domain_name` after provisioning.\n"]
    pub fn regional_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regional_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_zone_id` after provisioning.\n"]
    pub fn regional_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regional_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\n"]
    pub fn security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataApiGatewayDomainName {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataApiGatewayDomainName { }

impl terrars::ToListMappable for DataApiGatewayDomainName {
    type O = terrars::ListRef<DataApiGatewayDomainNameRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataApiGatewayDomainName_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_api_gateway_domain_name".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApiGatewayDomainName {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildDataApiGatewayDomainName {
    pub fn build(self, stack: &mut terrars::Stack) -> DataApiGatewayDomainName {
        let out = DataApiGatewayDomainName(std::rc::Rc::new(DataApiGatewayDomainName_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataApiGatewayDomainNameData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApiGatewayDomainNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiGatewayDomainNameRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataApiGatewayDomainNameRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_upload_date` after provisioning.\n"]
    pub fn certificate_upload_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_upload_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_domain_name` after provisioning.\n"]
    pub fn cloudfront_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudfront_zone_id` after provisioning.\n"]
    pub fn cloudfront_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudfront_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_configuration` after provisioning.\n"]
    pub fn endpoint_configuration(&self) -> terrars::ListRef<DataApiGatewayDomainNameEndpointConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_certificate_arn` after provisioning.\n"]
    pub fn regional_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regional_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_certificate_name` after provisioning.\n"]
    pub fn regional_certificate_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regional_certificate_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_domain_name` after provisioning.\n"]
    pub fn regional_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regional_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regional_zone_id` after provisioning.\n"]
    pub fn regional_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regional_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_policy` after provisioning.\n"]
    pub fn security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataApiGatewayDomainNameEndpointConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataApiGatewayDomainNameEndpointConfigurationEl {
    #[doc = "Set the field `types`.\n"]
    pub fn set_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApiGatewayDomainNameEndpointConfigurationEl {
    type O = terrars::BlockAssignable<DataApiGatewayDomainNameEndpointConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiGatewayDomainNameEndpointConfigurationEl {}

impl BuildDataApiGatewayDomainNameEndpointConfigurationEl {
    pub fn build(self) -> DataApiGatewayDomainNameEndpointConfigurationEl {
        DataApiGatewayDomainNameEndpointConfigurationEl { types: core::default::Default::default() }
    }
}

pub struct DataApiGatewayDomainNameEndpointConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApiGatewayDomainNameEndpointConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApiGatewayDomainNameEndpointConfigurationElRef {
        DataApiGatewayDomainNameEndpointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiGatewayDomainNameEndpointConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecuritylakeCustomLogSourceData_ {
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
    event_classes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    source_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<SecuritylakeCustomLogSource_Configuration>>,
    dynamic: SecuritylakeCustomLogSourceDynamic,
}

struct SecuritylakeCustomLogSource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecuritylakeCustomLogSourceData_>,
}

#[derive(Clone)]
pub struct SecuritylakeCustomLogSource(std::rc::Rc<SecuritylakeCustomLogSource_>);

impl SecuritylakeCustomLogSource {
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

    #[doc = "Set the field `event_classes`.\n"]
    pub fn set_event_classes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().event_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_version`.\n"]
    pub fn set_source_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecuritylakeCustomLogSource_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<SecuritylakeCustomLogSource_AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_classes` after provisioning.\n"]
    pub fn event_classes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.event_classes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_details` after provisioning.\n"]
    pub fn provider_details(&self) -> terrars::ListRef<SecuritylakeCustomLogSource_ProviderDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<SecuritylakeCustomLogSource_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl terrars::Referable for SecuritylakeCustomLogSource {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecuritylakeCustomLogSource { }

impl terrars::ToListMappable for SecuritylakeCustomLogSource {
    type O = terrars::ListRef<SecuritylakeCustomLogSourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecuritylakeCustomLogSource_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securitylake_custom_log_source".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecuritylakeCustomLogSource {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub source_name: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeCustomLogSource {
    pub fn build(self, stack: &mut terrars::Stack) -> SecuritylakeCustomLogSource {
        let out = SecuritylakeCustomLogSource(std::rc::Rc::new(SecuritylakeCustomLogSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecuritylakeCustomLogSourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                event_classes: core::default::Default::default(),
                region: core::default::Default::default(),
                source_name: self.source_name,
                source_version: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecuritylakeCustomLogSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeCustomLogSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecuritylakeCustomLogSourceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<SecuritylakeCustomLogSource_AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_classes` after provisioning.\n"]
    pub fn event_classes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.event_classes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provider_details` after provisioning.\n"]
    pub fn provider_details(&self) -> terrars::ListRef<SecuritylakeCustomLogSource_ProviderDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<SecuritylakeCustomLogSource_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeCustomLogSourceDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<SecuritylakeCustomLogSource_Configuration>>,
}

#[derive(Serialize)]
pub struct SecuritylakeCustomLogSource_Attributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeCustomLogSource_Attributes {
    #[doc = "Set the field `crawler_arn`.\n"]
    pub fn set_crawler_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crawler_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_arn`.\n"]
    pub fn set_database_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_arn`.\n"]
    pub fn set_table_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeCustomLogSource_Attributes {
    type O = terrars::BlockAssignable<SecuritylakeCustomLogSource_Attributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeCustomLogSource_Attributes {}

impl BuildSecuritylakeCustomLogSource_Attributes {
    pub fn build(self) -> SecuritylakeCustomLogSource_Attributes {
        SecuritylakeCustomLogSource_Attributes {
            crawler_arn: core::default::Default::default(),
            database_arn: core::default::Default::default(),
            table_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeCustomLogSource_AttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeCustomLogSource_AttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecuritylakeCustomLogSource_AttributesRef {
        SecuritylakeCustomLogSource_AttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeCustomLogSource_AttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_arn` after provisioning.\n"]
    pub fn crawler_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawler_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_arn` after provisioning.\n"]
    pub fn database_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeCustomLogSource_ProviderDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeCustomLogSource_ProviderDetails {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeCustomLogSource_ProviderDetails {
    type O = terrars::BlockAssignable<SecuritylakeCustomLogSource_ProviderDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeCustomLogSource_ProviderDetails {}

impl BuildSecuritylakeCustomLogSource_ProviderDetails {
    pub fn build(self) -> SecuritylakeCustomLogSource_ProviderDetails {
        SecuritylakeCustomLogSource_ProviderDetails {
            location: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeCustomLogSource_ProviderDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeCustomLogSource_ProviderDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeCustomLogSource_ProviderDetailsRef {
        SecuritylakeCustomLogSource_ProviderDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeCustomLogSource_ProviderDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeCustomLogSource_Configuration_CrawlerConfiguration {
    role_arn: terrars::PrimField<std::string::String>,
}

impl SecuritylakeCustomLogSource_Configuration_CrawlerConfiguration { }

impl terrars::ToListMappable for SecuritylakeCustomLogSource_Configuration_CrawlerConfiguration {
    type O = terrars::BlockAssignable<SecuritylakeCustomLogSource_Configuration_CrawlerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeCustomLogSource_Configuration_CrawlerConfiguration {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeCustomLogSource_Configuration_CrawlerConfiguration {
    pub fn build(self) -> SecuritylakeCustomLogSource_Configuration_CrawlerConfiguration {
        SecuritylakeCustomLogSource_Configuration_CrawlerConfiguration { role_arn: self.role_arn }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeCustomLogSource_Configuration_CrawlerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeCustomLogSource_Configuration_CrawlerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeCustomLogSource_Configuration_CrawlerConfigurationRef {
        SecuritylakeCustomLogSource_Configuration_CrawlerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeCustomLogSource_Configuration_CrawlerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeCustomLogSource_Configuration_ProviderIdentity {
    external_id: terrars::PrimField<std::string::String>,
    principal: terrars::PrimField<std::string::String>,
}

impl SecuritylakeCustomLogSource_Configuration_ProviderIdentity { }

impl terrars::ToListMappable for SecuritylakeCustomLogSource_Configuration_ProviderIdentity {
    type O = terrars::BlockAssignable<SecuritylakeCustomLogSource_Configuration_ProviderIdentity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeCustomLogSource_Configuration_ProviderIdentity {
    #[doc = ""]
    pub external_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeCustomLogSource_Configuration_ProviderIdentity {
    pub fn build(self) -> SecuritylakeCustomLogSource_Configuration_ProviderIdentity {
        SecuritylakeCustomLogSource_Configuration_ProviderIdentity {
            external_id: self.external_id,
            principal: self.principal,
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeCustomLogSource_Configuration_ProviderIdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeCustomLogSource_Configuration_ProviderIdentityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeCustomLogSource_Configuration_ProviderIdentityRef {
        SecuritylakeCustomLogSource_Configuration_ProviderIdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeCustomLogSource_Configuration_ProviderIdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeCustomLogSource_ConfigurationDynamic {
    crawler_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeCustomLogSource_Configuration_CrawlerConfiguration>>,
    provider_identity: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeCustomLogSource_Configuration_ProviderIdentity>>,
}

#[derive(Serialize)]
pub struct SecuritylakeCustomLogSource_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_configuration: std
    ::option
    ::Option<std::vec::Vec<SecuritylakeCustomLogSource_Configuration_CrawlerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_identity: std::option::Option<std::vec::Vec<SecuritylakeCustomLogSource_Configuration_ProviderIdentity>>,
    dynamic: SecuritylakeCustomLogSource_ConfigurationDynamic,
}

impl SecuritylakeCustomLogSource_Configuration {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecuritylakeCustomLogSource_Configuration_CrawlerConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crawler_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `provider_identity`.\n"]
    pub fn set_provider_identity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecuritylakeCustomLogSource_Configuration_ProviderIdentity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provider_identity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provider_identity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecuritylakeCustomLogSource_Configuration {
    type O = terrars::BlockAssignable<SecuritylakeCustomLogSource_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeCustomLogSource_Configuration {}

impl BuildSecuritylakeCustomLogSource_Configuration {
    pub fn build(self) -> SecuritylakeCustomLogSource_Configuration {
        SecuritylakeCustomLogSource_Configuration {
            crawler_configuration: core::default::Default::default(),
            provider_identity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeCustomLogSource_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeCustomLogSource_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecuritylakeCustomLogSource_ConfigurationRef {
        SecuritylakeCustomLogSource_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeCustomLogSource_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> terrars::ListRef<SecuritylakeCustomLogSource_Configuration_CrawlerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crawler_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_identity` after provisioning.\n"]
    pub fn provider_identity(&self) -> terrars::ListRef<SecuritylakeCustomLogSource_Configuration_ProviderIdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider_identity", self.base))
    }
}

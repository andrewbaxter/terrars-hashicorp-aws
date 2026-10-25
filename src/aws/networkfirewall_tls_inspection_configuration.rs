#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallTlsInspectionConfigurationData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<terrars::ListField<NetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkfirewallTlsInspectionConfiguration_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_inspection_configuration: std
    ::option
    ::Option<std::vec::Vec<NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration>>,
    dynamic: NetworkfirewallTlsInspectionConfigurationDynamic,
}

struct NetworkfirewallTlsInspectionConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkfirewallTlsInspectionConfigurationData_>,
}

#[derive(Clone)]
pub struct NetworkfirewallTlsInspectionConfiguration(std::rc::Rc<NetworkfirewallTlsInspectionConfiguration_>);

impl NetworkfirewallTlsInspectionConfiguration {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<NetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration>>,
    ) -> Self {
        self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkfirewallTlsInspectionConfiguration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_inspection_configuration`.\n"]
    pub fn set_tls_inspection_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tls_inspection_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tls_inspection_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(
        &self,
    ) -> terrars::ListRef<NetworkfirewallTlsInspectionConfiguration_CertificateAuthorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> terrars::ListRef<NetworkfirewallTlsInspectionConfiguration_CertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<NetworkfirewallTlsInspectionConfiguration_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_associations` after provisioning.\n"]
    pub fn number_of_associations(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_associations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_inspection_configuration_id` after provisioning.\n"]
    pub fn tls_inspection_configuration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_inspection_configuration_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallTlsInspectionConfiguration_TimeoutsRef {
        NetworkfirewallTlsInspectionConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tls_inspection_configuration` after provisioning.\n"]
    pub fn tls_inspection_configuration(
        &self,
    ) -> terrars::ListRef<NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_inspection_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkfirewallTlsInspectionConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkfirewallTlsInspectionConfiguration { }

impl terrars::ToListMappable for NetworkfirewallTlsInspectionConfiguration {
    type O = terrars::ListRef<NetworkfirewallTlsInspectionConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkfirewallTlsInspectionConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_networkfirewall_tls_inspection_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallTlsInspectionConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkfirewallTlsInspectionConfiguration {
        let out =
            NetworkfirewallTlsInspectionConfiguration(std::rc::Rc::new(NetworkfirewallTlsInspectionConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(NetworkfirewallTlsInspectionConfigurationData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    description: core::default::Default::default(),
                    encryption_configuration: core::default::Default::default(),
                    name: self.name,
                    region: core::default::Default::default(),
                    tags: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    tls_inspection_configuration: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallTlsInspectionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallTlsInspectionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkfirewallTlsInspectionConfigurationRef {
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

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(
        &self,
    ) -> terrars::ListRef<NetworkfirewallTlsInspectionConfiguration_CertificateAuthorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificates` after provisioning.\n"]
    pub fn certificates(&self) -> terrars::ListRef<NetworkfirewallTlsInspectionConfiguration_CertificatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<NetworkfirewallTlsInspectionConfiguration_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_associations` after provisioning.\n"]
    pub fn number_of_associations(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_associations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_inspection_configuration_id` after provisioning.\n"]
    pub fn tls_inspection_configuration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_inspection_configuration_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallTlsInspectionConfiguration_TimeoutsRef {
        NetworkfirewallTlsInspectionConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tls_inspection_configuration` after provisioning.\n"]
    pub fn tls_inspection_configuration(
        &self,
    ) -> terrars::ListRef<NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_inspection_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallTlsInspectionConfigurationDynamic {
    tls_inspection_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_CertificateAuthority {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_serial: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallTlsInspectionConfiguration_CertificateAuthority {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_serial`.\n"]
    pub fn set_certificate_serial(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_serial = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallTlsInspectionConfiguration_CertificateAuthority {
    type O = terrars::BlockAssignable<NetworkfirewallTlsInspectionConfiguration_CertificateAuthority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_CertificateAuthority {}

impl BuildNetworkfirewallTlsInspectionConfiguration_CertificateAuthority {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfiguration_CertificateAuthority {
        NetworkfirewallTlsInspectionConfiguration_CertificateAuthority {
            certificate_arn: core::default::Default::default(),
            certificate_serial: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_CertificateAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallTlsInspectionConfiguration_CertificateAuthorityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_CertificateAuthorityRef {
        NetworkfirewallTlsInspectionConfiguration_CertificateAuthorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_CertificateAuthorityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_serial` after provisioning.\n"]
    pub fn certificate_serial(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_serial", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_Certificates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_serial: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallTlsInspectionConfiguration_Certificates {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_serial`.\n"]
    pub fn set_certificate_serial(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_serial = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallTlsInspectionConfiguration_Certificates {
    type O = terrars::BlockAssignable<NetworkfirewallTlsInspectionConfiguration_Certificates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_Certificates {}

impl BuildNetworkfirewallTlsInspectionConfiguration_Certificates {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfiguration_Certificates {
        NetworkfirewallTlsInspectionConfiguration_Certificates {
            certificate_arn: core::default::Default::default(),
            certificate_serial: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_CertificatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallTlsInspectionConfiguration_CertificatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_CertificatesRef {
        NetworkfirewallTlsInspectionConfiguration_CertificatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_CertificatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_serial` after provisioning.\n"]
    pub fn certificate_serial(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_serial", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration {
    type O = terrars::BlockAssignable<NetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration {}

impl BuildNetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration {
        NetworkfirewallTlsInspectionConfiguration_EncryptionConfiguration {
            key_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallTlsInspectionConfiguration_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_EncryptionConfigurationRef {
        NetworkfirewallTlsInspectionConfiguration_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallTlsInspectionConfiguration_Timeouts {
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

impl terrars::ToListMappable for NetworkfirewallTlsInspectionConfiguration_Timeouts {
    type O = terrars::BlockAssignable<NetworkfirewallTlsInspectionConfiguration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_Timeouts {}

impl BuildNetworkfirewallTlsInspectionConfiguration_Timeouts {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfiguration_Timeouts {
        NetworkfirewallTlsInspectionConfiguration_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallTlsInspectionConfiguration_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TimeoutsRef {
        NetworkfirewallTlsInspectionConfiguration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TimeoutsRef {
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
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revoked_status_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unknown_status_action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus {
    #[doc = "Set the field `revoked_status_action`.\n"]
    pub fn set_revoked_status_action(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.revoked_status_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unknown_status_action`.\n"]
    pub fn set_unknown_status_action(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unknown_status_action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus {}

impl BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus {
            revoked_status_action: core::default::Default::default(),
            unknown_status_action: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatusRef {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `revoked_status_action` after provisioning.\n"]
    pub fn revoked_status_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revoked_status_action", self.base))
    }

    #[doc = "Get a reference to the value of field `unknown_status_action` after provisioning.\n"]
    pub fn unknown_status_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unknown_status_action", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination {
    address_definition: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination {

}

impl terrars
::ToListMappable for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination {
    #[doc = ""]
    pub address_definition: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination {
            address_definition: self.address_definition,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationRef {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_definition` after provisioning.\n"]
    pub fn address_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts {
    from_port: terrars::PrimField<f64>,
    to_port: terrars::PrimField<f64>,
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts {

}

impl terrars
::ToListMappable for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts {
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub to_port: terrars::PrimField<f64>,
}

impl BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts {
            from_port: self.from_port,
            to_port: self.to_port,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPortsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPortsRef {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPortsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source {
    address_definition: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source {

}

impl terrars
::ToListMappable for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source {
    #[doc = ""]
    pub address_definition: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source {
            address_definition: self.address_definition,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourceRef {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_definition` after provisioning.\n"]
    pub fn address_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts {
    from_port: terrars::PrimField<f64>,
    to_port: terrars::PrimField<f64>,
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts {

}

impl terrars
::ToListMappable for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts {
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub to_port: terrars::PrimField<f64>,
}

impl BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts {
            from_port: self.from_port,
            to_port: self.to_port,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePortsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePortsRef {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePortsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ScopeDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination,
        >,
    >,
    destination_ports: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts,
        >,
    >,
    source: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source,
        >,
    >,
    source_ports: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope {
    protocols: terrars::SetField<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_ports: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ports: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts,
        >,
    >,
    dynamic: NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ScopeDynamic,
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_ports`.\n"]
    pub fn set_destination_ports(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPorts,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_ports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_ports = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_Source,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_ports`.\n"]
    pub fn set_source_ports(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePorts,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_ports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_ports = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope {
    #[doc = ""]
    pub protocols: terrars::SetField<terrars::PrimField<f64>>,
}

impl BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope {
            protocols: self.protocols,
            destination: core::default::Default::default(),
            destination_ports: core::default::Default::default(),
            source: core::default::Default::default(),
            source_ports: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ScopeRef {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_ports` after provisioning.\n"]
    pub fn destination_ports(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_DestinationPortsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_ports", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ports` after provisioning.\n"]
    pub fn source_ports(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope_SourcePortsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ports", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate {
    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate {}

impl BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate {
            resource_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificateRef {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfigurationDynamic {
    check_certificate_revocation_status: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus,
        >,
    >,
    scope: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope,
        >,
    >,
    server_certificate: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_certificate_revocation_status: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_certificate: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate,
        >,
    >,
    dynamic: NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfigurationDynamic,
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration {
    #[doc = "Set the field `certificate_authority_arn`.\n"]
    pub fn set_certificate_authority_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_authority_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `check_certificate_revocation_status`.\n"]
    pub fn set_check_certificate_revocation_status(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatus,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.check_certificate_revocation_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.check_certificate_revocation_status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_Scope,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scope = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_certificate`.\n"]
    pub fn set_server_certificate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificate,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.server_certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.server_certificate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration {}

impl BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration {
    pub fn build(
        self,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration {
            certificate_authority_arn: core::default::Default::default(),
            check_certificate_revocation_status: core::default::Default::default(),
            scope: core::default::Default::default(),
            server_certificate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfigurationRef {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `check_certificate_revocation_status` after provisioning.\n"]
    pub fn check_certificate_revocation_status(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_CheckCertificateRevocationStatusRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.check_certificate_revocation_status", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ScopeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `server_certificate` after provisioning.\n"]
    pub fn server_certificate(
        &self,
    ) -> terrars
    ::ListRef<
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration_ServerCertificateRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfigurationDynamic {
    server_certificate_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_certificate_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration>,
    >,
    dynamic: NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfigurationDynamic,
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration {
    #[doc = "Set the field `server_certificate_configuration`.\n"]
    pub fn set_server_certificate_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.server_certificate_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.server_certificate_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration {
    type O = terrars::BlockAssignable<NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration {}

impl BuildNetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration {
    pub fn build(self) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration {
            server_certificate_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfigurationRef {
        NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `server_certificate_configuration` after provisioning.\n"]
    pub fn server_certificate_configuration(
        &self,
    ) -> terrars
    ::ListRef<NetworkfirewallTlsInspectionConfiguration_TlsInspectionConfiguration_ServerCertificateConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_certificate_configuration", self.base))
    }
}

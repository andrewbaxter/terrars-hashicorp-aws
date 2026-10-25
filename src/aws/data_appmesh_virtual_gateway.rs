#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshVirtualGatewayData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    mesh_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataAppmeshVirtualGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAppmeshVirtualGatewayData_>,
}

#[derive(Clone)]
pub struct DataAppmeshVirtualGateway(std::rc::Rc<DataAppmeshVirtualGateway_>);

impl DataAppmeshVirtualGateway {
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

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataAppmeshVirtualGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAppmeshVirtualGateway { }

impl terrars::ToListMappable for DataAppmeshVirtualGateway {
    type O = terrars::ListRef<DataAppmeshVirtualGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAppmeshVirtualGateway_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_appmesh_virtual_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshVirtualGateway {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataAppmeshVirtualGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAppmeshVirtualGateway {
        let out = DataAppmeshVirtualGateway(std::rc::Rc::new(DataAppmeshVirtualGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAppmeshVirtualGatewayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppmeshVirtualGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppmeshVirtualGatewayRef {
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

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_chain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_chain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\n"]
    pub fn set_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File {
            certificate_chain: core::default::Default::default(),
            private_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_FileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_FileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_FileRef {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_FileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_FileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds {
    #[doc = "Set the field `secret_name`.\n"]
    pub fn set_secret_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds {
            secret_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_SdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_SdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_SdsRef {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_SdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_SdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_name` after provisioning.\n"]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds>>,
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File>>,
    ) -> Self {
        self.file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds>>,
    ) -> Self {
        self.sds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {}

impl BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_SdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
    type O =
        terrars
        ::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match {
            exact: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std
    ::option
    ::Option<
        terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match>,
    >,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_Match,
                        >,
                    >,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
    type O =
        terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames {
            match_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(
        &self,
    ) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
    #[doc = "Set the field `certificate_authority_arns`.\n"]
    pub fn set_certificate_authority_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.certificate_authority_arns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
    type O =
        terrars
        ::BlockAssignable<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {}

impl BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm {
            certificate_authority_arns: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arns` after provisioning.\n"]
    pub fn certificate_authority_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.certificate_authority_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_chain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File {
    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_chain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File {
            certificate_chain: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_FileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_FileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_FileRef {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_FileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_FileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm: std
    ::option
    ::Option<
        terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds>>,
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_Acm,
                        >,
                    >,
    ) -> Self {
        self.acm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File>>,
    ) -> Self {
        self.file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds>>,
    ) -> Self {
        self.sds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
    type O =
        terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {}

impl BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(
        &self,
    ) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust_AcmRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_SdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std
    ::option
    ::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust: std
    ::option
    ::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust>>,
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames>,
                    >,
    ) -> Self {
        self.subject_alternative_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_Trust>,
                    >,
    ) -> Self {
        self.trust = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation {}

impl BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(
        &self,
    ) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation_TrustRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std
    ::option
    ::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std
    ::option
    ::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation>>,
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Certificate>,
                    >,
    ) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce`.\n"]
    pub fn set_enforce(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(mut self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_Validation>,
                    >,
    ) -> Self {
        self.validation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls {}

impl BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls {
            certificate: core::default::Default::default(),
            enforce: core::default::Default::default(),
            ports: core::default::Default::default(),
            validation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_TlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_TlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_TlsRef {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_TlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_TlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(
        &self,
    ) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce` after provisioning.\n"]
    pub fn enforce(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(
        &self,
    ) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls_ValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls>>,
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy {
    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_Tls>>,
    ) -> Self {
        self.tls = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy {}

impl BuildDataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy { tls: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicyRef {
        DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy_TlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_policy: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy>>,
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaults {
    #[doc = "Set the field `client_policy`.\n"]
    pub fn set_client_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicy>>,
    ) -> Self {
        self.client_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_BackendDefaults {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_BackendDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_BackendDefaults {}

impl BuildDataAppmeshVirtualGateway_Spec_BackendDefaults {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_BackendDefaults {
        DataAppmeshVirtualGateway_Spec_BackendDefaults { client_policy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_BackendDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_BackendDefaultsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_BackendDefaultsRef {
        DataAppmeshVirtualGateway_Spec_BackendDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_BackendDefaultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_policy` after provisioning.\n"]
    pub fn client_policy(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_BackendDefaults_ClientPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_requests: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc {
    #[doc = "Set the field `max_requests`.\n"]
    pub fn set_max_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_requests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc {
        DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc {
            max_requests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_GrpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_GrpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_GrpcRef {
        DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_GrpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_GrpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_connections: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_requests: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http {
    #[doc = "Set the field `max_connections`.\n"]
    pub fn set_max_connections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_pending_requests`.\n"]
    pub fn set_max_pending_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_pending_requests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http {
        DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http {
            max_connections: core::default::Default::default(),
            max_pending_requests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_HttpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_HttpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_HttpRef {
        DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_HttpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_HttpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_connections` after provisioning.\n"]
    pub fn max_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `max_pending_requests` after provisioning.\n"]
    pub fn max_pending_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pending_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_requests: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2 {
    #[doc = "Set the field `max_requests`.\n"]
    pub fn set_max_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_requests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2 {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2 {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2 {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2 {
        DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2 {
            max_requests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2Ref {
        DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_requests` after provisioning.\n"]
    pub fn max_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_requests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Grpc>>,
    ) -> Self {
        self.grpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http>>,
    ) -> Self {
        self.http = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http2`.\n"]
    pub fn set_http2(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2>>,
    ) -> Self {
        self.http2 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_ConnectionPool {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_ConnectionPool {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool {
        DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
            http2: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_ConnectionPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_ConnectionPoolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_ConnectionPoolRef {
        DataAppmeshVirtualGateway_Spec_Listener_ConnectionPoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_ConnectionPoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_HttpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http", self.base))
    }

    #[doc = "Get a reference to the value of field `http2` after provisioning.\n"]
    pub fn http2(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool_Http2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_HealthCheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_millis: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_millis: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unhealthy_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_HealthCheck {
    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.healthy_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_millis`.\n"]
    pub fn set_interval_millis(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval_millis = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_millis`.\n"]
    pub fn set_timeout_millis(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_millis = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unhealthy_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_HealthCheck {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_HealthCheck {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_HealthCheck {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_HealthCheck {
        DataAppmeshVirtualGateway_Spec_Listener_HealthCheck {
            healthy_threshold: core::default::Default::default(),
            interval_millis: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            timeout_millis: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_HealthCheckRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_HealthCheckRef {
        DataAppmeshVirtualGateway_Spec_Listener_HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_HealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_millis` after provisioning.\n"]
    pub fn interval_millis(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_millis", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_millis` after provisioning.\n"]
    pub fn timeout_millis(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_millis", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_PortMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_PortMapping {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_PortMapping {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_PortMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_PortMapping {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_PortMapping {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_PortMapping {
        DataAppmeshVirtualGateway_Spec_Listener_PortMapping {
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_PortMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_PortMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_PortMappingRef {
        DataAppmeshVirtualGateway_Spec_Listener_PortMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_PortMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm {
    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm {
            certificate_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_AcmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_AcmRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_AcmRef {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_AcmRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_AcmRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate {
    #[doc = "Set the field `acm`.\n"]
    pub fn set_acm(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Acm>>,
    ) -> Self {
        self.acm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_File>>,
    ) -> Self {
        self.file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds>>,
    ) -> Self {
        self.sds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate {
            acm: core::default::Default::default(),
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_Tls_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_CertificateRef {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm` after provisioning.\n"]
    pub fn acm(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_AcmRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acm", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_SdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sds: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_File>>,
    ) -> Self {
        self.file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sds`.\n"]
    pub fn set_sds(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_Sds>>,
    ) -> Self {
        self.sds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust {
            file: core::default::Default::default(),
            sds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_TrustRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_TrustRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_TrustRef {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_TrustRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_TrustRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `sds` after provisioning.\n"]
    pub fn sds(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate_SdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std
    ::option
    ::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation {
    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNames>,
                    >,
    ) -> Self {
        self.subject_alternative_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust`.\n"]
    pub fn set_trust(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_Trust>>,
    ) -> Self {
        self.trust = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls_Validation {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation {
            subject_alternative_names: core::default::Default::default(),
            trust: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls_ValidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_Tls_ValidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_Tls_ValidationRef {
        DataAppmeshVirtualGateway_Spec_Listener_Tls_ValidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls_ValidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(
        &self,
    ) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_SubjectAlternativeNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.base))
    }

    #[doc = "Get a reference to the value of field `trust` after provisioning.\n"]
    pub fn trust(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation_TrustRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_Tls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener_Tls {
    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Certificate>>,
    ) -> Self {
        self.certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation`.\n"]
    pub fn set_validation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls_Validation>>,
    ) -> Self {
        self.validation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener_Tls {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener_Tls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener_Tls {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener_Tls {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener_Tls {
        DataAppmeshVirtualGateway_Spec_Listener_Tls {
            certificate: core::default::Default::default(),
            mode: core::default::Default::default(),
            validation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Listener_TlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Listener_TlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Listener_TlsRef {
        DataAppmeshVirtualGateway_Spec_Listener_TlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Listener_TlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `validation` after provisioning.\n"]
    pub fn validation(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_Tls_ValidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Listener {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_pool: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_HealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_mapping: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_PortMapping>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls>>,
}

impl DataAppmeshVirtualGateway_Spec_Listener {
    #[doc = "Set the field `connection_pool`.\n"]
    pub fn set_connection_pool(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPool>>,
    ) -> Self {
        self.connection_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_HealthCheck>>,
    ) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_mapping`.\n"]
    pub fn set_port_mapping(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_PortMapping>>,
    ) -> Self {
        self.port_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls`.\n"]
    pub fn set_tls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener_Tls>>,
    ) -> Self {
        self.tls = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Listener {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Listener>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Listener {}

impl BuildDataAppmeshVirtualGateway_Spec_Listener {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Listener {
        DataAppmeshVirtualGateway_Spec_Listener {
            connection_pool: core::default::Default::default(),
            health_check: core::default::Default::default(),
            port_mapping: core::default::Default::default(),
            tls: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_ListenerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_ListenerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshVirtualGateway_Spec_ListenerRef {
        DataAppmeshVirtualGateway_Spec_ListenerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_ListenerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_pool` after provisioning.\n"]
    pub fn connection_pool(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_ConnectionPoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_HealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_PortMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `tls` after provisioning.\n"]
    pub fn tls(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Listener_TlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json {}

impl BuildDataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json {
        DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_JsonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_JsonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_JsonRef {
        DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_JsonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_JsonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format {
    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_Json>>,
    ) -> Self {
        self.json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format {}

impl BuildDataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format {
        DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format {
            json: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_FormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_FormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_FormatRef {
        DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_FormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_FormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format_JsonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_Format>>,
    ) -> Self {
        self.format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Logging_AccessLog_File {}

impl BuildDataAppmeshVirtualGateway_Spec_Logging_AccessLog_File {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File {
        DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File {
            format: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Logging_AccessLog_FileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Logging_AccessLog_FileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Logging_AccessLog_FileRef {
        DataAppmeshVirtualGateway_Spec_Logging_AccessLog_FileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Logging_AccessLog_FileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File_FormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Logging_AccessLog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File>>,
}

impl DataAppmeshVirtualGateway_Spec_Logging_AccessLog {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_File>>,
    ) -> Self {
        self.file = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Logging_AccessLog {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Logging_AccessLog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Logging_AccessLog {}

impl BuildDataAppmeshVirtualGateway_Spec_Logging_AccessLog {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Logging_AccessLog {
        DataAppmeshVirtualGateway_Spec_Logging_AccessLog { file: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_Logging_AccessLogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_Logging_AccessLogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshVirtualGateway_Spec_Logging_AccessLogRef {
        DataAppmeshVirtualGateway_Spec_Logging_AccessLogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_Logging_AccessLogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Logging_AccessLog_FileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec_Logging {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_log: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging_AccessLog>>,
}

impl DataAppmeshVirtualGateway_Spec_Logging {
    #[doc = "Set the field `access_log`.\n"]
    pub fn set_access_log(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging_AccessLog>>,
    ) -> Self {
        self.access_log = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec_Logging {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec_Logging {}

impl BuildDataAppmeshVirtualGateway_Spec_Logging {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec_Logging {
        DataAppmeshVirtualGateway_Spec_Logging { access_log: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_Spec_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_Spec_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshVirtualGateway_Spec_LoggingRef {
        DataAppmeshVirtualGateway_Spec_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_Spec_LoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_log` after provisioning.\n"]
    pub fn access_log(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_Logging_AccessLogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_log", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshVirtualGateway_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_defaults: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    listener: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging>>,
}

impl DataAppmeshVirtualGateway_Spec {
    #[doc = "Set the field `backend_defaults`.\n"]
    pub fn set_backend_defaults(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_BackendDefaults>>,
    ) -> Self {
        self.backend_defaults = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Listener>>,
    ) -> Self {
        self.listener = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshVirtualGateway_Spec_Logging>>,
    ) -> Self {
        self.logging = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshVirtualGateway_Spec {
    type O = terrars::BlockAssignable<DataAppmeshVirtualGateway_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshVirtualGateway_Spec {}

impl BuildDataAppmeshVirtualGateway_Spec {
    pub fn build(self) -> DataAppmeshVirtualGateway_Spec {
        DataAppmeshVirtualGateway_Spec {
            backend_defaults: core::default::Default::default(),
            listener: core::default::Default::default(),
            logging: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshVirtualGateway_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshVirtualGateway_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshVirtualGateway_SpecRef {
        DataAppmeshVirtualGateway_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshVirtualGateway_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backend_defaults` after provisioning.\n"]
    pub fn backend_defaults(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_BackendDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backend_defaults", self.base))
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_ListenerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<DataAppmeshVirtualGateway_Spec_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.base))
    }
}

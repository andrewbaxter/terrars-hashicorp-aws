#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMskBootstrapBrokersData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cluster_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataMskBootstrapBrokers_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMskBootstrapBrokersData_>,
}

#[derive(Clone)]
pub struct DataMskBootstrapBrokers(std::rc::Rc<DataMskBootstrapBrokers_>);

impl DataMskBootstrapBrokers {
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

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_iam_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_scram_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_tls_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_sasl_iam", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.extract_ref()))
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
}

impl terrars::Referable for DataMskBootstrapBrokers {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMskBootstrapBrokers { }

impl terrars::ToListMappable for DataMskBootstrapBrokers {
    type O = terrars::ListRef<DataMskBootstrapBrokersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMskBootstrapBrokers_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_msk_bootstrap_brokers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMskBootstrapBrokers {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataMskBootstrapBrokers {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMskBootstrapBrokers {
        let out = DataMskBootstrapBrokers(std::rc::Rc::new(DataMskBootstrapBrokers_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMskBootstrapBrokersData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_arn: self.cluster_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMskBootstrapBrokersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMskBootstrapBrokersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMskBootstrapBrokersRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers` after provisioning.\n"]
    pub fn bootstrap_brokers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_iam", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_public_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_public_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_public_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_public_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_public_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_iam", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_iam_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_iam_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_iam_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_sasl_scram", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_sasl_scram_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_sasl_scram_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_sasl_scram_ipv6", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_tls_ipv6` after provisioning.\n"]
    pub fn bootstrap_brokers_tls_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_brokers_tls_ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_iam` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_sasl_iam", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_sasl_scram` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_sasl_scram(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_sasl_scram", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `bootstrap_brokers_vpc_connectivity_tls` after provisioning.\n"]
    pub fn bootstrap_brokers_vpc_connectivity_tls(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_brokers_vpc_connectivity_tls", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.extract_ref()))
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
}

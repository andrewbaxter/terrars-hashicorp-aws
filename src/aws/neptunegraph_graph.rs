#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NeptunegraphGraphData_ {
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
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    graph_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    graph_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    provisioned_memory: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_connectivity: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NeptunegraphGraph_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_search_configuration: std::option::Option<std::vec::Vec<NeptunegraphGraph_VectorSearchConfiguration>>,
    dynamic: NeptunegraphGraphDynamic,
}

struct NeptunegraphGraph_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NeptunegraphGraphData_>,
}

#[derive(Clone)]
pub struct NeptunegraphGraph(std::rc::Rc<NeptunegraphGraph_>);

impl NeptunegraphGraph {
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
        "Set the field `deletion_protection`.\nA value that indicates whether the graph has deletion protection enabled. The graph can't be deleted when deletion protection is enabled."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `graph_name`.\nThe graph name. For example: my-graph-1.\n\t\t\t\t\t\t\t\tThe name must contain from 1 to 63 letters, numbers, or hyphens, \n\t\t\t\t\t\t\t\tand its first character must be a letter. It cannot end with a hyphen or contain two consecutive hyphens.\n\t\t\t\t\t\t\t\tIf you don't specify a graph name, a unique graph name is generated for you using the prefix graph-for, \n\t\t\t\t\t\t\t\tfollowed by a combination of Stack Name and a UUID."]
    pub fn set_graph_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().graph_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `graph_name_prefix`.\nAllows user to specify name prefix and have remainder of name automatically generated."]
    pub fn set_graph_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().graph_name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_identifier`.\nSpecifies a KMS key to use to encrypt data in the new graph.  Value must be ARN of KMS Key."]
    pub fn set_kms_key_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `public_connectivity`.\nSpecifies whether or not the graph can be reachable over the internet. \n\t\t\t\t\t\t\t\tAll access to graphs is IAM authenticated.\n\t\t\t\t\t\t\t\tWhen the graph is publicly available, its domain name system (DNS) endpoint resolves to \n\t\t\t\t\t\t\t\tthe public IP address from the internet. When the graph isn't publicly available, you need \n\t\t\t\t\t\t\t\tto create a PrivateGraphEndpoint in a given VPC to ensure the DNS name resolves to a private \n\t\t\t\t\t\t\t\tIP address that is reachable from the VPC."]
    pub fn set_public_connectivity(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_connectivity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_count`.\nThe number of replicas in other AZs.  Value must be between 0 and 2."]
    pub fn set_replica_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replica_count = std::option::Option::Some(v.into());
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
    pub fn set_timeouts(self, v: impl std::convert::Into<NeptunegraphGraph_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vector_search_configuration`.\n"]
    pub fn set_vector_search_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NeptunegraphGraph_VectorSearchConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vector_search_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vector_search_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nA value that indicates whether the graph has deletion protection enabled. The graph can't be deleted when deletion protection is enabled."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `graph_name` after provisioning.\nThe graph name. For example: my-graph-1.\n\t\t\t\t\t\t\t\tThe name must contain from 1 to 63 letters, numbers, or hyphens, \n\t\t\t\t\t\t\t\tand its first character must be a letter. It cannot end with a hyphen or contain two consecutive hyphens.\n\t\t\t\t\t\t\t\tIf you don't specify a graph name, a unique graph name is generated for you using the prefix graph-for, \n\t\t\t\t\t\t\t\tfollowed by a combination of Stack Name and a UUID."]
    pub fn graph_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graph_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `graph_name_prefix` after provisioning.\nAllows user to specify name prefix and have remainder of name automatically generated."]
    pub fn graph_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graph_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_identifier` after provisioning.\nSpecifies a KMS key to use to encrypt data in the new graph.  Value must be ARN of KMS Key."]
    pub fn kms_key_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_memory` after provisioning.\nThe provisioned memory-optimized Neptune Capacity Units (m-NCUs) to use for the graph."]
    pub fn provisioned_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_memory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_connectivity` after provisioning.\nSpecifies whether or not the graph can be reachable over the internet. \n\t\t\t\t\t\t\t\tAll access to graphs is IAM authenticated.\n\t\t\t\t\t\t\t\tWhen the graph is publicly available, its domain name system (DNS) endpoint resolves to \n\t\t\t\t\t\t\t\tthe public IP address from the internet. When the graph isn't publicly available, you need \n\t\t\t\t\t\t\t\tto create a PrivateGraphEndpoint in a given VPC to ensure the DNS name resolves to a private \n\t\t\t\t\t\t\t\tIP address that is reachable from the VPC."]
    pub fn public_connectivity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_connectivity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nThe number of replicas in other AZs.  Value must be between 0 and 2."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
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
    pub fn timeouts(&self) -> NeptunegraphGraph_TimeoutsRef {
        NeptunegraphGraph_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vector_search_configuration` after provisioning.\n"]
    pub fn vector_search_configuration(&self) -> terrars::ListRef<NeptunegraphGraph_VectorSearchConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vector_search_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for NeptunegraphGraph {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NeptunegraphGraph { }

impl terrars::ToListMappable for NeptunegraphGraph {
    type O = terrars::ListRef<NeptunegraphGraphRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NeptunegraphGraph_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_neptunegraph_graph".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNeptunegraphGraph {
    pub tf_id: std::string::String,
    #[doc = "The provisioned memory-optimized Neptune Capacity Units (m-NCUs) to use for the graph."]
    pub provisioned_memory: terrars::PrimField<f64>,
}

impl BuildNeptunegraphGraph {
    pub fn build(self, stack: &mut terrars::Stack) -> NeptunegraphGraph {
        let out = NeptunegraphGraph(std::rc::Rc::new(NeptunegraphGraph_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NeptunegraphGraphData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                graph_name: core::default::Default::default(),
                graph_name_prefix: core::default::Default::default(),
                kms_key_identifier: core::default::Default::default(),
                provisioned_memory: self.provisioned_memory,
                public_connectivity: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_count: core::default::Default::default(),
                tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vector_search_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NeptunegraphGraphRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NeptunegraphGraphRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NeptunegraphGraphRef {
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

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nA value that indicates whether the graph has deletion protection enabled. The graph can't be deleted when deletion protection is enabled."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `graph_name` after provisioning.\nThe graph name. For example: my-graph-1.\n\t\t\t\t\t\t\t\tThe name must contain from 1 to 63 letters, numbers, or hyphens, \n\t\t\t\t\t\t\t\tand its first character must be a letter. It cannot end with a hyphen or contain two consecutive hyphens.\n\t\t\t\t\t\t\t\tIf you don't specify a graph name, a unique graph name is generated for you using the prefix graph-for, \n\t\t\t\t\t\t\t\tfollowed by a combination of Stack Name and a UUID."]
    pub fn graph_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graph_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `graph_name_prefix` after provisioning.\nAllows user to specify name prefix and have remainder of name automatically generated."]
    pub fn graph_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graph_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_identifier` after provisioning.\nSpecifies a KMS key to use to encrypt data in the new graph.  Value must be ARN of KMS Key."]
    pub fn kms_key_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_memory` after provisioning.\nThe provisioned memory-optimized Neptune Capacity Units (m-NCUs) to use for the graph."]
    pub fn provisioned_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_memory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_connectivity` after provisioning.\nSpecifies whether or not the graph can be reachable over the internet. \n\t\t\t\t\t\t\t\tAll access to graphs is IAM authenticated.\n\t\t\t\t\t\t\t\tWhen the graph is publicly available, its domain name system (DNS) endpoint resolves to \n\t\t\t\t\t\t\t\tthe public IP address from the internet. When the graph isn't publicly available, you need \n\t\t\t\t\t\t\t\tto create a PrivateGraphEndpoint in a given VPC to ensure the DNS name resolves to a private \n\t\t\t\t\t\t\t\tIP address that is reachable from the VPC."]
    pub fn public_connectivity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_connectivity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nThe number of replicas in other AZs.  Value must be between 0 and 2."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.extract_ref()))
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
    pub fn timeouts(&self) -> NeptunegraphGraph_TimeoutsRef {
        NeptunegraphGraph_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vector_search_configuration` after provisioning.\n"]
    pub fn vector_search_configuration(&self) -> terrars::ListRef<NeptunegraphGraph_VectorSearchConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vector_search_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NeptunegraphGraphDynamic {
    vector_search_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<NeptunegraphGraph_VectorSearchConfiguration>>,
}

#[derive(Serialize)]
pub struct NeptunegraphGraph_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NeptunegraphGraph_Timeouts {
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

impl terrars::ToListMappable for NeptunegraphGraph_Timeouts {
    type O = terrars::BlockAssignable<NeptunegraphGraph_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNeptunegraphGraph_Timeouts {}

impl BuildNeptunegraphGraph_Timeouts {
    pub fn build(self) -> NeptunegraphGraph_Timeouts {
        NeptunegraphGraph_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NeptunegraphGraph_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NeptunegraphGraph_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NeptunegraphGraph_TimeoutsRef {
        NeptunegraphGraph_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NeptunegraphGraph_TimeoutsRef {
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
pub struct NeptunegraphGraph_VectorSearchConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_search_dimension: std::option::Option<terrars::PrimField<f64>>,
}

impl NeptunegraphGraph_VectorSearchConfiguration {
    #[doc =
        "Set the field `vector_search_dimension`.\nSpecifies the number of dimensions for vector embeddings.  Value must be between 1 and 65,535."]
    pub fn set_vector_search_dimension(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vector_search_dimension = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NeptunegraphGraph_VectorSearchConfiguration {
    type O = terrars::BlockAssignable<NeptunegraphGraph_VectorSearchConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNeptunegraphGraph_VectorSearchConfiguration {}

impl BuildNeptunegraphGraph_VectorSearchConfiguration {
    pub fn build(self) -> NeptunegraphGraph_VectorSearchConfiguration {
        NeptunegraphGraph_VectorSearchConfiguration { vector_search_dimension: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NeptunegraphGraph_VectorSearchConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NeptunegraphGraph_VectorSearchConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NeptunegraphGraph_VectorSearchConfigurationRef {
        NeptunegraphGraph_VectorSearchConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NeptunegraphGraph_VectorSearchConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `vector_search_dimension` after provisioning.\nSpecifies the number of dimensions for vector embeddings.  Value must be between 1 and 65,535."]
    pub fn vector_search_dimension(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vector_search_dimension", self.base))
    }
}

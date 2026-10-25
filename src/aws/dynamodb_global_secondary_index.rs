#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DynamodbGlobalSecondaryIndexData_ {
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
    index_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    table_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warm_throughput: std::option::Option<DynamodbGlobalSecondaryIndex_WarmThroughput>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_schema: std::option::Option<std::vec::Vec<DynamodbGlobalSecondaryIndex_KeySchema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_throughput: std::option::Option<std::vec::Vec<DynamodbGlobalSecondaryIndex_OnDemandThroughput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    projection: std::option::Option<std::vec::Vec<DynamodbGlobalSecondaryIndex_Projection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std::option::Option<std::vec::Vec<DynamodbGlobalSecondaryIndex_ProvisionedThroughput>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DynamodbGlobalSecondaryIndex_Timeouts>,
    dynamic: DynamodbGlobalSecondaryIndexDynamic,
}

struct DynamodbGlobalSecondaryIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DynamodbGlobalSecondaryIndexData_>,
}

#[derive(Clone)]
pub struct DynamodbGlobalSecondaryIndex(std::rc::Rc<DynamodbGlobalSecondaryIndex_>);

impl DynamodbGlobalSecondaryIndex {
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

    #[doc = "Set the field `warm_throughput`.\n"]
    pub fn set_warm_throughput(self, v: impl std::convert::Into<DynamodbGlobalSecondaryIndex_WarmThroughput>) -> Self {
        self.0.data.borrow_mut().warm_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_schema`.\n"]
    pub fn set_key_schema(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_KeySchema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().key_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.key_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_demand_throughput`.\n"]
    pub fn set_on_demand_throughput(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_OnDemandThroughput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_demand_throughput = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_demand_throughput = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `projection`.\n"]
    pub fn set_projection(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_Projection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().projection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.projection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `provisioned_throughput`.\n"]
    pub fn set_provisioned_throughput(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_ProvisionedThroughput>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().provisioned_throughput = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.provisioned_throughput = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DynamodbGlobalSecondaryIndex_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> DynamodbGlobalSecondaryIndex_WarmThroughputRef {
        DynamodbGlobalSecondaryIndex_WarmThroughputRef::new(
            self.shared().clone(),
            format!("{}.warm_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_schema` after provisioning.\n"]
    pub fn key_schema(&self) -> terrars::ListRef<DynamodbGlobalSecondaryIndex_KeySchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> terrars::ListRef<DynamodbGlobalSecondaryIndex_OnDemandThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `projection` after provisioning.\n"]
    pub fn projection(&self) -> terrars::ListRef<DynamodbGlobalSecondaryIndex_ProjectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.projection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(&self) -> terrars::ListRef<DynamodbGlobalSecondaryIndex_ProvisionedThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbGlobalSecondaryIndex_TimeoutsRef {
        DynamodbGlobalSecondaryIndex_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DynamodbGlobalSecondaryIndex {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DynamodbGlobalSecondaryIndex { }

impl terrars::ToListMappable for DynamodbGlobalSecondaryIndex {
    type O = terrars::ListRef<DynamodbGlobalSecondaryIndexRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DynamodbGlobalSecondaryIndex_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dynamodb_global_secondary_index".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDynamodbGlobalSecondaryIndex {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub index_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_name: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbGlobalSecondaryIndex {
    pub fn build(self, stack: &mut terrars::Stack) -> DynamodbGlobalSecondaryIndex {
        let out = DynamodbGlobalSecondaryIndex(std::rc::Rc::new(DynamodbGlobalSecondaryIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DynamodbGlobalSecondaryIndexData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                index_name: self.index_name,
                region: core::default::Default::default(),
                table_name: self.table_name,
                warm_throughput: core::default::Default::default(),
                key_schema: core::default::Default::default(),
                on_demand_throughput: core::default::Default::default(),
                projection: core::default::Default::default(),
                provisioned_throughput: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DynamodbGlobalSecondaryIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbGlobalSecondaryIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DynamodbGlobalSecondaryIndexRef {
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

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warm_throughput` after provisioning.\n"]
    pub fn warm_throughput(&self) -> DynamodbGlobalSecondaryIndex_WarmThroughputRef {
        DynamodbGlobalSecondaryIndex_WarmThroughputRef::new(
            self.shared().clone(),
            format!("{}.warm_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `key_schema` after provisioning.\n"]
    pub fn key_schema(&self) -> terrars::ListRef<DynamodbGlobalSecondaryIndex_KeySchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_demand_throughput` after provisioning.\n"]
    pub fn on_demand_throughput(&self) -> terrars::ListRef<DynamodbGlobalSecondaryIndex_OnDemandThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `projection` after provisioning.\n"]
    pub fn projection(&self) -> terrars::ListRef<DynamodbGlobalSecondaryIndex_ProjectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.projection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_throughput` after provisioning.\n"]
    pub fn provisioned_throughput(&self) -> terrars::ListRef<DynamodbGlobalSecondaryIndex_ProvisionedThroughputRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provisioned_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DynamodbGlobalSecondaryIndex_TimeoutsRef {
        DynamodbGlobalSecondaryIndex_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DynamodbGlobalSecondaryIndexDynamic {
    key_schema: std::option::Option<terrars::DynamicBlock<DynamodbGlobalSecondaryIndex_KeySchema>>,
    on_demand_throughput: std
    ::option
    ::Option<terrars::DynamicBlock<DynamodbGlobalSecondaryIndex_OnDemandThroughput>>,
    projection: std::option::Option<terrars::DynamicBlock<DynamodbGlobalSecondaryIndex_Projection>>,
    provisioned_throughput: std
    ::option
    ::Option<terrars::DynamicBlock<DynamodbGlobalSecondaryIndex_ProvisionedThroughput>>,
}

#[derive(Serialize)]
pub struct DynamodbGlobalSecondaryIndex_WarmThroughput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_units_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_units_per_second: std::option::Option<terrars::PrimField<f64>>,
}

impl DynamodbGlobalSecondaryIndex_WarmThroughput {
    #[doc = "Set the field `read_units_per_second`.\n"]
    pub fn set_read_units_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.read_units_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_units_per_second`.\n"]
    pub fn set_write_units_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.write_units_per_second = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbGlobalSecondaryIndex_WarmThroughput {
    type O = terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_WarmThroughput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbGlobalSecondaryIndex_WarmThroughput {}

impl BuildDynamodbGlobalSecondaryIndex_WarmThroughput {
    pub fn build(self) -> DynamodbGlobalSecondaryIndex_WarmThroughput {
        DynamodbGlobalSecondaryIndex_WarmThroughput {
            read_units_per_second: core::default::Default::default(),
            write_units_per_second: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbGlobalSecondaryIndex_WarmThroughputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbGlobalSecondaryIndex_WarmThroughputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DynamodbGlobalSecondaryIndex_WarmThroughputRef {
        DynamodbGlobalSecondaryIndex_WarmThroughputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbGlobalSecondaryIndex_WarmThroughputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read_units_per_second` after provisioning.\n"]
    pub fn read_units_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_units_per_second", self.base))
    }

    #[doc = "Get a reference to the value of field `write_units_per_second` after provisioning.\n"]
    pub fn write_units_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_units_per_second", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbGlobalSecondaryIndex_KeySchema {
    attribute_name: terrars::PrimField<std::string::String>,
    attribute_type: terrars::PrimField<std::string::String>,
    key_type: terrars::PrimField<std::string::String>,
}

impl DynamodbGlobalSecondaryIndex_KeySchema { }

impl terrars::ToListMappable for DynamodbGlobalSecondaryIndex_KeySchema {
    type O = terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_KeySchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbGlobalSecondaryIndex_KeySchema {
    #[doc = ""]
    pub attribute_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub attribute_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key_type: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbGlobalSecondaryIndex_KeySchema {
    pub fn build(self) -> DynamodbGlobalSecondaryIndex_KeySchema {
        DynamodbGlobalSecondaryIndex_KeySchema {
            attribute_name: self.attribute_name,
            attribute_type: self.attribute_type,
            key_type: self.key_type,
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbGlobalSecondaryIndex_KeySchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbGlobalSecondaryIndex_KeySchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbGlobalSecondaryIndex_KeySchemaRef {
        DynamodbGlobalSecondaryIndex_KeySchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbGlobalSecondaryIndex_KeySchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_name` after provisioning.\n"]
    pub fn attribute_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_name", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_type` after provisioning.\n"]
    pub fn attribute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key_type` after provisioning.\n"]
    pub fn key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbGlobalSecondaryIndex_OnDemandThroughput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_read_request_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_write_request_units: std::option::Option<terrars::PrimField<f64>>,
}

impl DynamodbGlobalSecondaryIndex_OnDemandThroughput {
    #[doc = "Set the field `max_read_request_units`.\n"]
    pub fn set_max_read_request_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_read_request_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_write_request_units`.\n"]
    pub fn set_max_write_request_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_write_request_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbGlobalSecondaryIndex_OnDemandThroughput {
    type O = terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_OnDemandThroughput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbGlobalSecondaryIndex_OnDemandThroughput {}

impl BuildDynamodbGlobalSecondaryIndex_OnDemandThroughput {
    pub fn build(self) -> DynamodbGlobalSecondaryIndex_OnDemandThroughput {
        DynamodbGlobalSecondaryIndex_OnDemandThroughput {
            max_read_request_units: core::default::Default::default(),
            max_write_request_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbGlobalSecondaryIndex_OnDemandThroughputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbGlobalSecondaryIndex_OnDemandThroughputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DynamodbGlobalSecondaryIndex_OnDemandThroughputRef {
        DynamodbGlobalSecondaryIndex_OnDemandThroughputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbGlobalSecondaryIndex_OnDemandThroughputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_read_request_units` after provisioning.\n"]
    pub fn max_read_request_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_read_request_units", self.base))
    }

    #[doc = "Get a reference to the value of field `max_write_request_units` after provisioning.\n"]
    pub fn max_write_request_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_write_request_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbGlobalSecondaryIndex_Projection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_key_attributes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    projection_type: terrars::PrimField<std::string::String>,
}

impl DynamodbGlobalSecondaryIndex_Projection {
    #[doc = "Set the field `non_key_attributes`.\n"]
    pub fn set_non_key_attributes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.non_key_attributes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbGlobalSecondaryIndex_Projection {
    type O = terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_Projection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbGlobalSecondaryIndex_Projection {
    #[doc = ""]
    pub projection_type: terrars::PrimField<std::string::String>,
}

impl BuildDynamodbGlobalSecondaryIndex_Projection {
    pub fn build(self) -> DynamodbGlobalSecondaryIndex_Projection {
        DynamodbGlobalSecondaryIndex_Projection {
            non_key_attributes: core::default::Default::default(),
            projection_type: self.projection_type,
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbGlobalSecondaryIndex_ProjectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbGlobalSecondaryIndex_ProjectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbGlobalSecondaryIndex_ProjectionRef {
        DynamodbGlobalSecondaryIndex_ProjectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbGlobalSecondaryIndex_ProjectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `non_key_attributes` after provisioning.\n"]
    pub fn non_key_attributes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.non_key_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `projection_type` after provisioning.\n"]
    pub fn projection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.projection_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbGlobalSecondaryIndex_ProvisionedThroughput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_capacity_units: std::option::Option<terrars::PrimField<f64>>,
}

impl DynamodbGlobalSecondaryIndex_ProvisionedThroughput {
    #[doc = "Set the field `read_capacity_units`.\n"]
    pub fn set_read_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.read_capacity_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_capacity_units`.\n"]
    pub fn set_write_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.write_capacity_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DynamodbGlobalSecondaryIndex_ProvisionedThroughput {
    type O = terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_ProvisionedThroughput>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbGlobalSecondaryIndex_ProvisionedThroughput {}

impl BuildDynamodbGlobalSecondaryIndex_ProvisionedThroughput {
    pub fn build(self) -> DynamodbGlobalSecondaryIndex_ProvisionedThroughput {
        DynamodbGlobalSecondaryIndex_ProvisionedThroughput {
            read_capacity_units: core::default::Default::default(),
            write_capacity_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbGlobalSecondaryIndex_ProvisionedThroughputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbGlobalSecondaryIndex_ProvisionedThroughputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DynamodbGlobalSecondaryIndex_ProvisionedThroughputRef {
        DynamodbGlobalSecondaryIndex_ProvisionedThroughputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbGlobalSecondaryIndex_ProvisionedThroughputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read_capacity_units` after provisioning.\n"]
    pub fn read_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `write_capacity_units` after provisioning.\n"]
    pub fn write_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_capacity_units", self.base))
    }
}

#[derive(Serialize)]
pub struct DynamodbGlobalSecondaryIndex_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DynamodbGlobalSecondaryIndex_Timeouts {
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

impl terrars::ToListMappable for DynamodbGlobalSecondaryIndex_Timeouts {
    type O = terrars::BlockAssignable<DynamodbGlobalSecondaryIndex_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDynamodbGlobalSecondaryIndex_Timeouts {}

impl BuildDynamodbGlobalSecondaryIndex_Timeouts {
    pub fn build(self) -> DynamodbGlobalSecondaryIndex_Timeouts {
        DynamodbGlobalSecondaryIndex_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DynamodbGlobalSecondaryIndex_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DynamodbGlobalSecondaryIndex_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DynamodbGlobalSecondaryIndex_TimeoutsRef {
        DynamodbGlobalSecondaryIndex_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DynamodbGlobalSecondaryIndex_TimeoutsRef {
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

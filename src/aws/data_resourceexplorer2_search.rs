#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataResourceexplorer2SearchData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    query_string: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataResourceexplorer2Search_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataResourceexplorer2SearchData_>,
}

#[derive(Clone)]
pub struct DataResourceexplorer2Search(std::rc::Rc<DataResourceexplorer2Search_>);

impl DataResourceexplorer2Search {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_arn`.\n"]
    pub fn set_view_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().view_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_count` after provisioning.\n"]
    pub fn resource_count(&self) -> terrars::ListRef<DataResourceexplorer2Search_ResourceCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataResourceexplorer2Search_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_arn` after provisioning.\n"]
    pub fn view_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_arn", self.extract_ref()))
    }
}

impl terrars::Referable for DataResourceexplorer2Search {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataResourceexplorer2Search { }

impl terrars::ToListMappable for DataResourceexplorer2Search {
    type O = terrars::ListRef<DataResourceexplorer2SearchRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataResourceexplorer2Search_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_resourceexplorer2_search".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataResourceexplorer2Search {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub query_string: terrars::PrimField<std::string::String>,
}

impl BuildDataResourceexplorer2Search {
    pub fn build(self, stack: &mut terrars::Stack) -> DataResourceexplorer2Search {
        let out = DataResourceexplorer2Search(std::rc::Rc::new(DataResourceexplorer2Search_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataResourceexplorer2SearchData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                query_string: self.query_string,
                region: core::default::Default::default(),
                view_arn: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataResourceexplorer2SearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourceexplorer2SearchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataResourceexplorer2SearchRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_count` after provisioning.\n"]
    pub fn resource_count(&self) -> terrars::ListRef<DataResourceexplorer2Search_ResourceCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<DataResourceexplorer2Search_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_arn` after provisioning.\n"]
    pub fn view_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_arn", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataResourceexplorer2Search_ResourceCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    complete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_resources: std::option::Option<terrars::PrimField<f64>>,
}

impl DataResourceexplorer2Search_ResourceCount {
    #[doc = "Set the field `complete`.\n"]
    pub fn set_complete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.complete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_resources`.\n"]
    pub fn set_total_resources(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_resources = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResourceexplorer2Search_ResourceCount {
    type O = terrars::BlockAssignable<DataResourceexplorer2Search_ResourceCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourceexplorer2Search_ResourceCount {}

impl BuildDataResourceexplorer2Search_ResourceCount {
    pub fn build(self) -> DataResourceexplorer2Search_ResourceCount {
        DataResourceexplorer2Search_ResourceCount {
            complete: core::default::Default::default(),
            total_resources: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataResourceexplorer2Search_ResourceCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourceexplorer2Search_ResourceCountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResourceexplorer2Search_ResourceCountRef {
        DataResourceexplorer2Search_ResourceCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourceexplorer2Search_ResourceCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `complete` after provisioning.\n"]
    pub fn complete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.complete", self.base))
    }

    #[doc = "Get a reference to the value of field `total_resources` after provisioning.\n"]
    pub fn total_resources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_resources", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResourceexplorer2Search_Resources_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_reported_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataResourceexplorer2Search_Resources_Properties {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_reported_at`.\n"]
    pub fn set_last_reported_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_reported_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResourceexplorer2Search_Resources_Properties {
    type O = terrars::BlockAssignable<DataResourceexplorer2Search_Resources_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourceexplorer2Search_Resources_Properties {}

impl BuildDataResourceexplorer2Search_Resources_Properties {
    pub fn build(self) -> DataResourceexplorer2Search_Resources_Properties {
        DataResourceexplorer2Search_Resources_Properties {
            data: core::default::Default::default(),
            last_reported_at: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataResourceexplorer2Search_Resources_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourceexplorer2Search_Resources_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataResourceexplorer2Search_Resources_PropertiesRef {
        DataResourceexplorer2Search_Resources_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourceexplorer2Search_Resources_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `last_reported_at` after provisioning.\n"]
    pub fn last_reported_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_reported_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResourceexplorer2Search_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_reported_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owning_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::ListField<DataResourceexplorer2Search_Resources_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataResourceexplorer2Search_Resources {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_reported_at`.\n"]
    pub fn set_last_reported_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_reported_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owning_account_id`.\n"]
    pub fn set_owning_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owning_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataResourceexplorer2Search_Resources_Properties>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResourceexplorer2Search_Resources {
    type O = terrars::BlockAssignable<DataResourceexplorer2Search_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourceexplorer2Search_Resources {}

impl BuildDataResourceexplorer2Search_Resources {
    pub fn build(self) -> DataResourceexplorer2Search_Resources {
        DataResourceexplorer2Search_Resources {
            arn: core::default::Default::default(),
            last_reported_at: core::default::Default::default(),
            owning_account_id: core::default::Default::default(),
            properties: core::default::Default::default(),
            region: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataResourceexplorer2Search_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourceexplorer2Search_ResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResourceexplorer2Search_ResourcesRef {
        DataResourceexplorer2Search_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourceexplorer2Search_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `last_reported_at` after provisioning.\n"]
    pub fn last_reported_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_reported_at", self.base))
    }

    #[doc = "Get a reference to the value of field `owning_account_id` after provisioning.\n"]
    pub fn owning_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owning_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<DataResourceexplorer2Search_Resources_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

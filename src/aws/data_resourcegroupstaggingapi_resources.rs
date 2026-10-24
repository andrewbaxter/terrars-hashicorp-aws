use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataResourcegroupstaggingapiResourcesData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_compliant_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_compliance_details: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type_filters: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_filter: std::option::Option<std::vec::Vec<DataResourcegroupstaggingapiResourcesTagFilterEl>>,
    dynamic: DataResourcegroupstaggingapiResourcesDynamic,
}

struct DataResourcegroupstaggingapiResources_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataResourcegroupstaggingapiResourcesData>,
}

#[derive(Clone)]
pub struct DataResourcegroupstaggingapiResources(std::rc::Rc<DataResourcegroupstaggingapiResources_>);

impl DataResourcegroupstaggingapiResources {
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

    #[doc = "Set the field `exclude_compliant_resources`.\n"]
    pub fn set_exclude_compliant_resources(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().exclude_compliant_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_compliance_details`.\n"]
    pub fn set_include_compliance_details(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_compliance_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn_list`.\n"]
    pub fn set_resource_arn_list(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_arn_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type_filters`.\n"]
    pub fn set_resource_type_filters(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_type_filters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_filter`.\n"]
    pub fn set_tag_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataResourcegroupstaggingapiResourcesTagFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tag_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tag_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `exclude_compliant_resources` after provisioning.\n"]
    pub fn exclude_compliant_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_compliant_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_compliance_details` after provisioning.\n"]
    pub fn include_compliance_details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_compliance_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn_list` after provisioning.\n"]
    pub fn resource_arn_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_arn_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tag_mapping_list` after provisioning.\n"]
    pub fn resource_tag_mapping_list(
        &self,
    ) -> terrars::ListRef<DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_tag_mapping_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type_filters` after provisioning.\n"]
    pub fn resource_type_filters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_type_filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_filter` after provisioning.\n"]
    pub fn tag_filter(&self) -> terrars::ListRef<DataResourcegroupstaggingapiResourcesTagFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_filter", self.extract_ref()))
    }
}

impl terrars::Referable for DataResourcegroupstaggingapiResources {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataResourcegroupstaggingapiResources { }

impl terrars::ToListMappable for DataResourcegroupstaggingapiResources {
    type O = terrars::ListRef<DataResourcegroupstaggingapiResourcesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataResourcegroupstaggingapiResources_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_resourcegroupstaggingapi_resources".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataResourcegroupstaggingapiResources {
    pub tf_id: std::string::String,
}

impl BuildDataResourcegroupstaggingapiResources {
    pub fn build(self, stack: &mut terrars::Stack) -> DataResourcegroupstaggingapiResources {
        let out = DataResourcegroupstaggingapiResources(std::rc::Rc::new(DataResourcegroupstaggingapiResources_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataResourcegroupstaggingapiResourcesData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                exclude_compliant_resources: core::default::Default::default(),
                id: core::default::Default::default(),
                include_compliance_details: core::default::Default::default(),
                resource_arn_list: core::default::Default::default(),
                resource_type_filters: core::default::Default::default(),
                tag_filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataResourcegroupstaggingapiResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourcegroupstaggingapiResourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataResourcegroupstaggingapiResourcesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `exclude_compliant_resources` after provisioning.\n"]
    pub fn exclude_compliant_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_compliant_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_compliance_details` after provisioning.\n"]
    pub fn include_compliance_details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_compliance_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn_list` after provisioning.\n"]
    pub fn resource_arn_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_arn_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_tag_mapping_list` after provisioning.\n"]
    pub fn resource_tag_mapping_list(
        &self,
    ) -> terrars::ListRef<DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_tag_mapping_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type_filters` after provisioning.\n"]
    pub fn resource_type_filters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_type_filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_filter` after provisioning.\n"]
    pub fn tag_filter(&self) -> terrars::ListRef<DataResourcegroupstaggingapiResourcesTagFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_filter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_status: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keys_with_noncompliant_values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_compliant_keys: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
    #[doc = "Set the field `compliance_status`.\n"]
    pub fn set_compliance_status(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.compliance_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keys_with_noncompliant_values`.\n"]
    pub fn set_keys_with_noncompliant_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.keys_with_noncompliant_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `non_compliant_keys`.\n"]
    pub fn set_non_compliant_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.non_compliant_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
    type O =
        terrars::BlockAssignable<DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {}

impl BuildDataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
    pub fn build(self) -> DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
        DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl {
            compliance_status: core::default::Default::default(),
            keys_with_noncompliant_values: core::default::Default::default(),
            non_compliant_keys: core::default::Default::default(),
        }
    }
}

pub struct DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
        DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compliance_status` after provisioning.\n"]
    pub fn compliance_status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_status", self.base))
    }

    #[doc = "Get a reference to the value of field `keys_with_noncompliant_values` after provisioning.\n"]
    pub fn keys_with_noncompliant_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.keys_with_noncompliant_values", self.base))
    }

    #[doc = "Get a reference to the value of field `non_compliant_keys` after provisioning.\n"]
    pub fn non_compliant_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.non_compliant_keys", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_details: std
    ::option
    ::Option<terrars::ListField<DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
    #[doc = "Set the field `compliance_details`.\n"]
    pub fn set_compliance_details(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsEl>,
                    >,
    ) -> Self {
        self.compliance_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
    type O = terrars::BlockAssignable<DataResourcegroupstaggingapiResourcesResourceTagMappingListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourcegroupstaggingapiResourcesResourceTagMappingListEl {}

impl BuildDataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
    pub fn build(self) -> DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
        DataResourcegroupstaggingapiResourcesResourceTagMappingListEl {
            compliance_details: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
        DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourcegroupstaggingapiResourcesResourceTagMappingListElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compliance_details` after provisioning.\n"]
    pub fn compliance_details(
        &self,
    ) -> terrars::ListRef<DataResourcegroupstaggingapiResourcesResourceTagMappingListElComplianceDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compliance_details", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResourcegroupstaggingapiResourcesTagFilterEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataResourcegroupstaggingapiResourcesTagFilterEl {
    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResourcegroupstaggingapiResourcesTagFilterEl {
    type O = terrars::BlockAssignable<DataResourcegroupstaggingapiResourcesTagFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourcegroupstaggingapiResourcesTagFilterEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildDataResourcegroupstaggingapiResourcesTagFilterEl {
    pub fn build(self) -> DataResourcegroupstaggingapiResourcesTagFilterEl {
        DataResourcegroupstaggingapiResourcesTagFilterEl {
            key: self.key,
            values: core::default::Default::default(),
        }
    }
}

pub struct DataResourcegroupstaggingapiResourcesTagFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResourcegroupstaggingapiResourcesTagFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataResourcegroupstaggingapiResourcesTagFilterElRef {
        DataResourcegroupstaggingapiResourcesTagFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourcegroupstaggingapiResourcesTagFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataResourcegroupstaggingapiResourcesDynamic {
    tag_filter: std::option::Option<terrars::DynamicBlock<DataResourcegroupstaggingapiResourcesTagFilterEl>>,
}

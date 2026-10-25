#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIdentitystoreGroupsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    identity_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataIdentitystoreGroups_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIdentitystoreGroupsData_>,
}

#[derive(Clone)]
pub struct DataIdentitystoreGroups(std::rc::Rc<DataIdentitystoreGroups_>);

impl DataIdentitystoreGroups {
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

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> terrars::ListRef<DataIdentitystoreGroups_GroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataIdentitystoreGroups {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIdentitystoreGroups { }

impl terrars::ToListMappable for DataIdentitystoreGroups {
    type O = terrars::ListRef<DataIdentitystoreGroupsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIdentitystoreGroups_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_identitystore_groups".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIdentitystoreGroups {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub identity_store_id: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreGroups {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIdentitystoreGroups {
        let out = DataIdentitystoreGroups(std::rc::Rc::new(DataIdentitystoreGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIdentitystoreGroupsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                identity_store_id: self.identity_store_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIdentitystoreGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIdentitystoreGroupsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> terrars::ListRef<DataIdentitystoreGroups_GroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroups_Groups_ExternalIds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreGroups_Groups_ExternalIds {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIdentitystoreGroups_Groups_ExternalIds {
    type O = terrars::BlockAssignable<DataIdentitystoreGroups_Groups_ExternalIds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroups_Groups_ExternalIds {}

impl BuildDataIdentitystoreGroups_Groups_ExternalIds {
    pub fn build(self) -> DataIdentitystoreGroups_Groups_ExternalIds {
        DataIdentitystoreGroups_Groups_ExternalIds {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreGroups_Groups_ExternalIdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroups_Groups_ExternalIdsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreGroups_Groups_ExternalIdsRef {
        DataIdentitystoreGroups_Groups_ExternalIdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroups_Groups_ExternalIdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroups_Groups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ids: std::option::Option<terrars::ListField<DataIdentitystoreGroups_Groups_ExternalIds>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_store_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreGroups_Groups {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_ids`.\n"]
    pub fn set_external_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataIdentitystoreGroups_Groups_ExternalIds>>,
    ) -> Self {
        self.external_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_store_id`.\n"]
    pub fn set_identity_store_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_store_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIdentitystoreGroups_Groups {
    type O = terrars::BlockAssignable<DataIdentitystoreGroups_Groups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroups_Groups {}

impl BuildDataIdentitystoreGroups_Groups {
    pub fn build(self) -> DataIdentitystoreGroups_Groups {
        DataIdentitystoreGroups_Groups {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            external_ids: core::default::Default::default(),
            group_id: core::default::Default::default(),
            identity_store_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreGroups_GroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroups_GroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreGroups_GroupsRef {
        DataIdentitystoreGroups_GroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroups_GroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> terrars::ListRef<DataIdentitystoreGroups_Groups_ExternalIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.base))
    }
}

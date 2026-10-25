#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIdentitystoreGroupData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    identity_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternate_identifier: std::option::Option<std::vec::Vec<DataIdentitystoreGroup_AlternateIdentifier>>,
    dynamic: DataIdentitystoreGroupDynamic,
}

struct DataIdentitystoreGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIdentitystoreGroupData_>,
}

#[derive(Clone)]
pub struct DataIdentitystoreGroup(std::rc::Rc<DataIdentitystoreGroup_>);

impl DataIdentitystoreGroup {
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

    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().group_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `alternate_identifier`.\n"]
    pub fn set_alternate_identifier(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIdentitystoreGroup_AlternateIdentifier>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alternate_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alternate_identifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> terrars::ListRef<DataIdentitystoreGroup_ExternalIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `alternate_identifier` after provisioning.\n"]
    pub fn alternate_identifier(&self) -> terrars::ListRef<DataIdentitystoreGroup_AlternateIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_identifier", self.extract_ref()))
    }
}

impl terrars::Referable for DataIdentitystoreGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIdentitystoreGroup { }

impl terrars::ToListMappable for DataIdentitystoreGroup {
    type O = terrars::ListRef<DataIdentitystoreGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIdentitystoreGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_identitystore_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIdentitystoreGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub identity_store_id: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIdentitystoreGroup {
        let out = DataIdentitystoreGroup(std::rc::Rc::new(DataIdentitystoreGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIdentitystoreGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                identity_store_id: self.identity_store_id,
                region: core::default::Default::default(),
                alternate_identifier: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIdentitystoreGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIdentitystoreGroupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> terrars::ListRef<DataIdentitystoreGroup_ExternalIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `alternate_identifier` after provisioning.\n"]
    pub fn alternate_identifier(&self) -> terrars::ListRef<DataIdentitystoreGroup_AlternateIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_identifier", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataIdentitystoreGroupDynamic {
    alternate_identifier: std::option::Option<terrars::DynamicBlock<DataIdentitystoreGroup_AlternateIdentifier>>,
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroup_ExternalIds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreGroup_ExternalIds {
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

impl terrars::ToListMappable for DataIdentitystoreGroup_ExternalIds {
    type O = terrars::BlockAssignable<DataIdentitystoreGroup_ExternalIds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroup_ExternalIds {}

impl BuildDataIdentitystoreGroup_ExternalIds {
    pub fn build(self) -> DataIdentitystoreGroup_ExternalIds {
        DataIdentitystoreGroup_ExternalIds {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreGroup_ExternalIdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroup_ExternalIdsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreGroup_ExternalIdsRef {
        DataIdentitystoreGroup_ExternalIdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroup_ExternalIdsRef {
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
pub struct DataIdentitystoreGroup_AlternateIdentifier_ExternalId {
    id: terrars::PrimField<std::string::String>,
    issuer: terrars::PrimField<std::string::String>,
}

impl DataIdentitystoreGroup_AlternateIdentifier_ExternalId { }

impl terrars::ToListMappable for DataIdentitystoreGroup_AlternateIdentifier_ExternalId {
    type O = terrars::BlockAssignable<DataIdentitystoreGroup_AlternateIdentifier_ExternalId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroup_AlternateIdentifier_ExternalId {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub issuer: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreGroup_AlternateIdentifier_ExternalId {
    pub fn build(self) -> DataIdentitystoreGroup_AlternateIdentifier_ExternalId {
        DataIdentitystoreGroup_AlternateIdentifier_ExternalId {
            id: self.id,
            issuer: self.issuer,
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreGroup_AlternateIdentifier_ExternalIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroup_AlternateIdentifier_ExternalIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIdentitystoreGroup_AlternateIdentifier_ExternalIdRef {
        DataIdentitystoreGroup_AlternateIdentifier_ExternalIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroup_AlternateIdentifier_ExternalIdRef {
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
pub struct DataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute {
    attribute_path: terrars::PrimField<std::string::String>,
    attribute_value: terrars::PrimField<std::string::String>,
}

impl DataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute { }

impl terrars::ToListMappable for DataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute {
    type O = terrars::BlockAssignable<DataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute {
    #[doc = ""]
    pub attribute_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub attribute_value: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute {
    pub fn build(self) -> DataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute {
        DataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute {
            attribute_path: self.attribute_path,
            attribute_value: self.attribute_value,
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreGroup_AlternateIdentifier_UniqueAttributeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroup_AlternateIdentifier_UniqueAttributeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIdentitystoreGroup_AlternateIdentifier_UniqueAttributeRef {
        DataIdentitystoreGroup_AlternateIdentifier_UniqueAttributeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroup_AlternateIdentifier_UniqueAttributeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_path` after provisioning.\n"]
    pub fn attribute_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_path", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIdentitystoreGroup_AlternateIdentifierDynamic {
    external_id: std::option::Option<terrars::DynamicBlock<DataIdentitystoreGroup_AlternateIdentifier_ExternalId>>,
    unique_attribute: std
    ::option
    ::Option<terrars::DynamicBlock<DataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute>>,
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroup_AlternateIdentifier {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<std::vec::Vec<DataIdentitystoreGroup_AlternateIdentifier_ExternalId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_attribute: std::option::Option<std::vec::Vec<DataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute>>,
    dynamic: DataIdentitystoreGroup_AlternateIdentifierDynamic,
}

impl DataIdentitystoreGroup_AlternateIdentifier {
    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataIdentitystoreGroup_AlternateIdentifier_ExternalId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unique_attribute`.\n"]
    pub fn set_unique_attribute(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataIdentitystoreGroup_AlternateIdentifier_UniqueAttribute>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unique_attribute = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unique_attribute = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataIdentitystoreGroup_AlternateIdentifier {
    type O = terrars::BlockAssignable<DataIdentitystoreGroup_AlternateIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroup_AlternateIdentifier {}

impl BuildDataIdentitystoreGroup_AlternateIdentifier {
    pub fn build(self) -> DataIdentitystoreGroup_AlternateIdentifier {
        DataIdentitystoreGroup_AlternateIdentifier {
            external_id: core::default::Default::default(),
            unique_attribute: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreGroup_AlternateIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroup_AlternateIdentifierRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIdentitystoreGroup_AlternateIdentifierRef {
        DataIdentitystoreGroup_AlternateIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroup_AlternateIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::ListRef<DataIdentitystoreGroup_AlternateIdentifier_ExternalIdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `unique_attribute` after provisioning.\n"]
    pub fn unique_attribute(&self) -> terrars::ListRef<DataIdentitystoreGroup_AlternateIdentifier_UniqueAttributeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unique_attribute", self.base))
    }
}

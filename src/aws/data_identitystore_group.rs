use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIdentitystoreGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    identity_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternate_identifier: Option<Vec<DataIdentitystoreGroupAlternateIdentifierEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataIdentitystoreGroupFilterEl>>,
    dynamic: DataIdentitystoreGroupDynamic,
}

struct DataIdentitystoreGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIdentitystoreGroupData>,
}

#[derive(Clone)]
pub struct DataIdentitystoreGroup(Rc<DataIdentitystoreGroup_>);

impl DataIdentitystoreGroup {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `group_id`.\n"]
    pub fn set_group_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().group_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `alternate_identifier`.\n"]
    pub fn set_alternate_identifier(
        self,
        v: impl Into<BlockAssignable<DataIdentitystoreGroupAlternateIdentifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alternate_identifier = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alternate_identifier = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataIdentitystoreGroupFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> ListRef<DataIdentitystoreGroupExternalIdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alternate_identifier` after provisioning.\n"]
    pub fn alternate_identifier(&self) -> ListRef<DataIdentitystoreGroupAlternateIdentifierElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alternate_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataIdentitystoreGroupFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl Datasource for DataIdentitystoreGroup {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataIdentitystoreGroup {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataIdentitystoreGroup {
    type O = ListRef<DataIdentitystoreGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataIdentitystoreGroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_identitystore_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIdentitystoreGroup {
    pub tf_id: String,
    #[doc= ""]
    pub identity_store_id: PrimField<String>,
}

impl BuildDataIdentitystoreGroup {
    pub fn build(self, stack: &mut Stack) -> DataIdentitystoreGroup {
        let out = DataIdentitystoreGroup(Rc::new(DataIdentitystoreGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIdentitystoreGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                group_id: core::default::Default::default(),
                id: core::default::Default::default(),
                identity_store_id: self.identity_store_id,
                alternate_identifier: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIdentitystoreGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIdentitystoreGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_ids` after provisioning.\n"]
    pub fn external_ids(&self) -> ListRef<DataIdentitystoreGroupExternalIdsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alternate_identifier` after provisioning.\n"]
    pub fn alternate_identifier(&self) -> ListRef<DataIdentitystoreGroupAlternateIdentifierElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alternate_identifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataIdentitystoreGroupFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroupExternalIdsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}

impl DataIdentitystoreGroupExternalIdsEl {
    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }
}

impl ToListMappable for DataIdentitystoreGroupExternalIdsEl {
    type O = BlockAssignable<DataIdentitystoreGroupExternalIdsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroupExternalIdsEl {}

impl BuildDataIdentitystoreGroupExternalIdsEl {
    pub fn build(self) -> DataIdentitystoreGroupExternalIdsEl {
        DataIdentitystoreGroupExternalIdsEl {
            id: core::default::Default::default(),
            issuer: core::default::Default::default(),
        }
    }
}

pub struct DataIdentitystoreGroupExternalIdsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreGroupExternalIdsElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreGroupExternalIdsElRef {
        DataIdentitystoreGroupExternalIdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroupExternalIdsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroupAlternateIdentifierElExternalIdEl {
    id: PrimField<String>,
    issuer: PrimField<String>,
}

impl DataIdentitystoreGroupAlternateIdentifierElExternalIdEl { }

impl ToListMappable for DataIdentitystoreGroupAlternateIdentifierElExternalIdEl {
    type O = BlockAssignable<DataIdentitystoreGroupAlternateIdentifierElExternalIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroupAlternateIdentifierElExternalIdEl {
    #[doc= ""]
    pub id: PrimField<String>,
    #[doc= ""]
    pub issuer: PrimField<String>,
}

impl BuildDataIdentitystoreGroupAlternateIdentifierElExternalIdEl {
    pub fn build(self) -> DataIdentitystoreGroupAlternateIdentifierElExternalIdEl {
        DataIdentitystoreGroupAlternateIdentifierElExternalIdEl {
            id: self.id,
            issuer: self.issuer,
        }
    }
}

pub struct DataIdentitystoreGroupAlternateIdentifierElExternalIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreGroupAlternateIdentifierElExternalIdElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreGroupAlternateIdentifierElExternalIdElRef {
        DataIdentitystoreGroupAlternateIdentifierElExternalIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroupAlternateIdentifierElExternalIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl {
    attribute_path: PrimField<String>,
    attribute_value: PrimField<String>,
}

impl DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl { }

impl ToListMappable for DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl {
    type O = BlockAssignable<DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl {
    #[doc= ""]
    pub attribute_path: PrimField<String>,
    #[doc= ""]
    pub attribute_value: PrimField<String>,
}

impl BuildDataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl {
    pub fn build(self) -> DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl {
        DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl {
            attribute_path: self.attribute_path,
            attribute_value: self.attribute_value,
        }
    }
}

pub struct DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeElRef {
        DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_path` after provisioning.\n"]
    pub fn attribute_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_path", self.base))
    }

    #[doc= "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIdentitystoreGroupAlternateIdentifierElDynamic {
    external_id: Option<DynamicBlock<DataIdentitystoreGroupAlternateIdentifierElExternalIdEl>>,
    unique_attribute: Option<DynamicBlock<DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl>>,
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroupAlternateIdentifierEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<Vec<DataIdentitystoreGroupAlternateIdentifierElExternalIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_attribute: Option<Vec<DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl>>,
    dynamic: DataIdentitystoreGroupAlternateIdentifierElDynamic,
}

impl DataIdentitystoreGroupAlternateIdentifierEl {
    #[doc= "Set the field `external_id`.\n"]
    pub fn set_external_id(
        mut self,
        v: impl Into<BlockAssignable<DataIdentitystoreGroupAlternateIdentifierElExternalIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_id = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_id = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `unique_attribute`.\n"]
    pub fn set_unique_attribute(
        mut self,
        v: impl Into<BlockAssignable<DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.unique_attribute = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.unique_attribute = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataIdentitystoreGroupAlternateIdentifierEl {
    type O = BlockAssignable<DataIdentitystoreGroupAlternateIdentifierEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroupAlternateIdentifierEl {}

impl BuildDataIdentitystoreGroupAlternateIdentifierEl {
    pub fn build(self) -> DataIdentitystoreGroupAlternateIdentifierEl {
        DataIdentitystoreGroupAlternateIdentifierEl {
            external_id: core::default::Default::default(),
            unique_attribute: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataIdentitystoreGroupAlternateIdentifierElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreGroupAlternateIdentifierElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreGroupAlternateIdentifierElRef {
        DataIdentitystoreGroupAlternateIdentifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroupAlternateIdentifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> ListRef<DataIdentitystoreGroupAlternateIdentifierElExternalIdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc= "Get a reference to the value of field `unique_attribute` after provisioning.\n"]
    pub fn unique_attribute(&self) -> ListRef<DataIdentitystoreGroupAlternateIdentifierElUniqueAttributeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.unique_attribute", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroupFilterEl {
    attribute_path: PrimField<String>,
    attribute_value: PrimField<String>,
}

impl DataIdentitystoreGroupFilterEl { }

impl ToListMappable for DataIdentitystoreGroupFilterEl {
    type O = BlockAssignable<DataIdentitystoreGroupFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroupFilterEl {
    #[doc= ""]
    pub attribute_path: PrimField<String>,
    #[doc= ""]
    pub attribute_value: PrimField<String>,
}

impl BuildDataIdentitystoreGroupFilterEl {
    pub fn build(self) -> DataIdentitystoreGroupFilterEl {
        DataIdentitystoreGroupFilterEl {
            attribute_path: self.attribute_path,
            attribute_value: self.attribute_value,
        }
    }
}

pub struct DataIdentitystoreGroupFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataIdentitystoreGroupFilterElRef {
    fn new(shared: StackShared, base: String) -> DataIdentitystoreGroupFilterElRef {
        DataIdentitystoreGroupFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroupFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attribute_path` after provisioning.\n"]
    pub fn attribute_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_path", self.base))
    }

    #[doc= "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataIdentitystoreGroupDynamic {
    alternate_identifier: Option<DynamicBlock<DataIdentitystoreGroupAlternateIdentifierEl>>,
    filter: Option<DynamicBlock<DataIdentitystoreGroupFilterEl>>,
}

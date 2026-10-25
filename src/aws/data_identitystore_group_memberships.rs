#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIdentitystoreGroupMembershipsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    group_id: terrars::PrimField<std::string::String>,
    identity_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataIdentitystoreGroupMemberships_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIdentitystoreGroupMembershipsData_>,
}

#[derive(Clone)]
pub struct DataIdentitystoreGroupMemberships(std::rc::Rc<DataIdentitystoreGroupMemberships_>);

impl DataIdentitystoreGroupMemberships {
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

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_memberships` after provisioning.\n"]
    pub fn group_memberships(&self) -> terrars::ListRef<DataIdentitystoreGroupMemberships_GroupMembershipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_memberships", self.extract_ref()))
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

impl terrars::Referable for DataIdentitystoreGroupMemberships {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIdentitystoreGroupMemberships { }

impl terrars::ToListMappable for DataIdentitystoreGroupMemberships {
    type O = terrars::ListRef<DataIdentitystoreGroupMembershipsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIdentitystoreGroupMemberships_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_identitystore_group_memberships".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIdentitystoreGroupMemberships {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub group_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub identity_store_id: terrars::PrimField<std::string::String>,
}

impl BuildDataIdentitystoreGroupMemberships {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIdentitystoreGroupMemberships {
        let out = DataIdentitystoreGroupMemberships(std::rc::Rc::new(DataIdentitystoreGroupMemberships_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIdentitystoreGroupMembershipsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                group_id: self.group_id,
                identity_store_id: self.identity_store_id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIdentitystoreGroupMembershipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroupMembershipsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIdentitystoreGroupMembershipsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_memberships` after provisioning.\n"]
    pub fn group_memberships(&self) -> terrars::ListRef<DataIdentitystoreGroupMemberships_GroupMembershipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_memberships", self.extract_ref()))
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
pub struct DataIdentitystoreGroupMemberships_GroupMemberships_MemberId {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreGroupMemberships_GroupMemberships_MemberId {
    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIdentitystoreGroupMemberships_GroupMemberships_MemberId {
    type O = terrars::BlockAssignable<DataIdentitystoreGroupMemberships_GroupMemberships_MemberId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroupMemberships_GroupMemberships_MemberId {}

impl BuildDataIdentitystoreGroupMemberships_GroupMemberships_MemberId {
    pub fn build(self) -> DataIdentitystoreGroupMemberships_GroupMemberships_MemberId {
        DataIdentitystoreGroupMemberships_GroupMemberships_MemberId { user_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreGroupMemberships_GroupMemberships_MemberIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroupMemberships_GroupMemberships_MemberIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIdentitystoreGroupMemberships_GroupMemberships_MemberIdRef {
        DataIdentitystoreGroupMemberships_GroupMemberships_MemberIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroupMemberships_GroupMemberships_MemberIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataIdentitystoreGroupMemberships_GroupMemberships {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_store_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    member_id: std::option::Option<DataIdentitystoreGroupMemberships_GroupMemberships_MemberId>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIdentitystoreGroupMemberships_GroupMemberships {
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

    #[doc = "Set the field `member_id`.\n"]
    pub fn set_member_id(
        mut self,
        v: impl std::convert::Into<DataIdentitystoreGroupMemberships_GroupMemberships_MemberId>,
    ) -> Self {
        self.member_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `membership_id`.\n"]
    pub fn set_membership_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.membership_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIdentitystoreGroupMemberships_GroupMemberships {
    type O = terrars::BlockAssignable<DataIdentitystoreGroupMemberships_GroupMemberships>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIdentitystoreGroupMemberships_GroupMemberships {}

impl BuildDataIdentitystoreGroupMemberships_GroupMemberships {
    pub fn build(self) -> DataIdentitystoreGroupMemberships_GroupMemberships {
        DataIdentitystoreGroupMemberships_GroupMemberships {
            group_id: core::default::Default::default(),
            identity_store_id: core::default::Default::default(),
            member_id: core::default::Default::default(),
            membership_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIdentitystoreGroupMemberships_GroupMembershipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIdentitystoreGroupMemberships_GroupMembershipsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataIdentitystoreGroupMemberships_GroupMembershipsRef {
        DataIdentitystoreGroupMemberships_GroupMembershipsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIdentitystoreGroupMemberships_GroupMembershipsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_store_id` after provisioning.\n"]
    pub fn identity_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_store_id", self.base))
    }

    #[doc = "Get a reference to the value of field `member_id` after provisioning.\n"]
    pub fn member_id(&self) -> DataIdentitystoreGroupMemberships_GroupMemberships_MemberIdRef {
        DataIdentitystoreGroupMemberships_GroupMemberships_MemberIdRef::new(
            self.shared().clone(),
            format!("{}.member_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `membership_id` after provisioning.\n"]
    pub fn membership_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_id", self.base))
    }
}

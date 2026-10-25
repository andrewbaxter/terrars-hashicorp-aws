#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataResiliencehubv2ServiceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataResiliencehubv2Service_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataResiliencehubv2ServiceData_>,
}

#[derive(Clone)]
pub struct DataResiliencehubv2Service(std::rc::Rc<DataResiliencehubv2Service_>);

impl DataResiliencehubv2Service {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associated_system` after provisioning.\n"]
    pub fn associated_system(&self) -> terrars::ListRef<DataResiliencehubv2Service_AssociatedSystemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_system", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_model` after provisioning.\n"]
    pub fn permission_model(&self) -> terrars::ListRef<DataResiliencehubv2Service_PermissionModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_arn` after provisioning.\n"]
    pub fn policy_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataResiliencehubv2Service {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataResiliencehubv2Service { }

impl terrars::ToListMappable for DataResiliencehubv2Service {
    type O = terrars::ListRef<DataResiliencehubv2ServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataResiliencehubv2Service_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_resiliencehubv2_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataResiliencehubv2Service {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataResiliencehubv2Service {
    pub fn build(self, stack: &mut terrars::Stack) -> DataResiliencehubv2Service {
        let out = DataResiliencehubv2Service(std::rc::Rc::new(DataResiliencehubv2Service_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataResiliencehubv2ServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataResiliencehubv2ServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResiliencehubv2ServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataResiliencehubv2ServiceRef {
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

    #[doc = "Get a reference to the value of field `associated_system` after provisioning.\n"]
    pub fn associated_system(&self) -> terrars::ListRef<DataResiliencehubv2Service_AssociatedSystemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associated_system", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission_model` after provisioning.\n"]
    pub fn permission_model(&self) -> terrars::ListRef<DataResiliencehubv2Service_PermissionModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permission_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_arn` after provisioning.\n"]
    pub fn policy_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataResiliencehubv2Service_AssociatedSystem {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_journey_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataResiliencehubv2Service_AssociatedSystem {
    #[doc = "Set the field `system_arn`.\n"]
    pub fn set_system_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.system_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_journey_ids`.\n"]
    pub fn set_user_journey_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_journey_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResiliencehubv2Service_AssociatedSystem {
    type O = terrars::BlockAssignable<DataResiliencehubv2Service_AssociatedSystem>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResiliencehubv2Service_AssociatedSystem {}

impl BuildDataResiliencehubv2Service_AssociatedSystem {
    pub fn build(self) -> DataResiliencehubv2Service_AssociatedSystem {
        DataResiliencehubv2Service_AssociatedSystem {
            system_arn: core::default::Default::default(),
            user_journey_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataResiliencehubv2Service_AssociatedSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResiliencehubv2Service_AssociatedSystemRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataResiliencehubv2Service_AssociatedSystemRef {
        DataResiliencehubv2Service_AssociatedSystemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResiliencehubv2Service_AssociatedSystemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `system_arn` after provisioning.\n"]
    pub fn system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `user_journey_ids` after provisioning.\n"]
    pub fn user_journey_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_journey_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResiliencehubv2Service_PermissionModel_CrossAccountRole {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataResiliencehubv2Service_PermissionModel_CrossAccountRole {
    #[doc = "Set the field `cross_account_role_arn`.\n"]
    pub fn set_cross_account_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_account_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResiliencehubv2Service_PermissionModel_CrossAccountRole {
    type O = terrars::BlockAssignable<DataResiliencehubv2Service_PermissionModel_CrossAccountRole>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResiliencehubv2Service_PermissionModel_CrossAccountRole {}

impl BuildDataResiliencehubv2Service_PermissionModel_CrossAccountRole {
    pub fn build(self) -> DataResiliencehubv2Service_PermissionModel_CrossAccountRole {
        DataResiliencehubv2Service_PermissionModel_CrossAccountRole {
            cross_account_role_arn: core::default::Default::default(),
            external_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataResiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataResiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
        DataResiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResiliencehubv2Service_PermissionModel_CrossAccountRoleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role_arn` after provisioning.\n"]
    pub fn cross_account_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_account_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResiliencehubv2Service_PermissionModel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_account_role: std
    ::option
    ::Option<terrars::ListField<DataResiliencehubv2Service_PermissionModel_CrossAccountRole>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoker_role_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataResiliencehubv2Service_PermissionModel {
    #[doc = "Set the field `cross_account_role`.\n"]
    pub fn set_cross_account_role(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataResiliencehubv2Service_PermissionModel_CrossAccountRole>>,
    ) -> Self {
        self.cross_account_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invoker_role_name`.\n"]
    pub fn set_invoker_role_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.invoker_role_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataResiliencehubv2Service_PermissionModel {
    type O = terrars::BlockAssignable<DataResiliencehubv2Service_PermissionModel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResiliencehubv2Service_PermissionModel {}

impl BuildDataResiliencehubv2Service_PermissionModel {
    pub fn build(self) -> DataResiliencehubv2Service_PermissionModel {
        DataResiliencehubv2Service_PermissionModel {
            cross_account_role: core::default::Default::default(),
            invoker_role_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataResiliencehubv2Service_PermissionModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataResiliencehubv2Service_PermissionModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataResiliencehubv2Service_PermissionModelRef {
        DataResiliencehubv2Service_PermissionModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResiliencehubv2Service_PermissionModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cross_account_role` after provisioning.\n"]
    pub fn cross_account_role(&self) -> terrars::ListRef<DataResiliencehubv2Service_PermissionModel_CrossAccountRoleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cross_account_role", self.base))
    }

    #[doc = "Get a reference to the value of field `invoker_role_name` after provisioning.\n"]
    pub fn invoker_role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoker_role_name", self.base))
    }
}

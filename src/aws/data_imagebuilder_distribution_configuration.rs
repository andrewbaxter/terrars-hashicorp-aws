#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderDistributionConfigurationData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataImagebuilderDistributionConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImagebuilderDistributionConfigurationData_>,
}

#[derive(Clone)]
pub struct DataImagebuilderDistributionConfiguration(std::rc::Rc<DataImagebuilderDistributionConfiguration_>);

impl DataImagebuilderDistributionConfiguration {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution` after provisioning.\n"]
    pub fn distribution(&self) -> terrars::SetRef<DataImagebuilderDistributionConfiguration_DistributionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.distribution", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataImagebuilderDistributionConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataImagebuilderDistributionConfiguration { }

impl terrars::ToListMappable for DataImagebuilderDistributionConfiguration {
    type O = terrars::ListRef<DataImagebuilderDistributionConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataImagebuilderDistributionConfiguration_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_imagebuilder_distribution_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataImagebuilderDistributionConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> DataImagebuilderDistributionConfiguration {
        let out =
            DataImagebuilderDistributionConfiguration(std::rc::Rc::new(DataImagebuilderDistributionConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataImagebuilderDistributionConfigurationData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    arn: self.arn,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    tags: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderDistributionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImagebuilderDistributionConfigurationRef {
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

    #[doc = "Get a reference to the value of field `date_created` after provisioning.\n"]
    pub fn date_created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_updated` after provisioning.\n"]
    pub fn date_updated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date_updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `distribution` after provisioning.\n"]
    pub fn distribution(&self) -> terrars::SetRef<DataImagebuilderDistributionConfiguration_DistributionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.distribution", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission {
    #[doc = "Set the field `organization_arns`.\n"]
    pub fn set_organization_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.organization_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_arns`.\n"]
    pub fn set_organizational_unit_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.organizational_unit_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_groups`.\n"]
    pub fn set_user_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_ids`.\n"]
    pub fn set_user_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission {
    type O =
        terrars
        ::BlockAssignable<
            DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission {
        DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission {
            organization_arns: core::default::Default::default(),
            organizational_unit_arns: core::default::Default::default(),
            user_groups: core::default::Default::default(),
            user_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermissionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermissionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermissionRef {
        DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermissionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermissionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `organization_arns` after provisioning.\n"]
    pub fn organization_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.organization_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_arns` after provisioning.\n"]
    pub fn organizational_unit_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.organizational_unit_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `user_groups` after provisioning.\n"]
    pub fn user_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `user_ids` after provisioning.\n"]
    pub fn user_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.user_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ami_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_permission: std
    ::option
    ::Option<
        terrars
        ::SetField<
            DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_account_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration {
    #[doc = "Set the field `ami_tags`.\n"]
    pub fn set_ami_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ami_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_permission`.\n"]
    pub fn set_launch_permission(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermission,
                        >,
                    >,
    ) -> Self {
        self.launch_permission = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_account_ids`.\n"]
    pub fn set_target_account_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_account_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration {
    type O =
        terrars
        ::BlockAssignable<DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration {
    pub fn build(self) -> DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration {
        DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration {
            ami_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            launch_permission: core::default::Default::default(),
            name: core::default::Default::default(),
            target_account_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfigurationRef {
        DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_tags` after provisioning.\n"]
    pub fn ami_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ami_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_permission` after provisioning.\n"]
    pub fn launch_permission(
        &self,
    ) -> terrars
    ::SetRef<DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration_LaunchPermissionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.launch_permission", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_account_ids` after provisioning.\n"]
    pub fn target_account_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_account_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository {
    #[doc = "Set the field `repository_name`.\n"]
    pub fn set_repository_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository {
    type O =
        terrars
        ::BlockAssignable<
            DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository {
        DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository {
            repository_name: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepositoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepositoryRef {
        DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repository_name` after provisioning.\n"]
    pub fn repository_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_repository: std
    ::option
    ::Option<
        terrars
        ::SetField<
            DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository,
        >,
    >,
}

impl DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration {
    #[doc = "Set the field `container_tags`.\n"]
    pub fn set_container_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.container_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_repository`.\n"]
    pub fn set_target_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepository,
                        >,
                    >,
    ) -> Self {
        self.target_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration {
    type O =
        terrars
        ::BlockAssignable<DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration {
    pub fn build(self) -> DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration {
        DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration {
            container_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            target_repository: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfigurationRef {
        DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_tags` after provisioning.\n"]
    pub fn container_tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.container_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `target_repository` after provisioning.\n"]
    pub fn target_repository(
        &self,
    ) -> terrars
    ::SetRef<
        DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration_TargetRepositoryRef,
    > {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate {
    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_name`.\n"]
    pub fn set_launch_template_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_version`.\n"]
    pub fn set_launch_template_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate {
    type O =
        terrars
        ::BlockAssignable<
            DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate {
        DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            launch_template_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplateRef {
        DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_id", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_name` after provisioning.\n"]
    pub fn launch_template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_version` after provisioning.\n"]
    pub fn launch_template_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_resource_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration {
    #[doc = "Set the field `target_resource_count`.\n"]
    pub fn set_target_resource_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_resource_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration {
        DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration {
            target_resource_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfigurationRef {
        DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_resource_count` after provisioning.\n"]
    pub fn target_resource_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_resource_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std
    ::option
    ::Option<
        terrars
        ::SetField<DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_parallel_launches: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_configuration: std
    ::option
    ::Option<
        terrars
        ::SetField<
            DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration,
        >,
    >,
}

impl DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplate,
                        >,
                    >,
    ) -> Self {
        self.launch_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_launches`.\n"]
    pub fn set_max_parallel_launches(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_parallel_launches = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_configuration`.\n"]
    pub fn set_snapshot_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfiguration,
                        >,
                    >,
    ) -> Self {
        self.snapshot_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration {
    type O =
        terrars::BlockAssignable<DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration {
    pub fn build(self) -> DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration {
        DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration {
            account_id: core::default::Default::default(),
            enabled: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            max_parallel_launches: core::default::Default::default(),
            snapshot_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfigurationRef {
        DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(
        &self,
    ) -> terrars
    ::SetRef<DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_LaunchTemplateRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.launch_template", self.base))
    }

    #[doc = "Get a reference to the value of field `max_parallel_launches` after provisioning.\n"]
    pub fn max_parallel_launches(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_launches", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_configuration` after provisioning.\n"]
    pub fn snapshot_configuration(
        &self,
    ) -> terrars
    ::SetRef<DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration_SnapshotConfigurationRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.snapshot_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_id`.\n"]
    pub fn set_launch_template_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.launch_template_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration {
    type O =
        terrars::BlockAssignable<DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration {
    pub fn build(self) -> DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration {
        DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration {
            account_id: core::default::Default::default(),
            default: core::default::Default::default(),
            launch_template_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfigurationRef {
        DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_id` after provisioning.\n"]
    pub fn launch_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_template_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_image_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration {
    #[doc = "Set the field `disk_image_format`.\n"]
    pub fn set_disk_image_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_image_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_name`.\n"]
    pub fn set_role_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_prefix`.\n"]
    pub fn set_s3_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration {
    type O = terrars::BlockAssignable<DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration {
    pub fn build(self) -> DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration {
        DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration {
            disk_image_format: core::default::Default::default(),
            role_name: core::default::Default::default(),
            s3_bucket: core::default::Default::default(),
            s3_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfigurationRef {
        DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_image_format` after provisioning.\n"]
    pub fn disk_image_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_image_format", self.base))
    }

    #[doc = "Get a reference to the value of field `role_name` after provisioning.\n"]
    pub fn role_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ami_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration {
    #[doc = "Set the field `ami_account_id`.\n"]
    pub fn set_ami_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ami_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_type`.\n"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_name`.\n"]
    pub fn set_parameter_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameter_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration {
    type O =
        terrars::BlockAssignable<DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration {
    pub fn build(self) -> DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration {
        DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration {
            ami_account_id: core::default::Default::default(),
            data_type: core::default::Default::default(),
            parameter_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfigurationRef {
        DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_account_id` after provisioning.\n"]
    pub fn ami_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `data_type` after provisioning.\n"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfiguration_Distribution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ami_distribution_configuration: std
    ::option
    ::Option<terrars::SetField<DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_distribution_configuration: std
    ::option
    ::Option<
        terrars::SetField<DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_launch_configuration: std
    ::option
    ::Option<terrars::SetField<DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_configuration: std
    ::option
    ::Option<terrars::SetField<DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_configuration_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_export_configuration: std
    ::option
    ::Option<terrars::SetField<DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssm_parameter_configuration: std
    ::option
    ::Option<terrars::SetField<DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration>>,
}

impl DataImagebuilderDistributionConfiguration_Distribution {
    #[doc = "Set the field `ami_distribution_configuration`.\n"]
    pub fn set_ami_distribution_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfiguration,
                        >,
                    >,
    ) -> Self {
        self.ami_distribution_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_distribution_configuration`.\n"]
    pub fn set_container_distribution_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfiguration,
                        >,
                    >,
    ) -> Self {
        self.container_distribution_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fast_launch_configuration`.\n"]
    pub fn set_fast_launch_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfiguration>,
                    >,
    ) -> Self {
        self.fast_launch_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template_configuration`.\n"]
    pub fn set_launch_template_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfiguration>,
                    >,
    ) -> Self {
        self.launch_template_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `license_configuration_arns`.\n"]
    pub fn set_license_configuration_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.license_configuration_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_export_configuration`.\n"]
    pub fn set_s3_export_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfiguration>,
                    >,
    ) -> Self {
        self.s3_export_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssm_parameter_configuration`.\n"]
    pub fn set_ssm_parameter_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfiguration>,
                    >,
    ) -> Self {
        self.ssm_parameter_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderDistributionConfiguration_Distribution {
    type O = terrars::BlockAssignable<DataImagebuilderDistributionConfiguration_Distribution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfiguration_Distribution {}

impl BuildDataImagebuilderDistributionConfiguration_Distribution {
    pub fn build(self) -> DataImagebuilderDistributionConfiguration_Distribution {
        DataImagebuilderDistributionConfiguration_Distribution {
            ami_distribution_configuration: core::default::Default::default(),
            container_distribution_configuration: core::default::Default::default(),
            fast_launch_configuration: core::default::Default::default(),
            launch_template_configuration: core::default::Default::default(),
            license_configuration_arns: core::default::Default::default(),
            region: core::default::Default::default(),
            s3_export_configuration: core::default::Default::default(),
            ssm_parameter_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataImagebuilderDistributionConfiguration_DistributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfiguration_DistributionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfiguration_DistributionRef {
        DataImagebuilderDistributionConfiguration_DistributionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfiguration_DistributionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_distribution_configuration` after provisioning.\n"]
    pub fn ami_distribution_configuration(
        &self,
    ) -> terrars::SetRef<DataImagebuilderDistributionConfiguration_Distribution_AmiDistributionConfigurationRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ami_distribution_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `container_distribution_configuration` after provisioning.\n"]
    pub fn container_distribution_configuration(
        &self,
    ) -> terrars
    ::SetRef<DataImagebuilderDistributionConfiguration_Distribution_ContainerDistributionConfigurationRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.container_distribution_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_launch_configuration` after provisioning.\n"]
    pub fn fast_launch_configuration(
        &self,
    ) -> terrars::SetRef<DataImagebuilderDistributionConfiguration_Distribution_FastLaunchConfigurationRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.fast_launch_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_configuration` after provisioning.\n"]
    pub fn launch_template_configuration(
        &self,
    ) -> terrars::SetRef<DataImagebuilderDistributionConfiguration_Distribution_LaunchTemplateConfigurationRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.launch_template_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `license_configuration_arns` after provisioning.\n"]
    pub fn license_configuration_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.license_configuration_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_export_configuration` after provisioning.\n"]
    pub fn s3_export_configuration(
        &self,
    ) -> terrars::SetRef<DataImagebuilderDistributionConfiguration_Distribution_S3ExportConfigurationRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.s3_export_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `ssm_parameter_configuration` after provisioning.\n"]
    pub fn ssm_parameter_configuration(
        &self,
    ) -> terrars::SetRef<DataImagebuilderDistributionConfiguration_Distribution_SsmParameterConfigurationRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ssm_parameter_configuration", self.base))
    }
}

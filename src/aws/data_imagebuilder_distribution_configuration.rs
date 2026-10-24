use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderDistributionConfigurationData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataImagebuilderDistributionConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImagebuilderDistributionConfigurationData>,
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
    pub fn distribution(&self) -> terrars::SetRef<DataImagebuilderDistributionConfigurationDistributionElRef> {
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
                data: std::cell::RefCell::new(DataImagebuilderDistributionConfigurationData {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    arn: self.arn,
                    id: core::default::Default::default(),
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
    pub fn distribution(&self) -> terrars::SetRef<DataImagebuilderDistributionConfigurationDistributionElRef> {
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
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
::ToListMappable for DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    type O =
        terrars
        ::BlockAssignable<
            DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
            organization_arns: core::default::Default::default(),
            organizational_unit_arns: core::default::Default::default(),
            user_groups: core::default::Default::default(),
            user_ids: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
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
pub struct DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
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
            DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_account_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
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
                            DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
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

impl terrars::ToListMappable for DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
            ami_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            launch_permission: core::default::Default::default(),
            name: core::default::Default::default(),
            target_account_ids: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
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
    ::SetRef<
        DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef,
    > {
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
pub struct DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
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
::ToListMappable for DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    type O =
        terrars
        ::BlockAssignable<
            DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
            repository_name: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
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
pub struct DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
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
            DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
        >,
    >,
}

impl DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
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
                            DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
                        >,
                    >,
    ) -> Self {
        self.target_repository = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
            container_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            target_repository: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
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
        DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef,
    > {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
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
::ToListMappable for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    type O =
        terrars
        ::BlockAssignable<
            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            launch_template_version: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
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
pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_resource_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    #[doc = "Set the field `target_resource_count`.\n"]
    pub fn set_target_resource_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_resource_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    pub fn build(
        self,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
            target_resource_count: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_resource_count` after provisioning.\n"]
    pub fn target_resource_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_resource_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std
    ::option
    ::Option<
        terrars
        ::SetField<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_parallel_launches: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_configuration: std
    ::option
    ::Option<
        terrars
        ::SetField<
            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
        >,
    >,
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
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
                            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl,
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
                            DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
                        >,
                    >,
    ) -> Self {
        self.snapshot_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    type O =
        terrars::BlockAssignable<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
            account_id: core::default::Default::default(),
            enabled: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            max_parallel_launches: core::default::Default::default(),
            snapshot_configuration: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
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
    ::SetRef<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef> {
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
    ::SetRef<
        DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef,
    > {
        terrars::SetRef::new(self.shared().clone(), format!("{}.snapshot_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
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

impl terrars::ToListMappable for DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
        DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
            account_id: core::default::Default::default(),
            default: core::default::Default::default(),
            launch_template_id: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
        DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
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
pub struct DataImagebuilderDistributionConfigurationDistributionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ami_distribution_configuration: std
    ::option
    ::Option<
        terrars::SetField<DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_distribution_configuration: std
    ::option
    ::Option<
        terrars::SetField<DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_launch_configuration: std
    ::option
    ::Option<terrars::SetField<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_configuration: std
    ::option
    ::Option<terrars::SetField<DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_configuration_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderDistributionConfigurationDistributionEl {
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
                            DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl,
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
                            DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl,
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
                        ::SetField<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>,
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
                        ::SetField<
                            DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl,
                        >,
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
}

impl terrars::ToListMappable for DataImagebuilderDistributionConfigurationDistributionEl {
    type O = terrars::BlockAssignable<DataImagebuilderDistributionConfigurationDistributionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationDistributionEl {}

impl BuildDataImagebuilderDistributionConfigurationDistributionEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationDistributionEl {
        DataImagebuilderDistributionConfigurationDistributionEl {
            ami_distribution_configuration: core::default::Default::default(),
            container_distribution_configuration: core::default::Default::default(),
            fast_launch_configuration: core::default::Default::default(),
            launch_template_configuration: core::default::Default::default(),
            license_configuration_arns: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationDistributionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderDistributionConfigurationDistributionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderDistributionConfigurationDistributionElRef {
        DataImagebuilderDistributionConfigurationDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationDistributionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_distribution_configuration` after provisioning.\n"]
    pub fn ami_distribution_configuration(
        &self,
    ) -> terrars::SetRef<DataImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ami_distribution_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `container_distribution_configuration` after provisioning.\n"]
    pub fn container_distribution_configuration(
        &self,
    ) -> terrars
    ::SetRef<DataImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.container_distribution_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_launch_configuration` after provisioning.\n"]
    pub fn fast_launch_configuration(
        &self,
    ) -> terrars::SetRef<DataImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.fast_launch_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template_configuration` after provisioning.\n"]
    pub fn launch_template_configuration(
        &self,
    ) -> terrars::SetRef<DataImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef> {
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
}

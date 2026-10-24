use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ImagebuilderDistributionConfigurationData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distribution: std::option::Option<std::vec::Vec<ImagebuilderDistributionConfigurationDistributionEl>>,
    dynamic: ImagebuilderDistributionConfigurationDynamic,
}

struct ImagebuilderDistributionConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ImagebuilderDistributionConfigurationData>,
}

#[derive(Clone)]
pub struct ImagebuilderDistributionConfiguration(std::rc::Rc<ImagebuilderDistributionConfiguration_>);

impl ImagebuilderDistributionConfiguration {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `distribution`.\n"]
    pub fn set_distribution(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ImagebuilderDistributionConfigurationDistributionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().distribution = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.distribution = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl terrars::Referable for ImagebuilderDistributionConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ImagebuilderDistributionConfiguration { }

impl terrars::ToListMappable for ImagebuilderDistributionConfiguration {
    type O = terrars::ListRef<ImagebuilderDistributionConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ImagebuilderDistributionConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_imagebuilder_distribution_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildImagebuilderDistributionConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderDistributionConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> ImagebuilderDistributionConfiguration {
        let out = ImagebuilderDistributionConfiguration(std::rc::Rc::new(ImagebuilderDistributionConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ImagebuilderDistributionConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                distribution: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ImagebuilderDistributionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderDistributionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ImagebuilderDistributionConfigurationRef {
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
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
::ToListMappable for ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    type O =
        terrars
        ::BlockAssignable<
            ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
    pub fn build(
        self,
    ) -> ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
        ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl {
            organization_arns: core::default::Default::default(),
            organizational_unit_arns: core::default::Default::default(),
            user_groups: core::default::Default::default(),
            user_ids: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
        ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef {
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

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElDynamic {
    launch_permission: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ami_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_account_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_permission: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl>,
    >,
    dynamic: ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElDynamic,
}

impl ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
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

    #[doc = "Set the field `launch_permission`.\n"]
    pub fn set_launch_permission(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_permission = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_permission = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    type O =
        terrars::BlockAssignable<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl {
            ami_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            name: core::default::Default::default(),
            target_account_ids: core::default::Default::default(),
            launch_permission: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_account_ids` after provisioning.\n"]
    pub fn target_account_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_account_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_permission` after provisioning.\n"]
    pub fn launch_permission(
        &self,
    ) -> terrars
    ::ListRef<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElLaunchPermissionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_permission", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    repository_name: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
}

impl ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl { }

impl terrars
::ToListMappable for ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    type O =
        terrars
        ::BlockAssignable<
            ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    #[doc = ""]
    pub repository_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
    pub fn build(
        self,
    ) -> ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl {
            repository_name: self.repository_name,
            service: self.service,
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef {
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

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElDynamic {
    target_repository: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_repository: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
        >,
    >,
    dynamic: ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElDynamic,
}

impl ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
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
                        ::BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl {
            container_tags: core::default::Default::default(),
            description: core::default::Default::default(),
            target_repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef {
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
    ::ListRef<
        ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElTargetRepositoryElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_repository", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
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
::ToListMappable for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    type O =
        terrars
        ::BlockAssignable<
            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
    pub fn build(
        self,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl {
            launch_template_id: core::default::Default::default(),
            launch_template_name: core::default::Default::default(),
            launch_template_version: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef {
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
pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_resource_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    #[doc = "Set the field `target_resource_count`.\n"]
    pub fn set_target_resource_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_resource_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {}

impl BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
    pub fn build(
        self,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl {
            target_resource_count: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_resource_count` after provisioning.\n"]
    pub fn target_resource_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_resource_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElDynamic {
    launch_template: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl>,
    >,
    snapshot_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    account_id: terrars::PrimField<std::string::String>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_parallel_launches: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std
    ::option
    ::Option<
        std::vec::Vec<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl>,
    >,
    dynamic: ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElDynamic,
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    #[doc = "Set the field `max_parallel_launches`.\n"]
    pub fn set_max_parallel_launches(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_parallel_launches = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template = std::option::Option::Some(d);
            },
        }
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
                        ::BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snapshot_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snapshot_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    type O = terrars::BlockAssignable<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl {
            account_id: self.account_id,
            enabled: self.enabled,
            max_parallel_launches: core::default::Default::default(),
            launch_template: core::default::Default::default(),
            snapshot_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElRef {
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

    #[doc = "Get a reference to the value of field `max_parallel_launches` after provisioning.\n"]
    pub fn max_parallel_launches(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_launches", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(
        &self,
    ) -> terrars
    ::ListRef<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_configuration` after provisioning.\n"]
    pub fn snapshot_configuration(
        &self,
    ) -> terrars
    ::ListRef<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationElSnapshotConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<bool>>,
    launch_template_id: terrars::PrimField<std::string::String>,
}

impl ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
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
}

impl terrars::ToListMappable for ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    type O =
        terrars::BlockAssignable<ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    #[doc = ""]
    pub launch_template_id: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
        ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl {
            account_id: core::default::Default::default(),
            default: core::default::Default::default(),
            launch_template_id: self.launch_template_id,
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
        ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationElRef {
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

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDistributionElDynamic {
    ami_distribution_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>,
    >,
    container_distribution_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl>,
    >,
    fast_launch_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>>,
    launch_template_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct ImagebuilderDistributionConfigurationDistributionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_configuration_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ami_distribution_configuration: std
    ::option
    ::Option<std::vec::Vec<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_distribution_configuration: std
    ::option
    ::Option<std::vec::Vec<ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_launch_configuration: std
    ::option
    ::Option<std::vec::Vec<ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template_configuration: std
    ::option
    ::Option<std::vec::Vec<ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl>>,
    dynamic: ImagebuilderDistributionConfigurationDistributionElDynamic,
}

impl ImagebuilderDistributionConfigurationDistributionEl {
    #[doc = "Set the field `license_configuration_arns`.\n"]
    pub fn set_license_configuration_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.license_configuration_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ami_distribution_configuration`.\n"]
    pub fn set_ami_distribution_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ami_distribution_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ami_distribution_configuration = std::option::Option::Some(d);
            },
        }
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
                        ::BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_distribution_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_distribution_configuration = std::option::Option::Some(d);
            },
        }
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
                        ::BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElFastLaunchConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fast_launch_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fast_launch_configuration = std::option::Option::Some(d);
            },
        }
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
                        ::BlockAssignable<
                            ImagebuilderDistributionConfigurationDistributionElLaunchTemplateConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_template_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_template_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ImagebuilderDistributionConfigurationDistributionEl {
    type O = terrars::BlockAssignable<ImagebuilderDistributionConfigurationDistributionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildImagebuilderDistributionConfigurationDistributionEl {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildImagebuilderDistributionConfigurationDistributionEl {
    pub fn build(self) -> ImagebuilderDistributionConfigurationDistributionEl {
        ImagebuilderDistributionConfigurationDistributionEl {
            license_configuration_arns: core::default::Default::default(),
            region: self.region,
            ami_distribution_configuration: core::default::Default::default(),
            container_distribution_configuration: core::default::Default::default(),
            fast_launch_configuration: core::default::Default::default(),
            launch_template_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ImagebuilderDistributionConfigurationDistributionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ImagebuilderDistributionConfigurationDistributionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ImagebuilderDistributionConfigurationDistributionElRef {
        ImagebuilderDistributionConfigurationDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ImagebuilderDistributionConfigurationDistributionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `license_configuration_arns` after provisioning.\n"]
    pub fn license_configuration_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.license_configuration_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `ami_distribution_configuration` after provisioning.\n"]
    pub fn ami_distribution_configuration(
        &self,
    ) -> terrars::ListRef<ImagebuilderDistributionConfigurationDistributionElAmiDistributionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ami_distribution_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `container_distribution_configuration` after provisioning.\n"]
    pub fn container_distribution_configuration(
        &self,
    ) -> terrars::ListRef<ImagebuilderDistributionConfigurationDistributionElContainerDistributionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_distribution_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ImagebuilderDistributionConfigurationDynamic {
    distribution: std::option::Option<terrars::DynamicBlock<ImagebuilderDistributionConfigurationDistributionEl>>,
}

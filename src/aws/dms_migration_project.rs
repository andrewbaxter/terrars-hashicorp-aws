#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DmsMigrationProjectData_ {
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
    instance_profile_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_rules: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_conversion_application_attributes: std
    ::option
    ::Option<std::vec::Vec<DmsMigrationProject_SchemaConversionApplicationAttributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_data_provider_descriptor: std::option::Option<std::vec::Vec<DmsMigrationProject_SourceDataProviderDescriptor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_data_provider_descriptor: std::option::Option<std::vec::Vec<DmsMigrationProject_TargetDataProviderDescriptor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DmsMigrationProject_Timeouts>,
    dynamic: DmsMigrationProjectDynamic,
}

struct DmsMigrationProject_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DmsMigrationProjectData_>,
}

#[derive(Clone)]
pub struct DmsMigrationProject(std::rc::Rc<DmsMigrationProject_>);

impl DmsMigrationProject {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `transformation_rules`.\n"]
    pub fn set_transformation_rules(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().transformation_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema_conversion_application_attributes`.\n"]
    pub fn set_schema_conversion_application_attributes(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DmsMigrationProject_SchemaConversionApplicationAttributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema_conversion_application_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema_conversion_application_attributes =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_data_provider_descriptor`.\n"]
    pub fn set_source_data_provider_descriptor(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsMigrationProject_SourceDataProviderDescriptor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_data_provider_descriptor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_data_provider_descriptor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_data_provider_descriptor`.\n"]
    pub fn set_target_data_provider_descriptor(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DmsMigrationProject_TargetDataProviderDescriptor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_data_provider_descriptor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_data_provider_descriptor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DmsMigrationProject_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_profile_arn` after provisioning.\n"]
    pub fn instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_profile_name` after provisioning.\n"]
    pub fn instance_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transformation_rules` after provisioning.\n"]
    pub fn transformation_rules(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transformation_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_conversion_application_attributes` after provisioning.\n"]
    pub fn schema_conversion_application_attributes(
        &self,
    ) -> terrars::ListRef<DmsMigrationProject_SchemaConversionApplicationAttributesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.schema_conversion_application_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_data_provider_descriptor` after provisioning.\n"]
    pub fn source_data_provider_descriptor(
        &self,
    ) -> terrars::ListRef<DmsMigrationProject_SourceDataProviderDescriptorRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.source_data_provider_descriptor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_data_provider_descriptor` after provisioning.\n"]
    pub fn target_data_provider_descriptor(
        &self,
    ) -> terrars::ListRef<DmsMigrationProject_TargetDataProviderDescriptorRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.target_data_provider_descriptor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsMigrationProject_TimeoutsRef {
        DmsMigrationProject_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DmsMigrationProject {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DmsMigrationProject { }

impl terrars::ToListMappable for DmsMigrationProject {
    type O = terrars::ListRef<DmsMigrationProjectRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DmsMigrationProject_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dms_migration_project".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDmsMigrationProject {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_profile_arn: terrars::PrimField<std::string::String>,
}

impl BuildDmsMigrationProject {
    pub fn build(self, stack: &mut terrars::Stack) -> DmsMigrationProject {
        let out = DmsMigrationProject(std::rc::Rc::new(DmsMigrationProject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DmsMigrationProjectData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                instance_profile_arn: self.instance_profile_arn,
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                transformation_rules: core::default::Default::default(),
                schema_conversion_application_attributes: core::default::Default::default(),
                source_data_provider_descriptor: core::default::Default::default(),
                target_data_provider_descriptor: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DmsMigrationProjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsMigrationProjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DmsMigrationProjectRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_profile_arn` after provisioning.\n"]
    pub fn instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_profile_name` after provisioning.\n"]
    pub fn instance_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transformation_rules` after provisioning.\n"]
    pub fn transformation_rules(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transformation_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_conversion_application_attributes` after provisioning.\n"]
    pub fn schema_conversion_application_attributes(
        &self,
    ) -> terrars::ListRef<DmsMigrationProject_SchemaConversionApplicationAttributesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.schema_conversion_application_attributes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_data_provider_descriptor` after provisioning.\n"]
    pub fn source_data_provider_descriptor(
        &self,
    ) -> terrars::ListRef<DmsMigrationProject_SourceDataProviderDescriptorRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.source_data_provider_descriptor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `target_data_provider_descriptor` after provisioning.\n"]
    pub fn target_data_provider_descriptor(
        &self,
    ) -> terrars::ListRef<DmsMigrationProject_TargetDataProviderDescriptorRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.target_data_provider_descriptor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsMigrationProject_TimeoutsRef {
        DmsMigrationProject_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DmsMigrationProjectDynamic {
    schema_conversion_application_attributes: std
    ::option
    ::Option<terrars::DynamicBlock<DmsMigrationProject_SchemaConversionApplicationAttributes>>,
    source_data_provider_descriptor: std
    ::option
    ::Option<terrars::DynamicBlock<DmsMigrationProject_SourceDataProviderDescriptor>>,
    target_data_provider_descriptor: std
    ::option
    ::Option<terrars::DynamicBlock<DmsMigrationProject_TargetDataProviderDescriptor>>,
}

#[derive(Serialize)]
pub struct DmsMigrationProject_SchemaConversionApplicationAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsMigrationProject_SchemaConversionApplicationAttributes {
    #[doc = "Set the field `s3_bucket_path`.\n"]
    pub fn set_s3_bucket_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket_role_arn`.\n"]
    pub fn set_s3_bucket_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_bucket_role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsMigrationProject_SchemaConversionApplicationAttributes {
    type O = terrars::BlockAssignable<DmsMigrationProject_SchemaConversionApplicationAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsMigrationProject_SchemaConversionApplicationAttributes {}

impl BuildDmsMigrationProject_SchemaConversionApplicationAttributes {
    pub fn build(self) -> DmsMigrationProject_SchemaConversionApplicationAttributes {
        DmsMigrationProject_SchemaConversionApplicationAttributes {
            s3_bucket_path: core::default::Default::default(),
            s3_bucket_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsMigrationProject_SchemaConversionApplicationAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsMigrationProject_SchemaConversionApplicationAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DmsMigrationProject_SchemaConversionApplicationAttributesRef {
        DmsMigrationProject_SchemaConversionApplicationAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsMigrationProject_SchemaConversionApplicationAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_path` after provisioning.\n"]
    pub fn s3_bucket_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_path", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_role_arn` after provisioning.\n"]
    pub fn s3_bucket_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsMigrationProject_SourceDataProviderDescriptor {
    data_provider_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsMigrationProject_SourceDataProviderDescriptor {
    #[doc = "Set the field `secrets_manager_access_role_arn`.\n"]
    pub fn set_secrets_manager_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_secret_id`.\n"]
    pub fn set_secrets_manager_secret_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_secret_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsMigrationProject_SourceDataProviderDescriptor {
    type O = terrars::BlockAssignable<DmsMigrationProject_SourceDataProviderDescriptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsMigrationProject_SourceDataProviderDescriptor {
    #[doc = ""]
    pub data_provider_arn: terrars::PrimField<std::string::String>,
}

impl BuildDmsMigrationProject_SourceDataProviderDescriptor {
    pub fn build(self) -> DmsMigrationProject_SourceDataProviderDescriptor {
        DmsMigrationProject_SourceDataProviderDescriptor {
            data_provider_arn: self.data_provider_arn,
            secrets_manager_access_role_arn: core::default::Default::default(),
            secrets_manager_secret_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsMigrationProject_SourceDataProviderDescriptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsMigrationProject_SourceDataProviderDescriptorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DmsMigrationProject_SourceDataProviderDescriptorRef {
        DmsMigrationProject_SourceDataProviderDescriptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsMigrationProject_SourceDataProviderDescriptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_provider_arn` after provisioning.\n"]
    pub fn data_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `data_provider_name` after provisioning.\n"]
    pub fn data_provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_provider_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_secret_id` after provisioning.\n"]
    pub fn secrets_manager_secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_secret_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsMigrationProject_TargetDataProviderDescriptor {
    data_provider_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager_secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsMigrationProject_TargetDataProviderDescriptor {
    #[doc = "Set the field `secrets_manager_access_role_arn`.\n"]
    pub fn set_secrets_manager_access_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_secret_id`.\n"]
    pub fn set_secrets_manager_secret_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secrets_manager_secret_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsMigrationProject_TargetDataProviderDescriptor {
    type O = terrars::BlockAssignable<DmsMigrationProject_TargetDataProviderDescriptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsMigrationProject_TargetDataProviderDescriptor {
    #[doc = ""]
    pub data_provider_arn: terrars::PrimField<std::string::String>,
}

impl BuildDmsMigrationProject_TargetDataProviderDescriptor {
    pub fn build(self) -> DmsMigrationProject_TargetDataProviderDescriptor {
        DmsMigrationProject_TargetDataProviderDescriptor {
            data_provider_arn: self.data_provider_arn,
            secrets_manager_access_role_arn: core::default::Default::default(),
            secrets_manager_secret_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DmsMigrationProject_TargetDataProviderDescriptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsMigrationProject_TargetDataProviderDescriptorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DmsMigrationProject_TargetDataProviderDescriptorRef {
        DmsMigrationProject_TargetDataProviderDescriptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsMigrationProject_TargetDataProviderDescriptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_provider_arn` after provisioning.\n"]
    pub fn data_provider_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `data_provider_name` after provisioning.\n"]
    pub fn data_provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_provider_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_access_role_arn` after provisioning.\n"]
    pub fn secrets_manager_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_access_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_secret_id` after provisioning.\n"]
    pub fn secrets_manager_secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secrets_manager_secret_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DmsMigrationProject_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DmsMigrationProject_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DmsMigrationProject_Timeouts {
    type O = terrars::BlockAssignable<DmsMigrationProject_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsMigrationProject_Timeouts {}

impl BuildDmsMigrationProject_Timeouts {
    pub fn build(self) -> DmsMigrationProject_Timeouts {
        DmsMigrationProject_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DmsMigrationProject_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsMigrationProject_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DmsMigrationProject_TimeoutsRef {
        DmsMigrationProject_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsMigrationProject_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppstreamImageData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    most_recent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAppstreamImage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAppstreamImageData_>,
}

#[derive(Clone)]
pub struct DataAppstreamImage(std::rc::Rc<DataAppstreamImage_>);

impl DataAppstreamImage {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `most_recent`.\n"]
    pub fn set_most_recent(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().most_recent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_regex`.\n"]
    pub fn set_name_regex(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `applications` after provisioning.\n"]
    pub fn applications(&self) -> terrars::ListRef<DataAppstreamImage_ApplicationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.applications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `appstream_agent_version` after provisioning.\n"]
    pub fn appstream_agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.appstream_agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image_arn` after provisioning.\n"]
    pub fn base_image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_builder_name` after provisioning.\n"]
    pub fn image_builder_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_builder_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_builder_supported` after provisioning.\n"]
    pub fn image_builder_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_builder_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_permissions` after provisioning.\n"]
    pub fn image_permissions(&self) -> terrars::ListRef<DataAppstreamImage_ImagePermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_base_image_released_date` after provisioning.\n"]
    pub fn public_base_image_released_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_base_image_released_date", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_change_reason` after provisioning.\n"]
    pub fn state_change_reason(&self) -> terrars::ListRef<DataAppstreamImage_StateChangeReasonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_change_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataAppstreamImage {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAppstreamImage { }

impl terrars::ToListMappable for DataAppstreamImage {
    type O = terrars::ListRef<DataAppstreamImageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAppstreamImage_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_appstream_image".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppstreamImage {
    pub tf_id: std::string::String,
}

impl BuildDataAppstreamImage {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAppstreamImage {
        let out = DataAppstreamImage(std::rc::Rc::new(DataAppstreamImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAppstreamImageData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                most_recent: core::default::Default::default(),
                name: core::default::Default::default(),
                name_regex: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppstreamImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppstreamImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppstreamImageRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `applications` after provisioning.\n"]
    pub fn applications(&self) -> terrars::ListRef<DataAppstreamImage_ApplicationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.applications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `appstream_agent_version` after provisioning.\n"]
    pub fn appstream_agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.appstream_agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base_image_arn` after provisioning.\n"]
    pub fn base_image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_builder_name` after provisioning.\n"]
    pub fn image_builder_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_builder_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_builder_supported` after provisioning.\n"]
    pub fn image_builder_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_builder_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_permissions` after provisioning.\n"]
    pub fn image_permissions(&self) -> terrars::ListRef<DataAppstreamImage_ImagePermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_permissions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_base_image_released_date` after provisioning.\n"]
    pub fn public_base_image_released_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_base_image_released_date", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_change_reason` after provisioning.\n"]
    pub fn state_change_reason(&self) -> terrars::ListRef<DataAppstreamImage_StateChangeReasonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_change_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppstreamImage_Applications_IconS3Location {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppstreamImage_Applications_IconS3Location {
    #[doc = "Set the field `s3_bucket`.\n"]
    pub fn set_s3_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key`.\n"]
    pub fn set_s3_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppstreamImage_Applications_IconS3Location {
    type O = terrars::BlockAssignable<DataAppstreamImage_Applications_IconS3Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppstreamImage_Applications_IconS3Location {}

impl BuildDataAppstreamImage_Applications_IconS3Location {
    pub fn build(self) -> DataAppstreamImage_Applications_IconS3Location {
        DataAppstreamImage_Applications_IconS3Location {
            s3_bucket: core::default::Default::default(),
            s3_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppstreamImage_Applications_IconS3LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppstreamImage_Applications_IconS3LocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppstreamImage_Applications_IconS3LocationRef {
        DataAppstreamImage_Applications_IconS3LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppstreamImage_Applications_IconS3LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key` after provisioning.\n"]
    pub fn s3_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppstreamImage_Applications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_block_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icon_s3_location: std::option::Option<terrars::ListField<DataAppstreamImage_Applications_IconS3Location>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icon_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_families: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platforms: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_directory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppstreamImage_Applications {
    #[doc = "Set the field `app_block_arn`.\n"]
    pub fn set_app_block_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.app_block_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_time`.\n"]
    pub fn set_created_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_time = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icon_s3_location`.\n"]
    pub fn set_icon_s3_location(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppstreamImage_Applications_IconS3Location>>,
    ) -> Self {
        self.icon_s3_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icon_url`.\n"]
    pub fn set_icon_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.icon_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_families`.\n"]
    pub fn set_instance_families(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_families = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_parameters`.\n"]
    pub fn set_launch_parameters(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.launch_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_path`.\n"]
    pub fn set_launch_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.launch_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platforms`.\n"]
    pub fn set_platforms(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.platforms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `working_directory`.\n"]
    pub fn set_working_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.working_directory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppstreamImage_Applications {
    type O = terrars::BlockAssignable<DataAppstreamImage_Applications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppstreamImage_Applications {}

impl BuildDataAppstreamImage_Applications {
    pub fn build(self) -> DataAppstreamImage_Applications {
        DataAppstreamImage_Applications {
            app_block_arn: core::default::Default::default(),
            arn: core::default::Default::default(),
            created_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
            icon_s3_location: core::default::Default::default(),
            icon_url: core::default::Default::default(),
            instance_families: core::default::Default::default(),
            launch_parameters: core::default::Default::default(),
            launch_path: core::default::Default::default(),
            metadata: core::default::Default::default(),
            name: core::default::Default::default(),
            platforms: core::default::Default::default(),
            working_directory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppstreamImage_ApplicationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppstreamImage_ApplicationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppstreamImage_ApplicationsRef {
        DataAppstreamImage_ApplicationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppstreamImage_ApplicationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_block_arn` after provisioning.\n"]
    pub fn app_block_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_block_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `icon_s3_location` after provisioning.\n"]
    pub fn icon_s3_location(&self) -> terrars::ListRef<DataAppstreamImage_Applications_IconS3LocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.icon_s3_location", self.base))
    }

    #[doc = "Get a reference to the value of field `icon_url` after provisioning.\n"]
    pub fn icon_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icon_url", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_families` after provisioning.\n"]
    pub fn instance_families(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_families", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_parameters` after provisioning.\n"]
    pub fn launch_parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_path` after provisioning.\n"]
    pub fn launch_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_path", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `platforms` after provisioning.\n"]
    pub fn platforms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.platforms", self.base))
    }

    #[doc = "Get a reference to the value of field `working_directory` after provisioning.\n"]
    pub fn working_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppstreamImage_ImagePermissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_fleet: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_image_builder: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAppstreamImage_ImagePermissions {
    #[doc = "Set the field `allow_fleet`.\n"]
    pub fn set_allow_fleet(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_fleet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_image_builder`.\n"]
    pub fn set_allow_image_builder(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_image_builder = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppstreamImage_ImagePermissions {
    type O = terrars::BlockAssignable<DataAppstreamImage_ImagePermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppstreamImage_ImagePermissions {}

impl BuildDataAppstreamImage_ImagePermissions {
    pub fn build(self) -> DataAppstreamImage_ImagePermissions {
        DataAppstreamImage_ImagePermissions {
            allow_fleet: core::default::Default::default(),
            allow_image_builder: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppstreamImage_ImagePermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppstreamImage_ImagePermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppstreamImage_ImagePermissionsRef {
        DataAppstreamImage_ImagePermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppstreamImage_ImagePermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_fleet` after provisioning.\n"]
    pub fn allow_fleet(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_fleet", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_image_builder` after provisioning.\n"]
    pub fn allow_image_builder(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_image_builder", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppstreamImage_StateChangeReason {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppstreamImage_StateChangeReason {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppstreamImage_StateChangeReason {
    type O = terrars::BlockAssignable<DataAppstreamImage_StateChangeReason>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppstreamImage_StateChangeReason {}

impl BuildDataAppstreamImage_StateChangeReason {
    pub fn build(self) -> DataAppstreamImage_StateChangeReason {
        DataAppstreamImage_StateChangeReason {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppstreamImage_StateChangeReasonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppstreamImage_StateChangeReasonRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppstreamImage_StateChangeReasonRef {
        DataAppstreamImage_StateChangeReasonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppstreamImage_StateChangeReasonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

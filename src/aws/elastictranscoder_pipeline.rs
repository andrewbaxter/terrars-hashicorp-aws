use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ElastictranscoderPipelineData {
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
    aws_kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    input_bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_config: std::option::Option<std::vec::Vec<ElastictranscoderPipelineContentConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_config_permissions: std::option::Option<std::vec::Vec<ElastictranscoderPipelineContentConfigPermissionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notifications: std::option::Option<std::vec::Vec<ElastictranscoderPipelineNotificationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thumbnail_config: std::option::Option<std::vec::Vec<ElastictranscoderPipelineThumbnailConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thumbnail_config_permissions: std
    ::option
    ::Option<std::vec::Vec<ElastictranscoderPipelineThumbnailConfigPermissionsEl>>,
    dynamic: ElastictranscoderPipelineDynamic,
}

struct ElastictranscoderPipeline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ElastictranscoderPipelineData>,
}

#[derive(Clone)]
pub struct ElastictranscoderPipeline(std::rc::Rc<ElastictranscoderPipeline_>);

impl ElastictranscoderPipeline {
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

    #[doc = "Set the field `aws_kms_key_arn`.\n"]
    pub fn set_aws_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aws_kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_bucket`.\n"]
    pub fn set_output_bucket(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().output_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_config`.\n"]
    pub fn set_content_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPipelineContentConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `content_config_permissions`.\n"]
    pub fn set_content_config_permissions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPipelineContentConfigPermissionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_config_permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_config_permissions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notifications`.\n"]
    pub fn set_notifications(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPipelineNotificationsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notifications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `thumbnail_config`.\n"]
    pub fn set_thumbnail_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPipelineThumbnailConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thumbnail_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.thumbnail_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `thumbnail_config_permissions`.\n"]
    pub fn set_thumbnail_config_permissions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ElastictranscoderPipelineThumbnailConfigPermissionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().thumbnail_config_permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.thumbnail_config_permissions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_kms_key_arn` after provisioning.\n"]
    pub fn aws_kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_bucket` after provisioning.\n"]
    pub fn input_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_bucket` after provisioning.\n"]
    pub fn output_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_config` after provisioning.\n"]
    pub fn content_config(&self) -> terrars::ListRef<ElastictranscoderPipelineContentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notifications` after provisioning.\n"]
    pub fn notifications(&self) -> terrars::ListRef<ElastictranscoderPipelineNotificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thumbnail_config` after provisioning.\n"]
    pub fn thumbnail_config(&self) -> terrars::ListRef<ElastictranscoderPipelineThumbnailConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thumbnail_config", self.extract_ref()))
    }
}

impl terrars::Referable for ElastictranscoderPipeline {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ElastictranscoderPipeline { }

impl terrars::ToListMappable for ElastictranscoderPipeline {
    type O = terrars::ListRef<ElastictranscoderPipelineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ElastictranscoderPipeline_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_elastictranscoder_pipeline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildElastictranscoderPipeline {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub input_bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role: terrars::PrimField<std::string::String>,
}

impl BuildElastictranscoderPipeline {
    pub fn build(self, stack: &mut terrars::Stack) -> ElastictranscoderPipeline {
        let out = ElastictranscoderPipeline(std::rc::Rc::new(ElastictranscoderPipeline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ElastictranscoderPipelineData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aws_kms_key_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                input_bucket: self.input_bucket,
                name: core::default::Default::default(),
                output_bucket: core::default::Default::default(),
                role: self.role,
                content_config: core::default::Default::default(),
                content_config_permissions: core::default::Default::default(),
                notifications: core::default::Default::default(),
                thumbnail_config: core::default::Default::default(),
                thumbnail_config_permissions: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ElastictranscoderPipelineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPipelineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ElastictranscoderPipelineRef {
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

    #[doc = "Get a reference to the value of field `aws_kms_key_arn` after provisioning.\n"]
    pub fn aws_kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_bucket` after provisioning.\n"]
    pub fn input_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_bucket` after provisioning.\n"]
    pub fn output_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_config` after provisioning.\n"]
    pub fn content_config(&self) -> terrars::ListRef<ElastictranscoderPipelineContentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notifications` after provisioning.\n"]
    pub fn notifications(&self) -> terrars::ListRef<ElastictranscoderPipelineNotificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thumbnail_config` after provisioning.\n"]
    pub fn thumbnail_config(&self) -> terrars::ListRef<ElastictranscoderPipelineThumbnailConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thumbnail_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ElastictranscoderPipelineContentConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPipelineContentConfigEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPipelineContentConfigEl {
    type O = terrars::BlockAssignable<ElastictranscoderPipelineContentConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPipelineContentConfigEl {}

impl BuildElastictranscoderPipelineContentConfigEl {
    pub fn build(self) -> ElastictranscoderPipelineContentConfigEl {
        ElastictranscoderPipelineContentConfigEl {
            bucket: core::default::Default::default(),
            storage_class: core::default::Default::default(),
        }
    }
}

pub struct ElastictranscoderPipelineContentConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPipelineContentConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElastictranscoderPipelineContentConfigElRef {
        ElastictranscoderPipelineContentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPipelineContentConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct ElastictranscoderPipelineContentConfigPermissionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grantee: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grantee_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPipelineContentConfigPermissionsEl {
    #[doc = "Set the field `access`.\n"]
    pub fn set_access(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grantee`.\n"]
    pub fn set_grantee(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grantee = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grantee_type`.\n"]
    pub fn set_grantee_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grantee_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPipelineContentConfigPermissionsEl {
    type O = terrars::BlockAssignable<ElastictranscoderPipelineContentConfigPermissionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPipelineContentConfigPermissionsEl {}

impl BuildElastictranscoderPipelineContentConfigPermissionsEl {
    pub fn build(self) -> ElastictranscoderPipelineContentConfigPermissionsEl {
        ElastictranscoderPipelineContentConfigPermissionsEl {
            access: core::default::Default::default(),
            grantee: core::default::Default::default(),
            grantee_type: core::default::Default::default(),
        }
    }
}

pub struct ElastictranscoderPipelineContentConfigPermissionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPipelineContentConfigPermissionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElastictranscoderPipelineContentConfigPermissionsElRef {
        ElastictranscoderPipelineContentConfigPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPipelineContentConfigPermissionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access` after provisioning.\n"]
    pub fn access(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access", self.base))
    }

    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grantee", self.base))
    }

    #[doc = "Get a reference to the value of field `grantee_type` after provisioning.\n"]
    pub fn grantee_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grantee_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ElastictranscoderPipelineNotificationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completed: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    progressing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warning: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPipelineNotificationsEl {
    #[doc = "Set the field `completed`.\n"]
    pub fn set_completed(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.completed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error`.\n"]
    pub fn set_error(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `progressing`.\n"]
    pub fn set_progressing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.progressing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warning`.\n"]
    pub fn set_warning(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.warning = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPipelineNotificationsEl {
    type O = terrars::BlockAssignable<ElastictranscoderPipelineNotificationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPipelineNotificationsEl {}

impl BuildElastictranscoderPipelineNotificationsEl {
    pub fn build(self) -> ElastictranscoderPipelineNotificationsEl {
        ElastictranscoderPipelineNotificationsEl {
            completed: core::default::Default::default(),
            error: core::default::Default::default(),
            progressing: core::default::Default::default(),
            warning: core::default::Default::default(),
        }
    }
}

pub struct ElastictranscoderPipelineNotificationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPipelineNotificationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElastictranscoderPipelineNotificationsElRef {
        ElastictranscoderPipelineNotificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPipelineNotificationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completed` after provisioning.\n"]
    pub fn completed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `progressing` after provisioning.\n"]
    pub fn progressing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.progressing", self.base))
    }

    #[doc = "Get a reference to the value of field `warning` after provisioning.\n"]
    pub fn warning(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning", self.base))
    }
}

#[derive(Serialize)]
pub struct ElastictranscoderPipelineThumbnailConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPipelineThumbnailConfigEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPipelineThumbnailConfigEl {
    type O = terrars::BlockAssignable<ElastictranscoderPipelineThumbnailConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPipelineThumbnailConfigEl {}

impl BuildElastictranscoderPipelineThumbnailConfigEl {
    pub fn build(self) -> ElastictranscoderPipelineThumbnailConfigEl {
        ElastictranscoderPipelineThumbnailConfigEl {
            bucket: core::default::Default::default(),
            storage_class: core::default::Default::default(),
        }
    }
}

pub struct ElastictranscoderPipelineThumbnailConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPipelineThumbnailConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ElastictranscoderPipelineThumbnailConfigElRef {
        ElastictranscoderPipelineThumbnailConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPipelineThumbnailConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct ElastictranscoderPipelineThumbnailConfigPermissionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grantee: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grantee_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ElastictranscoderPipelineThumbnailConfigPermissionsEl {
    #[doc = "Set the field `access`.\n"]
    pub fn set_access(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grantee`.\n"]
    pub fn set_grantee(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grantee = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grantee_type`.\n"]
    pub fn set_grantee_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grantee_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ElastictranscoderPipelineThumbnailConfigPermissionsEl {
    type O = terrars::BlockAssignable<ElastictranscoderPipelineThumbnailConfigPermissionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildElastictranscoderPipelineThumbnailConfigPermissionsEl {}

impl BuildElastictranscoderPipelineThumbnailConfigPermissionsEl {
    pub fn build(self) -> ElastictranscoderPipelineThumbnailConfigPermissionsEl {
        ElastictranscoderPipelineThumbnailConfigPermissionsEl {
            access: core::default::Default::default(),
            grantee: core::default::Default::default(),
            grantee_type: core::default::Default::default(),
        }
    }
}

pub struct ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
        ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ElastictranscoderPipelineThumbnailConfigPermissionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access` after provisioning.\n"]
    pub fn access(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access", self.base))
    }

    #[doc = "Get a reference to the value of field `grantee` after provisioning.\n"]
    pub fn grantee(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grantee", self.base))
    }

    #[doc = "Get a reference to the value of field `grantee_type` after provisioning.\n"]
    pub fn grantee_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grantee_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ElastictranscoderPipelineDynamic {
    content_config: std::option::Option<terrars::DynamicBlock<ElastictranscoderPipelineContentConfigEl>>,
    content_config_permissions: std
    ::option
    ::Option<terrars::DynamicBlock<ElastictranscoderPipelineContentConfigPermissionsEl>>,
    notifications: std::option::Option<terrars::DynamicBlock<ElastictranscoderPipelineNotificationsEl>>,
    thumbnail_config: std::option::Option<terrars::DynamicBlock<ElastictranscoderPipelineThumbnailConfigEl>>,
    thumbnail_config_permissions: std
    ::option
    ::Option<terrars::DynamicBlock<ElastictranscoderPipelineThumbnailConfigPermissionsEl>>,
}

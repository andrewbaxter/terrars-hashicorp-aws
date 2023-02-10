use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatasyncLocationS3Data {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_arns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    s3_bucket_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_storage_class: Option<PrimField<String>>,
    subdirectory: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_config: Option<Vec<DatasyncLocationS3S3ConfigEl>>,
    dynamic: DatasyncLocationS3Dynamic,
}

struct DatasyncLocationS3_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatasyncLocationS3Data>,
}

#[derive(Clone)]
pub struct DatasyncLocationS3(Rc<DatasyncLocationS3_>);

impl DatasyncLocationS3 {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `agent_arns`.\n"]
    pub fn set_agent_arns(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().agent_arns = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_storage_class`.\n"]
    pub fn set_s3_storage_class(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().s3_storage_class = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `s3_config`.\n"]
    pub fn set_s3_config(self, v: impl Into<BlockAssignable<DatasyncLocationS3S3ConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_config = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.agent_arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_storage_class` after provisioning.\n"]
    pub fn s3_storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_storage_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(&self) -> ListRef<DatasyncLocationS3S3ConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_config", self.extract_ref()))
    }
}

impl Referable for DatasyncLocationS3 {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DatasyncLocationS3 { }

impl ToListMappable for DatasyncLocationS3 {
    type O = ListRef<DatasyncLocationS3Ref>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatasyncLocationS3_ {
    fn extract_resource_type(&self) -> String {
        "aws_datasync_location_s3".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncLocationS3 {
    pub tf_id: String,
    #[doc= ""]
    pub s3_bucket_arn: PrimField<String>,
    #[doc= ""]
    pub subdirectory: PrimField<String>,
}

impl BuildDatasyncLocationS3 {
    pub fn build(self, stack: &mut Stack) -> DatasyncLocationS3 {
        let out = DatasyncLocationS3(Rc::new(DatasyncLocationS3_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatasyncLocationS3Data {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                agent_arns: core::default::Default::default(),
                id: core::default::Default::default(),
                s3_bucket_arn: self.s3_bucket_arn,
                s3_storage_class: core::default::Default::default(),
                subdirectory: self.subdirectory,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                s3_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncLocationS3Ref {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationS3Ref {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatasyncLocationS3Ref {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `agent_arns` after provisioning.\n"]
    pub fn agent_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.agent_arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_storage_class` after provisioning.\n"]
    pub fn s3_storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_storage_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_config` after provisioning.\n"]
    pub fn s3_config(&self) -> ListRef<DatasyncLocationS3S3ConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationS3S3ConfigEl {
    bucket_access_role_arn: PrimField<String>,
}

impl DatasyncLocationS3S3ConfigEl { }

impl ToListMappable for DatasyncLocationS3S3ConfigEl {
    type O = BlockAssignable<DatasyncLocationS3S3ConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationS3S3ConfigEl {
    #[doc= ""]
    pub bucket_access_role_arn: PrimField<String>,
}

impl BuildDatasyncLocationS3S3ConfigEl {
    pub fn build(self) -> DatasyncLocationS3S3ConfigEl {
        DatasyncLocationS3S3ConfigEl { bucket_access_role_arn: self.bucket_access_role_arn }
    }
}

pub struct DatasyncLocationS3S3ConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncLocationS3S3ConfigElRef {
    fn new(shared: StackShared, base: String) -> DatasyncLocationS3S3ConfigElRef {
        DatasyncLocationS3S3ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationS3S3ConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_access_role_arn` after provisioning.\n"]
    pub fn bucket_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_access_role_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationS3Dynamic {
    s3_config: Option<DynamicBlock<DatasyncLocationS3S3ConfigEl>>,
}

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataKendraThesaurusData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    index_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    thesaurus_id: PrimField<String>,
}

struct DataKendraThesaurus_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKendraThesaurusData>,
}

#[derive(Clone)]
pub struct DataKendraThesaurus(Rc<DataKendraThesaurus_>);

impl DataKendraThesaurus {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_size_bytes` after provisioning.\n"]
    pub fn file_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_size_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_s3_path` after provisioning.\n"]
    pub fn source_s3_path(&self) -> ListRef<DataKendraThesaurusSourceS3PathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_s3_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `synonym_rule_count` after provisioning.\n"]
    pub fn synonym_rule_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.synonym_rule_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `term_count` after provisioning.\n"]
    pub fn term_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.term_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `thesaurus_id` after provisioning.\n"]
    pub fn thesaurus_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.thesaurus_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl Datasource for DataKendraThesaurus {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataKendraThesaurus {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataKendraThesaurus {
    type O = ListRef<DataKendraThesaurusRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataKendraThesaurus_ {
    fn extract_datasource_type(&self) -> String {
        "aws_kendra_thesaurus".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataKendraThesaurus {
    pub tf_id: String,
    #[doc= ""]
    pub index_id: PrimField<String>,
    #[doc= ""]
    pub thesaurus_id: PrimField<String>,
}

impl BuildDataKendraThesaurus {
    pub fn build(self, stack: &mut Stack) -> DataKendraThesaurus {
        let out = DataKendraThesaurus(Rc::new(DataKendraThesaurus_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKendraThesaurusData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                index_id: self.index_id,
                tags: core::default::Default::default(),
                thesaurus_id: self.thesaurus_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataKendraThesaurusRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraThesaurusRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataKendraThesaurusRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_size_bytes` after provisioning.\n"]
    pub fn file_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_size_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_s3_path` after provisioning.\n"]
    pub fn source_s3_path(&self) -> ListRef<DataKendraThesaurusSourceS3PathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_s3_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `synonym_rule_count` after provisioning.\n"]
    pub fn synonym_rule_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.synonym_rule_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `term_count` after provisioning.\n"]
    pub fn term_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.term_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `thesaurus_id` after provisioning.\n"]
    pub fn thesaurus_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.thesaurus_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataKendraThesaurusSourceS3PathEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
}

impl DataKendraThesaurusSourceS3PathEl {
    #[doc= "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
}

impl ToListMappable for DataKendraThesaurusSourceS3PathEl {
    type O = BlockAssignable<DataKendraThesaurusSourceS3PathEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataKendraThesaurusSourceS3PathEl {}

impl BuildDataKendraThesaurusSourceS3PathEl {
    pub fn build(self) -> DataKendraThesaurusSourceS3PathEl {
        DataKendraThesaurusSourceS3PathEl {
            bucket: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct DataKendraThesaurusSourceS3PathElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataKendraThesaurusSourceS3PathElRef {
    fn new(shared: StackShared, base: String) -> DataKendraThesaurusSourceS3PathElRef {
        DataKendraThesaurusSourceS3PathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataKendraThesaurusSourceS3PathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

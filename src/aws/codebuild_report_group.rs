use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodebuildReportGroupData {
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
    delete_reports: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_config: std::option::Option<std::vec::Vec<CodebuildReportGroupExportConfigEl>>,
    dynamic: CodebuildReportGroupDynamic,
}

struct CodebuildReportGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodebuildReportGroupData>,
}

#[derive(Clone)]
pub struct CodebuildReportGroup(std::rc::Rc<CodebuildReportGroup_>);

impl CodebuildReportGroup {
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

    #[doc = "Set the field `delete_reports`.\n"]
    pub fn set_delete_reports(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_reports = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `export_config`.\n"]
    pub fn set_export_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildReportGroupExportConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().export_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.export_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_reports` after provisioning.\n"]
    pub fn delete_reports(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_reports", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_config` after provisioning.\n"]
    pub fn export_config(&self) -> terrars::ListRef<CodebuildReportGroupExportConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export_config", self.extract_ref()))
    }
}

impl terrars::Referable for CodebuildReportGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodebuildReportGroup { }

impl terrars::ToListMappable for CodebuildReportGroup {
    type O = terrars::ListRef<CodebuildReportGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodebuildReportGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codebuild_report_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodebuildReportGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildReportGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> CodebuildReportGroup {
        let out = CodebuildReportGroup(std::rc::Rc::new(CodebuildReportGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodebuildReportGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                delete_reports: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                export_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodebuildReportGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildReportGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodebuildReportGroupRef {
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

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_reports` after provisioning.\n"]
    pub fn delete_reports(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_reports", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_config` after provisioning.\n"]
    pub fn export_config(&self) -> terrars::ListRef<CodebuildReportGroupExportConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodebuildReportGroupExportConfigElS3DestinationEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_disabled: std::option::Option<terrars::PrimField<bool>>,
    encryption_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packaging: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodebuildReportGroupExportConfigElS3DestinationEl {
    #[doc = "Set the field `encryption_disabled`.\n"]
    pub fn set_encryption_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encryption_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `packaging`.\n"]
    pub fn set_packaging(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.packaging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildReportGroupExportConfigElS3DestinationEl {
    type O = terrars::BlockAssignable<CodebuildReportGroupExportConfigElS3DestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildReportGroupExportConfigElS3DestinationEl {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub encryption_key: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildReportGroupExportConfigElS3DestinationEl {
    pub fn build(self) -> CodebuildReportGroupExportConfigElS3DestinationEl {
        CodebuildReportGroupExportConfigElS3DestinationEl {
            bucket: self.bucket,
            encryption_disabled: core::default::Default::default(),
            encryption_key: self.encryption_key,
            packaging: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

pub struct CodebuildReportGroupExportConfigElS3DestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildReportGroupExportConfigElS3DestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodebuildReportGroupExportConfigElS3DestinationElRef {
        CodebuildReportGroupExportConfigElS3DestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildReportGroupExportConfigElS3DestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_disabled` after provisioning.\n"]
    pub fn encryption_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `packaging` after provisioning.\n"]
    pub fn packaging(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.packaging", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildReportGroupExportConfigElDynamic {
    s3_destination: std::option::Option<terrars::DynamicBlock<CodebuildReportGroupExportConfigElS3DestinationEl>>,
}

#[derive(Serialize)]
pub struct CodebuildReportGroupExportConfigEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_destination: std::option::Option<std::vec::Vec<CodebuildReportGroupExportConfigElS3DestinationEl>>,
    dynamic: CodebuildReportGroupExportConfigElDynamic,
}

impl CodebuildReportGroupExportConfigEl {
    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildReportGroupExportConfigElS3DestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodebuildReportGroupExportConfigEl {
    type O = terrars::BlockAssignable<CodebuildReportGroupExportConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildReportGroupExportConfigEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildReportGroupExportConfigEl {
    pub fn build(self) -> CodebuildReportGroupExportConfigEl {
        CodebuildReportGroupExportConfigEl {
            type_: self.type_,
            s3_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodebuildReportGroupExportConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildReportGroupExportConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildReportGroupExportConfigElRef {
        CodebuildReportGroupExportConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildReportGroupExportConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> terrars::ListRef<CodebuildReportGroupExportConfigElS3DestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildReportGroupDynamic {
    export_config: std::option::Option<terrars::DynamicBlock<CodebuildReportGroupExportConfigEl>>,
}

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueClassifierData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_classifier: std::option::Option<std::vec::Vec<GlueClassifierCsvClassifierEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grok_classifier: std::option::Option<std::vec::Vec<GlueClassifierGrokClassifierEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_classifier: std::option::Option<std::vec::Vec<GlueClassifierJsonClassifierEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xml_classifier: std::option::Option<std::vec::Vec<GlueClassifierXmlClassifierEl>>,
    dynamic: GlueClassifierDynamic,
}

struct GlueClassifier_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueClassifierData>,
}

#[derive(Clone)]
pub struct GlueClassifier(std::rc::Rc<GlueClassifier_>);

impl GlueClassifier {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_classifier`.\n"]
    pub fn set_csv_classifier(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueClassifierCsvClassifierEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().csv_classifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.csv_classifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grok_classifier`.\n"]
    pub fn set_grok_classifier(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueClassifierGrokClassifierEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grok_classifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grok_classifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json_classifier`.\n"]
    pub fn set_json_classifier(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueClassifierJsonClassifierEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().json_classifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.json_classifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `xml_classifier`.\n"]
    pub fn set_xml_classifier(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueClassifierXmlClassifierEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().xml_classifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.xml_classifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_classifier` after provisioning.\n"]
    pub fn csv_classifier(&self) -> terrars::ListRef<GlueClassifierCsvClassifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grok_classifier` after provisioning.\n"]
    pub fn grok_classifier(&self) -> terrars::ListRef<GlueClassifierGrokClassifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grok_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json_classifier` after provisioning.\n"]
    pub fn json_classifier(&self) -> terrars::ListRef<GlueClassifierJsonClassifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xml_classifier` after provisioning.\n"]
    pub fn xml_classifier(&self) -> terrars::ListRef<GlueClassifierXmlClassifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xml_classifier", self.extract_ref()))
    }
}

impl terrars::Referable for GlueClassifier {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueClassifier { }

impl terrars::ToListMappable for GlueClassifier {
    type O = terrars::ListRef<GlueClassifierRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueClassifier_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_classifier".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueClassifier {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueClassifier {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueClassifier {
        let out = GlueClassifier(std::rc::Rc::new(GlueClassifier_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueClassifierData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                csv_classifier: core::default::Default::default(),
                grok_classifier: core::default::Default::default(),
                json_classifier: core::default::Default::default(),
                xml_classifier: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueClassifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueClassifierRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueClassifierRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_classifier` after provisioning.\n"]
    pub fn csv_classifier(&self) -> terrars::ListRef<GlueClassifierCsvClassifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grok_classifier` after provisioning.\n"]
    pub fn grok_classifier(&self) -> terrars::ListRef<GlueClassifierGrokClassifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grok_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json_classifier` after provisioning.\n"]
    pub fn json_classifier(&self) -> terrars::ListRef<GlueClassifierJsonClassifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xml_classifier` after provisioning.\n"]
    pub fn xml_classifier(&self) -> terrars::ListRef<GlueClassifierXmlClassifierElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xml_classifier", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GlueClassifierCsvClassifierEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_single_column: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_datatype_configured: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_datatypes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_value_trimming: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quote_symbol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueClassifierCsvClassifierEl {
    #[doc = "Set the field `allow_single_column`.\n"]
    pub fn set_allow_single_column(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_single_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contains_header`.\n"]
    pub fn set_contains_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_datatype_configured`.\n"]
    pub fn set_custom_datatype_configured(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.custom_datatype_configured = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_datatypes`.\n"]
    pub fn set_custom_datatypes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_datatypes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_value_trimming`.\n"]
    pub fn set_disable_value_trimming(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_value_trimming = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quote_symbol`.\n"]
    pub fn set_quote_symbol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.quote_symbol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueClassifierCsvClassifierEl {
    type O = terrars::BlockAssignable<GlueClassifierCsvClassifierEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueClassifierCsvClassifierEl {}

impl BuildGlueClassifierCsvClassifierEl {
    pub fn build(self) -> GlueClassifierCsvClassifierEl {
        GlueClassifierCsvClassifierEl {
            allow_single_column: core::default::Default::default(),
            contains_header: core::default::Default::default(),
            custom_datatype_configured: core::default::Default::default(),
            custom_datatypes: core::default::Default::default(),
            delimiter: core::default::Default::default(),
            disable_value_trimming: core::default::Default::default(),
            header: core::default::Default::default(),
            quote_symbol: core::default::Default::default(),
        }
    }
}

pub struct GlueClassifierCsvClassifierElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueClassifierCsvClassifierElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueClassifierCsvClassifierElRef {
        GlueClassifierCsvClassifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueClassifierCsvClassifierElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_single_column` after provisioning.\n"]
    pub fn allow_single_column(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_single_column", self.base))
    }

    #[doc = "Get a reference to the value of field `contains_header` after provisioning.\n"]
    pub fn contains_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains_header", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_datatype_configured` after provisioning.\n"]
    pub fn custom_datatype_configured(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_datatype_configured", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_datatypes` after provisioning.\n"]
    pub fn custom_datatypes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_datatypes", self.base))
    }

    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_value_trimming` after provisioning.\n"]
    pub fn disable_value_trimming(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_value_trimming", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `quote_symbol` after provisioning.\n"]
    pub fn quote_symbol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quote_symbol", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueClassifierGrokClassifierEl {
    classification: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_patterns: std::option::Option<terrars::PrimField<std::string::String>>,
    grok_pattern: terrars::PrimField<std::string::String>,
}

impl GlueClassifierGrokClassifierEl {
    #[doc = "Set the field `custom_patterns`.\n"]
    pub fn set_custom_patterns(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_patterns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueClassifierGrokClassifierEl {
    type O = terrars::BlockAssignable<GlueClassifierGrokClassifierEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueClassifierGrokClassifierEl {
    #[doc = ""]
    pub classification: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub grok_pattern: terrars::PrimField<std::string::String>,
}

impl BuildGlueClassifierGrokClassifierEl {
    pub fn build(self) -> GlueClassifierGrokClassifierEl {
        GlueClassifierGrokClassifierEl {
            classification: self.classification,
            custom_patterns: core::default::Default::default(),
            grok_pattern: self.grok_pattern,
        }
    }
}

pub struct GlueClassifierGrokClassifierElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueClassifierGrokClassifierElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueClassifierGrokClassifierElRef {
        GlueClassifierGrokClassifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueClassifierGrokClassifierElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_patterns` after provisioning.\n"]
    pub fn custom_patterns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_patterns", self.base))
    }

    #[doc = "Get a reference to the value of field `grok_pattern` after provisioning.\n"]
    pub fn grok_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grok_pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueClassifierJsonClassifierEl {
    json_path: terrars::PrimField<std::string::String>,
}

impl GlueClassifierJsonClassifierEl { }

impl terrars::ToListMappable for GlueClassifierJsonClassifierEl {
    type O = terrars::BlockAssignable<GlueClassifierJsonClassifierEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueClassifierJsonClassifierEl {
    #[doc = ""]
    pub json_path: terrars::PrimField<std::string::String>,
}

impl BuildGlueClassifierJsonClassifierEl {
    pub fn build(self) -> GlueClassifierJsonClassifierEl {
        GlueClassifierJsonClassifierEl { json_path: self.json_path }
    }
}

pub struct GlueClassifierJsonClassifierElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueClassifierJsonClassifierElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueClassifierJsonClassifierElRef {
        GlueClassifierJsonClassifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueClassifierJsonClassifierElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_path` after provisioning.\n"]
    pub fn json_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_path", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueClassifierXmlClassifierEl {
    classification: terrars::PrimField<std::string::String>,
    row_tag: terrars::PrimField<std::string::String>,
}

impl GlueClassifierXmlClassifierEl { }

impl terrars::ToListMappable for GlueClassifierXmlClassifierEl {
    type O = terrars::BlockAssignable<GlueClassifierXmlClassifierEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueClassifierXmlClassifierEl {
    #[doc = ""]
    pub classification: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub row_tag: terrars::PrimField<std::string::String>,
}

impl BuildGlueClassifierXmlClassifierEl {
    pub fn build(self) -> GlueClassifierXmlClassifierEl {
        GlueClassifierXmlClassifierEl {
            classification: self.classification,
            row_tag: self.row_tag,
        }
    }
}

pub struct GlueClassifierXmlClassifierElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueClassifierXmlClassifierElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueClassifierXmlClassifierElRef {
        GlueClassifierXmlClassifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueClassifierXmlClassifierElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `row_tag` after provisioning.\n"]
    pub fn row_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_tag", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueClassifierDynamic {
    csv_classifier: std::option::Option<terrars::DynamicBlock<GlueClassifierCsvClassifierEl>>,
    grok_classifier: std::option::Option<terrars::DynamicBlock<GlueClassifierGrokClassifierEl>>,
    json_classifier: std::option::Option<terrars::DynamicBlock<GlueClassifierJsonClassifierEl>>,
    xml_classifier: std::option::Option<terrars::DynamicBlock<GlueClassifierXmlClassifierEl>>,
}

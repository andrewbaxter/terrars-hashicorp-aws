#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueClassifierData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_classifier: std::option::Option<std::vec::Vec<GlueClassifier_CsvClassifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grok_classifier: std::option::Option<std::vec::Vec<GlueClassifier_GrokClassifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_classifier: std::option::Option<std::vec::Vec<GlueClassifier_JsonClassifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xml_classifier: std::option::Option<std::vec::Vec<GlueClassifier_XmlClassifier>>,
    dynamic: GlueClassifierDynamic,
}

struct GlueClassifier_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueClassifierData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_classifier`.\n"]
    pub fn set_csv_classifier(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueClassifier_CsvClassifier>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueClassifier_GrokClassifier>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueClassifier_JsonClassifier>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GlueClassifier_XmlClassifier>>,
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_classifier` after provisioning.\n"]
    pub fn csv_classifier(&self) -> terrars::ListRef<GlueClassifier_CsvClassifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grok_classifier` after provisioning.\n"]
    pub fn grok_classifier(&self) -> terrars::ListRef<GlueClassifier_GrokClassifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grok_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json_classifier` after provisioning.\n"]
    pub fn json_classifier(&self) -> terrars::ListRef<GlueClassifier_JsonClassifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xml_classifier` after provisioning.\n"]
    pub fn xml_classifier(&self) -> terrars::ListRef<GlueClassifier_XmlClassifierRef> {
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
            data: std::cell::RefCell::new(GlueClassifierData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_classifier` after provisioning.\n"]
    pub fn csv_classifier(&self) -> terrars::ListRef<GlueClassifier_CsvClassifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grok_classifier` after provisioning.\n"]
    pub fn grok_classifier(&self) -> terrars::ListRef<GlueClassifier_GrokClassifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grok_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `json_classifier` after provisioning.\n"]
    pub fn json_classifier(&self) -> terrars::ListRef<GlueClassifier_JsonClassifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_classifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xml_classifier` after provisioning.\n"]
    pub fn xml_classifier(&self) -> terrars::ListRef<GlueClassifier_XmlClassifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xml_classifier", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GlueClassifierDynamic {
    csv_classifier: std::option::Option<terrars::DynamicBlock<GlueClassifier_CsvClassifier>>,
    grok_classifier: std::option::Option<terrars::DynamicBlock<GlueClassifier_GrokClassifier>>,
    json_classifier: std::option::Option<terrars::DynamicBlock<GlueClassifier_JsonClassifier>>,
    xml_classifier: std::option::Option<terrars::DynamicBlock<GlueClassifier_XmlClassifier>>,
}

#[derive(Serialize)]
pub struct GlueClassifier_CsvClassifier {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serde: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueClassifier_CsvClassifier {
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

    #[doc = "Set the field `serde`.\n"]
    pub fn set_serde(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.serde = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueClassifier_CsvClassifier {
    type O = terrars::BlockAssignable<GlueClassifier_CsvClassifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueClassifier_CsvClassifier {}

impl BuildGlueClassifier_CsvClassifier {
    pub fn build(self) -> GlueClassifier_CsvClassifier {
        GlueClassifier_CsvClassifier {
            allow_single_column: core::default::Default::default(),
            contains_header: core::default::Default::default(),
            custom_datatype_configured: core::default::Default::default(),
            custom_datatypes: core::default::Default::default(),
            delimiter: core::default::Default::default(),
            disable_value_trimming: core::default::Default::default(),
            header: core::default::Default::default(),
            quote_symbol: core::default::Default::default(),
            serde: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueClassifier_CsvClassifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueClassifier_CsvClassifierRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueClassifier_CsvClassifierRef {
        GlueClassifier_CsvClassifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueClassifier_CsvClassifierRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `serde` after provisioning.\n"]
    pub fn serde(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serde", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueClassifier_GrokClassifier {
    classification: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_patterns: std::option::Option<terrars::PrimField<std::string::String>>,
    grok_pattern: terrars::PrimField<std::string::String>,
}

impl GlueClassifier_GrokClassifier {
    #[doc = "Set the field `custom_patterns`.\n"]
    pub fn set_custom_patterns(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_patterns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueClassifier_GrokClassifier {
    type O = terrars::BlockAssignable<GlueClassifier_GrokClassifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueClassifier_GrokClassifier {
    #[doc = ""]
    pub classification: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub grok_pattern: terrars::PrimField<std::string::String>,
}

impl BuildGlueClassifier_GrokClassifier {
    pub fn build(self) -> GlueClassifier_GrokClassifier {
        GlueClassifier_GrokClassifier {
            classification: self.classification,
            custom_patterns: core::default::Default::default(),
            grok_pattern: self.grok_pattern,
        }
    }
}

#[allow(dead_code)]
pub struct GlueClassifier_GrokClassifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueClassifier_GrokClassifierRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueClassifier_GrokClassifierRef {
        GlueClassifier_GrokClassifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueClassifier_GrokClassifierRef {
    #[allow(dead_code)]
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
pub struct GlueClassifier_JsonClassifier {
    json_path: terrars::PrimField<std::string::String>,
}

impl GlueClassifier_JsonClassifier { }

impl terrars::ToListMappable for GlueClassifier_JsonClassifier {
    type O = terrars::BlockAssignable<GlueClassifier_JsonClassifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueClassifier_JsonClassifier {
    #[doc = ""]
    pub json_path: terrars::PrimField<std::string::String>,
}

impl BuildGlueClassifier_JsonClassifier {
    pub fn build(self) -> GlueClassifier_JsonClassifier {
        GlueClassifier_JsonClassifier { json_path: self.json_path }
    }
}

#[allow(dead_code)]
pub struct GlueClassifier_JsonClassifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueClassifier_JsonClassifierRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueClassifier_JsonClassifierRef {
        GlueClassifier_JsonClassifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueClassifier_JsonClassifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_path` after provisioning.\n"]
    pub fn json_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_path", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueClassifier_XmlClassifier {
    classification: terrars::PrimField<std::string::String>,
    row_tag: terrars::PrimField<std::string::String>,
}

impl GlueClassifier_XmlClassifier { }

impl terrars::ToListMappable for GlueClassifier_XmlClassifier {
    type O = terrars::BlockAssignable<GlueClassifier_XmlClassifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueClassifier_XmlClassifier {
    #[doc = ""]
    pub classification: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub row_tag: terrars::PrimField<std::string::String>,
}

impl BuildGlueClassifier_XmlClassifier {
    pub fn build(self) -> GlueClassifier_XmlClassifier {
        GlueClassifier_XmlClassifier {
            classification: self.classification,
            row_tag: self.row_tag,
        }
    }
}

#[allow(dead_code)]
pub struct GlueClassifier_XmlClassifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueClassifier_XmlClassifierRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueClassifier_XmlClassifierRef {
        GlueClassifier_XmlClassifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueClassifier_XmlClassifierRef {
    #[allow(dead_code)]
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

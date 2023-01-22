use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSesActiveReceiptRuleSetData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataSesActiveReceiptRuleSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSesActiveReceiptRuleSetData>,
}

#[derive(Clone)]
pub struct DataSesActiveReceiptRuleSet(Rc<DataSesActiveReceiptRuleSet_>);

impl DataSesActiveReceiptRuleSet {
    fn shared(&self) -> &StackShared {
        &self.0.shared
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

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_set_name` after provisioning.\n"]
    pub fn rule_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_set_name", self.extract_ref()))
    }
}

impl Datasource for DataSesActiveReceiptRuleSet {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataSesActiveReceiptRuleSet {
    type O = ListRef<DataSesActiveReceiptRuleSetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSesActiveReceiptRuleSet_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ses_active_receipt_rule_set".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSesActiveReceiptRuleSet {
    pub tf_id: String,
}

impl BuildDataSesActiveReceiptRuleSet {
    pub fn build(self, stack: &mut Stack) -> DataSesActiveReceiptRuleSet {
        let out = DataSesActiveReceiptRuleSet(Rc::new(DataSesActiveReceiptRuleSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSesActiveReceiptRuleSetData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSesActiveReceiptRuleSetRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSesActiveReceiptRuleSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSesActiveReceiptRuleSetRef {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_set_name` after provisioning.\n"]
    pub fn rule_set_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_set_name", self.extract_ref()))
    }
}
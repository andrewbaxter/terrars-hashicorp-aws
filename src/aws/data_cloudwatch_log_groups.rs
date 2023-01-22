use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudwatchLogGroupsData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group_name_prefix: Option<PrimField<String>>,
}

struct DataCloudwatchLogGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudwatchLogGroupsData>,
}

#[derive(Clone)]
pub struct DataCloudwatchLogGroups(Rc<DataCloudwatchLogGroups_>);

impl DataCloudwatchLogGroups {
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

    #[doc= "Set the field `log_group_name_prefix`.\n"]
    pub fn set_log_group_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().log_group_name_prefix = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_group_name_prefix` after provisioning.\n"]
    pub fn log_group_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_group_names` after provisioning.\n"]
    pub fn log_group_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.log_group_names", self.extract_ref()))
    }
}

impl Datasource for DataCloudwatchLogGroups {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataCloudwatchLogGroups {
    type O = ListRef<DataCloudwatchLogGroupsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudwatchLogGroups_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudwatch_log_groups".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudwatchLogGroups {
    pub tf_id: String,
}

impl BuildDataCloudwatchLogGroups {
    pub fn build(self, stack: &mut Stack) -> DataCloudwatchLogGroups {
        let out = DataCloudwatchLogGroups(Rc::new(DataCloudwatchLogGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudwatchLogGroupsData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                log_group_name_prefix: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudwatchLogGroupsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudwatchLogGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudwatchLogGroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_group_name_prefix` after provisioning.\n"]
    pub fn log_group_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `log_group_names` after provisioning.\n"]
    pub fn log_group_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.log_group_names", self.extract_ref()))
    }
}

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderDistributionConfigurationsData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataImagebuilderDistributionConfigurationsFilterEl>>,
    dynamic: DataImagebuilderDistributionConfigurationsDynamic,
}

struct DataImagebuilderDistributionConfigurations_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataImagebuilderDistributionConfigurationsData>,
}

#[derive(Clone)]
pub struct DataImagebuilderDistributionConfigurations(Rc<DataImagebuilderDistributionConfigurations_>);

impl DataImagebuilderDistributionConfigurations {
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

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataImagebuilderDistributionConfigurationsFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
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

    #[doc= "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }
}

impl Datasource for DataImagebuilderDistributionConfigurations {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataImagebuilderDistributionConfigurations {
    type O = ListRef<DataImagebuilderDistributionConfigurationsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataImagebuilderDistributionConfigurations_ {
    fn extract_datasource_type(&self) -> String {
        "aws_imagebuilder_distribution_configurations".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderDistributionConfigurations {
    pub tf_id: String,
}

impl BuildDataImagebuilderDistributionConfigurations {
    pub fn build(self, stack: &mut Stack) -> DataImagebuilderDistributionConfigurations {
        let out = DataImagebuilderDistributionConfigurations(Rc::new(DataImagebuilderDistributionConfigurations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataImagebuilderDistributionConfigurationsData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderDistributionConfigurationsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImagebuilderDistributionConfigurationsRef {
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

    #[doc= "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderDistributionConfigurationsFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataImagebuilderDistributionConfigurationsFilterEl { }

impl ToListMappable for DataImagebuilderDistributionConfigurationsFilterEl {
    type O = BlockAssignable<DataImagebuilderDistributionConfigurationsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderDistributionConfigurationsFilterEl {
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataImagebuilderDistributionConfigurationsFilterEl {
    pub fn build(self) -> DataImagebuilderDistributionConfigurationsFilterEl {
        DataImagebuilderDistributionConfigurationsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataImagebuilderDistributionConfigurationsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImagebuilderDistributionConfigurationsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataImagebuilderDistributionConfigurationsFilterElRef {
        DataImagebuilderDistributionConfigurationsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderDistributionConfigurationsFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataImagebuilderDistributionConfigurationsDynamic {
    filter: Option<DynamicBlock<DataImagebuilderDistributionConfigurationsFilterEl>>,
}

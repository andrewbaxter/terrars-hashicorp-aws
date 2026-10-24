use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataImagebuilderInfrastructureConfigurationsData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataImagebuilderInfrastructureConfigurationsFilterEl>>,
    dynamic: DataImagebuilderInfrastructureConfigurationsDynamic,
}

struct DataImagebuilderInfrastructureConfigurations_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataImagebuilderInfrastructureConfigurationsData>,
}

#[derive(Clone)]
pub struct DataImagebuilderInfrastructureConfigurations(std::rc::Rc<DataImagebuilderInfrastructureConfigurations_>);

impl DataImagebuilderInfrastructureConfigurations {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataImagebuilderInfrastructureConfigurationsFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }
}

impl terrars::Referable for DataImagebuilderInfrastructureConfigurations {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataImagebuilderInfrastructureConfigurations { }

impl terrars::ToListMappable for DataImagebuilderInfrastructureConfigurations {
    type O = terrars::ListRef<DataImagebuilderInfrastructureConfigurationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataImagebuilderInfrastructureConfigurations_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_imagebuilder_infrastructure_configurations".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImagebuilderInfrastructureConfigurations {
    pub tf_id: std::string::String,
}

impl BuildDataImagebuilderInfrastructureConfigurations {
    pub fn build(self, stack: &mut terrars::Stack) -> DataImagebuilderInfrastructureConfigurations {
        let out =
            DataImagebuilderInfrastructureConfigurations(
                std::rc::Rc::new(DataImagebuilderInfrastructureConfigurations_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataImagebuilderInfrastructureConfigurationsData {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        id: core::default::Default::default(),
                        filter: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImagebuilderInfrastructureConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderInfrastructureConfigurationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImagebuilderInfrastructureConfigurationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arns` after provisioning.\n"]
    pub fn arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.names", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataImagebuilderInfrastructureConfigurationsFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataImagebuilderInfrastructureConfigurationsFilterEl { }

impl terrars::ToListMappable for DataImagebuilderInfrastructureConfigurationsFilterEl {
    type O = terrars::BlockAssignable<DataImagebuilderInfrastructureConfigurationsFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataImagebuilderInfrastructureConfigurationsFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataImagebuilderInfrastructureConfigurationsFilterEl {
    pub fn build(self) -> DataImagebuilderInfrastructureConfigurationsFilterEl {
        DataImagebuilderInfrastructureConfigurationsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataImagebuilderInfrastructureConfigurationsFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataImagebuilderInfrastructureConfigurationsFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataImagebuilderInfrastructureConfigurationsFilterElRef {
        DataImagebuilderInfrastructureConfigurationsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataImagebuilderInfrastructureConfigurationsFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataImagebuilderInfrastructureConfigurationsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataImagebuilderInfrastructureConfigurationsFilterEl>>,
}

use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaProvisionedConcurrencyConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    function_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    provisioned_concurrent_executions: PrimField<f64>,
    qualifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LambdaProvisionedConcurrencyConfigTimeoutsEl>,
}

struct LambdaProvisionedConcurrencyConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LambdaProvisionedConcurrencyConfigData>,
}

#[derive(Clone)]
pub struct LambdaProvisionedConcurrencyConfig(Rc<LambdaProvisionedConcurrencyConfig_>);

impl LambdaProvisionedConcurrencyConfig {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LambdaProvisionedConcurrencyConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `provisioned_concurrent_executions` after provisioning.\n"]
    pub fn provisioned_concurrent_executions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_concurrent_executions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaProvisionedConcurrencyConfigTimeoutsElRef {
        LambdaProvisionedConcurrencyConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Resource for LambdaProvisionedConcurrencyConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for LambdaProvisionedConcurrencyConfig {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for LambdaProvisionedConcurrencyConfig {
    type O = ListRef<LambdaProvisionedConcurrencyConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LambdaProvisionedConcurrencyConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_lambda_provisioned_concurrency_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaProvisionedConcurrencyConfig {
    pub tf_id: String,
    #[doc= ""]
    pub function_name: PrimField<String>,
    #[doc= ""]
    pub provisioned_concurrent_executions: PrimField<f64>,
    #[doc= ""]
    pub qualifier: PrimField<String>,
}

impl BuildLambdaProvisionedConcurrencyConfig {
    pub fn build(self, stack: &mut Stack) -> LambdaProvisionedConcurrencyConfig {
        let out = LambdaProvisionedConcurrencyConfig(Rc::new(LambdaProvisionedConcurrencyConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LambdaProvisionedConcurrencyConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                function_name: self.function_name,
                id: core::default::Default::default(),
                provisioned_concurrent_executions: self.provisioned_concurrent_executions,
                qualifier: self.qualifier,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaProvisionedConcurrencyConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaProvisionedConcurrencyConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaProvisionedConcurrencyConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `provisioned_concurrent_executions` after provisioning.\n"]
    pub fn provisioned_concurrent_executions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_concurrent_executions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaProvisionedConcurrencyConfigTimeoutsElRef {
        LambdaProvisionedConcurrencyConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct LambdaProvisionedConcurrencyConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl LambdaProvisionedConcurrencyConfigTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for LambdaProvisionedConcurrencyConfigTimeoutsEl {
    type O = BlockAssignable<LambdaProvisionedConcurrencyConfigTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaProvisionedConcurrencyConfigTimeoutsEl {}

impl BuildLambdaProvisionedConcurrencyConfigTimeoutsEl {
    pub fn build(self) -> LambdaProvisionedConcurrencyConfigTimeoutsEl {
        LambdaProvisionedConcurrencyConfigTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct LambdaProvisionedConcurrencyConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LambdaProvisionedConcurrencyConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LambdaProvisionedConcurrencyConfigTimeoutsElRef {
        LambdaProvisionedConcurrencyConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaProvisionedConcurrencyConfigTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

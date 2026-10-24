use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Inspector2OrganizationConfigurationData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_enable: std::option::Option<std::vec::Vec<Inspector2OrganizationConfigurationAutoEnableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Inspector2OrganizationConfigurationTimeoutsEl>,
    dynamic: Inspector2OrganizationConfigurationDynamic,
}

struct Inspector2OrganizationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Inspector2OrganizationConfigurationData>,
}

#[derive(Clone)]
pub struct Inspector2OrganizationConfiguration(std::rc::Rc<Inspector2OrganizationConfiguration_>);

impl Inspector2OrganizationConfiguration {
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

    #[doc = "Set the field `auto_enable`.\n"]
    pub fn set_auto_enable(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2OrganizationConfigurationAutoEnableEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_enable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_enable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Inspector2OrganizationConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_account_limit_reached` after provisioning.\n"]
    pub fn max_account_limit_reached(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_account_limit_reached", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> terrars::ListRef<Inspector2OrganizationConfigurationAutoEnableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Inspector2OrganizationConfigurationTimeoutsElRef {
        Inspector2OrganizationConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for Inspector2OrganizationConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Inspector2OrganizationConfiguration { }

impl terrars::ToListMappable for Inspector2OrganizationConfiguration {
    type O = terrars::ListRef<Inspector2OrganizationConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Inspector2OrganizationConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_inspector2_organization_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInspector2OrganizationConfiguration {
    pub tf_id: std::string::String,
}

impl BuildInspector2OrganizationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> Inspector2OrganizationConfiguration {
        let out = Inspector2OrganizationConfiguration(std::rc::Rc::new(Inspector2OrganizationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Inspector2OrganizationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                auto_enable: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Inspector2OrganizationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2OrganizationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Inspector2OrganizationConfigurationRef {
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

    #[doc = "Get a reference to the value of field `max_account_limit_reached` after provisioning.\n"]
    pub fn max_account_limit_reached(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_account_limit_reached", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> terrars::ListRef<Inspector2OrganizationConfigurationAutoEnableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Inspector2OrganizationConfigurationTimeoutsElRef {
        Inspector2OrganizationConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2OrganizationConfigurationAutoEnableEl {
    ec2: terrars::PrimField<bool>,
    ecr: terrars::PrimField<bool>,
}

impl Inspector2OrganizationConfigurationAutoEnableEl { }

impl terrars::ToListMappable for Inspector2OrganizationConfigurationAutoEnableEl {
    type O = terrars::BlockAssignable<Inspector2OrganizationConfigurationAutoEnableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2OrganizationConfigurationAutoEnableEl {
    #[doc = ""]
    pub ec2: terrars::PrimField<bool>,
    #[doc = ""]
    pub ecr: terrars::PrimField<bool>,
}

impl BuildInspector2OrganizationConfigurationAutoEnableEl {
    pub fn build(self) -> Inspector2OrganizationConfigurationAutoEnableEl {
        Inspector2OrganizationConfigurationAutoEnableEl {
            ec2: self.ec2,
            ecr: self.ecr,
        }
    }
}

pub struct Inspector2OrganizationConfigurationAutoEnableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2OrganizationConfigurationAutoEnableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2OrganizationConfigurationAutoEnableElRef {
        Inspector2OrganizationConfigurationAutoEnableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2OrganizationConfigurationAutoEnableElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ec2` after provisioning.\n"]
    pub fn ec2(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2", self.base))
    }

    #[doc = "Get a reference to the value of field `ecr` after provisioning.\n"]
    pub fn ecr(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecr", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2OrganizationConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Inspector2OrganizationConfigurationTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Inspector2OrganizationConfigurationTimeoutsEl {
    type O = terrars::BlockAssignable<Inspector2OrganizationConfigurationTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2OrganizationConfigurationTimeoutsEl {}

impl BuildInspector2OrganizationConfigurationTimeoutsEl {
    pub fn build(self) -> Inspector2OrganizationConfigurationTimeoutsEl {
        Inspector2OrganizationConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Inspector2OrganizationConfigurationTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2OrganizationConfigurationTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2OrganizationConfigurationTimeoutsElRef {
        Inspector2OrganizationConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2OrganizationConfigurationTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct Inspector2OrganizationConfigurationDynamic {
    auto_enable: std::option::Option<terrars::DynamicBlock<Inspector2OrganizationConfigurationAutoEnableEl>>,
}

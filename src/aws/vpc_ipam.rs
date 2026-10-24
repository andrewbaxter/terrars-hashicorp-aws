use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcIpamData {
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
    cascade: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_regions: std::option::Option<std::vec::Vec<VpcIpamOperatingRegionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VpcIpamTimeoutsEl>,
    dynamic: VpcIpamDynamic,
}

struct VpcIpam_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VpcIpamData>,
}

#[derive(Clone)]
pub struct VpcIpam(std::rc::Rc<VpcIpam_>);

impl VpcIpam {
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

    #[doc = "Set the field `cascade`.\n"]
    pub fn set_cascade(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cascade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `operating_regions`.\n"]
    pub fn set_operating_regions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpcIpamOperatingRegionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().operating_regions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.operating_regions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VpcIpamTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cascade` after provisioning.\n"]
    pub fn cascade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cascade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_default_scope_id` after provisioning.\n"]
    pub fn private_default_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_default_scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_default_scope_id` after provisioning.\n"]
    pub fn public_default_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_default_scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_count` after provisioning.\n"]
    pub fn scope_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcIpamTimeoutsElRef {
        VpcIpamTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VpcIpam {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VpcIpam { }

impl terrars::ToListMappable for VpcIpam {
    type O = terrars::ListRef<VpcIpamRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VpcIpam_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_vpc_ipam".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcIpam {
    pub tf_id: std::string::String,
}

impl BuildVpcIpam {
    pub fn build(self, stack: &mut terrars::Stack) -> VpcIpam {
        let out = VpcIpam(std::rc::Rc::new(VpcIpam_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VpcIpamData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cascade: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                operating_regions: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcIpamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcIpamRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpcIpamRef {
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

    #[doc = "Get a reference to the value of field `cascade` after provisioning.\n"]
    pub fn cascade(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cascade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_default_scope_id` after provisioning.\n"]
    pub fn private_default_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_default_scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_default_scope_id` after provisioning.\n"]
    pub fn public_default_scope_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_default_scope_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_count` after provisioning.\n"]
    pub fn scope_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpcIpamTimeoutsElRef {
        VpcIpamTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpcIpamOperatingRegionsEl {
    region_name: terrars::PrimField<std::string::String>,
}

impl VpcIpamOperatingRegionsEl { }

impl terrars::ToListMappable for VpcIpamOperatingRegionsEl {
    type O = terrars::BlockAssignable<VpcIpamOperatingRegionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcIpamOperatingRegionsEl {
    #[doc = ""]
    pub region_name: terrars::PrimField<std::string::String>,
}

impl BuildVpcIpamOperatingRegionsEl {
    pub fn build(self) -> VpcIpamOperatingRegionsEl {
        VpcIpamOperatingRegionsEl { region_name: self.region_name }
    }
}

pub struct VpcIpamOperatingRegionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcIpamOperatingRegionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcIpamOperatingRegionsElRef {
        VpcIpamOperatingRegionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcIpamOperatingRegionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VpcIpamTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcIpamTimeoutsEl {
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

impl terrars::ToListMappable for VpcIpamTimeoutsEl {
    type O = terrars::BlockAssignable<VpcIpamTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcIpamTimeoutsEl {}

impl BuildVpcIpamTimeoutsEl {
    pub fn build(self) -> VpcIpamTimeoutsEl {
        VpcIpamTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct VpcIpamTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcIpamTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcIpamTimeoutsElRef {
        VpcIpamTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcIpamTimeoutsElRef {
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
struct VpcIpamDynamic {
    operating_regions: std::option::Option<terrars::DynamicBlock<VpcIpamOperatingRegionsEl>>,
}

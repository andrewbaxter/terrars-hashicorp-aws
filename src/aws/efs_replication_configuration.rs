use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EfsReplicationConfigurationData {
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
    source_file_system_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<EfsReplicationConfigurationDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EfsReplicationConfigurationTimeoutsEl>,
    dynamic: EfsReplicationConfigurationDynamic,
}

struct EfsReplicationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EfsReplicationConfigurationData>,
}

#[derive(Clone)]
pub struct EfsReplicationConfiguration(std::rc::Rc<EfsReplicationConfiguration_>);

impl EfsReplicationConfiguration {
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

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EfsReplicationConfigurationDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EfsReplicationConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `original_source_file_system_arn` after provisioning.\n"]
    pub fn original_source_file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.original_source_file_system_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_file_system_arn` after provisioning.\n"]
    pub fn source_file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_system_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_file_system_id` after provisioning.\n"]
    pub fn source_file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_file_system_region` after provisioning.\n"]
    pub fn source_file_system_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_system_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<EfsReplicationConfigurationDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EfsReplicationConfigurationTimeoutsElRef {
        EfsReplicationConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for EfsReplicationConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EfsReplicationConfiguration { }

impl terrars::ToListMappable for EfsReplicationConfiguration {
    type O = terrars::ListRef<EfsReplicationConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EfsReplicationConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_efs_replication_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEfsReplicationConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub source_file_system_id: terrars::PrimField<std::string::String>,
}

impl BuildEfsReplicationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> EfsReplicationConfiguration {
        let out = EfsReplicationConfiguration(std::rc::Rc::new(EfsReplicationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EfsReplicationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                source_file_system_id: self.source_file_system_id,
                destination: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EfsReplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsReplicationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EfsReplicationConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `original_source_file_system_arn` after provisioning.\n"]
    pub fn original_source_file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.original_source_file_system_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_file_system_arn` after provisioning.\n"]
    pub fn source_file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_system_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_file_system_id` after provisioning.\n"]
    pub fn source_file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_file_system_region` after provisioning.\n"]
    pub fn source_file_system_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_file_system_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<EfsReplicationConfigurationDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EfsReplicationConfigurationTimeoutsElRef {
        EfsReplicationConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct EfsReplicationConfigurationDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EfsReplicationConfigurationDestinationEl {
    #[doc = "Set the field `availability_zone_name`.\n"]
    pub fn set_availability_zone_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.availability_zone_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EfsReplicationConfigurationDestinationEl {
    type O = terrars::BlockAssignable<EfsReplicationConfigurationDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEfsReplicationConfigurationDestinationEl {}

impl BuildEfsReplicationConfigurationDestinationEl {
    pub fn build(self) -> EfsReplicationConfigurationDestinationEl {
        EfsReplicationConfigurationDestinationEl {
            availability_zone_name: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct EfsReplicationConfigurationDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsReplicationConfigurationDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EfsReplicationConfigurationDestinationElRef {
        EfsReplicationConfigurationDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EfsReplicationConfigurationDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone_name` after provisioning.\n"]
    pub fn availability_zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_name", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct EfsReplicationConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EfsReplicationConfigurationTimeoutsEl {
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
}

impl terrars::ToListMappable for EfsReplicationConfigurationTimeoutsEl {
    type O = terrars::BlockAssignable<EfsReplicationConfigurationTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEfsReplicationConfigurationTimeoutsEl {}

impl BuildEfsReplicationConfigurationTimeoutsEl {
    pub fn build(self) -> EfsReplicationConfigurationTimeoutsEl {
        EfsReplicationConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct EfsReplicationConfigurationTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EfsReplicationConfigurationTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EfsReplicationConfigurationTimeoutsElRef {
        EfsReplicationConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EfsReplicationConfigurationTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct EfsReplicationConfigurationDynamic {
    destination: std::option::Option<terrars::DynamicBlock<EfsReplicationConfigurationDestinationEl>>,
}

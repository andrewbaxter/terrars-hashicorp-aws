use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxOntapVolumeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    junction_path: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_style: Option<PrimField<String>>,
    size_in_megabytes: PrimField<f64>,
    storage_efficiency_enabled: PrimField<bool>,
    storage_virtual_machine_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiering_policy: Option<Vec<FsxOntapVolumeTieringPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FsxOntapVolumeTimeoutsEl>,
    dynamic: FsxOntapVolumeDynamic,
}

struct FsxOntapVolume_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FsxOntapVolumeData>,
}

#[derive(Clone)]
pub struct FsxOntapVolume(Rc<FsxOntapVolume_>);

impl FsxOntapVolume {
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

    #[doc= "Set the field `security_style`.\n"]
    pub fn set_security_style(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_style = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `volume_type`.\n"]
    pub fn set_volume_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().volume_type = Some(v.into());
        self
    }

    #[doc= "Set the field `tiering_policy`.\n"]
    pub fn set_tiering_policy(self, v: impl Into<BlockAssignable<FsxOntapVolumeTieringPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tiering_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tiering_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FsxOntapVolumeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flexcache_endpoint_type` after provisioning.\n"]
    pub fn flexcache_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flexcache_endpoint_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `junction_path` after provisioning.\n"]
    pub fn junction_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.junction_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ontap_volume_type` after provisioning.\n"]
    pub fn ontap_volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ontap_volume_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_style` after provisioning.\n"]
    pub fn security_style(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_style", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_in_megabytes` after provisioning.\n"]
    pub fn size_in_megabytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_in_megabytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_efficiency_enabled` after provisioning.\n"]
    pub fn storage_efficiency_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_efficiency_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_virtual_machine_id` after provisioning.\n"]
    pub fn storage_virtual_machine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_virtual_machine_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> ListRef<FsxOntapVolumeTieringPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tiering_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapVolumeTimeoutsElRef {
        FsxOntapVolumeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for FsxOntapVolume {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for FsxOntapVolume {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for FsxOntapVolume {
    type O = ListRef<FsxOntapVolumeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for FsxOntapVolume_ {
    fn extract_resource_type(&self) -> String {
        "aws_fsx_ontap_volume".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxOntapVolume {
    pub tf_id: String,
    #[doc= ""]
    pub junction_path: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub size_in_megabytes: PrimField<f64>,
    #[doc= ""]
    pub storage_efficiency_enabled: PrimField<bool>,
    #[doc= ""]
    pub storage_virtual_machine_id: PrimField<String>,
}

impl BuildFsxOntapVolume {
    pub fn build(self, stack: &mut Stack) -> FsxOntapVolume {
        let out = FsxOntapVolume(Rc::new(FsxOntapVolume_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FsxOntapVolumeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                junction_path: self.junction_path,
                name: self.name,
                security_style: core::default::Default::default(),
                size_in_megabytes: self.size_in_megabytes,
                storage_efficiency_enabled: self.storage_efficiency_enabled,
                storage_virtual_machine_id: self.storage_virtual_machine_id,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                volume_type: core::default::Default::default(),
                tiering_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxOntapVolumeRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxOntapVolumeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flexcache_endpoint_type` after provisioning.\n"]
    pub fn flexcache_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flexcache_endpoint_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `junction_path` after provisioning.\n"]
    pub fn junction_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.junction_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ontap_volume_type` after provisioning.\n"]
    pub fn ontap_volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ontap_volume_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `security_style` after provisioning.\n"]
    pub fn security_style(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_style", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_in_megabytes` after provisioning.\n"]
    pub fn size_in_megabytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_in_megabytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_efficiency_enabled` after provisioning.\n"]
    pub fn storage_efficiency_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_efficiency_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_virtual_machine_id` after provisioning.\n"]
    pub fn storage_virtual_machine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_virtual_machine_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiering_policy` after provisioning.\n"]
    pub fn tiering_policy(&self) -> ListRef<FsxOntapVolumeTieringPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tiering_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapVolumeTimeoutsElRef {
        FsxOntapVolumeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolumeTieringPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cooling_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl FsxOntapVolumeTieringPolicyEl {
    #[doc= "Set the field `cooling_period`.\n"]
    pub fn set_cooling_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cooling_period = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapVolumeTieringPolicyEl {
    type O = BlockAssignable<FsxOntapVolumeTieringPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeTieringPolicyEl {}

impl BuildFsxOntapVolumeTieringPolicyEl {
    pub fn build(self) -> FsxOntapVolumeTieringPolicyEl {
        FsxOntapVolumeTieringPolicyEl {
            cooling_period: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapVolumeTieringPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeTieringPolicyElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapVolumeTieringPolicyElRef {
        FsxOntapVolumeTieringPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeTieringPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cooling_period` after provisioning.\n"]
    pub fn cooling_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cooling_period", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapVolumeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FsxOntapVolumeTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for FsxOntapVolumeTimeoutsEl {
    type O = BlockAssignable<FsxOntapVolumeTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapVolumeTimeoutsEl {}

impl BuildFsxOntapVolumeTimeoutsEl {
    pub fn build(self) -> FsxOntapVolumeTimeoutsEl {
        FsxOntapVolumeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapVolumeTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FsxOntapVolumeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FsxOntapVolumeTimeoutsElRef {
        FsxOntapVolumeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapVolumeTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOntapVolumeDynamic {
    tiering_policy: Option<DynamicBlock<FsxOntapVolumeTieringPolicyEl>>,
}

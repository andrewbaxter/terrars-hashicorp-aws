use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEbsEncryptionByDefaultData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEbsEncryptionByDefaultTimeoutsEl>,
}

struct DataEbsEncryptionByDefault_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEbsEncryptionByDefaultData>,
}

#[derive(Clone)]
pub struct DataEbsEncryptionByDefault(Rc<DataEbsEncryptionByDefault_>);

impl DataEbsEncryptionByDefault {
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

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataEbsEncryptionByDefaultTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEbsEncryptionByDefaultTimeoutsElRef {
        DataEbsEncryptionByDefaultTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Datasource for DataEbsEncryptionByDefault {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataEbsEncryptionByDefault {
    type O = ListRef<DataEbsEncryptionByDefaultRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEbsEncryptionByDefault_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ebs_encryption_by_default".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEbsEncryptionByDefault {
    pub tf_id: String,
}

impl BuildDataEbsEncryptionByDefault {
    pub fn build(self, stack: &mut Stack) -> DataEbsEncryptionByDefault {
        let out = DataEbsEncryptionByDefault(Rc::new(DataEbsEncryptionByDefault_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEbsEncryptionByDefaultData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEbsEncryptionByDefaultRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEbsEncryptionByDefaultRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEbsEncryptionByDefaultRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEbsEncryptionByDefaultTimeoutsElRef {
        DataEbsEncryptionByDefaultTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEbsEncryptionByDefaultTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEbsEncryptionByDefaultTimeoutsEl {
    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEbsEncryptionByDefaultTimeoutsEl {
    type O = BlockAssignable<DataEbsEncryptionByDefaultTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEbsEncryptionByDefaultTimeoutsEl {}

impl BuildDataEbsEncryptionByDefaultTimeoutsEl {
    pub fn build(self) -> DataEbsEncryptionByDefaultTimeoutsEl {
        DataEbsEncryptionByDefaultTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEbsEncryptionByDefaultTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEbsEncryptionByDefaultTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEbsEncryptionByDefaultTimeoutsElRef {
        DataEbsEncryptionByDefaultTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEbsEncryptionByDefaultTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

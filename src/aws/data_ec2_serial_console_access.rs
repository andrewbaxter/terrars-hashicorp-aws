use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2SerialConsoleAccessData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2SerialConsoleAccessTimeoutsEl>,
}

struct DataEc2SerialConsoleAccess_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2SerialConsoleAccessData>,
}

#[derive(Clone)]
pub struct DataEc2SerialConsoleAccess(Rc<DataEc2SerialConsoleAccess_>);

impl DataEc2SerialConsoleAccess {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
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
    pub fn set_timeouts(self, v: impl Into<DataEc2SerialConsoleAccessTimeoutsEl>) -> Self {
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
    pub fn timeouts(&self) -> DataEc2SerialConsoleAccessTimeoutsElRef {
        DataEc2SerialConsoleAccessTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Datasource for DataEc2SerialConsoleAccess {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataEc2SerialConsoleAccess {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataEc2SerialConsoleAccess {
    type O = ListRef<DataEc2SerialConsoleAccessRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2SerialConsoleAccess_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_serial_console_access".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2SerialConsoleAccess {
    pub tf_id: String,
}

impl BuildDataEc2SerialConsoleAccess {
    pub fn build(self, stack: &mut Stack) -> DataEc2SerialConsoleAccess {
        let out = DataEc2SerialConsoleAccess(Rc::new(DataEc2SerialConsoleAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2SerialConsoleAccessData {
                depends_on: core::default::Default::default(),
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

pub struct DataEc2SerialConsoleAccessRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2SerialConsoleAccessRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2SerialConsoleAccessRef {
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
    pub fn timeouts(&self) -> DataEc2SerialConsoleAccessTimeoutsElRef {
        DataEc2SerialConsoleAccessTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2SerialConsoleAccessTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2SerialConsoleAccessTimeoutsEl {
    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2SerialConsoleAccessTimeoutsEl {
    type O = BlockAssignable<DataEc2SerialConsoleAccessTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2SerialConsoleAccessTimeoutsEl {}

impl BuildDataEc2SerialConsoleAccessTimeoutsEl {
    pub fn build(self) -> DataEc2SerialConsoleAccessTimeoutsEl {
        DataEc2SerialConsoleAccessTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2SerialConsoleAccessTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2SerialConsoleAccessTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2SerialConsoleAccessTimeoutsElRef {
        DataEc2SerialConsoleAccessTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2SerialConsoleAccessTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

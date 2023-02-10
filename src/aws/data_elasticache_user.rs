use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataElasticacheUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_string: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_password_required: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passwords: Option<SetField<PrimField<String>>>,
    user_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name: Option<PrimField<String>>,
}

struct DataElasticacheUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataElasticacheUserData>,
}

#[derive(Clone)]
pub struct DataElasticacheUser(Rc<DataElasticacheUser_>);

impl DataElasticacheUser {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `access_string`.\n"]
    pub fn set_access_string(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_string = Some(v.into());
        self
    }

    #[doc= "Set the field `engine`.\n"]
    pub fn set_engine(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `no_password_required`.\n"]
    pub fn set_no_password_required(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().no_password_required = Some(v.into());
        self
    }

    #[doc= "Set the field `passwords`.\n"]
    pub fn set_passwords(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().passwords = Some(v.into());
        self
    }

    #[doc= "Set the field `user_name`.\n"]
    pub fn set_user_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_name = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_string` after provisioning.\n"]
    pub fn access_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_string", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `no_password_required` after provisioning.\n"]
    pub fn no_password_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_password_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `passwords` after provisioning.\n"]
    pub fn passwords(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.passwords", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.extract_ref()))
    }
}

impl Referable for DataElasticacheUser {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataElasticacheUser { }

impl ToListMappable for DataElasticacheUser {
    type O = ListRef<DataElasticacheUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataElasticacheUser_ {
    fn extract_datasource_type(&self) -> String {
        "aws_elasticache_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataElasticacheUser {
    pub tf_id: String,
    #[doc= ""]
    pub user_id: PrimField<String>,
}

impl BuildDataElasticacheUser {
    pub fn build(self, stack: &mut Stack) -> DataElasticacheUser {
        let out = DataElasticacheUser(Rc::new(DataElasticacheUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataElasticacheUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                access_string: core::default::Default::default(),
                engine: core::default::Default::default(),
                id: core::default::Default::default(),
                no_password_required: core::default::Default::default(),
                passwords: core::default::Default::default(),
                user_id: self.user_id,
                user_name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataElasticacheUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataElasticacheUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataElasticacheUserRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `access_string` after provisioning.\n"]
    pub fn access_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_string", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `no_password_required` after provisioning.\n"]
    pub fn no_password_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.no_password_required", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `passwords` after provisioning.\n"]
    pub fn passwords(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.passwords", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_name` after provisioning.\n"]
    pub fn user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_name", self.extract_ref()))
    }
}

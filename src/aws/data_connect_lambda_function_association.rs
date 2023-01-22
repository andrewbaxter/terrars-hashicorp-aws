use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataConnectLambdaFunctionAssociationData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    function_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
}

struct DataConnectLambdaFunctionAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataConnectLambdaFunctionAssociationData>,
}

#[derive(Clone)]
pub struct DataConnectLambdaFunctionAssociation(Rc<DataConnectLambdaFunctionAssociation_>);

impl DataConnectLambdaFunctionAssociation {
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

    #[doc= "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }
}

impl Datasource for DataConnectLambdaFunctionAssociation {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataConnectLambdaFunctionAssociation {
    type O = ListRef<DataConnectLambdaFunctionAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataConnectLambdaFunctionAssociation_ {
    fn extract_datasource_type(&self) -> String {
        "aws_connect_lambda_function_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectLambdaFunctionAssociation {
    pub tf_id: String,
    #[doc= ""]
    pub function_arn: PrimField<String>,
    #[doc= ""]
    pub instance_id: PrimField<String>,
}

impl BuildDataConnectLambdaFunctionAssociation {
    pub fn build(self, stack: &mut Stack) -> DataConnectLambdaFunctionAssociation {
        let out = DataConnectLambdaFunctionAssociation(Rc::new(DataConnectLambdaFunctionAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataConnectLambdaFunctionAssociationData {
                provider: None,
                for_each: None,
                function_arn: self.function_arn,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectLambdaFunctionAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectLambdaFunctionAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataConnectLambdaFunctionAssociationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }
}
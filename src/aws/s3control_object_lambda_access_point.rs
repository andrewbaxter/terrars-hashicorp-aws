#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3controlObjectLambdaAccessPointData_ {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<S3controlObjectLambdaAccessPoint_Configuration>>,
    dynamic: S3controlObjectLambdaAccessPointDynamic,
}

struct S3controlObjectLambdaAccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3controlObjectLambdaAccessPointData_>,
}

#[derive(Clone)]
pub struct S3controlObjectLambdaAccessPoint(std::rc::Rc<S3controlObjectLambdaAccessPoint_>);

impl S3controlObjectLambdaAccessPoint {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3controlObjectLambdaAccessPoint_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<S3controlObjectLambdaAccessPoint_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl terrars::Referable for S3controlObjectLambdaAccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3controlObjectLambdaAccessPoint { }

impl terrars::ToListMappable for S3controlObjectLambdaAccessPoint {
    type O = terrars::ListRef<S3controlObjectLambdaAccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3controlObjectLambdaAccessPoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3control_object_lambda_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlObjectLambdaAccessPoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildS3controlObjectLambdaAccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> S3controlObjectLambdaAccessPoint {
        let out = S3controlObjectLambdaAccessPoint(std::rc::Rc::new(S3controlObjectLambdaAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3controlObjectLambdaAccessPointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlObjectLambdaAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlObjectLambdaAccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3controlObjectLambdaAccessPointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<S3controlObjectLambdaAccessPoint_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3controlObjectLambdaAccessPointDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<S3controlObjectLambdaAccessPoint_Configuration>>,
}

#[derive(Serialize)]
pub struct S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda {
    function_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_payload: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda {
    #[doc = "Set the field `function_payload`.\n"]
    pub fn set_function_payload(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.function_payload = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda {
    type O =
        terrars
        ::BlockAssignable<
            S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda {
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
}

impl BuildS3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda {
    pub fn build(
        self,
    ) -> S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda {
        S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda {
            function_arn: self.function_arn,
            function_payload: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambdaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambdaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambdaRef {
        S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambdaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambdaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `function_payload` after provisioning.\n"]
    pub fn function_payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_payload", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformationDynamic {
    aws_lambda: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_lambda: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda,
        >,
    >,
    dynamic: S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformationDynamic,
}

impl S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation {
    #[doc = "Set the field `aws_lambda`.\n"]
    pub fn set_aws_lambda(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambda,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_lambda = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_lambda = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation {
    type O =
        terrars
        ::BlockAssignable<
            S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation {}

impl BuildS3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation {
    pub fn build(
        self,
    ) -> S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation {
        S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation {
            aws_lambda: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformationRef {
        S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_lambda` after provisioning.\n"]
    pub fn aws_lambda(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation_AwsLambdaRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_lambda", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlObjectLambdaAccessPoint_Configuration_TransformationConfigurationDynamic {
    content_transformation: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration {
    actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_transformation: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation>,
    >,
    dynamic: S3controlObjectLambdaAccessPoint_Configuration_TransformationConfigurationDynamic,
}

impl S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration {
    #[doc = "Set the field `content_transformation`.\n"]
    pub fn set_content_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.content_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.content_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration {
    type O = terrars::BlockAssignable<S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration {
    #[doc = ""]
    pub actions: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildS3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration {
    pub fn build(self) -> S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration {
        S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration {
            actions: self.actions,
            content_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlObjectLambdaAccessPoint_Configuration_TransformationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlObjectLambdaAccessPoint_Configuration_TransformationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlObjectLambdaAccessPoint_Configuration_TransformationConfigurationRef {
        S3controlObjectLambdaAccessPoint_Configuration_TransformationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlObjectLambdaAccessPoint_Configuration_TransformationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `content_transformation` after provisioning.\n"]
    pub fn content_transformation(
        &self,
    ) -> terrars
    ::ListRef<S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration_ContentTransformationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_transformation", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlObjectLambdaAccessPoint_ConfigurationDynamic {
    transformation_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration>>,
}

#[derive(Serialize)]
pub struct S3controlObjectLambdaAccessPoint_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_features: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_watch_metrics_enabled: std::option::Option<terrars::PrimField<bool>>,
    supporting_access_point: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_configuration: std
    ::option
    ::Option<std::vec::Vec<S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration>>,
    dynamic: S3controlObjectLambdaAccessPoint_ConfigurationDynamic,
}

impl S3controlObjectLambdaAccessPoint_Configuration {
    #[doc = "Set the field `allowed_features`.\n"]
    pub fn set_allowed_features(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_features = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_watch_metrics_enabled`.\n"]
    pub fn set_cloud_watch_metrics_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cloud_watch_metrics_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transformation_configuration`.\n"]
    pub fn set_transformation_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<S3controlObjectLambdaAccessPoint_Configuration_TransformationConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transformation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3controlObjectLambdaAccessPoint_Configuration {
    type O = terrars::BlockAssignable<S3controlObjectLambdaAccessPoint_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlObjectLambdaAccessPoint_Configuration {
    #[doc = ""]
    pub supporting_access_point: terrars::PrimField<std::string::String>,
}

impl BuildS3controlObjectLambdaAccessPoint_Configuration {
    pub fn build(self) -> S3controlObjectLambdaAccessPoint_Configuration {
        S3controlObjectLambdaAccessPoint_Configuration {
            allowed_features: core::default::Default::default(),
            cloud_watch_metrics_enabled: core::default::Default::default(),
            supporting_access_point: self.supporting_access_point,
            transformation_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlObjectLambdaAccessPoint_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlObjectLambdaAccessPoint_ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlObjectLambdaAccessPoint_ConfigurationRef {
        S3controlObjectLambdaAccessPoint_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlObjectLambdaAccessPoint_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_features` after provisioning.\n"]
    pub fn allowed_features(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_features", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_watch_metrics_enabled` after provisioning.\n"]
    pub fn cloud_watch_metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_watch_metrics_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `supporting_access_point` after provisioning.\n"]
    pub fn supporting_access_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.supporting_access_point", self.base))
    }
}

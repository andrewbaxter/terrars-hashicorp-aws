#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2AllowedImagesSettingsData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_criterion: std::option::Option<std::vec::Vec<Ec2AllowedImagesSettings_ImageCriterion>>,
    dynamic: Ec2AllowedImagesSettingsDynamic,
}

struct Ec2AllowedImagesSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2AllowedImagesSettingsData_>,
}

#[derive(Clone)]
pub struct Ec2AllowedImagesSettings(std::rc::Rc<Ec2AllowedImagesSettings_>);

impl Ec2AllowedImagesSettings {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_criterion`.\n"]
    pub fn set_image_criterion(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Ec2AllowedImagesSettings_ImageCriterion>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_criterion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_criterion = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_criterion` after provisioning.\n"]
    pub fn image_criterion(&self) -> terrars::ListRef<Ec2AllowedImagesSettings_ImageCriterionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_criterion", self.extract_ref()))
    }
}

impl terrars::Referable for Ec2AllowedImagesSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ec2AllowedImagesSettings { }

impl terrars::ToListMappable for Ec2AllowedImagesSettings {
    type O = terrars::ListRef<Ec2AllowedImagesSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ec2AllowedImagesSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ec2_allowed_images_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2AllowedImagesSettings {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildEc2AllowedImagesSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2AllowedImagesSettings {
        let out = Ec2AllowedImagesSettings(std::rc::Rc::new(Ec2AllowedImagesSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Ec2AllowedImagesSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                state: self.state,
                image_criterion: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2AllowedImagesSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2AllowedImagesSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2AllowedImagesSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_criterion` after provisioning.\n"]
    pub fn image_criterion(&self) -> terrars::ListRef<Ec2AllowedImagesSettings_ImageCriterionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_criterion", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Ec2AllowedImagesSettingsDynamic {
    image_criterion: std::option::Option<terrars::DynamicBlock<Ec2AllowedImagesSettings_ImageCriterion>>,
}

#[derive(Serialize)]
pub struct Ec2AllowedImagesSettings_ImageCriterion_CreationDateCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_days_since_created: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2AllowedImagesSettings_ImageCriterion_CreationDateCondition {
    #[doc = "Set the field `maximum_days_since_created`.\n"]
    pub fn set_maximum_days_since_created(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_days_since_created = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2AllowedImagesSettings_ImageCriterion_CreationDateCondition {
    type O = terrars::BlockAssignable<Ec2AllowedImagesSettings_ImageCriterion_CreationDateCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2AllowedImagesSettings_ImageCriterion_CreationDateCondition {}

impl BuildEc2AllowedImagesSettings_ImageCriterion_CreationDateCondition {
    pub fn build(self) -> Ec2AllowedImagesSettings_ImageCriterion_CreationDateCondition {
        Ec2AllowedImagesSettings_ImageCriterion_CreationDateCondition {
            maximum_days_since_created: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2AllowedImagesSettings_ImageCriterion_CreationDateConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2AllowedImagesSettings_ImageCriterion_CreationDateConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2AllowedImagesSettings_ImageCriterion_CreationDateConditionRef {
        Ec2AllowedImagesSettings_ImageCriterion_CreationDateConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2AllowedImagesSettings_ImageCriterion_CreationDateConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_days_since_created` after provisioning.\n"]
    pub fn maximum_days_since_created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_days_since_created", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_days_since_deprecated: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition {
    #[doc = "Set the field `maximum_days_since_deprecated`.\n"]
    pub fn set_maximum_days_since_deprecated(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_days_since_deprecated = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition {
    type O = terrars::BlockAssignable<Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition {}

impl BuildEc2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition {
    pub fn build(self) -> Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition {
        Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition {
            maximum_days_since_deprecated: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeConditionRef {
        Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_days_since_deprecated` after provisioning.\n"]
    pub fn maximum_days_since_deprecated(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_days_since_deprecated", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2AllowedImagesSettings_ImageCriterionDynamic {
    creation_date_condition: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2AllowedImagesSettings_ImageCriterion_CreationDateCondition>>,
    deprecation_time_condition: std
    ::option
    ::Option<terrars::DynamicBlock<Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition>>,
}

#[derive(Serialize)]
pub struct Ec2AllowedImagesSettings_ImageCriterion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_providers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketplace_product_codes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_date_condition: std
    ::option
    ::Option<std::vec::Vec<Ec2AllowedImagesSettings_ImageCriterion_CreationDateCondition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deprecation_time_condition: std
    ::option
    ::Option<std::vec::Vec<Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition>>,
    dynamic: Ec2AllowedImagesSettings_ImageCriterionDynamic,
}

impl Ec2AllowedImagesSettings_ImageCriterion {
    #[doc = "Set the field `image_names`.\n"]
    pub fn set_image_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.image_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_providers`.\n"]
    pub fn set_image_providers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.image_providers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `marketplace_product_codes`.\n"]
    pub fn set_marketplace_product_codes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.marketplace_product_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_date_condition`.\n"]
    pub fn set_creation_date_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Ec2AllowedImagesSettings_ImageCriterion_CreationDateCondition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.creation_date_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.creation_date_condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deprecation_time_condition`.\n"]
    pub fn set_deprecation_time_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeCondition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deprecation_time_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deprecation_time_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Ec2AllowedImagesSettings_ImageCriterion {
    type O = terrars::BlockAssignable<Ec2AllowedImagesSettings_ImageCriterion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2AllowedImagesSettings_ImageCriterion {}

impl BuildEc2AllowedImagesSettings_ImageCriterion {
    pub fn build(self) -> Ec2AllowedImagesSettings_ImageCriterion {
        Ec2AllowedImagesSettings_ImageCriterion {
            image_names: core::default::Default::default(),
            image_providers: core::default::Default::default(),
            marketplace_product_codes: core::default::Default::default(),
            creation_date_condition: core::default::Default::default(),
            deprecation_time_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2AllowedImagesSettings_ImageCriterionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2AllowedImagesSettings_ImageCriterionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2AllowedImagesSettings_ImageCriterionRef {
        Ec2AllowedImagesSettings_ImageCriterionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2AllowedImagesSettings_ImageCriterionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_names` after provisioning.\n"]
    pub fn image_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.image_names", self.base))
    }

    #[doc = "Get a reference to the value of field `image_providers` after provisioning.\n"]
    pub fn image_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.image_providers", self.base))
    }

    #[doc = "Get a reference to the value of field `marketplace_product_codes` after provisioning.\n"]
    pub fn marketplace_product_codes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.marketplace_product_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_date_condition` after provisioning.\n"]
    pub fn creation_date_condition(
        &self,
    ) -> terrars::ListRef<Ec2AllowedImagesSettings_ImageCriterion_CreationDateConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creation_date_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecation_time_condition` after provisioning.\n"]
    pub fn deprecation_time_condition(
        &self,
    ) -> terrars::ListRef<Ec2AllowedImagesSettings_ImageCriterion_DeprecationTimeConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deprecation_time_condition", self.base))
    }
}

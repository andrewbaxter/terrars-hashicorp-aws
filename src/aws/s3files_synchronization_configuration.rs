#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3filesSynchronizationConfigurationData_ {
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
    file_system_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_version_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_data_rule: std::option::Option<std::vec::Vec<S3filesSynchronizationConfiguration_ExpirationDataRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    import_data_rule: std::option::Option<std::vec::Vec<S3filesSynchronizationConfiguration_ImportDataRule>>,
    dynamic: S3filesSynchronizationConfigurationDynamic,
}

struct S3filesSynchronizationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3filesSynchronizationConfigurationData_>,
}

#[derive(Clone)]
pub struct S3filesSynchronizationConfiguration(std::rc::Rc<S3filesSynchronizationConfiguration_>);

impl S3filesSynchronizationConfiguration {
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

    #[doc = "Set the field `latest_version_number`.\nLatest version number for optimistic locking"]
    pub fn set_latest_version_number(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().latest_version_number = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expiration_data_rule`.\n"]
    pub fn set_expiration_data_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3filesSynchronizationConfiguration_ExpirationDataRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().expiration_data_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.expiration_data_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `import_data_rule`.\n"]
    pub fn set_import_data_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3filesSynchronizationConfiguration_ImportDataRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().import_data_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.import_data_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\nFile system ID"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_version_number` after provisioning.\nLatest version number for optimistic locking"]
    pub fn latest_version_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_version_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for S3filesSynchronizationConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3filesSynchronizationConfiguration { }

impl terrars::ToListMappable for S3filesSynchronizationConfiguration {
    type O = terrars::ListRef<S3filesSynchronizationConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3filesSynchronizationConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3files_synchronization_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3filesSynchronizationConfiguration {
    pub tf_id: std::string::String,
    #[doc = "File system ID"]
    pub file_system_id: terrars::PrimField<std::string::String>,
}

impl BuildS3filesSynchronizationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3filesSynchronizationConfiguration {
        let out = S3filesSynchronizationConfiguration(std::rc::Rc::new(S3filesSynchronizationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3filesSynchronizationConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                file_system_id: self.file_system_id,
                latest_version_number: core::default::Default::default(),
                region: core::default::Default::default(),
                expiration_data_rule: core::default::Default::default(),
                import_data_rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3filesSynchronizationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3filesSynchronizationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3filesSynchronizationConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\nFile system ID"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_version_number` after provisioning.\nLatest version number for optimistic locking"]
    pub fn latest_version_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_version_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3filesSynchronizationConfigurationDynamic {
    expiration_data_rule: std
    ::option
    ::Option<terrars::DynamicBlock<S3filesSynchronizationConfiguration_ExpirationDataRule>>,
    import_data_rule: std
    ::option
    ::Option<terrars::DynamicBlock<S3filesSynchronizationConfiguration_ImportDataRule>>,
}

#[derive(Serialize)]
pub struct S3filesSynchronizationConfiguration_ExpirationDataRule {
    days_after_last_access: terrars::PrimField<f64>,
}

impl S3filesSynchronizationConfiguration_ExpirationDataRule { }

impl terrars::ToListMappable for S3filesSynchronizationConfiguration_ExpirationDataRule {
    type O = terrars::BlockAssignable<S3filesSynchronizationConfiguration_ExpirationDataRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3filesSynchronizationConfiguration_ExpirationDataRule {
    #[doc = "Days after last access before data expires"]
    pub days_after_last_access: terrars::PrimField<f64>,
}

impl BuildS3filesSynchronizationConfiguration_ExpirationDataRule {
    pub fn build(self) -> S3filesSynchronizationConfiguration_ExpirationDataRule {
        S3filesSynchronizationConfiguration_ExpirationDataRule {
            days_after_last_access: self.days_after_last_access,
        }
    }
}

#[allow(dead_code)]
pub struct S3filesSynchronizationConfiguration_ExpirationDataRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3filesSynchronizationConfiguration_ExpirationDataRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3filesSynchronizationConfiguration_ExpirationDataRuleRef {
        S3filesSynchronizationConfiguration_ExpirationDataRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3filesSynchronizationConfiguration_ExpirationDataRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_after_last_access` after provisioning.\nDays after last access before data expires"]
    pub fn days_after_last_access(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_after_last_access", self.base))
    }
}

#[derive(Serialize)]
pub struct S3filesSynchronizationConfiguration_ImportDataRule {
    prefix: terrars::PrimField<std::string::String>,
    size_less_than: terrars::PrimField<f64>,
    trigger: terrars::PrimField<std::string::String>,
}

impl S3filesSynchronizationConfiguration_ImportDataRule { }

impl terrars::ToListMappable for S3filesSynchronizationConfiguration_ImportDataRule {
    type O = terrars::BlockAssignable<S3filesSynchronizationConfiguration_ImportDataRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3filesSynchronizationConfiguration_ImportDataRule {
    #[doc = "S3 prefix for import"]
    pub prefix: terrars::PrimField<std::string::String>,
    #[doc = "Maximum file size to import"]
    pub size_less_than: terrars::PrimField<f64>,
    #[doc = "Import trigger type"]
    pub trigger: terrars::PrimField<std::string::String>,
}

impl BuildS3filesSynchronizationConfiguration_ImportDataRule {
    pub fn build(self) -> S3filesSynchronizationConfiguration_ImportDataRule {
        S3filesSynchronizationConfiguration_ImportDataRule {
            prefix: self.prefix,
            size_less_than: self.size_less_than,
            trigger: self.trigger,
        }
    }
}

#[allow(dead_code)]
pub struct S3filesSynchronizationConfiguration_ImportDataRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3filesSynchronizationConfiguration_ImportDataRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3filesSynchronizationConfiguration_ImportDataRuleRef {
        S3filesSynchronizationConfiguration_ImportDataRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3filesSynchronizationConfiguration_ImportDataRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nS3 prefix for import"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `size_less_than` after provisioning.\nMaximum file size to import"]
    pub fn size_less_than(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_less_than", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger` after provisioning.\nImport trigger type"]
    pub fn trigger(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger", self.base))
    }
}

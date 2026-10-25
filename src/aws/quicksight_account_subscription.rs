#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct QuicksightAccountSubscriptionData_ {
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
    account_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_directory_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_group: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_pro_group: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    authentication_method: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    author_group: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    author_pro_group: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory_id: std::option::Option<terrars::PrimField<std::string::String>>,
    edition: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_identity_center_instance_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_name: std::option::Option<terrars::PrimField<std::string::String>>,
    notification_email: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reader_group: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reader_pro_group: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    realm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<QuicksightAccountSubscription_Timeouts>,
}

struct QuicksightAccountSubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<QuicksightAccountSubscriptionData_>,
}

#[derive(Clone)]
pub struct QuicksightAccountSubscription(std::rc::Rc<QuicksightAccountSubscription_>);

impl QuicksightAccountSubscription {
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

    #[doc = "Set the field `active_directory_name`.\n"]
    pub fn set_active_directory_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().active_directory_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `admin_group`.\n"]
    pub fn set_admin_group(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().admin_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `admin_pro_group`.\n"]
    pub fn set_admin_pro_group(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().admin_pro_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `author_group`.\n"]
    pub fn set_author_group(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().author_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `author_pro_group`.\n"]
    pub fn set_author_pro_group(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().author_pro_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contact_number`.\n"]
    pub fn set_contact_number(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().contact_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `directory_id`.\n"]
    pub fn set_directory_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().directory_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_address`.\n"]
    pub fn set_email_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().first_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_identity_center_instance_arn`.\n"]
    pub fn set_iam_identity_center_instance_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().iam_identity_center_instance_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().last_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reader_group`.\n"]
    pub fn set_reader_group(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().reader_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reader_pro_group`.\n"]
    pub fn set_reader_pro_group(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().reader_pro_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `realm`.\n"]
    pub fn set_realm(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().realm = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<QuicksightAccountSubscription_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_name` after provisioning.\n"]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_subscription_status` after provisioning.\n"]
    pub fn account_subscription_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_subscription_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_directory_name` after provisioning.\n"]
    pub fn active_directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_group` after provisioning.\n"]
    pub fn admin_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_pro_group` after provisioning.\n"]
    pub fn admin_pro_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_pro_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `author_group` after provisioning.\n"]
    pub fn author_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.author_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `author_pro_group` after provisioning.\n"]
    pub fn author_pro_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.author_pro_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_number` after provisioning.\n"]
    pub fn contact_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_identity_center_instance_arn` after provisioning.\n"]
    pub fn iam_identity_center_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_identity_center_instance_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_email` after provisioning.\n"]
    pub fn notification_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_group` after provisioning.\n"]
    pub fn reader_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reader_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_pro_group` after provisioning.\n"]
    pub fn reader_pro_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reader_pro_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `realm` after provisioning.\n"]
    pub fn realm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QuicksightAccountSubscription_TimeoutsRef {
        QuicksightAccountSubscription_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for QuicksightAccountSubscription {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for QuicksightAccountSubscription { }

impl terrars::ToListMappable for QuicksightAccountSubscription {
    type O = terrars::ListRef<QuicksightAccountSubscriptionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for QuicksightAccountSubscription_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_quicksight_account_subscription".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildQuicksightAccountSubscription {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub authentication_method: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub edition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notification_email: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightAccountSubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> QuicksightAccountSubscription {
        let out = QuicksightAccountSubscription(std::rc::Rc::new(QuicksightAccountSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(QuicksightAccountSubscriptionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_name: self.account_name,
                active_directory_name: core::default::Default::default(),
                admin_group: core::default::Default::default(),
                admin_pro_group: core::default::Default::default(),
                authentication_method: self.authentication_method,
                author_group: core::default::Default::default(),
                author_pro_group: core::default::Default::default(),
                aws_account_id: core::default::Default::default(),
                contact_number: core::default::Default::default(),
                directory_id: core::default::Default::default(),
                edition: self.edition,
                email_address: core::default::Default::default(),
                first_name: core::default::Default::default(),
                iam_identity_center_instance_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                last_name: core::default::Default::default(),
                notification_email: self.notification_email,
                reader_group: core::default::Default::default(),
                reader_pro_group: core::default::Default::default(),
                realm: core::default::Default::default(),
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct QuicksightAccountSubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightAccountSubscriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl QuicksightAccountSubscriptionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_name` after provisioning.\n"]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_subscription_status` after provisioning.\n"]
    pub fn account_subscription_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_subscription_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_directory_name` after provisioning.\n"]
    pub fn active_directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_group` after provisioning.\n"]
    pub fn admin_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_pro_group` after provisioning.\n"]
    pub fn admin_pro_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_pro_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `author_group` after provisioning.\n"]
    pub fn author_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.author_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `author_pro_group` after provisioning.\n"]
    pub fn author_pro_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.author_pro_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_number` after provisioning.\n"]
    pub fn contact_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_identity_center_instance_arn` after provisioning.\n"]
    pub fn iam_identity_center_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_identity_center_instance_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_email` after provisioning.\n"]
    pub fn notification_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_group` after provisioning.\n"]
    pub fn reader_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reader_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reader_pro_group` after provisioning.\n"]
    pub fn reader_pro_group(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reader_pro_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `realm` after provisioning.\n"]
    pub fn realm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QuicksightAccountSubscription_TimeoutsRef {
        QuicksightAccountSubscription_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct QuicksightAccountSubscription_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightAccountSubscription_Timeouts {
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

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightAccountSubscription_Timeouts {
    type O = terrars::BlockAssignable<QuicksightAccountSubscription_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightAccountSubscription_Timeouts {}

impl BuildQuicksightAccountSubscription_Timeouts {
    pub fn build(self) -> QuicksightAccountSubscription_Timeouts {
        QuicksightAccountSubscription_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightAccountSubscription_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightAccountSubscription_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightAccountSubscription_TimeoutsRef {
        QuicksightAccountSubscription_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightAccountSubscription_TimeoutsRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

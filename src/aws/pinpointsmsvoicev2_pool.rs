#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Pinpointsmsvoicev2PoolData_ {
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
    deletion_protection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iso_country_code: std::option::Option<terrars::PrimField<std::string::String>>,
    message_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opt_out_list_name: std::option::Option<terrars::PrimField<std::string::String>>,
    origination_identities: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_opt_outs_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_routes_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    two_way_channel_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    two_way_channel_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    two_way_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Pinpointsmsvoicev2Pool_Timeouts>,
}

struct Pinpointsmsvoicev2Pool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Pinpointsmsvoicev2PoolData_>,
}

#[derive(Clone)]
pub struct Pinpointsmsvoicev2Pool(std::rc::Rc<Pinpointsmsvoicev2Pool_>);

impl Pinpointsmsvoicev2Pool {
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
        "Set the field `deletion_protection_enabled`.\nWhether deletion protection is enabled. When `true`, the pool cannot be deleted."]
    pub fn set_deletion_protection_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `iso_country_code`.\nTwo-character code, in ISO 3166-1 alpha-2 format, for the country or region of the pool. This field is optional for origination identity types that are not country-specific."]
    pub fn set_iso_country_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().iso_country_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `opt_out_list_name`.\nName of the opt-out list to associate with the pool. Inherited from the initial origination identity when omitted."]
    pub fn set_opt_out_list_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().opt_out_list_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `self_managed_opt_outs_enabled`.\nWhether the pool relies on self-managed opt-out handling. When `false`, AWS auto-replies to HELP/STOP requests and manages the opt-out list. Inherited from the initial origination identity when omitted."]
    pub fn set_self_managed_opt_outs_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().self_managed_opt_outs_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_routes_enabled`.\nWhether shared routes are enabled for the pool. When `true`, messages may use shared phone numbers or sender IDs in countries that allow it."]
    pub fn set_shared_routes_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().shared_routes_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `two_way_channel_arn`.\nARN of the two-way channel that receives inbound messages."]
    pub fn set_two_way_channel_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().two_way_channel_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `two_way_channel_role`.\nARN of the IAM role that End User Messaging SMS assumes to publish inbound messages to the two-way channel."]
    pub fn set_two_way_channel_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().two_way_channel_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `two_way_enabled`.\nWhether inbound message reception is enabled for the pool."]
    pub fn set_two_way_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().two_way_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Pinpointsmsvoicev2Pool_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nWhether deletion protection is enabled. When `true`, the pool cannot be deleted."]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iso_country_code` after provisioning.\nTwo-character code, in ISO 3166-1 alpha-2 format, for the country or region of the pool. This field is optional for origination identity types that are not country-specific."]
    pub fn iso_country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iso_country_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `message_type` after provisioning.\nType of message."]
    pub fn message_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `opt_out_list_name` after provisioning.\nName of the opt-out list to associate with the pool. Inherited from the initial origination identity when omitted."]
    pub fn opt_out_list_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opt_out_list_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origination_identities` after provisioning.\nSet of origination identity ARNs to associate with the pool. At least one origination identity is required at creation."]
    pub fn origination_identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.origination_identities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_managed_opt_outs_enabled` after provisioning.\nWhether the pool relies on self-managed opt-out handling. When `false`, AWS auto-replies to HELP/STOP requests and manages the opt-out list. Inherited from the initial origination identity when omitted."]
    pub fn self_managed_opt_outs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_managed_opt_outs_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `shared_routes_enabled` after provisioning.\nWhether shared routes are enabled for the pool. When `true`, messages may use shared phone numbers or sender IDs in countries that allow it."]
    pub fn shared_routes_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_routes_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `two_way_channel_arn` after provisioning.\nARN of the two-way channel that receives inbound messages."]
    pub fn two_way_channel_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.two_way_channel_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `two_way_channel_role` after provisioning.\nARN of the IAM role that End User Messaging SMS assumes to publish inbound messages to the two-way channel."]
    pub fn two_way_channel_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.two_way_channel_role", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `two_way_enabled` after provisioning.\nWhether inbound message reception is enabled for the pool."]
    pub fn two_way_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.two_way_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Pinpointsmsvoicev2Pool_TimeoutsRef {
        Pinpointsmsvoicev2Pool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Pinpointsmsvoicev2Pool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Pinpointsmsvoicev2Pool { }

impl terrars::ToListMappable for Pinpointsmsvoicev2Pool {
    type O = terrars::ListRef<Pinpointsmsvoicev2PoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Pinpointsmsvoicev2Pool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_pinpointsmsvoicev2_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPinpointsmsvoicev2Pool {
    pub tf_id: std::string::String,
    #[doc = "Type of message."]
    pub message_type: terrars::PrimField<std::string::String>,
    #[doc =
        "Set of origination identity ARNs to associate with the pool. At least one origination identity is required at creation."]
    pub origination_identities: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPinpointsmsvoicev2Pool {
    pub fn build(self, stack: &mut terrars::Stack) -> Pinpointsmsvoicev2Pool {
        let out = Pinpointsmsvoicev2Pool(std::rc::Rc::new(Pinpointsmsvoicev2Pool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Pinpointsmsvoicev2PoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_protection_enabled: core::default::Default::default(),
                iso_country_code: core::default::Default::default(),
                message_type: self.message_type,
                opt_out_list_name: core::default::Default::default(),
                origination_identities: self.origination_identities,
                region: core::default::Default::default(),
                self_managed_opt_outs_enabled: core::default::Default::default(),
                shared_routes_enabled: core::default::Default::default(),
                tags: core::default::Default::default(),
                two_way_channel_arn: core::default::Default::default(),
                two_way_channel_role: core::default::Default::default(),
                two_way_enabled: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Pinpointsmsvoicev2PoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Pinpointsmsvoicev2PoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Pinpointsmsvoicev2PoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\nWhether deletion protection is enabled. When `true`, the pool cannot be deleted."]
    pub fn deletion_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iso_country_code` after provisioning.\nTwo-character code, in ISO 3166-1 alpha-2 format, for the country or region of the pool. This field is optional for origination identity types that are not country-specific."]
    pub fn iso_country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iso_country_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `message_type` after provisioning.\nType of message."]
    pub fn message_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `opt_out_list_name` after provisioning.\nName of the opt-out list to associate with the pool. Inherited from the initial origination identity when omitted."]
    pub fn opt_out_list_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opt_out_list_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origination_identities` after provisioning.\nSet of origination identity ARNs to associate with the pool. At least one origination identity is required at creation."]
    pub fn origination_identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.origination_identities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_managed_opt_outs_enabled` after provisioning.\nWhether the pool relies on self-managed opt-out handling. When `false`, AWS auto-replies to HELP/STOP requests and manages the opt-out list. Inherited from the initial origination identity when omitted."]
    pub fn self_managed_opt_outs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_managed_opt_outs_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `shared_routes_enabled` after provisioning.\nWhether shared routes are enabled for the pool. When `true`, messages may use shared phone numbers or sender IDs in countries that allow it."]
    pub fn shared_routes_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_routes_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `two_way_channel_arn` after provisioning.\nARN of the two-way channel that receives inbound messages."]
    pub fn two_way_channel_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.two_way_channel_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `two_way_channel_role` after provisioning.\nARN of the IAM role that End User Messaging SMS assumes to publish inbound messages to the two-way channel."]
    pub fn two_way_channel_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.two_way_channel_role", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `two_way_enabled` after provisioning.\nWhether inbound message reception is enabled for the pool."]
    pub fn two_way_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.two_way_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Pinpointsmsvoicev2Pool_TimeoutsRef {
        Pinpointsmsvoicev2Pool_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Pinpointsmsvoicev2Pool_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Pinpointsmsvoicev2Pool_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Pinpointsmsvoicev2Pool_Timeouts {
    type O = terrars::BlockAssignable<Pinpointsmsvoicev2Pool_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPinpointsmsvoicev2Pool_Timeouts {}

impl BuildPinpointsmsvoicev2Pool_Timeouts {
    pub fn build(self) -> Pinpointsmsvoicev2Pool_Timeouts {
        Pinpointsmsvoicev2Pool_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Pinpointsmsvoicev2Pool_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Pinpointsmsvoicev2Pool_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Pinpointsmsvoicev2Pool_TimeoutsRef {
        Pinpointsmsvoicev2Pool_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Pinpointsmsvoicev2Pool_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

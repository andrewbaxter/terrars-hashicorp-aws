#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallVpcEndpointAssociationData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    firewall_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_mapping: std::option::Option<std::vec::Vec<NetworkfirewallVpcEndpointAssociation_SubnetMapping>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkfirewallVpcEndpointAssociation_Timeouts>,
    dynamic: NetworkfirewallVpcEndpointAssociationDynamic,
}

struct NetworkfirewallVpcEndpointAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkfirewallVpcEndpointAssociationData_>,
}

#[derive(Clone)]
pub struct NetworkfirewallVpcEndpointAssociation(std::rc::Rc<NetworkfirewallVpcEndpointAssociation_>);

impl NetworkfirewallVpcEndpointAssociation {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `subnet_mapping`.\n"]
    pub fn set_subnet_mapping(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallVpcEndpointAssociation_SubnetMapping>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subnet_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subnet_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkfirewallVpcEndpointAssociation_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_arn` after provisioning.\n"]
    pub fn firewall_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_arn` after provisioning.\n"]
    pub fn vpc_endpoint_association_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_association_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_id` after provisioning.\n"]
    pub fn vpc_endpoint_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_status` after provisioning.\n"]
    pub fn vpc_endpoint_association_status(
        &self,
    ) -> terrars::ListRef<NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatusRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_association_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> terrars::ListRef<NetworkfirewallVpcEndpointAssociation_SubnetMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallVpcEndpointAssociation_TimeoutsRef {
        NetworkfirewallVpcEndpointAssociation_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkfirewallVpcEndpointAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkfirewallVpcEndpointAssociation { }

impl terrars::ToListMappable for NetworkfirewallVpcEndpointAssociation {
    type O = terrars::ListRef<NetworkfirewallVpcEndpointAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkfirewallVpcEndpointAssociation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_networkfirewall_vpc_endpoint_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub firewall_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallVpcEndpointAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkfirewallVpcEndpointAssociation {
        let out = NetworkfirewallVpcEndpointAssociation(std::rc::Rc::new(NetworkfirewallVpcEndpointAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkfirewallVpcEndpointAssociationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                firewall_arn: self.firewall_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                vpc_id: self.vpc_id,
                subnet_mapping: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallVpcEndpointAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallVpcEndpointAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkfirewallVpcEndpointAssociationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_arn` after provisioning.\n"]
    pub fn firewall_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_arn` after provisioning.\n"]
    pub fn vpc_endpoint_association_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_association_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_id` after provisioning.\n"]
    pub fn vpc_endpoint_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_association_status` after provisioning.\n"]
    pub fn vpc_endpoint_association_status(
        &self,
    ) -> terrars::ListRef<NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatusRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_association_status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> terrars::ListRef<NetworkfirewallVpcEndpointAssociation_SubnetMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallVpcEndpointAssociation_TimeoutsRef {
        NetworkfirewallVpcEndpointAssociation_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallVpcEndpointAssociationDynamic {
    subnet_mapping: std::option::Option<terrars::DynamicBlock<NetworkfirewallVpcEndpointAssociation_SubnetMapping>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment {
    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment {
    type O =
        terrars
        ::BlockAssignable<
            NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment {}

impl BuildNetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment {
    pub fn build(
        self,
    ) -> NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment {
        NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment {
            endpoint_id: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_AttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_AttachmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_AttachmentRef {
        NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_AttachmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_AttachmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment: std
    ::option
    ::Option<
        terrars
        ::ListField<
            NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_Attachment,
                        >,
                    >,
    ) -> Self {
        self.attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState {
    type O =
        terrars
        ::BlockAssignable<NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState {}

impl BuildNetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState {
    pub fn build(self) -> NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState {
        NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState {
            attachment: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncStateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncStateRef {
        NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(
        &self,
    ) -> terrars
    ::ListRef<NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState_AttachmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    association_sync_state: std
    ::option
    ::Option<
        terrars::SetField<NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState>,
    >,
}

impl NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus {
    #[doc = "Set the field `association_sync_state`.\n"]
    pub fn set_association_sync_state(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncState,
                        >,
                    >,
    ) -> Self {
        self.association_sync_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus {
    type O = terrars::BlockAssignable<NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus {}

impl BuildNetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus {
    pub fn build(self) -> NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus {
        NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus {
            association_sync_state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatusRef {
        NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `association_sync_state` after provisioning.\n"]
    pub fn association_sync_state(
        &self,
    ) -> terrars::SetRef<NetworkfirewallVpcEndpointAssociation_VpcEndpointAssociationStatus_AssociationSyncStateRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.association_sync_state", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociation_SubnetMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_id: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallVpcEndpointAssociation_SubnetMapping {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallVpcEndpointAssociation_SubnetMapping {
    type O = terrars::BlockAssignable<NetworkfirewallVpcEndpointAssociation_SubnetMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociation_SubnetMapping {
    #[doc = ""]
    pub subnet_id: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallVpcEndpointAssociation_SubnetMapping {
    pub fn build(self) -> NetworkfirewallVpcEndpointAssociation_SubnetMapping {
        NetworkfirewallVpcEndpointAssociation_SubnetMapping {
            ip_address_type: core::default::Default::default(),
            subnet_id: self.subnet_id,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallVpcEndpointAssociation_SubnetMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallVpcEndpointAssociation_SubnetMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallVpcEndpointAssociation_SubnetMappingRef {
        NetworkfirewallVpcEndpointAssociation_SubnetMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociation_SubnetMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallVpcEndpointAssociation_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallVpcEndpointAssociation_Timeouts {
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
}

impl terrars::ToListMappable for NetworkfirewallVpcEndpointAssociation_Timeouts {
    type O = terrars::BlockAssignable<NetworkfirewallVpcEndpointAssociation_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallVpcEndpointAssociation_Timeouts {}

impl BuildNetworkfirewallVpcEndpointAssociation_Timeouts {
    pub fn build(self) -> NetworkfirewallVpcEndpointAssociation_Timeouts {
        NetworkfirewallVpcEndpointAssociation_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallVpcEndpointAssociation_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallVpcEndpointAssociation_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallVpcEndpointAssociation_TimeoutsRef {
        NetworkfirewallVpcEndpointAssociation_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallVpcEndpointAssociation_TimeoutsRef {
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
}

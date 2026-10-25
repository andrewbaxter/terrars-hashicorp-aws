#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Sesv2MultiRegionEndpointData_ {
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
    endpoint_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<std::vec::Vec<Sesv2MultiRegionEndpoint_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Sesv2MultiRegionEndpoint_Timeouts>,
    dynamic: Sesv2MultiRegionEndpointDynamic,
}

struct Sesv2MultiRegionEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Sesv2MultiRegionEndpointData_>,
}

#[derive(Clone)]
pub struct Sesv2MultiRegionEndpoint(std::rc::Rc<Sesv2MultiRegionEndpoint_>);

impl Sesv2MultiRegionEndpoint {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2MultiRegionEndpoint_Details>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Sesv2MultiRegionEndpoint_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<Sesv2MultiRegionEndpoint_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<Sesv2MultiRegionEndpoint_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Sesv2MultiRegionEndpoint_TimeoutsRef {
        Sesv2MultiRegionEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Sesv2MultiRegionEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Sesv2MultiRegionEndpoint { }

impl terrars::ToListMappable for Sesv2MultiRegionEndpoint {
    type O = terrars::ListRef<Sesv2MultiRegionEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Sesv2MultiRegionEndpoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sesv2_multi_region_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesv2MultiRegionEndpoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub endpoint_name: terrars::PrimField<std::string::String>,
}

impl BuildSesv2MultiRegionEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> Sesv2MultiRegionEndpoint {
        let out = Sesv2MultiRegionEndpoint(std::rc::Rc::new(Sesv2MultiRegionEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Sesv2MultiRegionEndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                endpoint_name: self.endpoint_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Sesv2MultiRegionEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2MultiRegionEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Sesv2MultiRegionEndpointRef {
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

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<Sesv2MultiRegionEndpoint_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<Sesv2MultiRegionEndpoint_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Sesv2MultiRegionEndpoint_TimeoutsRef {
        Sesv2MultiRegionEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Sesv2MultiRegionEndpointDynamic {
    details: std::option::Option<terrars::DynamicBlock<Sesv2MultiRegionEndpoint_Details>>,
}

#[derive(Serialize)]
pub struct Sesv2MultiRegionEndpoint_Routes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Sesv2MultiRegionEndpoint_Routes {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Sesv2MultiRegionEndpoint_Routes {
    type O = terrars::BlockAssignable<Sesv2MultiRegionEndpoint_Routes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2MultiRegionEndpoint_Routes {}

impl BuildSesv2MultiRegionEndpoint_Routes {
    pub fn build(self) -> Sesv2MultiRegionEndpoint_Routes {
        Sesv2MultiRegionEndpoint_Routes { region: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Sesv2MultiRegionEndpoint_RoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2MultiRegionEndpoint_RoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Sesv2MultiRegionEndpoint_RoutesRef {
        Sesv2MultiRegionEndpoint_RoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2MultiRegionEndpoint_RoutesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2MultiRegionEndpoint_Details_RoutesDetails {
    region: terrars::PrimField<std::string::String>,
}

impl Sesv2MultiRegionEndpoint_Details_RoutesDetails { }

impl terrars::ToListMappable for Sesv2MultiRegionEndpoint_Details_RoutesDetails {
    type O = terrars::BlockAssignable<Sesv2MultiRegionEndpoint_Details_RoutesDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2MultiRegionEndpoint_Details_RoutesDetails {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildSesv2MultiRegionEndpoint_Details_RoutesDetails {
    pub fn build(self) -> Sesv2MultiRegionEndpoint_Details_RoutesDetails {
        Sesv2MultiRegionEndpoint_Details_RoutesDetails { region: self.region }
    }
}

#[allow(dead_code)]
pub struct Sesv2MultiRegionEndpoint_Details_RoutesDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2MultiRegionEndpoint_Details_RoutesDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Sesv2MultiRegionEndpoint_Details_RoutesDetailsRef {
        Sesv2MultiRegionEndpoint_Details_RoutesDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2MultiRegionEndpoint_Details_RoutesDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2MultiRegionEndpoint_DetailsDynamic {
    routes_details: std::option::Option<terrars::DynamicBlock<Sesv2MultiRegionEndpoint_Details_RoutesDetails>>,
}

#[derive(Serialize)]
pub struct Sesv2MultiRegionEndpoint_Details {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes_details: std::option::Option<std::vec::Vec<Sesv2MultiRegionEndpoint_Details_RoutesDetails>>,
    dynamic: Sesv2MultiRegionEndpoint_DetailsDynamic,
}

impl Sesv2MultiRegionEndpoint_Details {
    #[doc = "Set the field `routes_details`.\n"]
    pub fn set_routes_details(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2MultiRegionEndpoint_Details_RoutesDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.routes_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.routes_details = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Sesv2MultiRegionEndpoint_Details {
    type O = terrars::BlockAssignable<Sesv2MultiRegionEndpoint_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2MultiRegionEndpoint_Details {}

impl BuildSesv2MultiRegionEndpoint_Details {
    pub fn build(self) -> Sesv2MultiRegionEndpoint_Details {
        Sesv2MultiRegionEndpoint_Details {
            routes_details: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Sesv2MultiRegionEndpoint_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2MultiRegionEndpoint_DetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Sesv2MultiRegionEndpoint_DetailsRef {
        Sesv2MultiRegionEndpoint_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2MultiRegionEndpoint_DetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `routes_details` after provisioning.\n"]
    pub fn routes_details(&self) -> terrars::ListRef<Sesv2MultiRegionEndpoint_Details_RoutesDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes_details", self.base))
    }
}

#[derive(Serialize)]
pub struct Sesv2MultiRegionEndpoint_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Sesv2MultiRegionEndpoint_Timeouts {
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

impl terrars::ToListMappable for Sesv2MultiRegionEndpoint_Timeouts {
    type O = terrars::BlockAssignable<Sesv2MultiRegionEndpoint_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2MultiRegionEndpoint_Timeouts {}

impl BuildSesv2MultiRegionEndpoint_Timeouts {
    pub fn build(self) -> Sesv2MultiRegionEndpoint_Timeouts {
        Sesv2MultiRegionEndpoint_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Sesv2MultiRegionEndpoint_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2MultiRegionEndpoint_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Sesv2MultiRegionEndpoint_TimeoutsRef {
        Sesv2MultiRegionEndpoint_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2MultiRegionEndpoint_TimeoutsRef {
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

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudsearchDomainData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_az: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_options: std::option::Option<std::vec::Vec<CloudsearchDomainEndpointOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_field: std::option::Option<std::vec::Vec<CloudsearchDomainIndexFieldEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_parameters: std::option::Option<std::vec::Vec<CloudsearchDomainScalingParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudsearchDomainTimeoutsEl>,
    dynamic: CloudsearchDomainDynamic,
}

struct CloudsearchDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudsearchDomainData>,
}

#[derive(Clone)]
pub struct CloudsearchDomain(std::rc::Rc<CloudsearchDomain_>);

impl CloudsearchDomain {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multi_az`.\n"]
    pub fn set_multi_az(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_az = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_options`.\n"]
    pub fn set_endpoint_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudsearchDomainEndpointOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `index_field`.\n"]
    pub fn set_index_field(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudsearchDomainIndexFieldEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().index_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.index_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_parameters`.\n"]
    pub fn set_scaling_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudsearchDomainScalingParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudsearchDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_service_endpoint` after provisioning.\n"]
    pub fn document_service_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_service_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_service_endpoint` after provisioning.\n"]
    pub fn search_service_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search_service_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_options` after provisioning.\n"]
    pub fn endpoint_options(&self) -> terrars::ListRef<CloudsearchDomainEndpointOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_parameters` after provisioning.\n"]
    pub fn scaling_parameters(&self) -> terrars::ListRef<CloudsearchDomainScalingParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudsearchDomainTimeoutsElRef {
        CloudsearchDomainTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudsearchDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudsearchDomain { }

impl terrars::ToListMappable for CloudsearchDomain {
    type O = terrars::ListRef<CloudsearchDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudsearchDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudsearch_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudsearchDomain {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudsearchDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudsearchDomain {
        let out = CloudsearchDomain(std::rc::Rc::new(CloudsearchDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudsearchDomainData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                multi_az: core::default::Default::default(),
                name: self.name,
                endpoint_options: core::default::Default::default(),
                index_field: core::default::Default::default(),
                scaling_parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudsearchDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudsearchDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudsearchDomainRef {
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

    #[doc = "Get a reference to the value of field `document_service_endpoint` after provisioning.\n"]
    pub fn document_service_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_service_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_service_endpoint` after provisioning.\n"]
    pub fn search_service_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search_service_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_options` after provisioning.\n"]
    pub fn endpoint_options(&self) -> terrars::ListRef<CloudsearchDomainEndpointOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_parameters` after provisioning.\n"]
    pub fn scaling_parameters(&self) -> terrars::ListRef<CloudsearchDomainScalingParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudsearchDomainTimeoutsElRef {
        CloudsearchDomainTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudsearchDomainEndpointOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_https: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_security_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudsearchDomainEndpointOptionsEl {
    #[doc = "Set the field `enforce_https`.\n"]
    pub fn set_enforce_https(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce_https = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_security_policy`.\n"]
    pub fn set_tls_security_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tls_security_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudsearchDomainEndpointOptionsEl {
    type O = terrars::BlockAssignable<CloudsearchDomainEndpointOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudsearchDomainEndpointOptionsEl {}

impl BuildCloudsearchDomainEndpointOptionsEl {
    pub fn build(self) -> CloudsearchDomainEndpointOptionsEl {
        CloudsearchDomainEndpointOptionsEl {
            enforce_https: core::default::Default::default(),
            tls_security_policy: core::default::Default::default(),
        }
    }
}

pub struct CloudsearchDomainEndpointOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudsearchDomainEndpointOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudsearchDomainEndpointOptionsElRef {
        CloudsearchDomainEndpointOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudsearchDomainEndpointOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforce_https` after provisioning.\n"]
    pub fn enforce_https(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_https", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_security_policy` after provisioning.\n"]
    pub fn tls_security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_security_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudsearchDomainIndexFieldEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facet: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    highlight: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "return", skip_serializing_if = "std::option::Option::is_none")]
    return_: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_fields: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CloudsearchDomainIndexFieldEl {
    #[doc = "Set the field `analysis_scheme`.\n"]
    pub fn set_analysis_scheme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.analysis_scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `facet`.\n"]
    pub fn set_facet(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.facet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `highlight`.\n"]
    pub fn set_highlight(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.highlight = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `return_`.\n"]
    pub fn set_return(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.return_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search`.\n"]
    pub fn set_search(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sort`.\n"]
    pub fn set_sort(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sort = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_fields`.\n"]
    pub fn set_source_fields(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_fields = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudsearchDomainIndexFieldEl {
    type O = terrars::BlockAssignable<CloudsearchDomainIndexFieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudsearchDomainIndexFieldEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCloudsearchDomainIndexFieldEl {
    pub fn build(self) -> CloudsearchDomainIndexFieldEl {
        CloudsearchDomainIndexFieldEl {
            analysis_scheme: core::default::Default::default(),
            default_value: core::default::Default::default(),
            facet: core::default::Default::default(),
            highlight: core::default::Default::default(),
            name: self.name,
            return_: core::default::Default::default(),
            search: core::default::Default::default(),
            sort: core::default::Default::default(),
            source_fields: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct CloudsearchDomainIndexFieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudsearchDomainIndexFieldElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudsearchDomainIndexFieldElRef {
        CloudsearchDomainIndexFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudsearchDomainIndexFieldElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analysis_scheme` after provisioning.\n"]
    pub fn analysis_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analysis_scheme", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.base))
    }

    #[doc = "Get a reference to the value of field `facet` after provisioning.\n"]
    pub fn facet(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.facet", self.base))
    }

    #[doc = "Get a reference to the value of field `highlight` after provisioning.\n"]
    pub fn highlight(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.highlight", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `return_` after provisioning.\n"]
    pub fn return_(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return", self.base))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\n"]
    pub fn search(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search", self.base))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sort", self.base))
    }

    #[doc = "Get a reference to the value of field `source_fields` after provisioning.\n"]
    pub fn source_fields(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudsearchDomainScalingParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_partition_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_replication_count: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudsearchDomainScalingParametersEl {
    #[doc = "Set the field `desired_instance_type`.\n"]
    pub fn set_desired_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.desired_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_partition_count`.\n"]
    pub fn set_desired_partition_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.desired_partition_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_replication_count`.\n"]
    pub fn set_desired_replication_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.desired_replication_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudsearchDomainScalingParametersEl {
    type O = terrars::BlockAssignable<CloudsearchDomainScalingParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudsearchDomainScalingParametersEl {}

impl BuildCloudsearchDomainScalingParametersEl {
    pub fn build(self) -> CloudsearchDomainScalingParametersEl {
        CloudsearchDomainScalingParametersEl {
            desired_instance_type: core::default::Default::default(),
            desired_partition_count: core::default::Default::default(),
            desired_replication_count: core::default::Default::default(),
        }
    }
}

pub struct CloudsearchDomainScalingParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudsearchDomainScalingParametersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudsearchDomainScalingParametersElRef {
        CloudsearchDomainScalingParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudsearchDomainScalingParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_instance_type` after provisioning.\n"]
    pub fn desired_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_partition_count` after provisioning.\n"]
    pub fn desired_partition_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_partition_count", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_replication_count` after provisioning.\n"]
    pub fn desired_replication_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_replication_count", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudsearchDomainTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudsearchDomainTimeoutsEl {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudsearchDomainTimeoutsEl {
    type O = terrars::BlockAssignable<CloudsearchDomainTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudsearchDomainTimeoutsEl {}

impl BuildCloudsearchDomainTimeoutsEl {
    pub fn build(self) -> CloudsearchDomainTimeoutsEl {
        CloudsearchDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudsearchDomainTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudsearchDomainTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudsearchDomainTimeoutsElRef {
        CloudsearchDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudsearchDomainTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudsearchDomainDynamic {
    endpoint_options: std::option::Option<terrars::DynamicBlock<CloudsearchDomainEndpointOptionsEl>>,
    index_field: std::option::Option<terrars::DynamicBlock<CloudsearchDomainIndexFieldEl>>,
    scaling_parameters: std::option::Option<terrars::DynamicBlock<CloudsearchDomainScalingParametersEl>>,
}

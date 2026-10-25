#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct QuicksightCustomPermissionsData_ {
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
    aws_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    custom_permissions_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities: std::option::Option<std::vec::Vec<QuicksightCustomPermissions_Capabilities>>,
    dynamic: QuicksightCustomPermissionsDynamic,
}

struct QuicksightCustomPermissions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<QuicksightCustomPermissionsData_>,
}

#[derive(Clone)]
pub struct QuicksightCustomPermissions(std::rc::Rc<QuicksightCustomPermissions_>);

impl QuicksightCustomPermissions {
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

    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightCustomPermissions_Capabilities>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capabilities = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capabilities = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_permissions_name` after provisioning.\n"]
    pub fn custom_permissions_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_permissions_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> terrars::ListRef<QuicksightCustomPermissions_CapabilitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capabilities", self.extract_ref()))
    }
}

impl terrars::Referable for QuicksightCustomPermissions {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for QuicksightCustomPermissions { }

impl terrars::ToListMappable for QuicksightCustomPermissions {
    type O = terrars::ListRef<QuicksightCustomPermissionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for QuicksightCustomPermissions_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_quicksight_custom_permissions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildQuicksightCustomPermissions {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub custom_permissions_name: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightCustomPermissions {
    pub fn build(self, stack: &mut terrars::Stack) -> QuicksightCustomPermissions {
        let out = QuicksightCustomPermissions(std::rc::Rc::new(QuicksightCustomPermissions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(QuicksightCustomPermissionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aws_account_id: core::default::Default::default(),
                custom_permissions_name: self.custom_permissions_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                capabilities: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct QuicksightCustomPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightCustomPermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl QuicksightCustomPermissionsRef {
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

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_permissions_name` after provisioning.\n"]
    pub fn custom_permissions_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_permissions_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> terrars::ListRef<QuicksightCustomPermissions_CapabilitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capabilities", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct QuicksightCustomPermissionsDynamic {
    capabilities: std::option::Option<terrars::DynamicBlock<QuicksightCustomPermissions_Capabilities>>,
}

#[derive(Serialize)]
pub struct QuicksightCustomPermissions_Capabilities {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_or_run_anomaly_detection_for_analyses: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chat_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_and_update_dashboard_email_reports: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_and_update_data_sources: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_and_update_datasets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_and_update_themes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_and_update_threshold_alerts: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_chat_agents: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_shared_folders: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_spice_dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dashboard: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_to_csv: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_to_csv_in_scheduled_reports: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_to_excel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_to_excel_in_scheduled_reports: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_to_pdf: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_to_pdf_in_scheduled_reports: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_content_in_scheduled_reports_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_base: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    perform_flow_ui_task: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    print_reports: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_without_approval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rename_shared_folders: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    research: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_analyses: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_dashboards: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_data_sources: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_datasets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscribe_dashboard_email_reports: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_agent_web_search: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_bedrock_models: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_account_spice_capacity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightCustomPermissions_Capabilities {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `add_or_run_anomaly_detection_for_analyses`.\n"]
    pub fn set_add_or_run_anomaly_detection_for_analyses(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.add_or_run_anomaly_detection_for_analyses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.analysis = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automate`.\n"]
    pub fn set_automate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.automate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chat_agent`.\n"]
    pub fn set_chat_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.chat_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_and_update_dashboard_email_reports`.\n"]
    pub fn set_create_and_update_dashboard_email_reports(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_and_update_dashboard_email_reports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_and_update_data_sources`.\n"]
    pub fn set_create_and_update_data_sources(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_and_update_data_sources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_and_update_datasets`.\n"]
    pub fn set_create_and_update_datasets(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_and_update_datasets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_and_update_themes`.\n"]
    pub fn set_create_and_update_themes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_and_update_themes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_and_update_threshold_alerts`.\n"]
    pub fn set_create_and_update_threshold_alerts(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_and_update_threshold_alerts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_chat_agents`.\n"]
    pub fn set_create_chat_agents(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_chat_agents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_shared_folders`.\n"]
    pub fn set_create_shared_folders(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_shared_folders = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_spice_dataset`.\n"]
    pub fn set_create_spice_dataset(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.create_spice_dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dashboard`.\n"]
    pub fn set_dashboard(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dashboard = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export_to_csv`.\n"]
    pub fn set_export_to_csv(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export_to_csv = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export_to_csv_in_scheduled_reports`.\n"]
    pub fn set_export_to_csv_in_scheduled_reports(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.export_to_csv_in_scheduled_reports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export_to_excel`.\n"]
    pub fn set_export_to_excel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export_to_excel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export_to_excel_in_scheduled_reports`.\n"]
    pub fn set_export_to_excel_in_scheduled_reports(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.export_to_excel_in_scheduled_reports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export_to_pdf`.\n"]
    pub fn set_export_to_pdf(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export_to_pdf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export_to_pdf_in_scheduled_reports`.\n"]
    pub fn set_export_to_pdf_in_scheduled_reports(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.export_to_pdf_in_scheduled_reports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flow`.\n"]
    pub fn set_flow(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_content_in_scheduled_reports_email`.\n"]
    pub fn set_include_content_in_scheduled_reports_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.include_content_in_scheduled_reports_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `knowledge_base`.\n"]
    pub fn set_knowledge_base(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.knowledge_base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `perform_flow_ui_task`.\n"]
    pub fn set_perform_flow_ui_task(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.perform_flow_ui_task = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `print_reports`.\n"]
    pub fn set_print_reports(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.print_reports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `publish_without_approval`.\n"]
    pub fn set_publish_without_approval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.publish_without_approval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rename_shared_folders`.\n"]
    pub fn set_rename_shared_folders(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rename_shared_folders = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `research`.\n"]
    pub fn set_research(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.research = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_analyses`.\n"]
    pub fn set_share_analyses(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.share_analyses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_dashboards`.\n"]
    pub fn set_share_dashboards(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.share_dashboards = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_data_sources`.\n"]
    pub fn set_share_data_sources(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.share_data_sources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_datasets`.\n"]
    pub fn set_share_datasets(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.share_datasets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `space`.\n"]
    pub fn set_space(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.space = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscribe_dashboard_email_reports`.\n"]
    pub fn set_subscribe_dashboard_email_reports(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.subscribe_dashboard_email_reports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_agent_web_search`.\n"]
    pub fn set_use_agent_web_search(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.use_agent_web_search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_bedrock_models`.\n"]
    pub fn set_use_bedrock_models(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.use_bedrock_models = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_account_spice_capacity`.\n"]
    pub fn set_view_account_spice_capacity(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.view_account_spice_capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightCustomPermissions_Capabilities {
    type O = terrars::BlockAssignable<QuicksightCustomPermissions_Capabilities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightCustomPermissions_Capabilities {}

impl BuildQuicksightCustomPermissions_Capabilities {
    pub fn build(self) -> QuicksightCustomPermissions_Capabilities {
        QuicksightCustomPermissions_Capabilities {
            action: core::default::Default::default(),
            add_or_run_anomaly_detection_for_analyses: core::default::Default::default(),
            analysis: core::default::Default::default(),
            automate: core::default::Default::default(),
            chat_agent: core::default::Default::default(),
            create_and_update_dashboard_email_reports: core::default::Default::default(),
            create_and_update_data_sources: core::default::Default::default(),
            create_and_update_datasets: core::default::Default::default(),
            create_and_update_themes: core::default::Default::default(),
            create_and_update_threshold_alerts: core::default::Default::default(),
            create_chat_agents: core::default::Default::default(),
            create_shared_folders: core::default::Default::default(),
            create_spice_dataset: core::default::Default::default(),
            dashboard: core::default::Default::default(),
            export_to_csv: core::default::Default::default(),
            export_to_csv_in_scheduled_reports: core::default::Default::default(),
            export_to_excel: core::default::Default::default(),
            export_to_excel_in_scheduled_reports: core::default::Default::default(),
            export_to_pdf: core::default::Default::default(),
            export_to_pdf_in_scheduled_reports: core::default::Default::default(),
            flow: core::default::Default::default(),
            include_content_in_scheduled_reports_email: core::default::Default::default(),
            knowledge_base: core::default::Default::default(),
            perform_flow_ui_task: core::default::Default::default(),
            print_reports: core::default::Default::default(),
            publish_without_approval: core::default::Default::default(),
            rename_shared_folders: core::default::Default::default(),
            research: core::default::Default::default(),
            share_analyses: core::default::Default::default(),
            share_dashboards: core::default::Default::default(),
            share_data_sources: core::default::Default::default(),
            share_datasets: core::default::Default::default(),
            space: core::default::Default::default(),
            subscribe_dashboard_email_reports: core::default::Default::default(),
            use_agent_web_search: core::default::Default::default(),
            use_bedrock_models: core::default::Default::default(),
            view_account_spice_capacity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightCustomPermissions_CapabilitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightCustomPermissions_CapabilitiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightCustomPermissions_CapabilitiesRef {
        QuicksightCustomPermissions_CapabilitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightCustomPermissions_CapabilitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `add_or_run_anomaly_detection_for_analyses` after provisioning.\n"]
    pub fn add_or_run_anomaly_detection_for_analyses(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.add_or_run_anomaly_detection_for_analyses", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `automate` after provisioning.\n"]
    pub fn automate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automate", self.base))
    }

    #[doc = "Get a reference to the value of field `chat_agent` after provisioning.\n"]
    pub fn chat_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chat_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `create_and_update_dashboard_email_reports` after provisioning.\n"]
    pub fn create_and_update_dashboard_email_reports(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_and_update_dashboard_email_reports", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `create_and_update_data_sources` after provisioning.\n"]
    pub fn create_and_update_data_sources(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_and_update_data_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `create_and_update_datasets` after provisioning.\n"]
    pub fn create_and_update_datasets(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_and_update_datasets", self.base))
    }

    #[doc = "Get a reference to the value of field `create_and_update_themes` after provisioning.\n"]
    pub fn create_and_update_themes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_and_update_themes", self.base))
    }

    #[doc = "Get a reference to the value of field `create_and_update_threshold_alerts` after provisioning.\n"]
    pub fn create_and_update_threshold_alerts(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_and_update_threshold_alerts", self.base))
    }

    #[doc = "Get a reference to the value of field `create_chat_agents` after provisioning.\n"]
    pub fn create_chat_agents(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_chat_agents", self.base))
    }

    #[doc = "Get a reference to the value of field `create_shared_folders` after provisioning.\n"]
    pub fn create_shared_folders(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_shared_folders", self.base))
    }

    #[doc = "Get a reference to the value of field `create_spice_dataset` after provisioning.\n"]
    pub fn create_spice_dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_spice_dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `dashboard` after provisioning.\n"]
    pub fn dashboard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard", self.base))
    }

    #[doc = "Get a reference to the value of field `export_to_csv` after provisioning.\n"]
    pub fn export_to_csv(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_to_csv", self.base))
    }

    #[doc = "Get a reference to the value of field `export_to_csv_in_scheduled_reports` after provisioning.\n"]
    pub fn export_to_csv_in_scheduled_reports(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_to_csv_in_scheduled_reports", self.base))
    }

    #[doc = "Get a reference to the value of field `export_to_excel` after provisioning.\n"]
    pub fn export_to_excel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_to_excel", self.base))
    }

    #[doc = "Get a reference to the value of field `export_to_excel_in_scheduled_reports` after provisioning.\n"]
    pub fn export_to_excel_in_scheduled_reports(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_to_excel_in_scheduled_reports", self.base))
    }

    #[doc = "Get a reference to the value of field `export_to_pdf` after provisioning.\n"]
    pub fn export_to_pdf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_to_pdf", self.base))
    }

    #[doc = "Get a reference to the value of field `export_to_pdf_in_scheduled_reports` after provisioning.\n"]
    pub fn export_to_pdf_in_scheduled_reports(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_to_pdf_in_scheduled_reports", self.base))
    }

    #[doc = "Get a reference to the value of field `flow` after provisioning.\n"]
    pub fn flow(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow", self.base))
    }

    #[doc = "Get a reference to the value of field `include_content_in_scheduled_reports_email` after provisioning.\n"]
    pub fn include_content_in_scheduled_reports_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_content_in_scheduled_reports_email", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `knowledge_base` after provisioning.\n"]
    pub fn knowledge_base(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base", self.base))
    }

    #[doc = "Get a reference to the value of field `perform_flow_ui_task` after provisioning.\n"]
    pub fn perform_flow_ui_task(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.perform_flow_ui_task", self.base))
    }

    #[doc = "Get a reference to the value of field `print_reports` after provisioning.\n"]
    pub fn print_reports(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.print_reports", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_without_approval` after provisioning.\n"]
    pub fn publish_without_approval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_without_approval", self.base))
    }

    #[doc = "Get a reference to the value of field `rename_shared_folders` after provisioning.\n"]
    pub fn rename_shared_folders(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rename_shared_folders", self.base))
    }

    #[doc = "Get a reference to the value of field `research` after provisioning.\n"]
    pub fn research(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.research", self.base))
    }

    #[doc = "Get a reference to the value of field `share_analyses` after provisioning.\n"]
    pub fn share_analyses(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_analyses", self.base))
    }

    #[doc = "Get a reference to the value of field `share_dashboards` after provisioning.\n"]
    pub fn share_dashboards(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_dashboards", self.base))
    }

    #[doc = "Get a reference to the value of field `share_data_sources` after provisioning.\n"]
    pub fn share_data_sources(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_data_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `share_datasets` after provisioning.\n"]
    pub fn share_datasets(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_datasets", self.base))
    }

    #[doc = "Get a reference to the value of field `space` after provisioning.\n"]
    pub fn space(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space", self.base))
    }

    #[doc = "Get a reference to the value of field `subscribe_dashboard_email_reports` after provisioning.\n"]
    pub fn subscribe_dashboard_email_reports(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscribe_dashboard_email_reports", self.base))
    }

    #[doc = "Get a reference to the value of field `use_agent_web_search` after provisioning.\n"]
    pub fn use_agent_web_search(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_agent_web_search", self.base))
    }

    #[doc = "Get a reference to the value of field `use_bedrock_models` after provisioning.\n"]
    pub fn use_bedrock_models(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_bedrock_models", self.base))
    }

    #[doc = "Get a reference to the value of field `view_account_spice_capacity` after provisioning.\n"]
    pub fn view_account_spice_capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_account_spice_capacity", self.base))
    }
}

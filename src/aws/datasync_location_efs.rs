use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatasyncLocationEfsData {
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
    access_point_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    efs_file_system_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_access_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    in_transit_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subdirectory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_config: std::option::Option<std::vec::Vec<DatasyncLocationEfsEc2ConfigEl>>,
    dynamic: DatasyncLocationEfsDynamic,
}

struct DatasyncLocationEfs_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatasyncLocationEfsData>,
}

#[derive(Clone)]
pub struct DatasyncLocationEfs(std::rc::Rc<DatasyncLocationEfs_>);

impl DatasyncLocationEfs {
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

    #[doc = "Set the field `access_point_arn`.\n"]
    pub fn set_access_point_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().access_point_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_access_role_arn`.\n"]
    pub fn set_file_system_access_role_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().file_system_access_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `in_transit_encryption`.\n"]
    pub fn set_in_transit_encryption(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().in_transit_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subdirectory`.\n"]
    pub fn set_subdirectory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subdirectory = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ec2_config`.\n"]
    pub fn set_ec2_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatasyncLocationEfsEc2ConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `efs_file_system_arn` after provisioning.\n"]
    pub fn efs_file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efs_file_system_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_access_role_arn` after provisioning.\n"]
    pub fn file_system_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `in_transit_encryption` after provisioning.\n"]
    pub fn in_transit_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.in_transit_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_config` after provisioning.\n"]
    pub fn ec2_config(&self) -> terrars::ListRef<DatasyncLocationEfsEc2ConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ec2_config", self.extract_ref()))
    }
}

impl terrars::Referable for DatasyncLocationEfs {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatasyncLocationEfs { }

impl terrars::ToListMappable for DatasyncLocationEfs {
    type O = terrars::ListRef<DatasyncLocationEfsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatasyncLocationEfs_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_datasync_location_efs".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncLocationEfs {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub efs_file_system_arn: terrars::PrimField<std::string::String>,
}

impl BuildDatasyncLocationEfs {
    pub fn build(self, stack: &mut terrars::Stack) -> DatasyncLocationEfs {
        let out = DatasyncLocationEfs(std::rc::Rc::new(DatasyncLocationEfs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatasyncLocationEfsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_point_arn: core::default::Default::default(),
                efs_file_system_arn: self.efs_file_system_arn,
                file_system_access_role_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                in_transit_encryption: core::default::Default::default(),
                subdirectory: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                ec2_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncLocationEfsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncLocationEfsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatasyncLocationEfsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `efs_file_system_arn` after provisioning.\n"]
    pub fn efs_file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efs_file_system_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_access_role_arn` after provisioning.\n"]
    pub fn file_system_access_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `in_transit_encryption` after provisioning.\n"]
    pub fn in_transit_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.in_transit_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_config` after provisioning.\n"]
    pub fn ec2_config(&self) -> terrars::ListRef<DatasyncLocationEfsEc2ConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ec2_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatasyncLocationEfsEc2ConfigEl {
    security_group_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_arn: terrars::PrimField<std::string::String>,
}

impl DatasyncLocationEfsEc2ConfigEl { }

impl terrars::ToListMappable for DatasyncLocationEfsEc2ConfigEl {
    type O = terrars::BlockAssignable<DatasyncLocationEfsEc2ConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationEfsEc2ConfigEl {
    #[doc = ""]
    pub security_group_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_arn: terrars::PrimField<std::string::String>,
}

impl BuildDatasyncLocationEfsEc2ConfigEl {
    pub fn build(self) -> DatasyncLocationEfsEc2ConfigEl {
        DatasyncLocationEfsEc2ConfigEl {
            security_group_arns: self.security_group_arns,
            subnet_arn: self.subnet_arn,
        }
    }
}

pub struct DatasyncLocationEfsEc2ConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncLocationEfsEc2ConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatasyncLocationEfsEc2ConfigElRef {
        DatasyncLocationEfsEc2ConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationEfsEc2ConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_arns` after provisioning.\n"]
    pub fn security_group_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_arn` after provisioning.\n"]
    pub fn subnet_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationEfsDynamic {
    ec2_config: std::option::Option<terrars::DynamicBlock<DatasyncLocationEfsEc2ConfigEl>>,
}

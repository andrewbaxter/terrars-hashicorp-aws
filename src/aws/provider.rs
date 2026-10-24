use serde::Serialize;

#[derive(Serialize)]
struct ProviderAwsData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_account_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_ca_bundle: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_metadata_service_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_metadata_service_endpoint_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forbidden_account_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_proxy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_force_path_style: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_use_path_style: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_config_files: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_credentials_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_credentials_files: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_credentials_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_get_ec2_platforms: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_metadata_api_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_region_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_requesting_account_id: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sts_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_dualstack_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_fips_endpoint: std::option::Option<terrars::PrimField<bool>>,
}

struct ProviderAws_ {
    data: std::cell::RefCell<ProviderAwsData>,
}

pub struct ProviderAws(std::rc::Rc<ProviderAws_>);

impl ProviderAws {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "aws", alias)
        } else {
            "aws".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc =
        "Set the field `access_key`.\nThe access key for API operations. You can retrieve this\nfrom the 'Security & Credentials' section of the AWS console."]
    pub fn set_access_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().access_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_account_ids`.\n"]
    pub fn set_allowed_account_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().allowed_account_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_ca_bundle`.\nFile containing custom root and intermediate certificates. Can also be configured using the `AWS_CA_BUNDLE` environment variable. (Setting `ca_bundle` in the shared config file is not supported.)"]
    pub fn set_custom_ca_bundle(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_ca_bundle = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ec2_metadata_service_endpoint`.\nAddress of the EC2 metadata service endpoint to use. Can also be configured using the `AWS_EC2_METADATA_SERVICE_ENDPOINT` environment variable."]
    pub fn set_ec2_metadata_service_endpoint(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().ec2_metadata_service_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ec2_metadata_service_endpoint_mode`.\nProtocol to use with EC2 metadata service endpoint.Valid values are `IPv4` and `IPv6`. Can also be configured using the `AWS_EC2_METADATA_SERVICE_ENDPOINT_MODE` environment variable."]
    pub fn set_ec2_metadata_service_endpoint_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().ec2_metadata_service_endpoint_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `forbidden_account_ids`.\n"]
    pub fn set_forbidden_account_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().forbidden_account_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `http_proxy`.\nThe address of an HTTP proxy to use when accessing the AWS API. Can also be configured using the `HTTP_PROXY` or `HTTPS_PROXY` environment variables."]
    pub fn set_http_proxy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().http_proxy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `insecure`.\nExplicitly allow the provider to perform \"insecure\" SSL requests. If omitted, default value is `false`"]
    pub fn set_insecure(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_retries`.\nThe maximum number of times an AWS API request is\nbeing executed. If the API request still fails, an error is\nthrown."]
    pub fn set_max_retries(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `profile`.\nThe profile for API operations. If not set, the default profile\ncreated with `aws configure` will be used."]
    pub fn set_profile(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().profile = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe region where AWS operations will take place. Examples\nare us-east-1, us-west-2, etc."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `s3_force_path_style`.\nSet this to true to enable the request to use path-style addressing,\ni.e., https://s3.amazonaws.com/BUCKET/KEY. By default, the S3 client will\nuse virtual hosted bucket addressing when possible\n(https://BUCKET.s3.amazonaws.com/KEY). Specific to the Amazon S3 service."]
    pub fn set_s3_force_path_style(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().s3_force_path_style = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `s3_use_path_style`.\nSet this to true to enable the request to use path-style addressing,\ni.e., https://s3.amazonaws.com/BUCKET/KEY. By default, the S3 client will\nuse virtual hosted bucket addressing when possible\n(https://BUCKET.s3.amazonaws.com/KEY). Specific to the Amazon S3 service."]
    pub fn set_s3_use_path_style(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().s3_use_path_style = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_key`.\nThe secret key for API operations. You can retrieve this\nfrom the 'Security & Credentials' section of the AWS console."]
    pub fn set_secret_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().secret_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_config_files`.\nList of paths to shared config files. If not set, defaults to [~/.aws/config]."]
    pub fn set_shared_config_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().shared_config_files = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_credentials_file`.\nThe path to the shared credentials file. If not set, defaults to ~/.aws/credentials."]
    pub fn set_shared_credentials_file(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().shared_credentials_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shared_credentials_files`.\nList of paths to shared credentials files. If not set, defaults to [~/.aws/credentials]."]
    pub fn set_shared_credentials_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().shared_credentials_files = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_credentials_validation`.\nSkip the credentials validation via STS API. Used for AWS API implementations that do not have STS available/implemented."]
    pub fn set_skip_credentials_validation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_credentials_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_get_ec2_platforms`.\nSkip getting the supported EC2 platforms. Used by users that don't have ec2:DescribeAccountAttributes permissions."]
    pub fn set_skip_get_ec2_platforms(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_get_ec2_platforms = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_metadata_api_check`.\nSkip the AWS Metadata API check. Used for AWS API implementations that do not have a metadata api endpoint."]
    pub fn set_skip_metadata_api_check(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().skip_metadata_api_check = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_region_validation`.\nSkip static validation of region name. Used by users of alternative AWS-like APIs or users w/ access to regions that are not public (yet)."]
    pub fn set_skip_region_validation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_region_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_requesting_account_id`.\nSkip requesting the account ID. Used for AWS API implementations that do not have IAM/STS API and/or metadata API."]
    pub fn set_skip_requesting_account_id(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_requesting_account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sts_region`.\nThe region where AWS STS operations will take place. Examples\nare us-east-1 and us-west-2."]
    pub fn set_sts_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sts_region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `token`.\nsession token. A session token is only required if you are\nusing temporary security credentials."]
    pub fn set_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_dualstack_endpoint`.\nResolve an endpoint with DualStack capability"]
    pub fn set_use_dualstack_endpoint(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_dualstack_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_fips_endpoint`.\nResolve an endpoint with FIPS capability"]
    pub fn set_use_fips_endpoint(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_fips_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderAws_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "aws".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "hashicorp/aws",
            "version": "4.51.0",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderAws {}

impl BuildProviderAws {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderAws {
        let out = ProviderAws(std::rc::Rc::new(ProviderAws_ { data: std::cell::RefCell::new(ProviderAwsData {
            alias: std::option::Option::None,
            access_key: core::default::Default::default(),
            allowed_account_ids: core::default::Default::default(),
            custom_ca_bundle: core::default::Default::default(),
            ec2_metadata_service_endpoint: core::default::Default::default(),
            ec2_metadata_service_endpoint_mode: core::default::Default::default(),
            forbidden_account_ids: core::default::Default::default(),
            http_proxy: core::default::Default::default(),
            insecure: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            profile: core::default::Default::default(),
            region: core::default::Default::default(),
            s3_force_path_style: core::default::Default::default(),
            s3_use_path_style: core::default::Default::default(),
            secret_key: core::default::Default::default(),
            shared_config_files: core::default::Default::default(),
            shared_credentials_file: core::default::Default::default(),
            shared_credentials_files: core::default::Default::default(),
            skip_credentials_validation: core::default::Default::default(),
            skip_get_ec2_platforms: core::default::Default::default(),
            skip_metadata_api_check: core::default::Default::default(),
            skip_region_validation: core::default::Default::default(),
            skip_requesting_account_id: core::default::Default::default(),
            sts_region: core::default::Default::default(),
            token: core::default::Default::default(),
            use_dualstack_endpoint: core::default::Default::default(),
            use_fips_endpoint: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}

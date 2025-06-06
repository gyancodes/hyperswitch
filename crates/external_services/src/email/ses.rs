use std::time::{Duration, SystemTime};

use aws_sdk_sesv2::{
    config::Region,
    operation::send_email::SendEmailError,
    types::{Body, Content, Destination, EmailContent, Message},
    Client,
};
use aws_sdk_sts::config::Credentials;
use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
use common_utils::{errors::CustomResult, pii};
use error_stack::{report, ResultExt};
use hyper::Uri;
use masking::PeekInterface;
use router_env::logger;

use crate::email::{EmailClient, EmailError, EmailResult, EmailSettings, IntermediateString};

/// Client for AWS SES operation
#[derive(Debug, Clone)]
pub struct AwsSes {
    sender: String,
    ses_config: SESConfig,
    settings: EmailSettings,
}

/// Struct that contains the AWS ses specific configs required to construct an SES email client
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct SESConfig {
    /// The arn of email role
    pub email_role_arn: String,

    /// The name of sts_session role
    pub sts_role_session_name: String,
}

impl SESConfig {
    /// Validation for the SES client specific configs
    pub fn validate(&self) -> Result<(), &'static str> {
        use common_utils::{ext_traits::ConfigExt, fp_utils::when};

        when(self.email_role_arn.is_default_or_empty(), || {
            Err("email.aws_ses.email_role_arn must not be empty")
        })?;

        when(self.sts_role_session_name.is_default_or_empty(), || {
            Err("email.aws_ses.sts_role_session_name must not be empty")
        })
    }
}

/// Errors that could occur during SES operations.
#[derive(Debug, thiserror::Error)]
pub enum AwsSesError {
    /// An error occurred in the SDK while sending email.
    #[error("Failed to Send Email {0:?}")]
    SendingFailure(Box<aws_sdk_sesv2::error::SdkError<SendEmailError>>),

    /// Configuration variable is missing to construct the email client
    #[error("Missing configuration variable {0}")]
    MissingConfigurationVariable(&'static str),

    /// Failed to assume the given STS role
    #[error("Failed to STS assume role: Role ARN: {role_arn}, Session name: {session_name}, Region: {region}")]
    AssumeRoleFailure {
        /// Aws region
        region: String,

        /// arn of email role
        role_arn: String,

        /// The name of sts_session role
        session_name: String,
    },

    /// Temporary credentials are missing
    #[error("Assumed role does not contain credentials for role user: {0:?}")]
    TemporaryCredentialsMissing(String),

    /// The proxy Connector cannot be built
    #[error("The proxy build cannot be built")]
    BuildingProxyConnectorFailed,
}

impl AwsSes {
    /// Constructs a new AwsSes client
    pub async fn create(
        conf: &EmailSettings,
        ses_config: &SESConfig,
        proxy_url: Option<impl AsRef<str>>,
    ) -> Self {
        // Build the client initially which will help us know if the email configuration is correct
        Self::create_client(conf, ses_config, proxy_url)
            .await
            .map_err(|error| logger::error!(?error, "Failed to initialize SES Client"))
            .ok();

        Self {
            sender: conf.sender_email.clone(),
            ses_config: ses_config.clone(),
            settings: conf.clone(),
        }
    }

    /// A helper function to create ses client
    pub async fn create_client(
        conf: &EmailSettings,
        ses_config: &SESConfig,
        proxy_url: Option<impl AsRef<str>>,
    ) -> CustomResult<Client, AwsSesError> {
        let sts_config = Self::get_shared_config(conf.aws_region.to_owned(), proxy_url.as_ref())?
            .load()
            .await;

        let role = aws_sdk_sts::Client::new(&sts_config)
            .assume_role()
            .role_arn(&ses_config.email_role_arn)
            .role_session_name(&ses_config.sts_role_session_name)
            .send()
            .await
            .change_context(AwsSesError::AssumeRoleFailure {
                region: conf.aws_region.to_owned(),
                role_arn: ses_config.email_role_arn.to_owned(),
                session_name: ses_config.sts_role_session_name.to_owned(),
            })?;

        let creds = role.credentials().ok_or(
            report!(AwsSesError::TemporaryCredentialsMissing(format!(
                "{role:?}"
            )))
            .attach_printable("Credentials object not available"),
        )?;

        let credentials = Credentials::new(
            creds.access_key_id(),
            creds.secret_access_key(),
            Some(creds.session_token().to_owned()),
            u64::try_from(creds.expiration().as_nanos())
                .ok()
                .map(Duration::from_nanos)
                .and_then(|val| SystemTime::UNIX_EPOCH.checked_add(val)),
            "custom_provider",
        );

        logger::debug!(
            "Obtained SES temporary credentials with expiry {:?}",
            credentials.expiry()
        );

        let ses_config = Self::get_shared_config(conf.aws_region.to_owned(), proxy_url)?
            .credentials_provider(credentials)
            .load()
            .await;

        Ok(Client::new(&ses_config))
    }

    fn get_shared_config(
        region: String,
        proxy_url: Option<impl AsRef<str>>,
    ) -> CustomResult<aws_config::ConfigLoader, AwsSesError> {
        let region_provider = Region::new(region);
        let mut config = aws_config::from_env().region(region_provider);

        if let Some(proxy_url) = proxy_url {
            let proxy_connector = Self::get_proxy_connector(proxy_url)?;
            let http_client = HyperClientBuilder::new().build(proxy_connector);
            config = config.http_client(http_client);
        };
        Ok(config)
    }

    fn get_proxy_connector(
        proxy_url: impl AsRef<str>,
    ) -> CustomResult<hyper_proxy::ProxyConnector<hyper::client::HttpConnector>, AwsSesError> {
        let proxy_uri = proxy_url
            .as_ref()
            .parse::<Uri>()
            .attach_printable("Unable to parse the proxy url {proxy_url}")
            .change_context(AwsSesError::BuildingProxyConnectorFailed)?;

        let proxy = hyper_proxy::Proxy::new(hyper_proxy::Intercept::All, proxy_uri);

        hyper_proxy::ProxyConnector::from_proxy(hyper::client::HttpConnector::new(), proxy)
            .change_context(AwsSesError::BuildingProxyConnectorFailed)
    }
}

#[async_trait::async_trait]
impl EmailClient for AwsSes {
    type RichText = Body;

    fn convert_to_rich_text(
        &self,
        intermediate_string: IntermediateString,
    ) -> CustomResult<Self::RichText, EmailError> {
        let email_body = Body::builder()
            .html(
                Content::builder()
                    .data(intermediate_string.into_inner())
                    .charset("UTF-8")
                    .build()
                    .change_context(EmailError::ContentBuildFailure)?,
            )
            .build();

        Ok(email_body)
    }

    async fn send_email(
        &self,
        recipient: pii::Email,
        subject: String,
        body: Self::RichText,
        proxy_url: Option<&String>,
    ) -> EmailResult<()> {
        // Not using the same email client which was created at startup as the role session would expire
        // Create a client every time when the email is being sent
        let email_client = Self::create_client(&self.settings, &self.ses_config, proxy_url)
            .await
            .change_context(EmailError::ClientBuildingFailure)?;

        email_client
            .send_email()
            .from_email_address(self.sender.to_owned())
            .destination(
                Destination::builder()
                    .to_addresses(recipient.peek())
                    .build(),
            )
            .content(
                EmailContent::builder()
                    .simple(
                        Message::builder()
                            .subject(
                                Content::builder()
                                    .data(subject)
                                    .build()
                                    .change_context(EmailError::ContentBuildFailure)?,
                            )
                            .body(body)
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await
            .map_err(|e| AwsSesError::SendingFailure(Box::new(e)))
            .change_context(EmailError::EmailSendingFailure)?;

        Ok(())
    }
}

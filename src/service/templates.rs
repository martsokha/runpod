use std::future::Future;

#[cfg(feature = "tracing")]
use crate::TRACING_TARGET_SERVICE;
use crate::model::{
    GetTemplateQuery, ListTemplatesQuery, Template, TemplateCreateInput, TemplateUpdateInput,
    Templates,
};

use crate::{Result, RunpodClient};

/// Trait for managing templates.
///
/// Provides methods for creating, listing, retrieving, updating, and deleting templates.
/// This trait is implemented on the [`RunpodClient`](crate::client::RunpodClient).
pub trait TemplatesService {
    /// Creates a new template.
    ///
    /// # Arguments
    ///
    /// * `input` - Configuration for the new template
    ///
    /// # Returns
    ///
    /// Returns the created template information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::TemplateCreateInput;
    /// # use runpod_sdk::service::TemplatesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::from_env()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = TemplateCreateInput {
    ///     name: "My Template".to_string(),
    ///     image_name: "my-image:latest".to_string(),
    ///     docker_start_cmd: None,
    ///     container_registry_auth_id: None,
    ///     env: None,
    ///     readme: None,
    ///     is_public: None,
    ///     is_serverless: None,
    ///     ports: None,
    ///     volume_in_gb: None,
    ///     container_disk_in_gb: None,
    ///     volume_mount_path: None,
    ///     category: None,
    ///     docker_entrypoint: None,
    /// };
    ///
    /// let template = client.create_template(input).await?;
    /// println!("Created template: {}", template.id);
    /// # Ok(())
    /// # }
    /// ```
    fn create_template(&self, input: TemplateCreateInput)
    -> impl Future<Output = Result<Template>>;

    /// Lists templates with optional filtering.
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a vector of templates matching the query criteria.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::ListTemplatesQuery;
    /// # use runpod_sdk::service::TemplatesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = ListTemplatesQuery {
    ///     include_public_templates: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let templates = client.list_templates(query).await?;
    /// println!("Found {} templates", templates.len());
    /// # Ok(())
    /// # }
    /// ```
    fn list_templates(&self, query: ListTemplatesQuery) -> impl Future<Output = Result<Templates>>;

    /// Gets a specific template by ID.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The unique identifier of the template
    /// * `query` - Query parameters for including additional information
    ///
    /// # Returns
    ///
    /// Returns the template information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::GetTemplateQuery;
    /// # use runpod_sdk::service::TemplatesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = GetTemplateQuery::default();
    ///
    /// let template = client.get_template("template_id", query).await?;
    /// println!("Template: {:?}", template);
    /// # Ok(())
    /// # }
    /// ```
    fn get_template(
        &self,
        template_id: &str,
        query: GetTemplateQuery,
    ) -> impl Future<Output = Result<Template>>;

    /// Updates an existing template.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The unique identifier of the template to update
    /// * `input` - Update parameters for the template
    ///
    /// # Returns
    ///
    /// Returns the updated template information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::model::TemplateUpdateInput;
    /// # use runpod_sdk::service::TemplatesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = TemplateUpdateInput {
    ///     name: Some("Updated Template".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let template = client.update_template("template_id", input).await?;
    /// println!("Updated template: {}", template.id);
    /// # Ok(())
    /// # }
    /// ```
    fn update_template(
        &self,
        template_id: &str,
        input: TemplateUpdateInput,
    ) -> impl Future<Output = Result<Template>>;

    /// Deletes a template.
    ///
    /// This operation will permanently remove the template.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The unique identifier of the template to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig, Result};
    /// # use runpod_sdk::service::TemplatesService;
    /// # async fn example() -> Result<()> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.delete_template("template_id").await?;
    /// println!("Template deleted");
    /// # Ok(())
    /// # }
    /// ```
    fn delete_template(&self, template_id: &str) -> impl Future<Output = Result<()>>;
}

impl TemplatesService for RunpodClient {
    async fn create_template(&self, input: TemplateCreateInput) -> Result<Template> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Creating template");

        let response = self.post("/templates").json(&input).send().await?;
        let response = response.error_for_status()?;
        let template: Template = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, template_id = %template.id, "Template created successfully");

        Ok(template)
    }

    async fn list_templates(&self, query: ListTemplatesQuery) -> Result<Templates> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Listing templates");

        let response = self.get("/templates").query(&query).send().await?;
        let response = response.error_for_status()?;
        let templates: Templates = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, count = templates.len(), "Templates retrieved successfully");

        Ok(templates)
    }

    async fn get_template(&self, template_id: &str, query: GetTemplateQuery) -> Result<Template> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Getting template");

        let path = format!("/templates/{}", template_id);
        let response = self.get(&path).query(&query).send().await?;
        let response = response.error_for_status()?;
        let template: Template = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Template retrieved successfully");

        Ok(template)
    }

    async fn update_template(
        &self,
        template_id: &str,
        input: TemplateUpdateInput,
    ) -> Result<Template> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Updating template");

        let path = format!("/templates/{}", template_id);
        let response = self.patch(&path).json(&input).send().await?;
        let response = response.error_for_status()?;
        let template: Template = response.json().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Template updated successfully");

        Ok(template)
    }

    async fn delete_template(&self, template_id: &str) -> Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Deleting template");

        let path = format!("/templates/{}", template_id);
        let response = self.delete(&path).send().await?;
        response.error_for_status()?;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: TRACING_TARGET_SERVICE, "Template deleted successfully");

        Ok(())
    }
}

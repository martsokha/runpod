use crate::Result;
use crate::client::RunpodClient;
use crate::model::{
    GetTemplateQuery, ListTemplatesQuery, Template, TemplateCreateInput, TemplateUpdateInput,
    Templates,
};

/// Service for managing templates
#[derive(Debug, Clone)]
pub struct TemplatesService {
    client: RunpodClient,
}

impl TemplatesService {
    /// Creates a new templates service
    pub(crate) fn new(client: RunpodClient) -> Self {
        Self { client }
    }

    /// Creates a new template
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::TemplateCreateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = TemplateCreateInput {
    ///     name: "My Template".to_string(),
    ///     image_name: "runpod/pytorch:latest".to_string(),
    ///     is_serverless: Some(false),
    ///     ..Default::default()
    /// };
    ///
    /// let template = client.templates().create(input).await?;
    /// println!("Created template: {}", template.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, input: TemplateCreateInput) -> Result<Template> {
        let response = self.client.post("/templates").json(&input).send().await?;
        let template = response.json().await?;
        Ok(template)
    }

    /// Lists templates
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::ListTemplatesQuery;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = ListTemplatesQuery {
    ///     include_public_templates: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let templates = client.templates().list(query).await?;
    /// println!("Found {} templates", templates.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, query: ListTemplatesQuery) -> Result<Templates> {
        let response = self.client.get("/templates").query(&query).send().await?;
        let templates = response.json().await?;
        Ok(templates)
    }

    /// Gets a template by ID
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::GetTemplateQuery;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let query = GetTemplateQuery::default();
    ///
    /// let template = client.templates().get("template_id", query).await?;
    /// println!("Template: {:?}", template);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, template_id: &str, query: GetTemplateQuery) -> Result<Template> {
        let path = format!("/templates/{}", template_id);
        let response = self.client.get(&path).query(&query).send().await?;
        let template = response.json().await?;
        Ok(template)
    }

    /// Updates a template (triggers rolling release for associated endpoints)
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # use runpod_sdk::model::TemplateUpdateInput;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// let input = TemplateUpdateInput {
    ///     name: Some("Updated Template".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let template = client.templates().update("template_id", input).await?;
    /// println!("Updated template: {}", template.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, template_id: &str, input: TemplateUpdateInput) -> Result<Template> {
        let path = format!("/templates/{}", template_id);
        let response = self.client.patch(&path).json(&input).send().await?;
        let template = response.json().await?;
        Ok(template)
    }

    /// Deletes a template
    ///
    /// # Example
    /// ```no_run
    /// # use runpod_sdk::{RunpodClient, RunpodConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = RunpodConfig::builder().with_api_key("your-api-key").build()?;
    /// let client = RunpodClient::new(config)?;
    ///
    /// client.templates().delete("template_id").await?;
    /// println!("Template deleted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, template_id: &str) -> Result<()> {
        let path = format!("/templates/{}", template_id);
        self.client.delete(&path).send().await?;
        Ok(())
    }
}

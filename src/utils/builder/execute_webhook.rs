use serde_json::Value;
use std::default::Default;
use ::internal::prelude::*;

/// A builder to create the inner content of a [`Webhook`]'s execution.
///
/// This is a structured way of cleanly creating the inner execution payload,
/// to reduce potential argument counts.
///
/// Refer to the documentation for [`execute_webhook`] on restrictions with
/// execution payloads and its fields.
///
/// # Examples
///
/// Creating two embeds, and then sending them as part of the delivery
/// payload of [`Webhook::execute`]:
///
/// ```rust,ignore
/// use serenity::client::rest;
/// use serenity::model::Embed;
/// use serenity::utils::Colour;
///
/// let id = 245037420704169985;
/// let token = "ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV";
///
/// let webhook = rest::get_webhook_with_token(id, token)
///     .expect("valid webhook");
///
/// let website = Embed::fake(|e| e
///     .title("The Rust Language Website")
///     .description("Rust is a systems programming language.")
///     .colour(Colour::from_rgb(222, 165, 132)));
///
/// let resources = Embed::fake(|e| e
///     .title("Rust Resources")
///     .description("A few resources to help with learning Rust")
///     .colour(0xDEA584)
///     .field(|f| f
///         .inline(false)
///         .name("The Rust Book")
///         .value("A comprehensive resource for all topics related to Rust"))
///     .field(|f| f
///         .inline(false)
///         .name("Rust by Example")
///         .value("A collection of Rust examples on topics, useable in-browser")));
///
/// let _ = webhook.execute(|w| w
///     .content("Here's some information on Rust:")
///     .embeds(vec![website, resources]));
/// ```
///
/// [`Webhook`]: ../../model/struct.Webhook.html
/// [`Webhook::execute`]: ../../model/struct.Webhook.html#method.execute
/// [`execute_webhook`]: ../../client/rest/fn.execute_webhook.html
pub struct ExecuteWebhook(pub JsonMap);

impl ExecuteWebhook {
    /// Override the default avatar of the webhook with an image URL.
    pub fn avatar_url(mut self, avatar_url: &str) -> Self {
        self.0.insert("avatar_url".to_owned(), Value::String(avatar_url.to_owned()));

        self
    }

    /// Set the content of the message.
    ///
    /// Note that when setting at least one embed via [`embeds`], this may be
    /// omitted.
    ///
    /// [`embeds`]: #method.embeds
    pub fn content(mut self, content: &str) -> Self {
        self.0.insert("content".to_owned(), Value::String(content.to_owned()));

        self
    }

    /// Set the embeds associated with the message.
    ///
    /// This should be used in combination with [`Embed::fake`], creating one
    /// or more fake embeds to send to the API.
    ///
    /// # Examples
    ///
    /// Refer to the [struct-level documentation] for an example on how to use
    /// embeds.
    ///
    /// [`Embed::fake`]: ../../model/struct.Embed.html#method.fake
    /// [`Webhook::execute`]: ../../model/struct.Webhook.html#method.execute
    /// [struct-level documentation]: #examples
    pub fn embeds(mut self, embeds: Vec<Value>) -> Self {
        self.0.insert("embeds".to_owned(), Value::Array(embeds));

        self
    }

    /// Whether the message is a text-to-speech message.
    ///
    /// Think carefully before setting this to `true`.
    pub fn tts(mut self, tts: bool) -> Self {
        self.0.insert("tts".to_owned(), Value::Bool(tts));

        self
    }

    /// Override the default username of the webhook.
    pub fn username(mut self, username: &str) -> Self {
        self.0.insert("username".to_owned(), Value::String(username.to_owned()));

        self
    }
}

impl Default for ExecuteWebhook {
    /// Returns a default set of values for a [`Webhook`] execution.
    ///
    /// The only default value is [`tts`] being set to `true`. In the event that
    /// there is a bug that Discord defaults `tts` to `true`, at least
    /// serenity won't be a part of it.
    ///
    /// [`Webhook`]: ../../model/struct.Webhook.html
    /// [`tts`]: #method.tts
    fn default() -> ExecuteWebhook {
        let mut map = Map::new();
        map.insert("tts".to_owned(), Value::Bool(false));

        ExecuteWebhook(map)
    }
}

# Unsent Rust SDK

Official Rust SDK for the [Unsent API](https://unsent.dev) - Send transactional emails with ease.

## Prerequisites

- [Unsent API key](https://app.unsent.dev/dev-settings/api-keys)
- [Verified domain](https://app.unsent.dev/domains)
- Rust 1.70 or higher

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
unsent = "1.0"
serde_json = "1.0"
```

## Usage

### Basic Setup

```rust
use unsent::{Client, models::EmailCreate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("un_xxxx")?;
    
    // Use the client
    Ok(())
}
```

### Environment Variables

You can also set your API key using environment variables:

```rust
// Set UNSENT_API_KEY in your environment
// Then initialize without passing the key
let client = Client::new("")?;
```

### Sending Emails

#### Simple Email

```rust
use unsent::{Client, models::EmailCreate, emails::EmailsClient};

let client = Client::new("un_xxxx")?;
let emails = EmailsClient::new(&client);

let email = EmailCreate {
    to: serde_json::json!("hello@acme.com"),
    from: "hello@company.com".to_string(),
    subject: Some("Unsent email".to_string()),
    html: Some(Some("<p>Unsent is the best email service provider to send emails</p>".to_string())),
    text: Some(Some("Unsent is the best email service provider to send emails".to_string())),
    ..Default::default()
};

let response = emails.send(&email)?;
println!("Email sent! ID: {}", response.id);
```

#### Email with Attachments

```rust
use unsent::models::{EmailCreate, Attachment};

let email = EmailCreate {
    to: serde_json::json!("hello@acme.com"),
    from: "hello@company.com".to_string(),
    subject: Some("Email with attachment".to_string()),
    html: Some(Some("<p>Please find the attachment below</p>".to_string())),
    attachments: Some(vec![
        serde_json::json!({
            "filename": "document.pdf",
            "content": "base64-encoded-content-here"
        })
    ]),
    ..Default::default()
};

let response = emails.send(&email)?;
```

#### Scheduled Email

```rust
use chrono::{Utc, Duration};

let scheduled_time = (Utc::now() + Duration::hours(1)).to_rfc3339();

let email = EmailCreate {
    to: serde_json::json!("hello@acme.com"),
    from: "hello@company.com".to_string(),
    subject: Some("Scheduled email".to_string()),
    html: Some(Some("<p>This email was scheduled</p>".to_string())),
    scheduled_at: Some(scheduled_time),
    ..Default::default()
};

let response = emails.send(&email)?;
```

#### Batch Emails

```rust
use unsent::models::EmailBatchItem;

let batch = vec![
    EmailBatchItem {
        to: serde_json::json!("user1@example.com"),
        from: "hello@company.com".to_string(),
        subject: Some("Hello User 1".to_string()),
        html: Some(Some("<p>Welcome User 1</p>".to_string())),
        ..Default::default()
    },
    EmailBatchItem {
        to: serde_json::json!("user2@example.com"),
        from: "hello@company.com".to_string(),
        subject: Some("Hello User 2".to_string()),
        html: Some(Some("<p>Welcome User 2</p>".to_string())),
        ..Default::default()
    },
];

let response = emails.batch(&batch)?;
println!("Sent {} emails", response.ids.len());
```

#### Idempotent Retries

To prevent duplicate emails when retrying failed requests, you can provide an idempotency key.

```rust
use unsent::models::RequestOptions;

let options = RequestOptions {
    idempotency_key: Some("unique-key-123".to_string()),
};

let response = emails.send_with_options(&email, &options)?;
```

### Managing Emails

#### Get Email Details

```rust
let email = emails.get("email_id")?;
println!("Email status: {}", email.data["status"]);
```

#### Update Email

```rust
use unsent::models::EmailUpdate;

let update = EmailUpdate {
    scheduled_at: None,
};

let response = emails.update("email_id", &update)?;
```

#### Cancel Scheduled Email

```rust
let response = emails.cancel("email_id")?;
println!("Email cancelled successfully");
```

### Managing Contacts

#### Create Contact

```rust
use unsent::{models::ContactCreate, contacts::ContactsClient};
use std::collections::HashMap;

let client = Client::new("un_xxxx")?;
let contacts = ContactsClient::new(&client);

let mut metadata = HashMap::new();
metadata.insert("company".to_string(), serde_json::json!("Acme Inc"));
metadata.insert("role".to_string(), serde_json::json!("Developer"));

let contact = ContactCreate {
    email: "user@example.com".to_string(),
    first_name: Some("John".to_string()),
    last_name: Some("Doe".to_string()),
    metadata: Some(metadata),
    
    // Optional fields
    phone_number: None,
    subscribed: None,
};

let response = contacts.create("contact_book_id", &contact)?;
```

#### Get Contact

```rust
let contact = contacts.get("contact_book_id", "contact_id")?;
```

#### Update Contact

```rust
use unsent::models::ContactUpdate;

let mut metadata = HashMap::new();
metadata.insert("role".to_string(), serde_json::json!("Senior Developer"));

let update = ContactUpdate {
    first_name: Some("Jane".to_string()),
    last_name: None,
    metadata: Some(metadata),
    
    // Optional
    email: None,
    phone_number: None,
    subscribed: None,
};

let response = contacts.update("contact_book_id", "contact_id", &update)?;
```

#### Upsert Contact

```rust
use unsent::models::ContactUpsert;

let upsert = ContactUpsert {
    email: Some("user@example.com".to_string()),
    first_name: Some("John".to_string()),
    last_name: Some("Doe".to_string()),
    metadata: None,
    
    // Optional fields
    phone_number: None,
    subscribed: None,
};

// Note: Pass upsert struct directly, serialization handles wrapper
let response = contacts.upsert("contact_book_id", "contact_id", &upsert)?;
```

#### Delete Contact

```rust
let response = contacts.delete("contact_book_id", "contact_id")?;
```

### Managing Campaigns

#### Create Campaign

```rust
use unsent::{models::CampaignCreate, campaigns::CampaignsClient};

let client = Client::new("un_xxxx")?;
let campaigns = CampaignsClient::new(&client);

let campaign = CampaignCreate {
    name: "Welcome Series".to_string(),
    subject: "Welcome to our service!".to_string(),
    html: "<p>Thanks for joining us!</p>".to_string(),
    from: "welcome@example.com".to_string(),
    contact_book_id: "cb_1234567890".to_string(),
    
    // Optional fields
    reply_to: None,
    cc: None,
    bcc: None,
    attachments: None,
};

let response = campaigns.create(&campaign)?;
println!("Campaign created! ID: {}", response.campaign.id);
```

#### Schedule Campaign

```rust
use unsent::models::CampaignSchedule;

let schedule = CampaignSchedule {
    scheduled_at: "2024-12-01T10:00:00Z".to_string(),
};

let response = campaigns.schedule(&campaign.id, &schedule)?;
```

#### Pause/Resume Campaigns

```rust
// Pause a campaign
let pause_resp = campaigns.pause("campaign_123")?;
println!("Campaign paused successfully!");

// Resume a campaign
let resume_resp = campaigns.resume("campaign_123")?;
println!("Campaign resumed successfully!");
```

#### Get Campaign Details

```rust
let campaign_item = campaigns.get("campaign_id")?;
println!("Campaign status: {:?}", campaign_item.status);
println!("Recipients: {:?}", campaign_item.stats.total);
println!("Sent: {:?}", campaign_item.stats.sent);
```

### Managing Domains

#### List Domains

```rust
use unsent::domains::DomainsClient;

let client = Client::new("un_xxxx")?;
let domains = DomainsClient::new(&client);

let domain_list = domains.list()?;
for domain in domain_list {
    println!("Domain: {}, Status: {:?}", domain.domain, domain.status);
}
```

#### Create Domain

```rust
use unsent::models::DomainCreate;

let domain = DomainCreate {
    domain: "example.com".to_string(),
};

let response = domains.create(&domain)?;
```

#### Verify Domain

```rust
let response = domains.verify("domain_id")?;
println!("Verification status: {}", response.success);
```

#### Get Domain

```rust
let domain = domains.get("domain_id")?;
```

#### Delete Domain

```rust
let response = domains.delete("domain_id")?;
```

### Managing Webhooks (Coming Soon)

> **Note**: Webhooks are currently in development and not fully implemented (server-side). The SDK includes these methods for future compatibility.

#### List Webhooks

```rust
use unsent::webhooks::WebhooksClient;

let client = Client::new("un_xxxx")?;
let webhooks = WebhooksClient::new(&client);

let refs = webhooks.list()?;
```

#### Create Webhook

```rust
use unsent::models::WebhookCreate;

let webhook = WebhookCreate {
    url: "https://example.com/webhook".to_string(),
    events: vec!["email.sent".to_string()],
};

let response = webhooks.create(&webhook)?;
println!("Webhook created! ID: {}", response.id);
```

#### Update Webhook

```rust
use unsent::models::WebhookUpdate;

let update = WebhookUpdate {
    url: Some("https://example.com/new-webhook".to_string()),
    events: None,
};

let response = webhooks.update("webhook_id", &update)?;
```

#### Delete Webhook

```rust
let response = webhooks.delete("webhook_id")?;
```

### Error Handling

The SDK uses Rust's `Result` type for error handling:

```rust
use unsent::{Client, UnsentError};

let client = Client::new("un_xxxx")?;

match emails.send(&email) {
    Ok(response) => println!("Email sent! ID: {}", response.id),
    Err(UnsentError::Api(api_error)) => {
        eprintln!("API Error: {} - {}", api_error.code, api_error.message);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

To disable automatic error raising:

```rust
let client = Client::new("un_xxxx")?.with_raise_on_error(false);
```

### Custom HTTP Client

For advanced use cases, you can provide your own HTTP client:

```rust
use reqwest::blocking::Client as HttpClient;
use std::time::Duration;

let http_client = HttpClient::builder()
    .timeout(Duration::from_secs(30))
    .build()?;

let client = Client::new("un_xxxx")?.with_http_client(http_client);
```

## API Reference

### Client Methods

- `Client::new(key)` - Initialize the client
- `.with_base_url(url)` - Set custom base URL
- `.with_http_client(client)` - Set custom HTTP client
- `.with_raise_on_error(raise)` - Set error handling behavior

### Email Methods

- `emails.send(payload)` - Send an email (alias for create)
- `emails.create(payload)` - Create and send an email
- `emails.batch(emails)` - Send multiple emails in batch
- `emails.get(email_id)` - Get email details
- `emails.update(email_id, payload)` - Update a scheduled email
- `emails.cancel(email_id)` - Cancel a scheduled email
- `emails.list(params)` - List emails with filters
- `emails.bounces(params)` - List bounced emails
- `emails.complaints(params)` - List spam complaints
- `emails.unsubscribes(params)` - List unsubscribes

### Contact Methods

- `contacts.list(book_id, params)` - List contacts
- `contacts.create(book_id, payload)` - Create a contact
- `contacts.get(book_id, contact_id)` - Get contact details
- `contacts.update(book_id, contact_id, payload)` - Update a contact
- `contacts.upsert(book_id, contact_id, payload)` - Upsert a contact
- `contacts.delete(book_id, contact_id)` - Delete a contact

### Contact Book Methods
- `contact_books.list()` - List contact books
- `contact_books.create(payload)` - Create contact book
- `contact_books.get(id)` - Get contact book
- `contact_books.update(id, payload)` - Update contact book
- `contact_books.delete(id)` - Delete contact book

### Campaign Methods

- `campaigns.list()` - List all campaigns
- `campaigns.create(payload)` - Create a campaign
- `campaigns.get(campaign_id)` - Get campaign details
- `campaigns.schedule(campaign_id, payload)` - Schedule a campaign
- `campaigns.pause(campaign_id)` - Pause a campaign
- `campaigns.resume(campaign_id)` - Resume a campaign

### Domain Methods

- `domains.list()` - List all domains
- `domains.create(payload)` - Create a domain
- `domains.verify(domain_id)` - Verify a domain
- `domains.get(domain_id)` - Get domain details
- `domains.delete(domain_id)` - Delete a domain

### Webhook Methods

- `webhooks.list()` - List all webhooks
- `webhooks.create(payload)` - Create a webhook
- `webhooks.update(id, payload)` - Update a webhook
- `webhooks.delete(id)` - Delete a webhook

### Other Resources
- `api_keys` - Manage API keys
- `settings` - Team settings
- `analytics` - Get email statistics
- `templates` - Manage email templates
- `suppressions` - Manage suppression lists

## Requirements

- Rust 1.70+

## License

MIT

## Support

- [Documentation](https://docs.unsent.dev)
- [GitHub Issues](https://github.com/souravsspace/unsent-rust/issues)
- [Discord Community](https://discord.gg/unsent)

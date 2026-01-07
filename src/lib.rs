//! Rust SDK for the Unsent API - Send transactional emails with ease
//!
//! # Example
//!
//! ```no_run
//! use unsent::{Client, models::EmailCreate};
//!
//! let client = Client::new("un_xxxx").unwrap();
//! let emails = unsent::emails::EmailsClient::new(&client);
//!
//! let email = EmailCreate {
//!     to: serde_json::json!("hello@acme.com"),
//!     from: "hello@company.com".to_string(),
//!     subject: Some("Test Email".to_string()),
//!     html: Some(Some("<p>Hello World</p>".to_string())),
//!     text: Some(Some("Hello World".to_string())),
//!     ..Default::default()
//! };
//!
//! let response = emails.send(&email).unwrap();
//! println!("Email sent! ID: {}", response.id);
//! ```

pub mod analytics;
pub mod api_keys;
pub mod campaigns;
pub mod client;
pub mod contact_books;
pub mod contacts;
pub mod domains;
pub mod emails;
pub mod models;
pub mod settings;
pub mod suppressions;
pub mod system;
pub mod templates;
pub mod types;
pub mod webhooks;

pub use client::{Client, Result, UnsentError};

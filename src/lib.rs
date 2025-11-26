//! Rust SDK for the Unsent API - Send transactional emails with ease
//!
//! # Example
//!
//! ```no_run
//! use unsent::{Client, types::EmailCreate};
//!
//! let client = Client::new("un_xxxx").unwrap();
//! let emails = unsent::emails::EmailsClient::new(&client);
//!
//! let email = EmailCreate {
//!     to: "hello@acme.com".to_string(),
//!     from: "hello@company.com".to_string(),
//!     subject: "Test Email".to_string(),
//!     html: Some("<p>Hello World</p>".to_string()),
//!     text: Some("Hello World".to_string()),
//!     reply_to: None,
//!     cc: None,
//!     bcc: None,
//!     attachments: None,
//!     scheduled_at: None,
//! };
//!
//! let response = emails.send(&email).unwrap();
//! println!("Email sent! ID: {}", response.id);
//! ```

pub mod types;
pub mod client;
pub mod emails;
pub mod contacts;
pub mod campaigns;
pub mod domains;

pub use client::{Client, UnsentError, Result};

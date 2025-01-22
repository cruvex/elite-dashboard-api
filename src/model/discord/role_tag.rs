use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleTag {
    pub bot_id: Option<String>,                  // The ID of the bot this role belongs to
    pub integration_id: Option<String>,          // The ID of the integration this role belongs to
    pub premium_subscriber: Option<()>,          // Whether this is the guild's Booster role
    pub subscription_listing_id: Option<String>, // The ID of this role's subscription SKU and listing
    pub available_for_purchase: Option<()>,      // Whether this role is available for purchase
    pub guild_connections: Option<()>,           // Whether this role is a guild's linked role
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: String,                    // snowflake
    pub name: String,                  // role name
    pub color: i32,                    // integer representation of hexadecimal color code
    pub hoist: bool,                   // if this role is pinned in the user listing
    pub icon: Option<String>,          // role icon hash
    pub unicode_emoji: Option<String>, // role unicode emoji
    pub position: i32,                 // position of this role (roles with the same position are sorted by id)
    pub permissions: String,           // permission bit set
    pub managed: bool,                 // whether this role is managed by an integration
    pub mentionable: bool,             // whether this role is mentionable
    // Unused for now pub tags: Option<Vec<RoleTag>>, // the tags this role has
    pub flags: i32, // role flags combined as a bitfield
}

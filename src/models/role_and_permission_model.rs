use crate::common_types::{CreatedAt, OrganizationId, PrimaryId, PublicId, UpdatedAt, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "role_status", rename_all = "snake_case")]
pub enum RoleStatus {
    Active,
    Deleted,
    Suspended,
}
#[derive(Debug, Clone, sqlx::FromRow)]
struct RoleAndPermission {
    pub id: PrimaryId,
    pub public_id: PublicId,
    pub organization_id: OrganizationId,
    pub name: String,
    pub name_sl: Option<String>,
    pub can_edit: bool,
    pub permissions: Permissions,
    pub status: RoleStatus,
    pub created_by_user_id: UserId,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct SettingsPermissions {
    #[serde(default)]
    pub branch_create: bool,
    #[serde(default)]
    pub branch_update: bool,
    #[serde(default)]
    pub branch_delete: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Permissions {
    #[serde(flatten)]
    #[serde(default)]
    pub settings: SettingsPermissions,
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dump_permission_to_db() {
        let permission = Permissions {
            settings: SettingsPermissions {
                branch_create: false,
                branch_update: false,
                branch_delete: false,
            }
        };
        let json_string = serde_json::to_string(&permission).expect("serialization failed");
        assert!(json_string.len() > 3); // at least 2 braces and some content.
    }

    #[test]
    fn default_value_if_missing() {
        let json_string = r#"{
            "unknown_col": 32,
            "settings" : {
                "branch_create" : false
            }
        }"#;
        let perms: Permissions = serde_json::from_str(json_string).expect("deserialization failed");
        println!("{:#?}", perms);
        assert_eq!(perms.settings.branch_update, false);
    }
}
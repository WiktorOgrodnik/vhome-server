use uuid::Uuid;

pub fn generate_invitation_code() -> String {
    Uuid::new_v4().to_string()
}

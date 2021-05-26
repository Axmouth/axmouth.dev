use backend_repo_pg::extra::UserRole;
use chrono::{Duration, Utc};
use jsonwebtoken::errors::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TOKEN_PREFIX: &str = "Bearer ";

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum AxmouthDotDevAudience {
    AxmouthDotDev,
    AdminDotAxmouthDotDev,
    Outside,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum AxmouthDotDevIssuer {
    AxmouthDotDev,
    AdminDotAxmouthDotDev,
    Outside,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: i32, // JWT subject
    display_name: String,
    role: UserRole,             // user role
    aud: AxmouthDotDevAudience, // audience
    exp: i64,                   // seconds since the epoch
    iat: i64,                   // issued at
    nbf: i64,                   // time before which the token can't be used
    iss: AxmouthDotDevIssuer,   // token issuer
    jti: uuid::Uuid,            // token identifuer
}

impl Claims {
    fn new(user_id: i32, role: UserRole, jti: Uuid, display_name: String, duration: i64) -> Self {
        Self {
            sub: user_id,
            role,
            display_name,
            aud: AxmouthDotDevAudience::AxmouthDotDev,
            exp: (Utc::now() + Duration::seconds(duration)).timestamp(),
            iat: Utc::now().timestamp(),
            nbf: Utc::now().timestamp(),
            iss: AxmouthDotDevIssuer::AxmouthDotDev,
            jti: jti,
        }
    }
    fn new_admin(
        user_id: i32,
        role: UserRole,
        jti: Uuid,
        display_name: String,
        duration: i64,
    ) -> Self {
        Self {
            sub: user_id,
            role,
            display_name,
            aud: AxmouthDotDevAudience::AdminDotAxmouthDotDev,
            exp: (Utc::now() + Duration::seconds(duration)).timestamp(),
            iat: Utc::now().timestamp(),
            nbf: Utc::now().timestamp(),
            iss: AxmouthDotDevIssuer::AdminDotAxmouthDotDev,
            jti: jti,
        }
    }

    pub fn user_id(&self) -> i32 {
        self.sub
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn role(&self) -> UserRole {
        self.role.clone()
    }

    pub fn jti(&self) -> uuid::Uuid {
        self.jti
    }

    pub fn is_staff(&self) -> bool {
        self.role == UserRole::Admin || self.role == UserRole::Moderator
    }

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    pub fn is_for_admin_site(&self) -> bool {
        self.aud == AxmouthDotDevAudience::AdminDotAxmouthDotDev
    }

    pub fn is_verified(&self) -> bool {
        self.role != UserRole::Ghost
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    pub fn new_refreshed(&self, jti: uuid::Uuid, duration: i64) -> Self {
        Self {
            sub: self.sub.clone(),
            role: self.role.clone(),
            display_name: self.display_name.clone(),
            aud: self.aud.clone(),
            exp: (Utc::now() + Duration::seconds(duration)).timestamp(),
            iat: Utc::now().timestamp(),
            nbf: Utc::now().timestamp(),
            iss: self.iss.clone(),
            jti,
        }
    }

    pub fn to_token(&self, secret: &str) -> String {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }
}

pub fn encode_token(
    secret: &str,
    sub: i32,
    role: UserRole,
    jti: Uuid,
    display_name: String,
    duration: i64,
) -> String {
    encode(
        &Header::default(),
        &Claims::new(sub, role, jti, display_name, duration),
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn encode_admin_token(
    secret: &str,
    sub: i32,
    role: UserRole,
    jti: Uuid,
    display_name: String,
    duration: i64,
) -> String {
    encode(
        &Header::default(),
        &Claims::new_admin(sub, role, jti, display_name, duration),
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn decode_token(secret: &str, token: &str) -> Result<Claims> {
    let validation = &mut Validation::default();
    validation.validate_nbf = true;
    validation.validate_exp = false;
    decode::<Claims>(
        token.trim_start_matches(TOKEN_PREFIX),
        &DecodingKey::from_secret(secret.as_ref()),
        validation,
    )
    .map(|token_data| token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn encode_decode_token() {
        let sub = 2323;
        let token = encode_token(
            "sikureto",
            sub,
            UserRole::User,
            Uuid::new_v4(),
            "usahname".to_string(),
            100,
        );
        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        );
        if let Err(e) = &decoded {
            println!("decode err: {}", e);
        }

        assert!(decoded.is_ok());
    }
}

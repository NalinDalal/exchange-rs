use crate::dto::{AuthResponse, SigninRequest, SignupRequest, UserResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use db::{User, UserRepository};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct AuthService {
    user_repo: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: UserRepository, jwt_secret: String) -> Self {
        Self { user_repo, jwt_secret }
    }

    pub async fn signup(&self, data: SignupRequest) -> Result<AuthResponse, String> {
        // Validate
        if !data.email.contains('@') {
            return Err("Invalid email".to_string());
        }
        if data.password.len() < 6 {
            return Err("Password must be at least 6 characters".to_string());
        }

        // Check existing
        if self.user_repo.find_by_email(&data.email).await
            .map_err(|e| format!("DB error: {}", e))?.is_some() {
            return Err("User already exists".to_string());
        }

        // Hash password
        let password_hash = hash(&data.password, DEFAULT_COST)
            .map_err(|e| format!("Hash error: {}", e))?;

        // Create user
        let user = self.user_repo.create(&data.email, &password_hash).await
            .map_err(|e| format!("Failed to create user: {}", e))?;

        let token = self.generate_token(&user.id)?;

        Ok(AuthResponse {
            user: user.into(),
            token,
        })
    }

    pub async fn signin(&self, data: SigninRequest) -> Result<AuthResponse, String> {
        let user = self.user_repo.find_by_email(&data.email).await
            .map_err(|e| format!("DB error: {}", e))?
            .ok_or("Invalid credentials")?;

        let valid = verify(&data.password, &user.password_hash)
            .map_err(|_| "Invalid credentials")?;

        if !valid {
            return Err("Invalid credentials".to_string());
        }

        let token = self.generate_token(&user.id)?;

        Ok(AuthResponse {
            user: user.into(),
            token,
        })
    }

    fn generate_token(&self, user_id: &Uuid) -> Result<String, String> {
        let now = chrono::Utc::now().timestamp() as usize;
        let exp = now + (24 * 60 * 60);

        let claims = Claims {
            sub: user_id.to_string(),
            exp,
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| format!("Token error: {}", e))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, String> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid token: {}", e))
    }
}

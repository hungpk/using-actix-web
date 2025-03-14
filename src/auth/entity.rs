use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub role: String,
    pub user_id: i32,
}
impl TokenClaims {
    pub fn from_token(token: &str) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
        let secret = Self::get_secret();
        let claims = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        );
        match claims {
            Ok(claims) => Ok(claims.claims),
            Err(err) => Err(err),
        }
    }

    pub fn create_token(claims: &TokenClaims) -> Result<String, jsonwebtoken::errors::Error> {
        let secret = Self::get_secret();
        let result = encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(secret.as_ref()),
        );
        match result {
            Ok(token) => Ok(token),
            Err(err) => Err(err),
        }
    }
    fn get_secret() -> String {
        var("JWT_SECRET").unwrap_or_else(|_| "this is very jwt secret".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_create_and_decode_token() {
        // Setup: Create a sample set of claims
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_claims = TokenClaims {
            sub: "test_user".to_string(),
            iat: now,
            exp: now + 3600, // Valid for 1 hour
            role: "user".to_string(),
            user_id: 1,
        };

        // Action: Create a token from the claims
        let token = TokenClaims::create_token(&test_claims);

        // Assertion 1: Token should not be empty
        assert!(token.is_ok());

        // Action: Decode the token
        let decoded_result = TokenClaims::from_token(token.unwrap().as_str());

        // Assertion 2: Decoding should be successful
        assert!(decoded_result.is_ok());

        // Assertion 3: Claims in decoded token should match the original claims
        let decoded_claims = decoded_result.unwrap();
        assert_eq!(decoded_claims.sub, test_claims.sub);
        assert_eq!(decoded_claims.role, test_claims.role);
        assert_eq!(decoded_claims.user_id, test_claims.user_id);
        assert!(decoded_claims.exp > decoded_claims.iat);
        assert!(decoded_claims.exp == test_claims.exp);
    }

    #[test]
    fn test_decode_invalid_token() {
        // Action: Attempt to decode a bad token
        let decoded_result = TokenClaims::from_token("this_is_not_a_valid_token");

        // Assertion: Decoding should fail
        assert!(decoded_result.is_err());
    }

    #[test]
    fn test_decode_expired_token() {
        // Setup: Create claims that are already expired.
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expired_claims = TokenClaims {
            sub: "test_user".to_string(),
            iat: now - 7200, // Issued 2 hours ago
            exp: now - 3600, // Expired 1 hour ago
            role: "user".to_string(),
            user_id: 1,
        };

        let expired_token = TokenClaims::create_token(&expired_claims).unwrap();

        // Action: Try to decode the expired token.
        let result = TokenClaims::from_token(&expired_token);

        // Assertion: It should produce an error
        assert!(result.is_err());
        if let Err(err) = result {
            if let jsonwebtoken::errors::ErrorKind::ExpiredSignature = err.kind() {
                assert!(true);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_get_secret_default() {
        // Setup: Ensure the JWT_SECRET environment variable is not set.
        env::remove_var("JWT_SECRET");

        // Action: call get_secret function.
        let secret = TokenClaims::get_secret();

        // Assertion: the default secret should be returned.
        assert_eq!(secret, "this is very jwt secret");
    }

    #[test]
    fn test_get_secret_from_env() {
        // Setup: Set the JWT_SECRET environment variable.
        env::set_var("JWT_SECRET", "my_test_secret");

        // Action: call get_secret function.
        let secret = TokenClaims::get_secret();

        // Assertion: the set secret should be returned.
        assert_eq!(secret, "my_test_secret");

        // Cleanup: remove the variable for other tests
        env::remove_var("JWT_SECRET");
    }
    #[test]
    fn test_decode_token_with_wrong_secret() {
        // Setup: create claims and token using secret "secret1"
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_claims = TokenClaims {
            sub: "test_user".to_string(),
            iat: now,
            exp: now + 3600,
            role: "user".to_string(),
            user_id: 1,
        };
        let token = TokenClaims::create_token(&test_claims).unwrap();
        // Setup : set JWT_SECRET to a different value.
        env::set_var("JWT_SECRET", "secret2");

        // Action: try to decode token.
        let result: Result<TokenClaims, jsonwebtoken::errors::Error> =
            TokenClaims::from_token(&token);

        // Assertion: decoding should fail
        assert!(result.is_err());
        if let Err(err) = result {
            if let jsonwebtoken::errors::ErrorKind::InvalidSignature = err.kind() {
                assert!(true);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
        // Cleanup: remove the variable for other tests
        env::remove_var("JWT_SECRET");
    }
}

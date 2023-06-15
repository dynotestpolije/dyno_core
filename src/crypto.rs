#[allow(unused)]
use crate::{DynoErr, DynoResult};

#[cfg(feature = "jwt_encode_decode")]
#[inline]
pub fn jwt_decode<T: serde::de::DeserializeOwned>(
    token: impl AsRef<str>,
    secret: &'_ [u8],
) -> DynoResult<jsonwebtoken::TokenData<T>> {
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    decode::<T>(
        token.as_ref(),
        &DecodingKey::from_rsa_pem(secret).map_err(DynoErr::jwt_error)?,
        &Validation::new(Algorithm::RS256),
    )
    .map_err(DynoErr::jwt_error)
}

#[cfg(feature = "jwt_encode_decode")]
#[inline]
pub fn jwt_encode<T: serde::Serialize>(claims: &T, secret: &'_ [u8]) -> DynoResult<String> {
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    encode(
        &Header::new(Algorithm::RS256),
        claims,
        &EncodingKey::from_rsa_pem(secret).map_err(DynoErr::jwt_error)?,
    )
    .map_err(DynoErr::jwt_error)
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, PartialEq)]
#[display(
    fmt = "TokenDetails {{ user_id:{user} token_id:{token_id}, expires_in:{expires_in:?}, token:{token:?} }}"
)]
#[derive(Debug, Clone)]
pub struct TokenDetails {
    pub user: crate::UserSession,
    pub token_id: uuid::Uuid,
    pub expires_in: Option<i64>,
    pub token: Option<String>,
}
#[cfg(feature = "jwt_encode_decode")]
impl TokenDetails {
    pub fn generate(
        user: crate::UserSession,
        max_age: i64,
        private_key: impl AsRef<[u8]>,
    ) -> DynoResult<TokenDetails> {
        use base64::{engine::general_purpose::STANDARD, Engine};
        let priv_key = Engine::decode(&STANDARD, private_key).map_err(DynoErr::any_error)?;
        let token_id = uuid::Uuid::new_v4();
        let claims = crate::TokenClaims::new(token_id, max_age, serde_json::to_string(&user)?);
        let token = jwt_encode(&claims, &priv_key).map(Some)?;

        Ok(TokenDetails {
            user,
            token_id,
            expires_in: Some(claims.exp),
            token,
        })
    }
    pub fn verify(
        token: impl AsRef<str>,
        public_key: impl AsRef<[u8]>,
    ) -> DynoResult<TokenDetails> {
        use base64::{engine::general_purpose::STANDARD, Engine};
        let pub_key = Engine::decode(&STANDARD, public_key).map_err(DynoErr::any_error)?;

        let token_data = jwt_decode::<crate::TokenClaims>(token, &pub_key)?;
        let claims = token_data.claims;

        Ok(TokenDetails {
            user: serde_json::from_str(&claims.sub)?,
            token_id: uuid::Uuid::try_parse(&claims.id)?,
            token: None,
            expires_in: Some(claims.exp),
        })
    }
}

#[cfg(feature = "password_hashing")]
#[inline]
pub fn hash_password<Pswd: AsRef<str>>(pswd: Pswd) -> DynoResult<String> {
    let salt = argon2::password_hash::SaltString::generate(&mut rand_core::OsRng);
    argon2::PasswordHasher::hash_password(
        &argon2::Argon2::default(),
        pswd.as_ref().as_bytes(),
        &salt,
    )
    .map(|x| x.to_string())
    .map_err(DynoErr::password_hash_error)
}

#[cfg(feature = "password_hashing")]
#[inline]
pub fn verify_hash_password<Lhs: AsRef<str>, Rhs: AsRef<str>>(
    pswd_lhs: Lhs,
    pswd_rhs: Rhs,
) -> bool {
    match argon2::PasswordHash::new(pswd_lhs.as_ref()).map(|hash| {
        match argon2::PasswordVerifier::verify_password(
            &argon2::Argon2::default(),
            pswd_rhs.as_ref().as_bytes(),
            &hash,
        ) {
            Ok(_) => true,
            Err(err) => {
                log::error!("{}", DynoErr::password_hash_error(err));
                false
            }
        }
    }) {
        Ok(verified) => verified,
        Err(err) => {
            log::error!("{}", DynoErr::password_hash_error(err));
            false
        }
    }
}

#[cfg(feature = "checksum")]
#[inline]
pub fn checksum_from_bytes(data: impl AsRef<[u8]>) -> String {
    format!("{:X}", <sha2::Sha256 as sha2::Digest>::digest(data))
}

#[cfg(feature = "checksum")]
#[inline]
pub fn compare_checksums(lhs: &'_ [u8], rhs: &'_ [u8]) -> bool {
    lhs.len() == rhs.len() && lhs == rhs
}

mod tests {

    #[cfg(feature = "jwt_encode_decode")]
    #[test]
    fn test_jwt() {
        const PRIVATE_KEY_TESTS: &[u8]= b"LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQpNSUlFb2dJQkFBS0NBUUJ5MnZzQlF4dkVOT2tTU2psNFVQbVVIUlBhbVRPWFVFdTNXMTA5am4yL1RWOGpyckVWCkpoOVg4MXoxOHVmV3BJKzY1V2pPZ1lpQ1ArWEdHU0doeFhlRlJQb1A2Ymd3TW1rbnpQY1BFMDd4dUhxWURHck0KeVJxazBLdTlmYVdwajFsNjBsbDZueGN6dkFxOWNIUTBWTFFBMU1ITkduOGpnYk9abHFqYTI1d0lHb1JWS3g5WQowQ0xIWHNtZnk0YklhMFVwMlRFWHZNVitiR2xrUTRUWU53cWc3V0tvTjluc0haaDZ1YTdRZStTQStHQW42N3dUCkkyUW5QQzcvL3JXWktSaWZKOXN6ekVHaWQ0OEtTZGtjNVZlTURVNUE0NFhJY3ZkOE9JQ3BMWWk5VWxTOWZrbzQKemVRNi9nRVB5SXNkenVGNFlwcEZnY0JrWE44Z3h6d2VjRHlwQWdNQkFBRUNnZ0VBS3BOdVZJdHg4WXZEa2xvZApiN0prc2prTk1RMTFpSFBsZlhUSzNaVENuSFdyL09nM0VLWHZISVZFUW8vbXpUUjNhMHE3VFVITFBxMDlHZkFhCi9ZTGJ5WFZHVlR2SVlXVnVnYWlETlAxdFdFR1FnR2lwOW52ZCtwYTM4eWh1anFwcSt4S2F4TS9RcEFiaTNGS2gKWVB5OU5VbXBCY1Q2dUVqT01QTWVhUExicGt6Yk9ENEUyY1VnSldLeXFoSjJHODVKMVJqaTlhUW5YNSt4QWM5MApTOEc3SExoUjBkWEpmKzZuZHViTmh4eVlrUjA2SDJiVk5vaTRwaWF6MlBvN0pYRWRsNThuU3J3TlY5eGxJV0RyClV5ZC9rVEtKZktlZXllQmk3ZVNOWkMvcHVobEFlQ091S2ZCVGNDWkl3ZEVmZVhZMHM1dk5SSWJESXBwOEovS1UKMW54bm1RS0JnUURhbUxVTEc5WUJrNkhsTjZnZWxqdXByWVVTZ1VwR3hVQXBFRHdMNkhTSzNtVUl3M2tzYUxSdApsVEJjVmRwNWZhRXkrcXRLQVhJNHMzSWRVYnBsazhvYWV2WU5JQTI3emswN3BLZ3dSWWlUVXQvUmI0MW4rT1JKCkxRd0ZWeDFuallNTFp1MmpHNC9Qb0pnL0RmU1ZSaVZnYytsOEFERWRSYXVWOE1ZeUFZRW1Hd0tCZ1FDR2dneUYKNjNyTFlPOW9xcGoyWGNQbFliUDlZNFNXZjVyNU9kUHhTTGU5MTdLMDM1WG5nOTl5bnIzOEI1UjF6bngzYXYzdApCejFPbmhnUmVTMThvL3JmOURnNUtVVFZmbW5MTU11bnFRZk9EdUQwMldybkJVbGdsTVQzMHlmY2RlVTRRS3FYCjFyaEg1U0lJcG1lNXl3cWQyYW9HUXovU29VYzZTR2E5Ymlwa2l3S0JnUUNrMWVBM1J6OUNJeHhDcDR4N3ZCZXoKWkdoalF1RktxblV1UWRLMXZBMGRwb1FUYnF0U0JJYUIxNjBIQTZwaVVJeWN6NDhZbUN5VWV5UkMzWjByK1p5KwpDUmkvWHZ0Y01qM2xDczNVMEtvalJhd0cxeEtQWTR4cGJiR1IyVllQL3hMaEp6VnMrRnlpV1Z4VXQrWkdKZ1lMClRIeTFCOFFCYWNid293WWhNU2M5SHdLQmdEY2hxdHhoVG0xNkIrY1F0YTJWZFN2WnhocC81ZUtCM3hWMEhHanoKYzB4KzBxTE1XZmNRMkU5MHZVNUdGUlhNVXJ2bndMaWNsUVphU21jdkVmUGFpb09zNkRhVnNiOElGM2pQUVVveApRQiszZllFZWRIREllL0Q0U1Z1QzB2M0wzTzRYbzFTbEQ5ZDRGV0VTUThXbGNXNk52dURlUTVTandqd2pYT2tiCjlJaE5Bb0dBY1c0czR4QnJIaW1ka3VZTWlRYmVXR1VSS25hdkd4dnlzMENUQkQ4T1EvZEI3M1hDQnZlNFFUUU4KbDMxVHQ3aWFXaDZWTk0vN0JaU2VYcVlTbHE2NDFUK2hPQjJGc2JrVHJaUDZrMERwbjNrQTcwWStVaWU3QmRrcgpRYmtOOExjdGNzRDQ1MHNrckx3aEFwTVBLbXBGQXBXNkhCRXhuK2thLzFwZFJwNlh6WlU9Ci0tLS0tRU5EIFJTQSBQUklWQVRFIEtFWS0tLS0t";
        const PUBLIC_KEY_TESTS: &[u8] = b"LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklUQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FRNEFNSUlCQ1FLQ0FRQnkydnNCUXh2RU5Pa1NTamw0VVBtVQpIUlBhbVRPWFVFdTNXMTA5am4yL1RWOGpyckVWSmg5WDgxejE4dWZXcEkrNjVXak9nWWlDUCtYR0dTR2h4WGVGClJQb1A2Ymd3TW1rbnpQY1BFMDd4dUhxWURHck15UnFrMEt1OWZhV3BqMWw2MGxsNm54Y3p2QXE5Y0hRMFZMUUEKMU1ITkduOGpnYk9abHFqYTI1d0lHb1JWS3g5WTBDTEhYc21meTRiSWEwVXAyVEVYdk1WK2JHbGtRNFRZTndxZwo3V0tvTjluc0haaDZ1YTdRZStTQStHQW42N3dUSTJRblBDNy8vcldaS1JpZko5c3p6RUdpZDQ4S1Nka2M1VmVNCkRVNUE0NFhJY3ZkOE9JQ3BMWWk5VWxTOWZrbzR6ZVE2L2dFUHlJc2R6dUY0WXBwRmdjQmtYTjhneHp3ZWNEeXAKQWdNQkFBRT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0t";

        let session = crate::UserSession {
            id: 1,
            uuid: uuid::Uuid::new_v4(),
            role: crate::role::Roles::User,
        };
        let token_details = super::TokenDetails::generate(session, 20, PRIVATE_KEY_TESTS);
        assert!(
            matches!(token_details, Ok(_)),
            "the TokenDetails::generate function should not be error - {token_details:?}"
        );
        let token_details = token_details.unwrap();

        let verify_token =
            super::TokenDetails::verify(token_details.token.unwrap(), PUBLIC_KEY_TESTS);
        assert!(
            matches!(verify_token, Ok(_)),
            "the TokenDetails::verify function should not be error - {verify_token:?}"
        );
        let verify_token = verify_token.unwrap();
        dbg!(&verify_token);

        assert_eq!(
            session.uuid, verify_token.user.uuid,
            "the session and verify tkoen use result must be the same"
        );
        assert_eq!(
            session.id, verify_token.user.id,
            "the session and verify tkoen use result must be the same"
        );
    }

    #[cfg(feature = "password_hashing")]
    #[test]
    fn test_password_hashing() {
        let pswd = "inipassword123";
        let hashed = super::hash_password(pswd);
        assert!(
            matches!(hashed, Ok(_)),
            "the hashing function should not be error"
        );
        assert!(super::verify_hash_password(hashed.unwrap(), pswd))
    }

    #[cfg(feature = "checksum")]
    #[test]
    fn test_checksum_from_byts() {
        let bytes = b"hello world";
        let checksum = super::checksum_from_bytes(bytes);

        assert_eq!(
            checksum,
            "B94D27B9934D3E08A52E52D7DA7DABFAC484EFE37A5380EE9088F7ACE2EFCDE9"
        );
    }

    #[cfg(feature = "checksum")]
    #[test]
    fn test_compare_checksum() {
        let bytes = b"hello world";
        let checksum = super::checksum_from_bytes(bytes);

        assert!(super::compare_checksums(
            checksum.as_bytes(),
            b"B94D27B9934D3E08A52E52D7DA7DABFAC484EFE37A5380EE9088F7ACE2EFCDE9"
        ))
    }
}

#[cfg(feature = "jwt_encode_decode")]
#[inline]
pub fn jwt_decode<T: serde::de::DeserializeOwned>(
    token: impl AsRef<str>,
    secret: &'_ [u8],
) -> crate::DynoResult<T> {
    jsonwebtoken::decode::<T>(
        token.as_ref(),
        &jsonwebtoken::DecodingKey::from_secret(secret),
        &jsonwebtoken::Validation::default(),
    )
    .map(|x| x.claims)
    .map_err(crate::DynoErr::jwt_error)
}

#[cfg(feature = "jwt_encode_decode")]
#[inline]
pub fn jwt_encode<T: serde::Serialize>(claims: T, secret: &'_ [u8]) -> crate::DynoResult<String> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret),
    )
    .map_err(crate::DynoErr::jwt_error)
}

#[cfg(feature = "password_hashing")]
#[inline]
pub fn hash_password<Pswd: AsRef<str>>(pswd: Pswd) -> crate::DynoResult<String> {
    let salt = argon2::password_hash::SaltString::generate(&mut rand_core::OsRng);
    argon2::PasswordHasher::hash_password(
        &argon2::Argon2::default(),
        pswd.as_ref().as_bytes(),
        &salt,
    )
    .map(|x| x.to_string())
    .map_err(crate::DynoErr::password_hash_error)
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
                log::error!("{}", crate::DynoErr::password_hash_error(err));
                false
            }
        }
    }) {
        Ok(verified) => verified,
        Err(err) => {
            log::error!("{}", crate::DynoErr::password_hash_error(err));
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

#[cfg(feature = "checksum")]
#[test]
fn test_checksum_from_byts() {
    let bytes = b"hello world";
    let checksum = checksum_from_bytes(bytes);

    assert_eq!(
        checksum,
        "B94D27B9934D3E08A52E52D7DA7DABFAC484EFE37A5380EE9088F7ACE2EFCDE9"
    );
}

#[cfg(feature = "checksum")]
#[test]
fn test_compare_checksum() {
    let bytes = b"hello world";
    let checksum = checksum_from_bytes(bytes);

    assert!(compare_checksums(
        checksum.as_bytes(),
        b"B94D27B9934D3E08A52E52D7DA7DABFAC484EFE37A5380EE9088F7ACE2EFCDE9"
    ))
}

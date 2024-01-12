pub fn is_password_correct(password: String, hash: &str) -> bool {
    let verfy_res = bcrypt::verify(password, hash);

    match verfy_res {
        Ok(is_correct) => is_correct,
        _ => false,
    }
}

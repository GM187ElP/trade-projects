#[cfg(test)]
mod test {
    use backend::service::normalize_email::normalize_email;

    #[test]
    fn normalized_email_test() {
        let result = normalize_email(" admin@gmail.com ".to_string());

        assert_eq!("ADMIN@GMAIL.COM".to_string(), result);
    }
}

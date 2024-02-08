pub fn federate_names(lf_file: String) -> Vec<String> {
    vec![
        String::from("federate1"),
        String::from("federate2"),
        lf_file[..lf_file.len().max(8)].to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp() {
        let expected = expect_test::expect![[
            r#"Cons((Cons((Symbol("name") . String("John Doe"))) . Cons((Cons((Symbol("age") . Number(PosInt(43)))) . Cons((Cons((Symbol("address") . Cons((Cons((Symbol("street") . Cons((String("10 Downing Street") . Null)))) . Cons((Cons((Symbol("city") . Cons((String("London") . Null)))) . Null)))))) . Cons((Cons((Symbol("phones") . Cons((String("+44 1234567") . Cons((String("+44 2345678") . Null)))))) . Null))))))))"#
        ]];
        expected.assert_eq(&format!(
            "{:?}",
            lexpr::from_str(
                r#"
        ((name . "John Doe")
            (age . 43)
            (address
            (street "10 Downing Street")
            (city "London"))
            (phones "+44 1234567" "+44 2345678"))
        "#,
            )
            .unwrap()
        ));
    }
    #[test]
    fn temp2() {
        let expected = expect_test::expect![[r#""#]];
        expected.assert_eq(&format!(
            "{:?}",
            lexpr::from_str(r#"((a . 42) (b . 43))"#).unwrap()
        ));
    }
}

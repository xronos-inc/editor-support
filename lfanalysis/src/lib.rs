pub fn federate_names(lf_file: String) -> Vec<String> {
    vec![
        String::from("federate1"),
        String::from("federate2"),
        lf_file[..lf_file.len().max(8)].to_string(),
    ]
}

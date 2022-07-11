#[cfg(test)]
mod tests {
    use unitypacker;
    const TEST_PROJECT: &str = "tests/Assets";

    #[test]
    fn scan_test() {
        let metas = unitypacker::find_unity_meta(&String::from(TEST_PROJECT),None).unwrap();
        assert_eq!(metas[0].guid, "37d6e1e5ec83e454eb86b47f81fe116a");
        assert_eq!(metas[1].guid, "80b54747fd9534ef3bf4f5dec0cb319a");
    }
}

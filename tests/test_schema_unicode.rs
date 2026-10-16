use serde_json::Value;

use crate::workdir::Workdir;

#[test]
fn schema_unicode_character_bounds_validate_original_csv() {
    // Explicitly cover a String column above and at the enum threshold.
    for threshold in [2, 50] {
        let wrk = Workdir::new(&format!("schema_unicode_character_bounds_{threshold}"));
        wrk.create_from_string(
            "unicode.csv",
            "name,ascii,optional,flag\n上海,abc,é,yes\n李,defg,,no\n😀😀,hij,e\u{301},yes\n",
        );
        let mut schema = wrk.command("schema");
        schema
            .arg("unicode.csv")
            .args(["--enum-threshold", &threshold.to_string()]);
        wrk.assert_success(&mut schema);

        let text: String = wrk.from_str(&wrk.path("unicode.csv.schema.json"));
        let schema: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(schema["properties"]["name"]["minLength"], 1);
        assert_eq!(schema["properties"]["name"]["maxLength"], 2);
        assert_eq!(schema["properties"]["ascii"]["minLength"], 3);
        assert_eq!(schema["properties"]["ascii"]["maxLength"], 4);
        assert_eq!(schema["properties"]["optional"]["minLength"], 0);
        assert_eq!(schema["properties"]["optional"]["maxLength"], 2);
        assert_eq!(
            schema["properties"]["flag"]["enum"],
            serde_json::json!(["no", "yes"])
        );
        assert_eq!(
            schema["properties"]["name"].get("enum").is_some(),
            threshold == 50
        );
        assert_eq!(
            schema["properties"]["ascii"].get("enum").is_some(),
            threshold == 50
        );

        let mut validate = wrk.command("validate");
        validate.args(["unicode.csv", "unicode.csv.schema.json"]);
        wrk.assert_success(&mut validate);
    }
}

#[cfg(test)]
mod test {
    use crate::parser::structs::Resume;
    use crate::parser::{json_get, remove_quotes, replace_html_vars};
    use serde_json::json;

    #[test]
    fn remove_quotes_test() {
        let double_quoted = remove_quotes("\"Quoted\"");
        assert_eq!(double_quoted, "Quoted");

        let many_quotes = remove_quotes("\"Q\"\"\"uoted\"\n\"New \"string\"");
        assert_eq!(many_quotes, "Quoted\nNew string");
    }

    fn get_resume_json() -> Resume {
        let json: Resume = match serde_json::from_str(
            r#"{
            "basics": {
              "name": "John Doe",
              "label": "Programmer",
              "email": "john@gmail.com",
              "phone": "(912) 555-4321",
              "website": "http://johndoe.com",
              "summary": "A brief summary on who I am",
              "location": {
                "country": "The Johnited States Of Doe",
                "address": "2712 Broadway St",
                "city": "San Francisco"
              },
              "profiles": [
                {
                  "network": "a",
                  "username": "b",
                  "url": "c"
                },
                {
                  "network": "a2",
                  "username": "b2",
                  "url": "c2"
                }
              ]
            },
            "work": [{
              "company": "Company",
              "position": "President",
              "website": "http://company.com",
              "start_date": "2013-01-01",
              "end_date": "2014-01-01",
              "summary": "Description..."
            }],
            "projects": [{
              "name": "a",
              "description": "b"
            }],
            "education": [{
              "institution": "University",
              "area": "Software Development",
              "study_type": "Bachelor",
              "start_date": "2011-01-01",
              "end_date": "2013-01-01",
              "courses": [
                "DB1101 - Basic SQL"
              ],
              "location": "Washington DC, US"
            }],
            "skills": [{ "name": "Web Development" }],
            "languages": [
              {
                "language": "English",
                "level": "Native"
              },
              {
                "language": "Spanish",
                "level": "Fluent"
              }
            ]
          }"#,
        ) {
            Ok(result) => result,
            Err(e) => panic!(e),
        };

        json
    }

    fn json_to_value(json: Resume) -> serde_json::value::Value {
        let result: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&json).unwrap())
                .expect("Cannot parse JSON");
        result
    }

    #[test]
    fn json_get_simple() {
        let json: Resume = get_resume_json();
        let value = json_to_value(json);

        let res_1 = json_get(&value, "basics");
        assert_eq!(res_1.is_object(), true);
        assert_eq!(res_1.get("name").unwrap(), "John Doe");
        assert_eq!(res_1.get("profiles").unwrap().is_array(), true);

        let res_2 = json_get(&value, "skills");
        assert_eq!(res_2.is_array(), true);
        assert_eq!(
            res_2.get(0).unwrap().get("name").unwrap(),
            "Web Development"
        );

        let res_3 = json_get(&value, "languages");
        assert_eq!(res_3.is_array(), true);
        assert_eq!(res_3.get(0).unwrap().get("language").unwrap(), "English");
        assert_eq!(res_3.get(1).unwrap().get("language").unwrap(), "Spanish");
    }

    #[test]
    fn json_get_nested() {
        let json: Resume = get_resume_json();
        let value = json_to_value(json);
        let res = json_get(&value, "basics.location.city");
        assert_eq!(remove_quotes(&res.to_string()), "San Francisco");
    }

    #[test]
    fn json_get_custom() {
        let json = json!({
            "int": 1,
            "bool": true,
            "nested": {
                "arr": ["str1", "str2"],
                "obj": {
                    "int": 2
                }
            }
        });
        let res_1 = json_get(&json, "int");
        assert_eq!(res_1, 1);

        let res_2 = json_get(&json, "bool");
        assert_eq!(res_2, true);

        let res_3 = json_get(&json, "nested");
        assert_eq!(res_3.is_object(), true);

        let res_4 = json_get(&json, "nested.obj.int");
        assert_eq!(res_4, 2);

        let res_5 = json_get(&json, "nested.obj");
        assert_eq!(res_5.get("int").unwrap(), 2);

        let res_6 = json_get(&json, "nested.arr");
        assert_eq!(res_6.get(0).unwrap(), "str1");
        assert_eq!(res_6.get(1).unwrap(), "str2");
    }

    #[test]
    #[should_panic]
    fn json_get_unexisted_prop() {
        let json = json!({});
        json_get(&json, "prop");
    }

    #[test]
    #[should_panic]
    fn json_get_unexisted_nested_prop() {
        let json = json!({
            "prop": 1
        });
        json_get(&json, "prop.a");
    }

    #[test]
    fn replace_html_vars_simple() {
        let res = replace_html_vars(
            r#"
            <div>{{basics.name}} {{ basics.label }}</div>
            <p>{{ basics.location.city}} {{basics.location.country    }}</p>
        "#,
            get_resume_json(),
        );
        assert_eq!(
            res,
            r#"
            <div>John Doe Programmer</div>
            <p>San Francisco The Johnited States Of Doe</p>
        "#
        );
    }

    #[test]
    fn replace_html_array_vars() {
        let escape = |s: &str| s.replace(" ", "");
        let html_1 = escape(
            r#"
            {!basics.profiles
                <p>{network}</p>
                <h1>{ username }</h1>
                <font>{   url}</font>
            !}
        "#,
        );
        let res_1 = replace_html_vars(&html_1, get_resume_json());
        assert_eq!(
            res_1,
            escape(
                r#"
            <p>a</p>
            <h1>b</h1>
            <font>c</font>
            <p>a2</p>
            <h1>b2</h1>
            <font>c2</font>

        "#
            )
        );

        let html_2 = escape(
            r#"
            {! languages
                <p>{language  }</p>
                <h1>{ level}</h1>
                <font id="{language}"></font>
            !}
            {! projects
                {name} { description }!}
        "#,
        );
        let res_2 = replace_html_vars(&html_2, get_resume_json());
        assert_eq!(
            res_2,
            escape(
                r#"
            <p>English</p>
            <h1>Native</h1>
            <font id="English"></font>
            <p>Spanish</p>
            <h1>Fluent</h1>
            <font id="Spanish"></font>

            a b
        "#
            )
        );
    }
}

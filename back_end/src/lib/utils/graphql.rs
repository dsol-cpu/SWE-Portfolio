use juniper::{ Value as JuniperValue, Object };
use serde_json::Value as JsonValue;
use std::convert::TryFrom;

/// Converts a serde_json::Value into a juniper::Value
pub fn convert_to_juniper_value(
    json: JsonValue
) -> Result<JuniperValue, Box<dyn std::error::Error>> {
    match json {
        JsonValue::Null => Ok(JuniperValue::null()),
        JsonValue::Bool(b) => Ok(JuniperValue::scalar(b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i >= i64::from(i32::MIN) && i <= i64::from(i32::MAX) {
                    Ok(JuniperValue::scalar(i32::try_from(i)?))
                } else {
                    // Handle numbers outside i32 range as floats
                    Ok(JuniperValue::scalar(n.as_f64().unwrap()))
                }
            } else if let Some(f) = n.as_f64() {
                Ok(JuniperValue::scalar(f))
            } else {
                Err("Invalid number format".into())
            }
        }
        JsonValue::String(s) => Ok(JuniperValue::scalar(s)),
        JsonValue::Array(arr) => {
            let converted: Result<Vec<JuniperValue>, Box<dyn std::error::Error>> = arr
                .into_iter()
                .map(convert_to_juniper_value)
                .collect();
            Ok(JuniperValue::list(converted?))
        }
        JsonValue::Object(map) => {
            let mut object = Object::with_capacity(map.len());
            for (key, value) in map {
                object.add_field(key, convert_to_juniper_value(value)?);
            }
            Ok(JuniperValue::object(object))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_convert_basic_types() {
        assert_eq!(convert_to_juniper_value(json!(null)).unwrap(), JuniperValue::null());
        assert_eq!(convert_to_juniper_value(json!(true)).unwrap(), JuniperValue::scalar(true));
        assert_eq!(convert_to_juniper_value(json!(42)).unwrap(), JuniperValue::scalar(42));
        assert_eq!(convert_to_juniper_value(json!(3.14)).unwrap(), JuniperValue::scalar(3.14));
        assert_eq!(
            convert_to_juniper_value(json!("hello")).unwrap(),
            JuniperValue::scalar("hello".to_string())
        );
    }

    #[test]
    fn test_convert_array() {
        let json_array = json!([1, "test", true]);
        let juniper_array = convert_to_juniper_value(json_array).unwrap();

        if let JuniperValue::List(items) = juniper_array {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], JuniperValue::scalar(1));
            assert_eq!(items[1], JuniperValue::scalar("test".to_string()));
            assert_eq!(items[2], JuniperValue::scalar(true));
        } else {
            panic!("Expected List variant");
        }
    }

    #[test]
    fn test_convert_object() {
        let json_obj =
            json!({
            "name": "test",
            "age": 30,
            "active": true
        });
        let juniper_obj = convert_to_juniper_value(json_obj).unwrap();

        if let JuniperValue::Object(obj) = juniper_obj {
            assert_eq!(
                obj.get_field_value("name").unwrap(),
                &JuniperValue::scalar("test".to_string())
            );
            assert_eq!(obj.get_field_value("age").unwrap(), &JuniperValue::scalar(30));
            assert_eq!(obj.get_field_value("active").unwrap(), &JuniperValue::scalar(true));
        } else {
            panic!("Expected Object variant");
        }
    }

    #[test]
    fn test_large_numbers() {
        let large_number = i64::from(i32::MAX) + 1;
        let json_large = json!(large_number);
        let result = convert_to_juniper_value(json_large).unwrap();
        assert!(matches!(result, JuniperValue::Scalar(_)));
    }
}

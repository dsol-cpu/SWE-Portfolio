use async_graphql::{
    InputType,
    OutputType,
    InputValueError,
    InputValueResult,
    Value,
    registry::Registry,
    parser::types::Field,
    parser::Positioned,
    ContextSelectionSet,
    registry::MetaType,
    ServerResult,
};
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use std::borrow::Cow;
use std::future::Future;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeScalar(pub DateTime<Utc>);

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct Repository {
    name: String,
    description: Option<String>,
    languages: Option<Vec<Language>>,
    last_updated_at: DateTimeScalar,
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct Language {
    name: String,
    color: String,
}

impl OutputType for DateTimeScalar {
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("DateTime")
    }

    fn create_type_info(registry: &mut Registry) -> String {
        registry.create_input_type::<Self, _>(async_graphql::registry::MetaTypeId::Scalar, |_| {
            MetaType::Scalar {
                name: "DateTime".to_string(),
                description: Some(
                    "A scalar representing a date and time in RFC3339 format".to_string()
                ),
                is_valid: None,
                visible: None,
                inaccessible: false,
                tags: vec![],
                specified_by_url: None,
            }
        })
    }

    fn resolve(
        &self,
        _ctx: &ContextSelectionSet<'_>,
        _field: &Positioned<Field>
    ) -> impl Future<Output = ServerResult<Value>> + Send {
        let value = Value::String(self.0.to_rfc3339());
        async move { Ok(value) }
    }
}

impl InputType for DateTimeScalar {
    type RawValueType = Value;

    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed("DateTime")
    }

    fn create_type_info(registry: &mut Registry) -> String {
        registry.create_input_type::<Self, _>(async_graphql::registry::MetaTypeId::Scalar, |_| {
            MetaType::Scalar {
                name: "DateTime".to_string(),
                description: Some(
                    "A scalar representing a date and time in RFC3339 format".to_string()
                ),
                is_valid: None,
                visible: None,
                inaccessible: false,
                tags: vec![],
                specified_by_url: None,
            }
        })
    }

    fn parse(value: Option<Value>) -> InputValueResult<Self> {
        match value {
            Some(Value::String(s)) => {
                DateTime::parse_from_rfc3339(&s)
                    .map(|dt| DateTimeScalar(dt.with_timezone(&Utc)))
                    .map_err(|err| InputValueError::custom(err.to_string()))
            }
            Some(v) => Err(InputValueError::expected_type(v)),
            None => Err(InputValueError::custom("No value provided")),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }

    fn as_raw_value(&self) -> Option<&Value> {
        static VALUE: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
        Some(VALUE.get_or_init(|| self.to_value()))
    }
}

impl From<DateTime<Utc>> for DateTimeScalar {
    fn from(dt: DateTime<Utc>) -> Self {
        DateTimeScalar(dt)
    }
}

impl From<DateTimeScalar> for DateTime<Utc> {
    fn from(scalar: DateTimeScalar) -> Self {
        scalar.0
    }
}

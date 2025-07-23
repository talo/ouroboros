use crate::Type;

impl sqlx::Type<sqlx::Postgres> for Type {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        sqlx::types::JsonValue::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Type {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        serde_json::to_value(self)
            .expect("type is valid json")
            .encode_by_ref(buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Type {
    fn decode(
        value: <sqlx::Postgres as sqlx::database::Database>::ValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(serde_json::from_value(sqlx::types::JsonValue::decode(
            value,
        )?)?)
    }
}

impl sqlx::postgres::PgHasArrayType for Type {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::types::JsonValue::array_type_info()
    }
}

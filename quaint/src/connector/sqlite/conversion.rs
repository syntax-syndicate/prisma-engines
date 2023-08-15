use std::convert::TryFrom;

use crate::{
    ast::Value,
    connector::{
        queryable::{GetRow, ToColumnNames},
        TypeIdentifier,
    },
    error::{Error, ErrorKind},
};

use libsql::{Column, Row as LibsqlRow, Rows as SqliteRows, Value as LibsqlValue, ValueRef};

#[cfg(feature = "chrono")]
use chrono::TimeZone;

impl TypeIdentifier for Column<'_> {
    fn is_real(&self) -> bool {
        match self.decl_type() {
            Some(n) if n.starts_with("DECIMAL") => true,
            Some(n) if n.starts_with("decimal") => true,
            _ => false,
        }
    }

    fn is_float(&self) -> bool {
        matches!(self.decl_type(), Some("FLOAT") | Some("float"))
    }

    fn is_double(&self) -> bool {
        matches!(
            self.decl_type(),
            Some("DOUBLE")
                | Some("double")
                | Some("DOUBLE PRECISION")
                | Some("double precision")
                | Some("numeric")
                | Some("NUMERIC")
                | Some("real")
                | Some("REAL")
        )
    }

    fn is_int32(&self) -> bool {
        matches!(
            self.decl_type(),
            Some("TINYINT")
                | Some("tinyint")
                | Some("SMALLINT")
                | Some("smallint")
                | Some("MEDIUMINT")
                | Some("mediumint")
                | Some("INT")
                | Some("int")
                | Some("INTEGER")
                | Some("integer")
                | Some("SERIAL")
                | Some("serial")
                | Some("INT2")
                | Some("int2")
        )
    }

    fn is_int64(&self) -> bool {
        matches!(
            self.decl_type(),
            Some("BIGINT")
                | Some("bigint")
                | Some("UNSIGNED BIG INT")
                | Some("unsigned big int")
                | Some("INT8")
                | Some("int8")
        )
    }

    fn is_datetime(&self) -> bool {
        matches!(
            self.decl_type(),
            Some("DATETIME") | Some("datetime") | Some("TIMESTAMP") | Some("timestamp")
        )
    }

    fn is_time(&self) -> bool {
        false
    }

    fn is_date(&self) -> bool {
        matches!(self.decl_type(), Some("DATE") | Some("date"))
    }

    fn is_text(&self) -> bool {
        match self.decl_type() {
            Some("TEXT") | Some("text") => true,
            Some("CLOB") | Some("clob") => true,
            Some(n) if n.starts_with("CHARACTER") => true,
            Some(n) if n.starts_with("character") => true,
            Some(n) if n.starts_with("VARCHAR") => true,
            Some(n) if n.starts_with("varchar") => true,
            Some(n) if n.starts_with("VARYING CHARACTER") => true,
            Some(n) if n.starts_with("varying character") => true,
            Some(n) if n.starts_with("NCHAR") => true,
            Some(n) if n.starts_with("nchar") => true,
            Some(n) if n.starts_with("NATIVE CHARACTER") => true,
            Some(n) if n.starts_with("native character") => true,
            Some(n) if n.starts_with("NVARCHAR") => true,
            Some(n) if n.starts_with("nvarchar") => true,
            _ => false,
        }
    }

    fn is_bytes(&self) -> bool {
        matches!(self.decl_type(), Some("BLOB") | Some("blob"))
    }

    fn is_bool(&self) -> bool {
        matches!(self.decl_type(), Some("BOOLEAN") | Some("boolean"))
    }

    fn is_json(&self) -> bool {
        false
    }
    fn is_enum(&self) -> bool {
        false
    }
    fn is_null(&self) -> bool {
        self.decl_type().is_none()
    }
}

impl<'a> GetRow for LibsqlRow {
    fn get_result_row(&self) -> crate::Result<Vec<Value<'static>>> {
        let statement = self.as_ref();
        let mut row = Vec::with_capacity(statement.columns().len());

        for (i, column) in statement.columns().iter().enumerate() {
            let pv = match self.get_ref_unwrap(i) {
                ValueRef::Null => match column {
                    // NOTE: A value without decl_type would be Int32(None)
                    c if c.is_int32() | c.is_null() => Value::Int32(None),
                    c if c.is_int64() => Value::Int64(None),
                    c if c.is_text() => Value::Text(None),
                    c if c.is_bytes() => Value::Bytes(None),
                    c if c.is_float() => Value::Float(None),
                    c if c.is_double() => Value::Double(None),
                    #[cfg(feature = "bigdecimal")]
                    c if c.is_real() => Value::Numeric(None),
                    #[cfg(feature = "chrono")]
                    c if c.is_datetime() => Value::DateTime(None),
                    #[cfg(feature = "chrono")]
                    c if c.is_date() => Value::Date(None),
                    c if c.is_bool() => Value::Boolean(None),
                    c => match c.decl_type() {
                        Some(n) => {
                            let msg = format!("Value {n} not supported");
                            let kind = ErrorKind::conversion(msg);

                            return Err(Error::builder(kind).build());
                        }
                        // When we don't know what to do, the default value would be Int32(None)
                        None => Value::Int32(None),
                    },
                },
                ValueRef::Integer(i) => {
                    match column {
                        c if c.is_bool() => {
                            if i == 0 {
                                Value::boolean(false)
                            } else {
                                Value::boolean(true)
                            }
                        }
                        #[cfg(feature = "chrono")]
                        c if c.is_date() => {
                            let dt = chrono::NaiveDateTime::from_timestamp_opt(i / 1000, 0).unwrap();
                            Value::date(dt.date())
                        }
                        #[cfg(feature = "chrono")]
                        c if c.is_datetime() => {
                            let dt = chrono::Utc.timestamp_millis_opt(i).unwrap();
                            Value::datetime(dt)
                        }
                        c if c.is_int32() => {
                            if let Ok(converted) = i32::try_from(i) {
                                Value::int32(converted)
                            } else {
                                let msg = format!("Value {} does not fit in an INT column, try migrating the '{}' column type to BIGINT", i, c.name());
                                let kind = ErrorKind::conversion(msg);

                                return Err(Error::builder(kind).build());
                            }
                        }
                        // NOTE: When SQLite does not know what type the return is (for example at explicit values and RETURNING statements) we will 'assume' int64
                        _ => Value::int64(i),
                    }
                }
                #[cfg(feature = "bigdecimal")]
                ValueRef::Real(f) if column.is_real() => {
                    use bigdecimal::{BigDecimal, FromPrimitive};

                    Value::numeric(BigDecimal::from_f64(f).unwrap())
                }
                ValueRef::Real(f) => Value::double(f),
                #[cfg(feature = "chrono")]
                ValueRef::Text(bytes) if column.is_datetime() => {
                    let parse_res = std::str::from_utf8(bytes).map_err(|_| {
                        let builder = Error::builder(ErrorKind::ConversionError(
                            "Failed to read contents of SQLite datetime column as UTF-8".into(),
                        ));
                        builder.build()
                    });

                    parse_res.and_then(|s| {
                        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                            .map(|nd| chrono::DateTime::<chrono::Utc>::from_utc(nd, chrono::Utc))
                            .or_else(|_| {
                                chrono::DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&chrono::Utc))
                            })
                            .or_else(|_| {
                                chrono::DateTime::parse_from_rfc2822(s).map(|dt| dt.with_timezone(&chrono::Utc))
                            })
                            .map(Value::datetime)
                            .map_err(|chrono_error| {
                                let builder =
                                    Error::builder(ErrorKind::ConversionError(chrono_error.to_string().into()));
                                builder.build()
                            })
                    })?
                }
                ValueRef::Text(bytes) => Value::text(String::from_utf8(bytes.to_vec())?),
                ValueRef::Blob(bytes) => Value::bytes(bytes.to_owned()),
            };

            row.push(pv);
        }

        Ok(row)
    }
}

impl<'a> ToColumnNames for SqliteRows {
    fn to_column_names(&self) -> Vec<String> {
        self.as_ref().column_names().into_iter().map(|c| c.into()).collect()
    }
}

impl<'a> TryFrom<&Value<'a>> for LibsqlValue {
    type Error = Error;

    fn try_from(value: &Value<'a>) -> Result<LibsqlValue, Self::Error> {
        let value = match value {
            Value::Int32(integer) => integer.map(LibsqlValue::from).map(Ok),
            // TODO(libsql): From implementation for i64
            Value::Int64(integer) => integer.map(LibsqlValue::Integer).map(Ok),
            // TODO(libsql): From implementation for f32
            Value::Float(float) => float.map(|f| f as f64).map(LibsqlValue::Real).map(Ok),
            // TODO(libsql): From implementation for f64
            Value::Double(double) => double.map(LibsqlValue::Real).map(Ok),
            // TODO(libsql): important: this clones the string, rusqlite retained a reference via ValueRef
            Value::Text(cow) => cow.as_ref().map(|cow| LibsqlValue::from(cow.as_ref())).map(Ok),
            // TODO(libsql): important: this clones the string, rusqlite retained a reference via ValueRef
            Value::Enum(cow) => cow.as_ref().map(|cow| LibsqlValue::from(cow.as_ref())).map(Ok),
            // TODO(libsql): From implementation for bool
            Value::Boolean(boo) => boo.map(|b| LibsqlValue::Integer(b as i64)).map(Ok),
            // TODO(libsql): From implementation for u8
            Value::Char(c) => c.map(|c| LibsqlValue::Integer(c as i64)).map(Ok),
            // TODO(libsql): important: this clones the bytes, rusqlite retained a reference via ValueRef
            Value::Bytes(bytes) => bytes.as_ref().map(|bytes| LibsqlValue::from(bytes.to_vec())).map(Ok),
            Value::Array(_) => {
                let msg = "Arrays are not supported in SQLite.";
                let kind = ErrorKind::conversion(msg);

                let mut builder = Error::builder(kind);
                builder.set_original_message(msg);

                return Err(builder.build());
            }
            #[cfg(feature = "bigdecimal")]
            Value::Numeric(d) => d.as_ref().map(|d| {
                // TODO(libsql): From implementation for f64
                d.to_string().parse::<f64>().map(LibsqlValue::Real).map_err(|err| {
                    Error::builder(ErrorKind::conversion("BigDecimal is not a f64."))
                        .set_original_message(err.to_string())
                        .build()
                })
            }),
            #[cfg(feature = "json")]
            Value::Json(value) => value.as_ref().map(|value| {
                // TODO(libsql): From implementation for String
                serde_json::to_string(value).map(LibsqlValue::Text).map_err(|err| {
                    Error::builder(ErrorKind::conversion("JSON serialization error"))
                        .set_original_message(err.to_string())
                        .build()
                })
            }),
            // TODO(libsql): important: this clones the string, rusqlite retained a reference via ValueRef
            Value::Xml(cow) => cow.as_ref().map(|cow| LibsqlValue::from(cow.as_ref())).map(Ok),
            #[cfg(feature = "uuid")]
            // TODO(libsql): From implementation for String
            Value::Uuid(value) => value
                .map(|value| LibsqlValue::Text(value.hyphenated().to_string()))
                .map(Ok),
            #[cfg(feature = "chrono")]
            // TODO(libsql): From implementation for i64
            Value::DateTime(value) => value
                .map(|value| LibsqlValue::Integer(value.timestamp_millis()))
                .map(Ok),
            #[cfg(feature = "chrono")]
            Value::Date(date) => date
                .and_then(|date| date.and_hms_opt(0, 0, 0))
                // TODO(libsql): From implementation for i64
                .map(|dt| LibsqlValue::Integer(dt.timestamp_millis()))
                .map(Ok),
            #[cfg(feature = "chrono")]
            Value::Time(time) => time
                .and_then(|time| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).map(|d| (d, time)))
                .and_then(|(date, time)| {
                    use chrono::Timelike;
                    date.and_hms_opt(time.hour(), time.minute(), time.second())
                })
                // TODO(libsql): From implementation for i64
                .map(|dt| LibsqlValue::Integer(dt.timestamp_millis()))
                .map(Ok),
        };

        match value {
            Some(value) => value,
            None => Ok(LibsqlValue::from(LibsqlValue::Null)),
        }
    }
}

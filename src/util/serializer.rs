pub mod date_serializer {
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    fn time_to_json(t: NaiveDateTime) -> String {
        let datetime: DateTime<Local> = Local.from_utc_datetime(&t);
        let datetime_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        // log::info!("datetime: {}", &datetime_string);
        datetime_string
    }

    pub fn serialize<S: Serializer>(
        time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").map_err(D::Error::custom)?)
    }
}

pub mod option_date_serializer {

    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    fn time_to_json(t: NaiveDateTime) -> Option<String> {
        let datetime: DateTime<Local> = Local.from_utc_datetime(&t);
        let datetime_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        // log::info!("datetime: {}", &datetime_string);
        Some(datetime_string)
    }

    pub fn serialize<S: Serializer>(
        time: &Option<NaiveDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let option_time = time.clone();
        match option_time {
            Some(value) => time_to_json(value).serialize(serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<NaiveDateTime>, D::Error> {
        let time: Option<String> = Option::deserialize(deserializer)?;
        let datetime = time.map(|t| {
            NaiveDateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S")
                .map_err(D::Error::custom)
                .unwrap()
        });

        Ok(datetime)
    }
}


pub mod sorts_serializer {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    use crate::dto::request::sort_request::Sort;

    pub fn serialize<S: Serializer>(
        sorts: &Option<Vec<Sort>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match sorts {
            Some(vec) => {
                let json_string = serde_json::to_string(vec).map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&json_string)
            }
            None => serializer.serialize_str("[]"),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Vec<Sort>>, D::Error> {
        let sorts_json: String = Deserialize::deserialize(deserializer)?;
        match serde_json::from_str(&sorts_json) {
            Ok(value) => {
                return Ok(Some(value));
            }
            Err(error) => {
                let deserialize_error = D::Error::custom(format!("{}", error));
                return Err(deserialize_error);
            }
        };
    }
}



pub mod filters_serializer {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    use crate::dto::request::filter_request::Filter;

    pub fn serialize<S: Serializer>(
        sorts: &Option<Vec<Filter>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match sorts {
            Some(vec) => {
                let json_string = serde_json::to_string(vec).map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&json_string)
            }
            None => serializer.serialize_str("[]"),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Vec<Filter>>, D::Error> {
        let sorts_json_result = Deserialize::deserialize(deserializer);
        let sorts_json: String;
        match sorts_json_result {
            Ok(value) => {
                sorts_json = value;
            }
            Err(error) => {
                // return Err(CustomError::DeserializeError(format!("{}", error).into()));
                return Err(error);
            }
        };
        match serde_json::from_str(&sorts_json) {
            Ok(value) => {
                let vec: Vec<Filter> = value;
                return Ok(Some(vec));
            }
            Err(error) => {
                // return Err(CustomError::DeserializeError(format!("{}", error).into()));
                let deserialize_error = D::Error::custom(format!("{}", error));
                return Err(deserialize_error);
            }
        };
    }
}

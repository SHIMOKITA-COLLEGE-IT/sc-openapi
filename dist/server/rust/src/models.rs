#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

/// `Program`, `House`のモデル
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Generation {
    /// Airtable's Record ID
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub r#type: models::GenerationType,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "coverImageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cover_image_url: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// Markdown
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "photoUrls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_urls: Option<Vec<String>>,

}

impl Generation {
    #[allow(clippy::new_without_default)]
    pub fn new(id: String, r#type: models::GenerationType, display_name: String, ) -> Generation {
        Generation {
            id,
            r#type,
            display_name,
            from: None,
            to: None,
            cover_image_url: None,
            title: None,
            description: None,
            photo_urls: None,
        }
    }
}

/// Converts the Generation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Generation {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),

            // Skipping type in query parameter serialization


            Some("displayName".to_string()),
            Some(self.display_name.to_string()),

            // Skipping from in query parameter serialization

            // Skipping to in query parameter serialization


            self.cover_image_url.as_ref().map(|cover_image_url| {
                vec![
                    "coverImageUrl".to_string(),
                    cover_image_url.to_string(),
                ].join(",")
            }),


            self.title.as_ref().map(|title| {
                vec![
                    "title".to_string(),
                    title.to_string(),
                ].join(",")
            }),


            self.description.as_ref().map(|description| {
                vec![
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),


            self.photo_urls.as_ref().map(|photo_urls| {
                vec![
                    "photoUrls".to_string(),
                    photo_urls.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Generation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Generation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub r#type: Vec<models::GenerationType>,
            pub display_name: Vec<String>,
            pub from: Vec<chrono::DateTime::<chrono::Utc>>,
            pub to: Vec<chrono::DateTime::<chrono::Utc>>,
            pub cover_image_url: Vec<String>,
            pub title: Vec<String>,
            pub description: Vec<String>,
            pub photo_urls: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Generation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::GenerationType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "displayName" => intermediate_rep.display_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "from" => intermediate_rep.from.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "to" => intermediate_rep.to.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "coverImageUrl" => intermediate_rep.cover_image_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "photoUrls" => return std::result::Result::Err("Parsing a container in this style is not supported in Generation".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Generation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Generation {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in Generation".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in Generation".to_string())?,
            display_name: intermediate_rep.display_name.into_iter().next().ok_or_else(|| "displayName missing in Generation".to_string())?,
            from: intermediate_rep.from.into_iter().next(),
            to: intermediate_rep.to.into_iter().next(),
            cover_image_url: intermediate_rep.cover_image_url.into_iter().next(),
            title: intermediate_rep.title.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            photo_urls: intermediate_rep.photo_urls.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Generation> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Generation>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Generation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Generation - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Generation> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Generation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Generation - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enum
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum GenerationType {
    #[serde(rename = "program")]
    Program,
    #[serde(rename = "house")]
    House,
}

impl std::fmt::Display for GenerationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GenerationType::Program => write!(f, "program"),
            GenerationType::House => write!(f, "house"),
        }
    }
}

impl std::str::FromStr for GenerationType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "program" => std::result::Result::Ok(GenerationType::Program),
            "house" => std::result::Result::Ok(GenerationType::House),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// `Committee`, `Club`のモデル
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Group {
    /// Airtable's Record ID
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub r#type: models::GroupType,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "archivedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived_at: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "emoji")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emoji: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// Markdown
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "slackChannel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slack_channel: Option<String>,

    #[serde(rename = "photoUrls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_urls: Option<Vec<String>>,

}

impl Group {
    #[allow(clippy::new_without_default)]
    pub fn new(id: String, r#type: models::GroupType, display_name: String, ) -> Group {
        Group {
            id,
            r#type,
            display_name,
            created_at: None,
            archived_at: None,
            emoji: None,
            title: None,
            description: None,
            slack_channel: None,
            photo_urls: None,
        }
    }
}

/// Converts the Group value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Group {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),

            // Skipping type in query parameter serialization


            Some("displayName".to_string()),
            Some(self.display_name.to_string()),

            // Skipping createdAt in query parameter serialization

            // Skipping archivedAt in query parameter serialization


            self.emoji.as_ref().map(|emoji| {
                vec![
                    "emoji".to_string(),
                    emoji.to_string(),
                ].join(",")
            }),


            self.title.as_ref().map(|title| {
                vec![
                    "title".to_string(),
                    title.to_string(),
                ].join(",")
            }),


            self.description.as_ref().map(|description| {
                vec![
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),


            self.slack_channel.as_ref().map(|slack_channel| {
                vec![
                    "slackChannel".to_string(),
                    slack_channel.to_string(),
                ].join(",")
            }),


            self.photo_urls.as_ref().map(|photo_urls| {
                vec![
                    "photoUrls".to_string(),
                    photo_urls.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Group value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Group {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub r#type: Vec<models::GroupType>,
            pub display_name: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub archived_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub emoji: Vec<String>,
            pub title: Vec<String>,
            pub description: Vec<String>,
            pub slack_channel: Vec<String>,
            pub photo_urls: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Group".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<models::GroupType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "displayName" => intermediate_rep.display_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "archivedAt" => intermediate_rep.archived_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "emoji" => intermediate_rep.emoji.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "slackChannel" => intermediate_rep.slack_channel.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "photoUrls" => return std::result::Result::Err("Parsing a container in this style is not supported in Group".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Group".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Group {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in Group".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in Group".to_string())?,
            display_name: intermediate_rep.display_name.into_iter().next().ok_or_else(|| "displayName missing in Group".to_string())?,
            created_at: intermediate_rep.created_at.into_iter().next(),
            archived_at: intermediate_rep.archived_at.into_iter().next(),
            emoji: intermediate_rep.emoji.into_iter().next(),
            title: intermediate_rep.title.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            slack_channel: intermediate_rep.slack_channel.into_iter().next(),
            photo_urls: intermediate_rep.photo_urls.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Group> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Group>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Group>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Group - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Group> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Group as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Group - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enum
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum GroupType {
    #[serde(rename = "committee")]
    Committee,
    #[serde(rename = "club")]
    Club,
    #[serde(rename = "project")]
    Project,
}

impl std::fmt::Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GroupType::Committee => write!(f, "committee"),
            GroupType::Club => write!(f, "club"),
            GroupType::Project => write!(f, "project"),
        }
    }
}

impl std::str::FromStr for GroupType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "committee" => std::result::Result::Ok(GroupType::Committee),
            "club" => std::result::Result::Ok(GroupType::Club),
            "project" => std::result::Result::Ok(GroupType::Project),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Enum
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum Locale {
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "en")]
    En,
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Locale::Ja => write!(f, "ja"),
            Locale::En => write!(f, "en"),
        }
    }
}

impl std::str::FromStr for Locale {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ja" => std::result::Result::Ok(Locale::Ja),
            "en" => std::result::Result::Ok(Locale::En),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LoginRequest {
    /// 取得方法は[こちら](https://firebase.google.com/docs/auth/admin/verify-id-tokens#retrieve_id_tokens_on_clients)
    #[serde(rename = "firebaseIdToken")]
    pub firebase_id_token: String,

}

impl LoginRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(firebase_id_token: String, ) -> LoginRequest {
        LoginRequest {
            firebase_id_token,
        }
    }
}

/// Converts the LoginRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LoginRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("firebaseIdToken".to_string()),
            Some(self.firebase_id_token.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LoginRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LoginRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub firebase_id_token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LoginRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "firebaseIdToken" => intermediate_rep.firebase_id_token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LoginRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LoginRequest {
            firebase_id_token: intermediate_rep.firebase_id_token.into_iter().next().ok_or_else(|| "firebaseIdToken missing in LoginRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LoginRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LoginRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LoginRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LoginRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LoginRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LoginRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LoginRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 初期登録フォーム未回答の場合、userも返す
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<models::User>,

}

impl LoginResponse {
    #[allow(clippy::new_without_default)]
    pub fn new(access_token: String, ) -> LoginResponse {
        LoginResponse {
            access_token,
            user: None,
        }
    }
}

/// Converts the LoginResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LoginResponse {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("accessToken".to_string()),
            Some(self.access_token.to_string()),

            // Skipping user in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LoginResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LoginResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub access_token: Vec<String>,
            pub user: Vec<models::User>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LoginResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "accessToken" => intermediate_rep.access_token.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "user" => intermediate_rep.user.push(<models::User as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LoginResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LoginResponse {
            access_token: intermediate_rep.access_token.into_iter().next().ok_or_else(|| "accessToken missing in LoginResponse".to_string())?,
            user: intermediate_rep.user.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LoginResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LoginResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LoginResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LoginResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LoginResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LoginResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LoginResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Enum
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum Pronoun {
    #[serde(rename = "SHE_HER")]
    SheHer,
    #[serde(rename = "HE_HIM")]
    HeHim,
    #[serde(rename = "THEY_THEM")]
    TheyThem,
}

impl std::fmt::Display for Pronoun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Pronoun::SheHer => write!(f, "SHE_HER"),
            Pronoun::HeHim => write!(f, "HE_HIM"),
            Pronoun::TheyThem => write!(f, "THEY_THEM"),
        }
    }
}

impl std::str::FromStr for Pronoun {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "SHE_HER" => std::result::Result::Ok(Pronoun::SheHer),
            "HE_HIM" => std::result::Result::Ok(Pronoun::HeHim),
            "THEY_THEM" => std::result::Result::Ok(Pronoun::TheyThem),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Enum
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum Role {
    #[serde(rename = "resident")]
    Resident,
    #[serde(rename = "alumni")]
    Alumni,
    #[serde(rename = "staff")]
    Staff,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Role::Resident => write!(f, "resident"),
            Role::Alumni => write!(f, "alumni"),
            Role::Staff => write!(f, "staff"),
        }
    }
}

impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "resident" => std::result::Result::Ok(Role::Resident),
            "alumni" => std::result::Result::Ok(Role::Alumni),
            "staff" => std::result::Result::Ok(Role::Staff),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SocialBrand {
    /// Airtable's Record ID
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "svgIconUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svg_icon_url: Option<String>,

    #[serde(rename = "urlPrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url_prefix: Option<String>,

    #[serde(rename = "formPrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub form_prefix: Option<String>,

}

impl SocialBrand {
    #[allow(clippy::new_without_default)]
    pub fn new(id: String, name: String, ) -> SocialBrand {
        SocialBrand {
            id,
            name,
            svg_icon_url: None,
            url_prefix: None,
            form_prefix: None,
        }
    }
}

/// Converts the SocialBrand value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SocialBrand {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            self.svg_icon_url.as_ref().map(|svg_icon_url| {
                vec![
                    "svgIconUrl".to_string(),
                    svg_icon_url.to_string(),
                ].join(",")
            }),


            self.url_prefix.as_ref().map(|url_prefix| {
                vec![
                    "urlPrefix".to_string(),
                    url_prefix.to_string(),
                ].join(",")
            }),


            self.form_prefix.as_ref().map(|form_prefix| {
                vec![
                    "formPrefix".to_string(),
                    form_prefix.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SocialBrand value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SocialBrand {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub svg_icon_url: Vec<String>,
            pub url_prefix: Vec<String>,
            pub form_prefix: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SocialBrand".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "svgIconUrl" => intermediate_rep.svg_icon_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "urlPrefix" => intermediate_rep.url_prefix.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "formPrefix" => intermediate_rep.form_prefix.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SocialBrand".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SocialBrand {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in SocialBrand".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in SocialBrand".to_string())?,
            svg_icon_url: intermediate_rep.svg_icon_url.into_iter().next(),
            url_prefix: intermediate_rep.url_prefix.into_iter().next(),
            form_prefix: intermediate_rep.form_prefix.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SocialBrand> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SocialBrand>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SocialBrand>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SocialBrand - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SocialBrand> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SocialBrand as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SocialBrand - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateUserRequest {
    #[serde(rename = "i18n")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub i18n: Option<Vec<models::UpdateUserRequestI18nInner>>,

    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<String>,

    #[serde(rename = "pronoun")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pronoun: Option<models::Pronoun>,

    #[serde(rename = "birthday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub birthday: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "hideAge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hide_age: Option<bool>,

    #[serde(rename = "socialLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub social_links: Option<Vec<models::UpdateUserRequestSocialLinksInner>>,

    #[serde(rename = "roomNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub room_number: Option<f64>,

    #[serde(rename = "postNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub post_number: Option<f64>,

    #[serde(rename = "selfIntroductionSlideUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub self_introduction_slide_url: Option<String>,

    /// Airtable's Record IDs
    #[serde(rename = "generations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub generations: Option<Vec<String>>,

    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<models::UpdateUserRequestGroupsInner>>,

    /// 肩書き
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<String>,

    /// Markdown
    #[serde(rename = "bio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<String>,

    #[serde(rename = "photoUrls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_urls: Option<Vec<String>>,

    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_at: Option<chrono::DateTime::<chrono::Utc>>,

}

impl UpdateUserRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> UpdateUserRequest {
        UpdateUserRequest {
            i18n: None,
            image_url: None,
            pronoun: None,
            birthday: None,
            hide_age: Some(false),
            social_links: None,
            room_number: None,
            post_number: None,
            self_introduction_slide_url: None,
            generations: None,
            groups: None,
            position: None,
            bio: None,
            photo_urls: None,
            last_modified_at: None,
        }
    }
}

/// Converts the UpdateUserRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateUserRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping i18n in query parameter serialization


            self.image_url.as_ref().map(|image_url| {
                vec![
                    "imageUrl".to_string(),
                    image_url.to_string(),
                ].join(",")
            }),

            // Skipping pronoun in query parameter serialization

            // Skipping birthday in query parameter serialization


            self.hide_age.as_ref().map(|hide_age| {
                vec![
                    "hideAge".to_string(),
                    hide_age.to_string(),
                ].join(",")
            }),

            // Skipping socialLinks in query parameter serialization


            self.room_number.as_ref().map(|room_number| {
                vec![
                    "roomNumber".to_string(),
                    room_number.to_string(),
                ].join(",")
            }),


            self.post_number.as_ref().map(|post_number| {
                vec![
                    "postNumber".to_string(),
                    post_number.to_string(),
                ].join(",")
            }),


            self.self_introduction_slide_url.as_ref().map(|self_introduction_slide_url| {
                vec![
                    "selfIntroductionSlideUrl".to_string(),
                    self_introduction_slide_url.to_string(),
                ].join(",")
            }),


            self.generations.as_ref().map(|generations| {
                vec![
                    "generations".to_string(),
                    generations.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping groups in query parameter serialization


            self.position.as_ref().map(|position| {
                vec![
                    "position".to_string(),
                    position.to_string(),
                ].join(",")
            }),


            self.bio.as_ref().map(|bio| {
                vec![
                    "bio".to_string(),
                    bio.to_string(),
                ].join(",")
            }),


            self.photo_urls.as_ref().map(|photo_urls| {
                vec![
                    "photoUrls".to_string(),
                    photo_urls.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping lastModifiedAt in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateUserRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateUserRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub i18n: Vec<Vec<models::UpdateUserRequestI18nInner>>,
            pub image_url: Vec<String>,
            pub pronoun: Vec<models::Pronoun>,
            pub birthday: Vec<chrono::DateTime::<chrono::Utc>>,
            pub hide_age: Vec<bool>,
            pub social_links: Vec<Vec<models::UpdateUserRequestSocialLinksInner>>,
            pub room_number: Vec<f64>,
            pub post_number: Vec<f64>,
            pub self_introduction_slide_url: Vec<String>,
            pub generations: Vec<Vec<String>>,
            pub groups: Vec<Vec<models::UpdateUserRequestGroupsInner>>,
            pub position: Vec<String>,
            pub bio: Vec<String>,
            pub photo_urls: Vec<Vec<String>>,
            pub last_modified_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateUserRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "i18n" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateUserRequest".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "imageUrl" => intermediate_rep.image_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pronoun" => intermediate_rep.pronoun.push(<models::Pronoun as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "birthday" => intermediate_rep.birthday.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hideAge" => intermediate_rep.hide_age.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "socialLinks" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateUserRequest".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "roomNumber" => intermediate_rep.room_number.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "postNumber" => intermediate_rep.post_number.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "selfIntroductionSlideUrl" => intermediate_rep.self_introduction_slide_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "generations" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateUserRequest".to_string()),
                    "groups" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateUserRequest".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "position" => intermediate_rep.position.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "bio" => intermediate_rep.bio.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "photoUrls" => return std::result::Result::Err("Parsing a container in this style is not supported in UpdateUserRequest".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "lastModifiedAt" => intermediate_rep.last_modified_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateUserRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateUserRequest {
            i18n: intermediate_rep.i18n.into_iter().next(),
            image_url: intermediate_rep.image_url.into_iter().next(),
            pronoun: intermediate_rep.pronoun.into_iter().next(),
            birthday: intermediate_rep.birthday.into_iter().next(),
            hide_age: intermediate_rep.hide_age.into_iter().next(),
            social_links: intermediate_rep.social_links.into_iter().next(),
            room_number: intermediate_rep.room_number.into_iter().next(),
            post_number: intermediate_rep.post_number.into_iter().next(),
            self_introduction_slide_url: intermediate_rep.self_introduction_slide_url.into_iter().next(),
            generations: intermediate_rep.generations.into_iter().next(),
            groups: intermediate_rep.groups.into_iter().next(),
            position: intermediate_rep.position.into_iter().next(),
            bio: intermediate_rep.bio.into_iter().next(),
            photo_urls: intermediate_rep.photo_urls.into_iter().next(),
            last_modified_at: intermediate_rep.last_modified_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateUserRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateUserRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateUserRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateUserRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateUserRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateUserRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateUserRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateUserRequestGroupsInner {
    /// Airtable's Record ID
    #[serde(rename = "groupId")]
    pub group_id: String,

    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<chrono::DateTime::<chrono::Utc>>,

}

impl UpdateUserRequestGroupsInner {
    #[allow(clippy::new_without_default)]
    pub fn new(group_id: String, ) -> UpdateUserRequestGroupsInner {
        UpdateUserRequestGroupsInner {
            group_id,
            from: None,
            to: None,
        }
    }
}

/// Converts the UpdateUserRequestGroupsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateUserRequestGroupsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("groupId".to_string()),
            Some(self.group_id.to_string()),

            // Skipping from in query parameter serialization

            // Skipping to in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateUserRequestGroupsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateUserRequestGroupsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub group_id: Vec<String>,
            pub from: Vec<chrono::DateTime::<chrono::Utc>>,
            pub to: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateUserRequestGroupsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "groupId" => intermediate_rep.group_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "from" => intermediate_rep.from.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "to" => intermediate_rep.to.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateUserRequestGroupsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateUserRequestGroupsInner {
            group_id: intermediate_rep.group_id.into_iter().next().ok_or_else(|| "groupId missing in UpdateUserRequestGroupsInner".to_string())?,
            from: intermediate_rep.from.into_iter().next(),
            to: intermediate_rep.to.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateUserRequestGroupsInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateUserRequestGroupsInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateUserRequestGroupsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateUserRequestGroupsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateUserRequestGroupsInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateUserRequestGroupsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateUserRequestGroupsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateUserRequestI18nInner {
    #[serde(rename = "locale")]
    pub locale: models::Locale,

    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub first_name: Option<String>,

    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_name: Option<String>,

    #[serde(rename = "phoneticFirstName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phonetic_first_name: Option<String>,

    #[serde(rename = "phoneticLastName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phonetic_last_name: Option<String>,

    #[serde(rename = "nickName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub nick_name: Option<String>,

    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub middle_name: Option<String>,

    #[serde(rename = "phoneticMiddleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phonetic_middle_name: Option<String>,

    #[serde(rename = "maidenName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maiden_name: Option<String>,

    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prefix: Option<String>,

    #[serde(rename = "suffix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suffix: Option<String>,

}

impl UpdateUserRequestI18nInner {
    #[allow(clippy::new_without_default)]
    pub fn new(locale: models::Locale, ) -> UpdateUserRequestI18nInner {
        UpdateUserRequestI18nInner {
            locale,
            first_name: None,
            last_name: None,
            phonetic_first_name: None,
            phonetic_last_name: None,
            nick_name: None,
            middle_name: None,
            phonetic_middle_name: None,
            maiden_name: None,
            prefix: None,
            suffix: None,
        }
    }
}

/// Converts the UpdateUserRequestI18nInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateUserRequestI18nInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping locale in query parameter serialization


            self.first_name.as_ref().map(|first_name| {
                vec![
                    "firstName".to_string(),
                    first_name.to_string(),
                ].join(",")
            }),


            self.last_name.as_ref().map(|last_name| {
                vec![
                    "lastName".to_string(),
                    last_name.to_string(),
                ].join(",")
            }),


            self.phonetic_first_name.as_ref().map(|phonetic_first_name| {
                vec![
                    "phoneticFirstName".to_string(),
                    phonetic_first_name.to_string(),
                ].join(",")
            }),


            self.phonetic_last_name.as_ref().map(|phonetic_last_name| {
                vec![
                    "phoneticLastName".to_string(),
                    phonetic_last_name.to_string(),
                ].join(",")
            }),


            self.nick_name.as_ref().map(|nick_name| {
                vec![
                    "nickName".to_string(),
                    nick_name.to_string(),
                ].join(",")
            }),


            self.middle_name.as_ref().map(|middle_name| {
                vec![
                    "middleName".to_string(),
                    middle_name.to_string(),
                ].join(",")
            }),


            self.phonetic_middle_name.as_ref().map(|phonetic_middle_name| {
                vec![
                    "phoneticMiddleName".to_string(),
                    phonetic_middle_name.to_string(),
                ].join(",")
            }),


            self.maiden_name.as_ref().map(|maiden_name| {
                vec![
                    "maidenName".to_string(),
                    maiden_name.to_string(),
                ].join(",")
            }),


            self.prefix.as_ref().map(|prefix| {
                vec![
                    "prefix".to_string(),
                    prefix.to_string(),
                ].join(",")
            }),


            self.suffix.as_ref().map(|suffix| {
                vec![
                    "suffix".to_string(),
                    suffix.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateUserRequestI18nInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateUserRequestI18nInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub locale: Vec<models::Locale>,
            pub first_name: Vec<String>,
            pub last_name: Vec<String>,
            pub phonetic_first_name: Vec<String>,
            pub phonetic_last_name: Vec<String>,
            pub nick_name: Vec<String>,
            pub middle_name: Vec<String>,
            pub phonetic_middle_name: Vec<String>,
            pub maiden_name: Vec<String>,
            pub prefix: Vec<String>,
            pub suffix: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateUserRequestI18nInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "locale" => intermediate_rep.locale.push(<models::Locale as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "firstName" => intermediate_rep.first_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastName" => intermediate_rep.last_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phoneticFirstName" => intermediate_rep.phonetic_first_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phoneticLastName" => intermediate_rep.phonetic_last_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nickName" => intermediate_rep.nick_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "middleName" => intermediate_rep.middle_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phoneticMiddleName" => intermediate_rep.phonetic_middle_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maidenName" => intermediate_rep.maiden_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "prefix" => intermediate_rep.prefix.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "suffix" => intermediate_rep.suffix.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateUserRequestI18nInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateUserRequestI18nInner {
            locale: intermediate_rep.locale.into_iter().next().ok_or_else(|| "locale missing in UpdateUserRequestI18nInner".to_string())?,
            first_name: intermediate_rep.first_name.into_iter().next(),
            last_name: intermediate_rep.last_name.into_iter().next(),
            phonetic_first_name: intermediate_rep.phonetic_first_name.into_iter().next(),
            phonetic_last_name: intermediate_rep.phonetic_last_name.into_iter().next(),
            nick_name: intermediate_rep.nick_name.into_iter().next(),
            middle_name: intermediate_rep.middle_name.into_iter().next(),
            phonetic_middle_name: intermediate_rep.phonetic_middle_name.into_iter().next(),
            maiden_name: intermediate_rep.maiden_name.into_iter().next(),
            prefix: intermediate_rep.prefix.into_iter().next(),
            suffix: intermediate_rep.suffix.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateUserRequestI18nInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateUserRequestI18nInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateUserRequestI18nInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateUserRequestI18nInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateUserRequestI18nInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateUserRequestI18nInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateUserRequestI18nInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateUserRequestSocialLinksInner {
    /// Airtable's Record ID
    #[serde(rename = "brandId")]
    pub brand_id: String,

    #[serde(rename = "username")]
    pub username: String,

}

impl UpdateUserRequestSocialLinksInner {
    #[allow(clippy::new_without_default)]
    pub fn new(brand_id: String, username: String, ) -> UpdateUserRequestSocialLinksInner {
        UpdateUserRequestSocialLinksInner {
            brand_id,
            username,
        }
    }
}

/// Converts the UpdateUserRequestSocialLinksInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UpdateUserRequestSocialLinksInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("brandId".to_string()),
            Some(self.brand_id.to_string()),


            Some("username".to_string()),
            Some(self.username.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UpdateUserRequestSocialLinksInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UpdateUserRequestSocialLinksInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub brand_id: Vec<String>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UpdateUserRequestSocialLinksInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "brandId" => intermediate_rep.brand_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UpdateUserRequestSocialLinksInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UpdateUserRequestSocialLinksInner {
            brand_id: intermediate_rep.brand_id.into_iter().next().ok_or_else(|| "brandId missing in UpdateUserRequestSocialLinksInner".to_string())?,
            username: intermediate_rep.username.into_iter().next().ok_or_else(|| "username missing in UpdateUserRequestSocialLinksInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UpdateUserRequestSocialLinksInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UpdateUserRequestSocialLinksInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UpdateUserRequestSocialLinksInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UpdateUserRequestSocialLinksInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UpdateUserRequestSocialLinksInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UpdateUserRequestSocialLinksInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UpdateUserRequestSocialLinksInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct User {
    /// Airtable's Record ID
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "role")]
    pub role: models::Role,

    #[serde(rename = "i18n")]
    pub i18n: Vec<models::UserI18n>,

    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<String>,

    #[serde(rename = "pronoun")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pronoun: Option<models::Pronoun>,

    #[serde(rename = "birthday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub birthday: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "hideAge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hide_age: Option<bool>,

    #[serde(rename = "socialLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub social_links: Option<Vec<models::UserSocialLink>>,

    #[serde(rename = "roomNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub room_number: Option<f64>,

    #[serde(rename = "postNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub post_number: Option<f64>,

    #[serde(rename = "selfIntroductionSlideUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub self_introduction_slide_url: Option<String>,

    #[serde(rename = "programs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub programs: Option<Vec<models::Generation>>,

    #[serde(rename = "houses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub houses: Option<Vec<models::Generation>>,

    #[serde(rename = "committees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committees: Option<Vec<models::UserGroupBelonging>>,

    #[serde(rename = "clubs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clubs: Option<Vec<models::UserGroupBelonging>>,

    /// 肩書き
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<String>,

    /// Markdown
    #[serde(rename = "bio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<String>,

    #[serde(rename = "photoUrls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_urls: Option<Vec<String>>,

    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_at: Option<chrono::DateTime::<chrono::Utc>>,

}

impl User {
    #[allow(clippy::new_without_default)]
    pub fn new(id: String, email: String, role: models::Role, i18n: Vec<models::UserI18n>, ) -> User {
        User {
            id,
            email,
            role,
            i18n,
            image_url: None,
            pronoun: None,
            birthday: None,
            hide_age: Some(false),
            social_links: None,
            room_number: None,
            post_number: None,
            self_introduction_slide_url: None,
            programs: None,
            houses: None,
            committees: None,
            clubs: None,
            position: None,
            bio: None,
            photo_urls: None,
            last_modified_at: None,
        }
    }
}

/// Converts the User value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for User {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("email".to_string()),
            Some(self.email.to_string()),

            // Skipping role in query parameter serialization

            // Skipping i18n in query parameter serialization


            self.image_url.as_ref().map(|image_url| {
                vec![
                    "imageUrl".to_string(),
                    image_url.to_string(),
                ].join(",")
            }),

            // Skipping pronoun in query parameter serialization

            // Skipping birthday in query parameter serialization


            self.hide_age.as_ref().map(|hide_age| {
                vec![
                    "hideAge".to_string(),
                    hide_age.to_string(),
                ].join(",")
            }),

            // Skipping socialLinks in query parameter serialization


            self.room_number.as_ref().map(|room_number| {
                vec![
                    "roomNumber".to_string(),
                    room_number.to_string(),
                ].join(",")
            }),


            self.post_number.as_ref().map(|post_number| {
                vec![
                    "postNumber".to_string(),
                    post_number.to_string(),
                ].join(",")
            }),


            self.self_introduction_slide_url.as_ref().map(|self_introduction_slide_url| {
                vec![
                    "selfIntroductionSlideUrl".to_string(),
                    self_introduction_slide_url.to_string(),
                ].join(",")
            }),

            // Skipping programs in query parameter serialization

            // Skipping houses in query parameter serialization

            // Skipping committees in query parameter serialization

            // Skipping clubs in query parameter serialization


            self.position.as_ref().map(|position| {
                vec![
                    "position".to_string(),
                    position.to_string(),
                ].join(",")
            }),


            self.bio.as_ref().map(|bio| {
                vec![
                    "bio".to_string(),
                    bio.to_string(),
                ].join(",")
            }),


            self.photo_urls.as_ref().map(|photo_urls| {
                vec![
                    "photoUrls".to_string(),
                    photo_urls.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping lastModifiedAt in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a User value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for User {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub email: Vec<String>,
            pub role: Vec<models::Role>,
            pub i18n: Vec<Vec<models::UserI18n>>,
            pub image_url: Vec<String>,
            pub pronoun: Vec<models::Pronoun>,
            pub birthday: Vec<chrono::DateTime::<chrono::Utc>>,
            pub hide_age: Vec<bool>,
            pub social_links: Vec<Vec<models::UserSocialLink>>,
            pub room_number: Vec<f64>,
            pub post_number: Vec<f64>,
            pub self_introduction_slide_url: Vec<String>,
            pub programs: Vec<Vec<models::Generation>>,
            pub houses: Vec<Vec<models::Generation>>,
            pub committees: Vec<Vec<models::UserGroupBelonging>>,
            pub clubs: Vec<Vec<models::UserGroupBelonging>>,
            pub position: Vec<String>,
            pub bio: Vec<String>,
            pub photo_urls: Vec<Vec<String>>,
            pub last_modified_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing User".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "role" => intermediate_rep.role.push(<models::Role as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "i18n" => return std::result::Result::Err("Parsing a container in this style is not supported in User".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "imageUrl" => intermediate_rep.image_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pronoun" => intermediate_rep.pronoun.push(<models::Pronoun as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "birthday" => intermediate_rep.birthday.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hideAge" => intermediate_rep.hide_age.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "socialLinks" => return std::result::Result::Err("Parsing a container in this style is not supported in User".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "roomNumber" => intermediate_rep.room_number.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "postNumber" => intermediate_rep.post_number.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "selfIntroductionSlideUrl" => intermediate_rep.self_introduction_slide_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "programs" => return std::result::Result::Err("Parsing a container in this style is not supported in User".to_string()),
                    "houses" => return std::result::Result::Err("Parsing a container in this style is not supported in User".to_string()),
                    "committees" => return std::result::Result::Err("Parsing a container in this style is not supported in User".to_string()),
                    "clubs" => return std::result::Result::Err("Parsing a container in this style is not supported in User".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "position" => intermediate_rep.position.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "bio" => intermediate_rep.bio.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "photoUrls" => return std::result::Result::Err("Parsing a container in this style is not supported in User".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "lastModifiedAt" => intermediate_rep.last_modified_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing User".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(User {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in User".to_string())?,
            email: intermediate_rep.email.into_iter().next().ok_or_else(|| "email missing in User".to_string())?,
            role: intermediate_rep.role.into_iter().next().ok_or_else(|| "role missing in User".to_string())?,
            i18n: intermediate_rep.i18n.into_iter().next().ok_or_else(|| "i18n missing in User".to_string())?,
            image_url: intermediate_rep.image_url.into_iter().next(),
            pronoun: intermediate_rep.pronoun.into_iter().next(),
            birthday: intermediate_rep.birthday.into_iter().next(),
            hide_age: intermediate_rep.hide_age.into_iter().next(),
            social_links: intermediate_rep.social_links.into_iter().next(),
            room_number: intermediate_rep.room_number.into_iter().next(),
            post_number: intermediate_rep.post_number.into_iter().next(),
            self_introduction_slide_url: intermediate_rep.self_introduction_slide_url.into_iter().next(),
            programs: intermediate_rep.programs.into_iter().next(),
            houses: intermediate_rep.houses.into_iter().next(),
            committees: intermediate_rep.committees.into_iter().next(),
            clubs: intermediate_rep.clubs.into_iter().next(),
            position: intermediate_rep.position.into_iter().next(),
            bio: intermediate_rep.bio.into_iter().next(),
            photo_urls: intermediate_rep.photo_urls.into_iter().next(),
            last_modified_at: intermediate_rep.last_modified_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<User> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<User>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<User>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for User - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<User> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <User as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into User - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserGroupBelonging {
    #[serde(rename = "group")]
    pub group: models::Group,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<chrono::DateTime::<chrono::Utc>>,

}

impl UserGroupBelonging {
    #[allow(clippy::new_without_default)]
    pub fn new(group: models::Group, display_name: String, ) -> UserGroupBelonging {
        UserGroupBelonging {
            group,
            display_name,
            from: None,
            to: None,
        }
    }
}

/// Converts the UserGroupBelonging value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UserGroupBelonging {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping group in query parameter serialization


            Some("displayName".to_string()),
            Some(self.display_name.to_string()),

            // Skipping from in query parameter serialization

            // Skipping to in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UserGroupBelonging value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UserGroupBelonging {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub group: Vec<models::Group>,
            pub display_name: Vec<String>,
            pub from: Vec<chrono::DateTime::<chrono::Utc>>,
            pub to: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UserGroupBelonging".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "group" => intermediate_rep.group.push(<models::Group as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "displayName" => intermediate_rep.display_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "from" => intermediate_rep.from.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "to" => intermediate_rep.to.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UserGroupBelonging".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UserGroupBelonging {
            group: intermediate_rep.group.into_iter().next().ok_or_else(|| "group missing in UserGroupBelonging".to_string())?,
            display_name: intermediate_rep.display_name.into_iter().next().ok_or_else(|| "displayName missing in UserGroupBelonging".to_string())?,
            from: intermediate_rep.from.into_iter().next(),
            to: intermediate_rep.to.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UserGroupBelonging> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UserGroupBelonging>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UserGroupBelonging>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UserGroupBelonging - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UserGroupBelonging> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UserGroupBelonging as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UserGroupBelonging - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserI18n {
    #[serde(rename = "locale")]
    pub locale: models::Locale,

    #[serde(rename = "firstName")]
    pub first_name: String,

    #[serde(rename = "lastName")]
    pub last_name: String,

    #[serde(rename = "phoneticFirstName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phonetic_first_name: Option<String>,

    #[serde(rename = "phoneticLastName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phonetic_last_name: Option<String>,

    #[serde(rename = "nickName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub nick_name: Option<String>,

    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub middle_name: Option<String>,

    #[serde(rename = "phoneticMiddleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phonetic_middle_name: Option<String>,

    #[serde(rename = "maidenName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maiden_name: Option<String>,

    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prefix: Option<String>,

    #[serde(rename = "suffix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suffix: Option<String>,

}

impl UserI18n {
    #[allow(clippy::new_without_default)]
    pub fn new(locale: models::Locale, first_name: String, last_name: String, ) -> UserI18n {
        UserI18n {
            locale,
            first_name,
            last_name,
            phonetic_first_name: None,
            phonetic_last_name: None,
            nick_name: None,
            middle_name: None,
            phonetic_middle_name: None,
            maiden_name: None,
            prefix: None,
            suffix: None,
        }
    }
}

/// Converts the UserI18n value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UserI18n {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping locale in query parameter serialization


            Some("firstName".to_string()),
            Some(self.first_name.to_string()),


            Some("lastName".to_string()),
            Some(self.last_name.to_string()),


            self.phonetic_first_name.as_ref().map(|phonetic_first_name| {
                vec![
                    "phoneticFirstName".to_string(),
                    phonetic_first_name.to_string(),
                ].join(",")
            }),


            self.phonetic_last_name.as_ref().map(|phonetic_last_name| {
                vec![
                    "phoneticLastName".to_string(),
                    phonetic_last_name.to_string(),
                ].join(",")
            }),


            self.nick_name.as_ref().map(|nick_name| {
                vec![
                    "nickName".to_string(),
                    nick_name.to_string(),
                ].join(",")
            }),


            self.middle_name.as_ref().map(|middle_name| {
                vec![
                    "middleName".to_string(),
                    middle_name.to_string(),
                ].join(",")
            }),


            self.phonetic_middle_name.as_ref().map(|phonetic_middle_name| {
                vec![
                    "phoneticMiddleName".to_string(),
                    phonetic_middle_name.to_string(),
                ].join(",")
            }),


            self.maiden_name.as_ref().map(|maiden_name| {
                vec![
                    "maidenName".to_string(),
                    maiden_name.to_string(),
                ].join(",")
            }),


            self.prefix.as_ref().map(|prefix| {
                vec![
                    "prefix".to_string(),
                    prefix.to_string(),
                ].join(",")
            }),


            self.suffix.as_ref().map(|suffix| {
                vec![
                    "suffix".to_string(),
                    suffix.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UserI18n value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UserI18n {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub locale: Vec<models::Locale>,
            pub first_name: Vec<String>,
            pub last_name: Vec<String>,
            pub phonetic_first_name: Vec<String>,
            pub phonetic_last_name: Vec<String>,
            pub nick_name: Vec<String>,
            pub middle_name: Vec<String>,
            pub phonetic_middle_name: Vec<String>,
            pub maiden_name: Vec<String>,
            pub prefix: Vec<String>,
            pub suffix: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UserI18n".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "locale" => intermediate_rep.locale.push(<models::Locale as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "firstName" => intermediate_rep.first_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastName" => intermediate_rep.last_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phoneticFirstName" => intermediate_rep.phonetic_first_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phoneticLastName" => intermediate_rep.phonetic_last_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nickName" => intermediate_rep.nick_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "middleName" => intermediate_rep.middle_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phoneticMiddleName" => intermediate_rep.phonetic_middle_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maidenName" => intermediate_rep.maiden_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "prefix" => intermediate_rep.prefix.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "suffix" => intermediate_rep.suffix.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UserI18n".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UserI18n {
            locale: intermediate_rep.locale.into_iter().next().ok_or_else(|| "locale missing in UserI18n".to_string())?,
            first_name: intermediate_rep.first_name.into_iter().next().ok_or_else(|| "firstName missing in UserI18n".to_string())?,
            last_name: intermediate_rep.last_name.into_iter().next().ok_or_else(|| "lastName missing in UserI18n".to_string())?,
            phonetic_first_name: intermediate_rep.phonetic_first_name.into_iter().next(),
            phonetic_last_name: intermediate_rep.phonetic_last_name.into_iter().next(),
            nick_name: intermediate_rep.nick_name.into_iter().next(),
            middle_name: intermediate_rep.middle_name.into_iter().next(),
            phonetic_middle_name: intermediate_rep.phonetic_middle_name.into_iter().next(),
            maiden_name: intermediate_rep.maiden_name.into_iter().next(),
            prefix: intermediate_rep.prefix.into_iter().next(),
            suffix: intermediate_rep.suffix.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UserI18n> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UserI18n>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UserI18n>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UserI18n - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UserI18n> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UserI18n as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UserI18n - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserSocialLink {
    #[serde(rename = "brand")]
    pub brand: models::SocialBrand,

    #[serde(rename = "username")]
    pub username: String,

}

impl UserSocialLink {
    #[allow(clippy::new_without_default)]
    pub fn new(brand: models::SocialBrand, username: String, ) -> UserSocialLink {
        UserSocialLink {
            brand,
            username,
        }
    }
}

/// Converts the UserSocialLink value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for UserSocialLink {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping brand in query parameter serialization


            Some("username".to_string()),
            Some(self.username.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UserSocialLink value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UserSocialLink {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub brand: Vec<models::SocialBrand>,
            pub username: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UserSocialLink".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "brand" => intermediate_rep.brand.push(<models::SocialBrand as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "username" => intermediate_rep.username.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UserSocialLink".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UserSocialLink {
            brand: intermediate_rep.brand.into_iter().next().ok_or_else(|| "brand missing in UserSocialLink".to_string())?,
            username: intermediate_rep.username.into_iter().next().ok_or_else(|| "username missing in UserSocialLink".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UserSocialLink> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<UserSocialLink>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UserSocialLink>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for UserSocialLink - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<UserSocialLink> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UserSocialLink as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into UserSocialLink - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


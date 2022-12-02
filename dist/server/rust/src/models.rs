#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

/// `Program`, `House/Home`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Generation {
    /// Airtable's Record ID
    #[serde(rename = "id")]
    pub id: serde_json::Value,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: serde_json::Value,

    #[serde(rename = "displayName")]
    pub display_name: serde_json::Value,

    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<serde_json::Value>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<serde_json::Value>,

    #[serde(rename = "coverImageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cover_image_url: Option<serde_json::Value>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<serde_json::Value>,

    /// Markdown
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<serde_json::Value>,

    #[serde(rename = "photoUrls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_urls: Option<serde_json::Value>,

}

impl Generation {
    #[allow(clippy::new_without_default)]
    pub fn new(id: serde_json::Value, r#type: serde_json::Value, display_name: serde_json::Value, ) -> Generation {
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
            // Skipping id in query parameter serialization

            // Skipping type in query parameter serialization

            // Skipping displayName in query parameter serialization

            // Skipping from in query parameter serialization

            // Skipping to in query parameter serialization

            // Skipping coverImageUrl in query parameter serialization

            // Skipping title in query parameter serialization

            // Skipping description in query parameter serialization

            // Skipping photoUrls in query parameter serialization

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
            pub id: Vec<serde_json::Value>,
            pub r#type: Vec<serde_json::Value>,
            pub display_name: Vec<serde_json::Value>,
            pub from: Vec<serde_json::Value>,
            pub to: Vec<serde_json::Value>,
            pub cover_image_url: Vec<serde_json::Value>,
            pub title: Vec<serde_json::Value>,
            pub description: Vec<serde_json::Value>,
            pub photo_urls: Vec<serde_json::Value>,
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
                    "id" => intermediate_rep.id.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "displayName" => intermediate_rep.display_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "from" => intermediate_rep.from.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "to" => intermediate_rep.to.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "coverImageUrl" => intermediate_rep.cover_image_url.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "photoUrls" => intermediate_rep.photo_urls.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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


/// `Committee`, `Club`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Group {
    /// Airtable's Record ID
    #[serde(rename = "id")]
    pub id: serde_json::Value,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: serde_json::Value,

    #[serde(rename = "displayName")]
    pub display_name: serde_json::Value,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<serde_json::Value>,

    #[serde(rename = "archivedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived_at: Option<serde_json::Value>,

    #[serde(rename = "emoji")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emoji: Option<serde_json::Value>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<serde_json::Value>,

    /// Markdown
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<serde_json::Value>,

    #[serde(rename = "slackChannel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slack_channel: Option<serde_json::Value>,

    #[serde(rename = "photoUrls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_urls: Option<serde_json::Value>,

}

impl Group {
    #[allow(clippy::new_without_default)]
    pub fn new(id: serde_json::Value, r#type: serde_json::Value, display_name: serde_json::Value, ) -> Group {
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
            // Skipping id in query parameter serialization

            // Skipping type in query parameter serialization

            // Skipping displayName in query parameter serialization

            // Skipping createdAt in query parameter serialization

            // Skipping archivedAt in query parameter serialization

            // Skipping emoji in query parameter serialization

            // Skipping title in query parameter serialization

            // Skipping description in query parameter serialization

            // Skipping slackChannel in query parameter serialization

            // Skipping photoUrls in query parameter serialization

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
            pub id: Vec<serde_json::Value>,
            pub r#type: Vec<serde_json::Value>,
            pub display_name: Vec<serde_json::Value>,
            pub created_at: Vec<serde_json::Value>,
            pub archived_at: Vec<serde_json::Value>,
            pub emoji: Vec<serde_json::Value>,
            pub title: Vec<serde_json::Value>,
            pub description: Vec<serde_json::Value>,
            pub slack_channel: Vec<serde_json::Value>,
            pub photo_urls: Vec<serde_json::Value>,
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
                    "id" => intermediate_rep.id.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "displayName" => intermediate_rep.display_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "archivedAt" => intermediate_rep.archived_at.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "emoji" => intermediate_rep.emoji.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "slackChannel" => intermediate_rep.slack_channel.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "photoUrls" => intermediate_rep.photo_urls.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PostLogin200Response {
    #[serde(rename = "accessToken")]
    pub access_token: serde_json::Value,

    /// 初期登録フォーム未回答の場合のみ 
    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<models::User>,

}

impl PostLogin200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(access_token: serde_json::Value, ) -> PostLogin200Response {
        PostLogin200Response {
            access_token,
            user: None,
        }
    }
}

/// Converts the PostLogin200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PostLogin200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping accessToken in query parameter serialization

            // Skipping user in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostLogin200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostLogin200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub access_token: Vec<serde_json::Value>,
            pub user: Vec<models::User>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostLogin200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "accessToken" => intermediate_rep.access_token.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "user" => intermediate_rep.user.push(<models::User as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostLogin200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostLogin200Response {
            access_token: intermediate_rep.access_token.into_iter().next().ok_or_else(|| "accessToken missing in PostLogin200Response".to_string())?,
            user: intermediate_rep.user.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostLogin200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PostLogin200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostLogin200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PostLogin200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PostLogin200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostLogin200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PostLogin200Response - {}",
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
pub struct PostLoginRequest {
    /// 取得方法は[こちら](https://firebase.google.com/docs/auth/admin/verify-id-tokens#retrieve_id_tokens_on_clients) 
    #[serde(rename = "firebaseIdToken")]
    pub firebase_id_token: serde_json::Value,

}

impl PostLoginRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(firebase_id_token: serde_json::Value, ) -> PostLoginRequest {
        PostLoginRequest {
            firebase_id_token,
        }
    }
}

/// Converts the PostLoginRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PostLoginRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping firebaseIdToken in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PostLoginRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PostLoginRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub firebase_id_token: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PostLoginRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "firebaseIdToken" => intermediate_rep.firebase_id_token.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PostLoginRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PostLoginRequest {
            firebase_id_token: intermediate_rep.firebase_id_token.into_iter().next().ok_or_else(|| "firebaseIdToken missing in PostLoginRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PostLoginRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PostLoginRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PostLoginRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PostLoginRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PostLoginRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PostLoginRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PostLoginRequest - {}",
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
pub struct PutUsersRequest {
    #[serde(rename = "i18n")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub i18n: Option<serde_json::Value>,

    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<serde_json::Value>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "pronoun")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pronoun: Option<serde_json::Value>,

    #[serde(rename = "birthday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub birthday: Option<serde_json::Value>,

    #[serde(rename = "hideAge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hide_age: Option<serde_json::Value>,

    #[serde(rename = "socialLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub social_links: Option<serde_json::Value>,

    #[serde(rename = "roomNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub room_number: Option<serde_json::Value>,

    #[serde(rename = "postNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub post_number: Option<serde_json::Value>,

    #[serde(rename = "selfIntroductionSlideUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub self_introduction_slide_url: Option<serde_json::Value>,

    #[serde(rename = "programs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub programs: Option<serde_json::Value>,

    #[serde(rename = "houses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub houses: Option<serde_json::Value>,

    #[serde(rename = "committees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committees: Option<serde_json::Value>,

    #[serde(rename = "clubs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clubs: Option<serde_json::Value>,

    /// 肩書き
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<serde_json::Value>,

    /// Markdown
    #[serde(rename = "bio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<serde_json::Value>,

    #[serde(rename = "photoUrls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_urls: Option<serde_json::Value>,

}

impl PutUsersRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PutUsersRequest {
        PutUsersRequest {
            i18n: None,
            image_url: None,
            pronoun: None,
            birthday: None,
            hide_age: None,
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
        }
    }
}

/// Converts the PutUsersRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PutUsersRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping i18n in query parameter serialization

            // Skipping imageUrl in query parameter serialization

            // Skipping pronoun in query parameter serialization

            // Skipping birthday in query parameter serialization

            // Skipping hideAge in query parameter serialization

            // Skipping socialLinks in query parameter serialization

            // Skipping roomNumber in query parameter serialization

            // Skipping postNumber in query parameter serialization

            // Skipping selfIntroductionSlideUrl in query parameter serialization

            // Skipping programs in query parameter serialization

            // Skipping houses in query parameter serialization

            // Skipping committees in query parameter serialization

            // Skipping clubs in query parameter serialization

            // Skipping position in query parameter serialization

            // Skipping bio in query parameter serialization

            // Skipping photoUrls in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PutUsersRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PutUsersRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub i18n: Vec<serde_json::Value>,
            pub image_url: Vec<serde_json::Value>,
            pub pronoun: Vec<serde_json::Value>,
            pub birthday: Vec<serde_json::Value>,
            pub hide_age: Vec<serde_json::Value>,
            pub social_links: Vec<serde_json::Value>,
            pub room_number: Vec<serde_json::Value>,
            pub post_number: Vec<serde_json::Value>,
            pub self_introduction_slide_url: Vec<serde_json::Value>,
            pub programs: Vec<serde_json::Value>,
            pub houses: Vec<serde_json::Value>,
            pub committees: Vec<serde_json::Value>,
            pub clubs: Vec<serde_json::Value>,
            pub position: Vec<serde_json::Value>,
            pub bio: Vec<serde_json::Value>,
            pub photo_urls: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PutUsersRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "i18n" => intermediate_rep.i18n.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "imageUrl" => intermediate_rep.image_url.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pronoun" => intermediate_rep.pronoun.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "birthday" => intermediate_rep.birthday.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hideAge" => intermediate_rep.hide_age.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "socialLinks" => intermediate_rep.social_links.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "roomNumber" => intermediate_rep.room_number.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "postNumber" => intermediate_rep.post_number.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "selfIntroductionSlideUrl" => intermediate_rep.self_introduction_slide_url.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "programs" => intermediate_rep.programs.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "houses" => intermediate_rep.houses.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "committees" => intermediate_rep.committees.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "clubs" => intermediate_rep.clubs.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "position" => intermediate_rep.position.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "bio" => intermediate_rep.bio.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "photoUrls" => intermediate_rep.photo_urls.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PutUsersRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PutUsersRequest {
            i18n: intermediate_rep.i18n.into_iter().next(),
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
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PutUsersRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PutUsersRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PutUsersRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PutUsersRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PutUsersRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PutUsersRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PutUsersRequest - {}",
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
pub struct SocialBrand {
    /// Airtable's Record ID
    #[serde(rename = "id")]
    pub id: serde_json::Value,

    #[serde(rename = "name")]
    pub name: serde_json::Value,

    #[serde(rename = "svgIconUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svg_icon_url: Option<serde_json::Value>,

    #[serde(rename = "urlPrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url_prefix: Option<serde_json::Value>,

    #[serde(rename = "formPrefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub form_prefix: Option<serde_json::Value>,

}

impl SocialBrand {
    #[allow(clippy::new_without_default)]
    pub fn new(id: serde_json::Value, name: serde_json::Value, ) -> SocialBrand {
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
            // Skipping id in query parameter serialization

            // Skipping name in query parameter serialization

            // Skipping svgIconUrl in query parameter serialization

            // Skipping urlPrefix in query parameter serialization

            // Skipping formPrefix in query parameter serialization

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
            pub id: Vec<serde_json::Value>,
            pub name: Vec<serde_json::Value>,
            pub svg_icon_url: Vec<serde_json::Value>,
            pub url_prefix: Vec<serde_json::Value>,
            pub form_prefix: Vec<serde_json::Value>,
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
                    "id" => intermediate_rep.id.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "svgIconUrl" => intermediate_rep.svg_icon_url.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "urlPrefix" => intermediate_rep.url_prefix.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "formPrefix" => intermediate_rep.form_prefix.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct User {
    /// Airtable's Record ID
    #[serde(rename = "id")]
    pub id: serde_json::Value,

    #[serde(rename = "email")]
    pub email: serde_json::Value,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "role")]
    pub role: serde_json::Value,

    #[serde(rename = "i18n")]
    pub i18n: serde_json::Value,

    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<serde_json::Value>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "pronoun")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pronoun: Option<serde_json::Value>,

    #[serde(rename = "birthday")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub birthday: Option<serde_json::Value>,

    #[serde(rename = "hideAge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hide_age: Option<serde_json::Value>,

    #[serde(rename = "socialLinks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub social_links: Option<serde_json::Value>,

    #[serde(rename = "roomNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub room_number: Option<serde_json::Value>,

    #[serde(rename = "postNumber")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub post_number: Option<serde_json::Value>,

    #[serde(rename = "selfIntroductionSlideUrl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub self_introduction_slide_url: Option<serde_json::Value>,

    #[serde(rename = "programs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub programs: Option<serde_json::Value>,

    #[serde(rename = "houses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub houses: Option<serde_json::Value>,

    #[serde(rename = "committees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committees: Option<serde_json::Value>,

    #[serde(rename = "clubs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clubs: Option<serde_json::Value>,

    /// 肩書き
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<serde_json::Value>,

    /// Markdown
    #[serde(rename = "bio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<serde_json::Value>,

    #[serde(rename = "photoUrls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo_urls: Option<serde_json::Value>,

    #[serde(rename = "lastModifiedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_at: Option<serde_json::Value>,

}

impl User {
    #[allow(clippy::new_without_default)]
    pub fn new(id: serde_json::Value, email: serde_json::Value, role: serde_json::Value, i18n: serde_json::Value, ) -> User {
        User {
            id,
            email,
            role,
            i18n,
            image_url: None,
            pronoun: None,
            birthday: None,
            hide_age: None,
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
            // Skipping id in query parameter serialization

            // Skipping email in query parameter serialization

            // Skipping role in query parameter serialization

            // Skipping i18n in query parameter serialization

            // Skipping imageUrl in query parameter serialization

            // Skipping pronoun in query parameter serialization

            // Skipping birthday in query parameter serialization

            // Skipping hideAge in query parameter serialization

            // Skipping socialLinks in query parameter serialization

            // Skipping roomNumber in query parameter serialization

            // Skipping postNumber in query parameter serialization

            // Skipping selfIntroductionSlideUrl in query parameter serialization

            // Skipping programs in query parameter serialization

            // Skipping houses in query parameter serialization

            // Skipping committees in query parameter serialization

            // Skipping clubs in query parameter serialization

            // Skipping position in query parameter serialization

            // Skipping bio in query parameter serialization

            // Skipping photoUrls in query parameter serialization

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
            pub id: Vec<serde_json::Value>,
            pub email: Vec<serde_json::Value>,
            pub role: Vec<serde_json::Value>,
            pub i18n: Vec<serde_json::Value>,
            pub image_url: Vec<serde_json::Value>,
            pub pronoun: Vec<serde_json::Value>,
            pub birthday: Vec<serde_json::Value>,
            pub hide_age: Vec<serde_json::Value>,
            pub social_links: Vec<serde_json::Value>,
            pub room_number: Vec<serde_json::Value>,
            pub post_number: Vec<serde_json::Value>,
            pub self_introduction_slide_url: Vec<serde_json::Value>,
            pub programs: Vec<serde_json::Value>,
            pub houses: Vec<serde_json::Value>,
            pub committees: Vec<serde_json::Value>,
            pub clubs: Vec<serde_json::Value>,
            pub position: Vec<serde_json::Value>,
            pub bio: Vec<serde_json::Value>,
            pub photo_urls: Vec<serde_json::Value>,
            pub last_modified_at: Vec<serde_json::Value>,
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
                    "id" => intermediate_rep.id.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "email" => intermediate_rep.email.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "role" => intermediate_rep.role.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "i18n" => intermediate_rep.i18n.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "imageUrl" => intermediate_rep.image_url.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pronoun" => intermediate_rep.pronoun.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "birthday" => intermediate_rep.birthday.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hideAge" => intermediate_rep.hide_age.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "socialLinks" => intermediate_rep.social_links.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "roomNumber" => intermediate_rep.room_number.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "postNumber" => intermediate_rep.post_number.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "selfIntroductionSlideUrl" => intermediate_rep.self_introduction_slide_url.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "programs" => intermediate_rep.programs.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "houses" => intermediate_rep.houses.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "committees" => intermediate_rep.committees.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "clubs" => intermediate_rep.clubs.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "position" => intermediate_rep.position.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "bio" => intermediate_rep.bio.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "photoUrls" => intermediate_rep.photo_urls.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastModifiedAt" => intermediate_rep.last_modified_at.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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
    pub display_name: serde_json::Value,

    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<serde_json::Value>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<serde_json::Value>,

}

impl UserGroupBelonging {
    #[allow(clippy::new_without_default)]
    pub fn new(group: models::Group, display_name: serde_json::Value, ) -> UserGroupBelonging {
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

            // Skipping displayName in query parameter serialization

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
            pub display_name: Vec<serde_json::Value>,
            pub from: Vec<serde_json::Value>,
            pub to: Vec<serde_json::Value>,
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
                    "displayName" => intermediate_rep.display_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "from" => intermediate_rep.from.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "to" => intermediate_rep.to.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "locale")]
    pub locale: serde_json::Value,

    #[serde(rename = "firstName")]
    pub first_name: serde_json::Value,

    #[serde(rename = "lastName")]
    pub last_name: serde_json::Value,

    #[serde(rename = "phoneticFirstName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phonetic_first_name: Option<serde_json::Value>,

    #[serde(rename = "phoneticLastName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phonetic_last_name: Option<serde_json::Value>,

    #[serde(rename = "nickName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub nick_name: Option<serde_json::Value>,

    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub middle_name: Option<serde_json::Value>,

    #[serde(rename = "phoneticMiddleName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub phonetic_middle_name: Option<serde_json::Value>,

    #[serde(rename = "maidenName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maiden_name: Option<serde_json::Value>,

    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prefix: Option<serde_json::Value>,

    #[serde(rename = "suffix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suffix: Option<serde_json::Value>,

}

impl UserI18n {
    #[allow(clippy::new_without_default)]
    pub fn new(locale: serde_json::Value, first_name: serde_json::Value, last_name: serde_json::Value, ) -> UserI18n {
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

            // Skipping firstName in query parameter serialization

            // Skipping lastName in query parameter serialization

            // Skipping phoneticFirstName in query parameter serialization

            // Skipping phoneticLastName in query parameter serialization

            // Skipping nickName in query parameter serialization

            // Skipping middleName in query parameter serialization

            // Skipping phoneticMiddleName in query parameter serialization

            // Skipping maidenName in query parameter serialization

            // Skipping prefix in query parameter serialization

            // Skipping suffix in query parameter serialization

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
            pub locale: Vec<serde_json::Value>,
            pub first_name: Vec<serde_json::Value>,
            pub last_name: Vec<serde_json::Value>,
            pub phonetic_first_name: Vec<serde_json::Value>,
            pub phonetic_last_name: Vec<serde_json::Value>,
            pub nick_name: Vec<serde_json::Value>,
            pub middle_name: Vec<serde_json::Value>,
            pub phonetic_middle_name: Vec<serde_json::Value>,
            pub maiden_name: Vec<serde_json::Value>,
            pub prefix: Vec<serde_json::Value>,
            pub suffix: Vec<serde_json::Value>,
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
                    "locale" => intermediate_rep.locale.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "firstName" => intermediate_rep.first_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastName" => intermediate_rep.last_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phoneticFirstName" => intermediate_rep.phonetic_first_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phoneticLastName" => intermediate_rep.phonetic_last_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "nickName" => intermediate_rep.nick_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "middleName" => intermediate_rep.middle_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "phoneticMiddleName" => intermediate_rep.phonetic_middle_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maidenName" => intermediate_rep.maiden_name.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "prefix" => intermediate_rep.prefix.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "suffix" => intermediate_rep.suffix.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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
    pub username: serde_json::Value,

}

impl UserSocialLink {
    #[allow(clippy::new_without_default)]
    pub fn new(brand: models::SocialBrand, username: serde_json::Value, ) -> UserSocialLink {
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

            // Skipping username in query parameter serialization

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
            pub username: Vec<serde_json::Value>,
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
                    "username" => intermediate_rep.username.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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


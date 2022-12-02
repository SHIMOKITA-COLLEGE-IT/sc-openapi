# User

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Airtable's Record ID | 
**email** | **String** |  | 
**role** | [***models::Role**](Role.md) |  | 
**i18n** | [**Vec<models::UserI18n>**](UserI18n.md) |  | 
**image_url** | **String** |  | [optional] [default to None]
**pronoun** | [***models::Pronoun**](Pronoun.md) |  | [optional] [default to None]
**birthday** | [***chrono::DateTime::<chrono::Utc>**](date.md) |  | [optional] [default to None]
**hide_age** | **bool** |  | [optional] [default to Some(false)]
**social_links** | [**Vec<models::UserSocialLink>**](UserSocialLink.md) |  | [optional] [default to None]
**room_number** | **f64** |  | [optional] [default to None]
**post_number** | **f64** |  | [optional] [default to None]
**self_introduction_slide_url** | **String** |  | [optional] [default to None]
**programs** | [**Vec<models::Generation>**](Generation.md) |  | [optional] [default to None]
**houses** | [**Vec<models::Generation>**](Generation.md) |  | [optional] [default to None]
**committees** | [**Vec<models::UserGroupBelonging>**](UserGroupBelonging.md) |  | [optional] [default to None]
**clubs** | [**Vec<models::UserGroupBelonging>**](UserGroupBelonging.md) |  | [optional] [default to None]
**position** | **String** | 肩書き | [optional] [default to None]
**bio** | **String** | Markdown | [optional] [default to None]
**photo_urls** | **Vec<String>** |  | [optional] [default to None]
**last_modified_at** | [***chrono::DateTime::<chrono::Utc>**](date.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



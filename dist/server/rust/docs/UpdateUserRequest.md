# UpdateUserRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**i18n** | [**Vec<models::UpdateUserRequestI18nInner>**](UpdateUserRequest_i18n_inner.md) |  | [optional] [default to None]
**image_url** | **String** |  | [optional] [default to None]
**pronoun** | [***models::Pronoun**](Pronoun.md) |  | [optional] [default to None]
**birthday** | [***chrono::DateTime::<chrono::Utc>**](date.md) |  | [optional] [default to None]
**hide_age** | **bool** |  | [optional] [default to Some(false)]
**social_links** | [**Vec<models::UpdateUserRequestSocialLinksInner>**](UpdateUserRequest_socialLinks_inner.md) |  | [optional] [default to None]
**room_number** | **f64** |  | [optional] [default to None]
**post_number** | **f64** |  | [optional] [default to None]
**self_introduction_slide_url** | **String** |  | [optional] [default to None]
**generations** | **Vec<String>** | Airtable's Record IDs | [optional] [default to None]
**groups** | [**Vec<models::UpdateUserRequestGroupsInner>**](UpdateUserRequest_groups_inner.md) |  | [optional] [default to None]
**position** | **String** | 肩書き | [optional] [default to None]
**bio** | **String** | Markdown | [optional] [default to None]
**photo_urls** | **Vec<String>** |  | [optional] [default to None]
**last_modified_at** | [***chrono::DateTime::<chrono::Utc>**](date.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



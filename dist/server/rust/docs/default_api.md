# default_api

All URIs are relative to *https://stoplight.io/mocks/college-app/college-app/406762*

Method | HTTP request | Description
------------- | ------------- | -------------
**get-generations**](default_api.md#get-generations) | **GET** /generations | Get Generations
**get-generations-recordId**](default_api.md#get-generations-recordId) | **GET** /generations/{recordId} | Get Generation
**get-groups**](default_api.md#get-groups) | **GET** /groups | Get Groups
**get-groups-recordId**](default_api.md#get-groups-recordId) | **GET** /groups/{recordId} | Get Group
**get-users**](default_api.md#get-users) | **GET** /users | Get Users
**get-users-recordId**](default_api.md#get-users-recordId) | **GET** /users/{recordId} | Get User
**get-users-search**](default_api.md#get-users-search) | **GET** /users/search | Search Users (WIP)
**post-login**](default_api.md#post-login) | **POST** /login | Login
**put-users**](default_api.md#put-users) | **PUT** /users | Update User


# **get-generations**
> serde_json::Value get-generations(ctx, r#type)
Get Generations

指定したtypeの`Generation`を全て取得する

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **r#type** | [****](.md)|  | 

### Return type

[**serde_json::Value**](AnyType.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-generations-recordId**
> models::Generation get-generations-recordId(ctx, record_id)
Get Generation

指定のgenerationを取得

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **record_id** | [****](.md)|  | 

### Return type

[**models::Generation**](Generation.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-groups**
> serde_json::Value get-groups(ctx, optional)
Get Groups

指定したtypeの`Group`を全て取得する

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **r#type** | [****](.md)|  | 

### Return type

[**serde_json::Value**](AnyType.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-groups-recordId**
> models::Group get-groups-recordId(ctx, record_id)
Get Group

指定のGroupを取得

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **record_id** | [****](.md)|  | 

### Return type

[**models::Group**](Group.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-users**
> serde_json::Value get-users(ctx, )
Get Users

全てのユーザーを取得する

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](AnyType.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-users-recordId**
> models::User get-users-recordId(ctx, record_id)
Get User

指定のuserを取得

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **record_id** | [****](.md)|  | 

### Return type

[**models::User**](User.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get-users-search**
> serde_json::Value get-users-search(ctx, query)
Search Users (WIP)

## TODO  - [ ] フィルタリング - [ ] ページネーション

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | [****](.md)| 検索ワード | 

### Return type

[**serde_json::Value**](AnyType.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post-login**
> models::PostLogin200Response post-login(optional)
Login

## 概要 accessTokenを取得するためのエンドポイント。  初期登録フォーム未回答の場合、User情報を取得する。  ## レスポンスによるクライアント側の分岐  - 200   - レスポンスにuserがある     - 初期登録フォームに遷移、default値にuserを使用   - レスポンスにuserがない     - Homeに遷移 - 401   - 「このエラーが出たら運営に報告してね」エラーページに遷移 - 403   - 「このアカウントは無効化されてるよ」エラーページに遷移 - 404   - 「まだ運営側でデータ登録が完了していないよ」エラーページに遷移 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **post_login_request** | [**PostLoginRequest**](PostLoginRequest.md)|  | 

### Return type

[**models::PostLogin200Response**](post_login_200_response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put-users**
> put-users(ctx, optional)
Update User

Airtable上のUser情報をアップデートする

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **put_users_request** | [**PutUsersRequest**](PutUsersRequest.md)|  | 

### Return type

 (empty response body)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


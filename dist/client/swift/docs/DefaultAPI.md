# DefaultAPI

All URIs are relative to *https://stoplight.io/mocks/college-app/college-app/406762*

Method | HTTP request | Description
------------- | ------------- | -------------
[**getGenerations**](DefaultAPI.md#getgenerations) | **GET** /generations | Get Generations
[**getGenerationsRecordId**](DefaultAPI.md#getgenerationsrecordid) | **GET** /generations/{recordId} | Get Generation
[**getGroups**](DefaultAPI.md#getgroups) | **GET** /groups | Get Groups
[**getGroupsRecordId**](DefaultAPI.md#getgroupsrecordid) | **GET** /groups/{recordId} | Get Group
[**getUsers**](DefaultAPI.md#getusers) | **GET** /users | Get Users
[**getUsersRecordId**](DefaultAPI.md#getusersrecordid) | **GET** /users/{recordId} | Get User
[**getUsersSearch**](DefaultAPI.md#getuserssearch) | **GET** /users/search | Search Users (WIP)
[**postLogin**](DefaultAPI.md#postlogin) | **POST** /login | Login
[**putUsers**](DefaultAPI.md#putusers) | **PUT** /users | Update User


# **getGenerations**
```swift
    open class func getGenerations(type: ModelType_getGenerations, completion: @escaping (_ data: AnyCodable?, _ error: Error?) -> Void)
```

Get Generations

指定したtypeの`Generation`を全て取得する

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let type = TODO // AnyCodable | 

// Get Generations
DefaultAPI.getGenerations(type: type) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **type** | [**AnyCodable**](.md) |  | 

### Return type

[**AnyCodable**](AnyCodable.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getGenerationsRecordId**
```swift
    open class func getGenerationsRecordId(recordId: AnyCodable, completion: @escaping (_ data: Generation?, _ error: Error?) -> Void)
```

Get Generation

指定のgenerationを取得

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let recordId = TODO // AnyCodable | 

// Get Generation
DefaultAPI.getGenerationsRecordId(recordId: recordId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **recordId** | [**AnyCodable**](.md) |  | 

### Return type

[**Generation**](Generation.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getGroups**
```swift
    open class func getGroups(type: ModelType_getGroups? = nil, completion: @escaping (_ data: AnyCodable?, _ error: Error?) -> Void)
```

Get Groups

指定したtypeの`Group`を全て取得する

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let type = TODO // AnyCodable |  (optional)

// Get Groups
DefaultAPI.getGroups(type: type) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **type** | [**AnyCodable**](.md) |  | [optional] 

### Return type

[**AnyCodable**](AnyCodable.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getGroupsRecordId**
```swift
    open class func getGroupsRecordId(recordId: AnyCodable, completion: @escaping (_ data: Group?, _ error: Error?) -> Void)
```

Get Group

指定のGroupを取得

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let recordId = TODO // AnyCodable | 

// Get Group
DefaultAPI.getGroupsRecordId(recordId: recordId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **recordId** | [**AnyCodable**](.md) |  | 

### Return type

[**Group**](Group.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUsers**
```swift
    open class func getUsers(completion: @escaping (_ data: AnyCodable?, _ error: Error?) -> Void)
```

Get Users

全てのユーザーを取得する

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient


// Get Users
DefaultAPI.getUsers() { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**AnyCodable**](AnyCodable.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUsersRecordId**
```swift
    open class func getUsersRecordId(recordId: AnyCodable, completion: @escaping (_ data: User?, _ error: Error?) -> Void)
```

Get User

指定のuserを取得

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let recordId = TODO // AnyCodable | 

// Get User
DefaultAPI.getUsersRecordId(recordId: recordId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **recordId** | [**AnyCodable**](.md) |  | 

### Return type

[**User**](User.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUsersSearch**
```swift
    open class func getUsersSearch(query: AnyCodable, completion: @escaping (_ data: AnyCodable?, _ error: Error?) -> Void)
```

Search Users (WIP)

## TODO  - [ ] フィルタリング - [ ] ページネーション

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let query = TODO // AnyCodable | 検索ワード

// Search Users (WIP)
DefaultAPI.getUsersSearch(query: query) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | [**AnyCodable**](.md) | 検索ワード | 

### Return type

[**AnyCodable**](AnyCodable.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postLogin**
```swift
    open class func postLogin(postLoginRequest: PostLoginRequest? = nil, completion: @escaping (_ data: PostLogin200Response?, _ error: Error?) -> Void)
```

Login

## 概要 accessTokenを取得するためのエンドポイント。  初期登録フォーム未回答の場合、User情報を取得する。  ## レスポンスによるクライアント側の分岐  - 200   - レスポンスにuserがある     - 初期登録フォームに遷移、default値にuserを使用   - レスポンスにuserがない     - Homeに遷移 - 401   - 「このエラーが出たら運営に報告してね」エラーページに遷移 - 403   - 「このアカウントは無効化されてるよ」エラーページに遷移 - 404   - 「まだ運営側でデータ登録が完了していないよ」エラーページに遷移 

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let postLoginRequest = post_login_request(firebaseIdToken: "TODO") // PostLoginRequest |  (optional)

// Login
DefaultAPI.postLogin(postLoginRequest: postLoginRequest) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **postLoginRequest** | [**PostLoginRequest**](PostLoginRequest.md) |  | [optional] 

### Return type

[**PostLogin200Response**](PostLogin200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putUsers**
```swift
    open class func putUsers(putUsersRequest: PutUsersRequest? = nil, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```

Update User

Airtable上のUser情報をアップデートする

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let putUsersRequest = put_users_request(i18n: "TODO", imageUrl: "TODO", pronoun: "TODO", birthday: "TODO", hideAge: "TODO", socialLinks: "TODO", roomNumber: "TODO", postNumber: "TODO", selfIntroductionSlideUrl: "TODO", programs: "TODO", houses: "TODO", committees: "TODO", clubs: "TODO", position: "TODO", bio: "TODO", photoUrls: "TODO") // PutUsersRequest |  (optional)

// Update User
DefaultAPI.putUsers(putUsersRequest: putUsersRequest) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **putUsersRequest** | [**PutUsersRequest**](PutUsersRequest.md) |  | [optional] 

### Return type

Void (empty response body)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# DefaultAPI

All URIs are relative to *https://stoplight.io/mocks/college-app/college-app/406762*

Method | HTTP request | Description
------------- | ------------- | -------------
[**getGenerations**](DefaultAPI.md#getgenerations) | **GET** /generations | Get Generations
[**getGenerationsRecordId**](DefaultAPI.md#getgenerationsrecordid) | **GET** /generations/{recordId} | Get Generation
[**getGroups**](DefaultAPI.md#getgroups) | **GET** /groups | Get Groups
[**getGroupsRecordId**](DefaultAPI.md#getgroupsrecordid) | **GET** /groups/{recordId} | Get Group
[**getSocialBrands**](DefaultAPI.md#getsocialbrands) | **GET** /social-brands | Get SocialBrands
[**getUsers**](DefaultAPI.md#getusers) | **GET** /users | Get Users
[**getUsersRecordId**](DefaultAPI.md#getusersrecordid) | **GET** /users/{recordId} | Get User
[**getUsersSearch**](DefaultAPI.md#getuserssearch) | **GET** /users/search | Search Users
[**postLogin**](DefaultAPI.md#postlogin) | **POST** /login | Login
[**putUsers**](DefaultAPI.md#putusers) | **PUT** /users | Update Me


# **getGenerations**
```swift
    open class func getGenerations(type: ModelType_getGenerations, completion: @escaping (_ data: [Generation]?, _ error: Error?) -> Void)
```

Get Generations

指定したtypeの`Generation`を全て取得する

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let type = "type_example" // String | 

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
 **type** | **String** |  | 

### Return type

[**[Generation]**](Generation.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getGenerationsRecordId**
```swift
    open class func getGenerationsRecordId(recordId: String, completion: @escaping (_ data: Generation?, _ error: Error?) -> Void)
```

Get Generation

指定のgenerationを取得

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let recordId = "recordId_example" // String | 

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
 **recordId** | **String** |  | 

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
    open class func getGroups(type: ModelType_getGroups, completion: @escaping (_ data: [Group]?, _ error: Error?) -> Void)
```

Get Groups

指定したtypeの`Group`を全て取得する

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let type = "type_example" // String | 

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
 **type** | **String** |  | 

### Return type

[**[Group]**](Group.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getGroupsRecordId**
```swift
    open class func getGroupsRecordId(recordId: String, completion: @escaping (_ data: Group?, _ error: Error?) -> Void)
```

Get Group

指定のGroupを取得

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let recordId = "recordId_example" // String | 

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
 **recordId** | **String** |  | 

### Return type

[**Group**](Group.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getSocialBrands**
```swift
    open class func getSocialBrands(completion: @escaping (_ data: [SocialBrand]?, _ error: Error?) -> Void)
```

Get SocialBrands

Social Brandを全て取得する

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient


// Get SocialBrands
DefaultAPI.getSocialBrands() { (response, error) in
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

[**[SocialBrand]**](SocialBrand.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUsers**
```swift
    open class func getUsers(completion: @escaping (_ data: [User]?, _ error: Error?) -> Void)
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

[**[User]**](User.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getUsersRecordId**
```swift
    open class func getUsersRecordId(recordId: String, completion: @escaping (_ data: User?, _ error: Error?) -> Void)
```

Get User

指定のuserを取得

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let recordId = "recordId_example" // String | 

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
 **recordId** | **String** |  | 

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
    open class func getUsersSearch(query: String, completion: @escaping (_ data: [User]?, _ error: Error?) -> Void)
```

Search Users

## TODO  - [ ] フィルタリング - [ ] ページネーション

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let query = "query_example" // String | 検索ワード

// Search Users
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
 **query** | **String** | 検索ワード | 

### Return type

[**[User]**](User.md)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postLogin**
```swift
    open class func postLogin(loginRequest: LoginRequest? = nil, completion: @escaping (_ data: LoginResponse?, _ error: Error?) -> Void)
```

Login

## 概要 accessTokenを取得するためのエンドポイント。  初期登録フォーム未回答の場合、User情報を取得する。  ## レスポンスによるクライアント側の分岐  - 200   - レスポンスにuserがある     - 初期登録フォームに遷移、default値にuserを使用   - レスポンスにuserがない     - Homeに遷移 - 401   - 「このエラーが出たら運営に報告してね」エラーページに遷移 - 403   - 「このアカウントは無効化されてるよ」エラーページに遷移 - 404   - 「まだ運営側でデータ登録が完了していないよ」エラーページに遷移 

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let loginRequest = LoginRequest(firebaseIdToken: "firebaseIdToken_example") // LoginRequest |  (optional)

// Login
DefaultAPI.postLogin(loginRequest: loginRequest) { (response, error) in
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
 **loginRequest** | [**LoginRequest**](LoginRequest.md) |  | [optional] 

### Return type

[**LoginResponse**](LoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **putUsers**
```swift
    open class func putUsers(updateUserRequest: UpdateUserRequest? = nil, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```

Update Me

Airtable上の自分のUser情報をアップデートする

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let updateUserRequest = UpdateUserRequest(i18n: [UpdateUserRequest_i18n_inner(locale: Locale(), firstName: "firstName_example", lastName: "lastName_example", phoneticFirstName: "phoneticFirstName_example", phoneticLastName: "phoneticLastName_example", nickName: "nickName_example", middleName: "middleName_example", phoneticMiddleName: "phoneticMiddleName_example", maidenName: "maidenName_example", _prefix: "_prefix_example", suffix: "suffix_example")], imageUrl: "imageUrl_example", pronoun: Pronoun(), birthday: Date(), hideAge: false, socialLinks: [UpdateUserRequest_socialLinks_inner(brandId: "brandId_example", username: "username_example")], roomNumber: 123, postNumber: 123, selfIntroductionSlideUrl: "selfIntroductionSlideUrl_example", generations: ["generations_example"], groups: [UpdateUserRequest_groups_inner(groupId: "groupId_example", from: Date(), to: Date())], position: "position_example", bio: "bio_example", photoUrls: ["photoUrls_example"], lastModifiedAt: Date()) // UpdateUserRequest |  (optional)

// Update Me
DefaultAPI.putUsers(updateUserRequest: updateUserRequest) { (response, error) in
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
 **updateUserRequest** | [**UpdateUserRequest**](UpdateUserRequest.md) |  | [optional] 

### Return type

Void (empty response body)

### Authorization

[accessToken](../README.md#accessToken)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


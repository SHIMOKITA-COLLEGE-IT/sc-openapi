//
// DefaultAPI.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

open class DefaultAPI {

    /**
     * enum for parameter type
     */
    public enum ModelType_getGenerations: AnyCodable, CaseIterable {
        case program = program
        case house = house
    }

    /**
     Get Generations
     
     - parameter type: (query)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getGenerations(type: ModelType_getGenerations, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: AnyCodable?, _ error: Error?) -> Void)) -> RequestTask {
        return getGenerationsWithRequestBuilder(type: type).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Get Generations
     - GET /generations
     - 指定したtypeの`Generation`を全て取得する
     - BASIC:
       - type: http
       - name: accessToken
     - parameter type: (query)  
     - returns: RequestBuilder<AnyCodable> 
     */
    open class func getGenerationsWithRequestBuilder(type: ModelType_getGenerations) -> RequestBuilder<AnyCodable> {
        let localVariablePath = "/generations"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        var localVariableUrlComponents = URLComponents(string: localVariableURLString)
        localVariableUrlComponents?.queryItems = APIHelper.mapValuesToQueryItems([
            "type": (wrappedValue: type.encodeToJSON(), isExplode: true),
        ])

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<AnyCodable>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: true)
    }

    /**
     Get Generation
     
     - parameter recordId: (path)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getGenerationsRecordId(recordId: AnyCodable, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: Generation?, _ error: Error?) -> Void)) -> RequestTask {
        return getGenerationsRecordIdWithRequestBuilder(recordId: recordId).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Get Generation
     - GET /generations/{recordId}
     - 指定のgenerationを取得
     - BASIC:
       - type: http
       - name: accessToken
     - parameter recordId: (path)  
     - returns: RequestBuilder<Generation> 
     */
    open class func getGenerationsRecordIdWithRequestBuilder(recordId: AnyCodable) -> RequestBuilder<Generation> {
        var localVariablePath = "/generations/{recordId}"
        let recordIdPreEscape = "\(APIHelper.mapValueToPathItem(recordId))"
        let recordIdPostEscape = recordIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{recordId}", with: recordIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<Generation>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: true)
    }

    /**
     * enum for parameter type
     */
    public enum ModelType_getGroups: AnyCodable, CaseIterable {
        case committee = committee
        case club = club
    }

    /**
     Get Groups
     
     - parameter type: (query)  (optional)
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getGroups(type: ModelType_getGroups? = nil, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: AnyCodable?, _ error: Error?) -> Void)) -> RequestTask {
        return getGroupsWithRequestBuilder(type: type).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Get Groups
     - GET /groups
     - 指定したtypeの`Group`を全て取得する
     - BASIC:
       - type: http
       - name: accessToken
     - parameter type: (query)  (optional)
     - returns: RequestBuilder<AnyCodable> 
     */
    open class func getGroupsWithRequestBuilder(type: ModelType_getGroups? = nil) -> RequestBuilder<AnyCodable> {
        let localVariablePath = "/groups"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        var localVariableUrlComponents = URLComponents(string: localVariableURLString)
        localVariableUrlComponents?.queryItems = APIHelper.mapValuesToQueryItems([
            "type": (wrappedValue: type?.encodeToJSON(), isExplode: true),
        ])

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<AnyCodable>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: true)
    }

    /**
     Get Group
     
     - parameter recordId: (path)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getGroupsRecordId(recordId: AnyCodable, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: Group?, _ error: Error?) -> Void)) -> RequestTask {
        return getGroupsRecordIdWithRequestBuilder(recordId: recordId).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Get Group
     - GET /groups/{recordId}
     - 指定のGroupを取得
     - BASIC:
       - type: http
       - name: accessToken
     - parameter recordId: (path)  
     - returns: RequestBuilder<Group> 
     */
    open class func getGroupsRecordIdWithRequestBuilder(recordId: AnyCodable) -> RequestBuilder<Group> {
        var localVariablePath = "/groups/{recordId}"
        let recordIdPreEscape = "\(APIHelper.mapValueToPathItem(recordId))"
        let recordIdPostEscape = recordIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{recordId}", with: recordIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<Group>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: true)
    }

    /**
     Get Users
     
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getUsers(apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: AnyCodable?, _ error: Error?) -> Void)) -> RequestTask {
        return getUsersWithRequestBuilder().execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Get Users
     - GET /users
     - 全てのユーザーを取得する
     - BASIC:
       - type: http
       - name: accessToken
     - returns: RequestBuilder<AnyCodable> 
     */
    open class func getUsersWithRequestBuilder() -> RequestBuilder<AnyCodable> {
        let localVariablePath = "/users"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<AnyCodable>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: true)
    }

    /**
     Get User
     
     - parameter recordId: (path)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getUsersRecordId(recordId: AnyCodable, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: User?, _ error: Error?) -> Void)) -> RequestTask {
        return getUsersRecordIdWithRequestBuilder(recordId: recordId).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Get User
     - GET /users/{recordId}
     - 指定のuserを取得
     - BASIC:
       - type: http
       - name: accessToken
     - parameter recordId: (path)  
     - returns: RequestBuilder<User> 
     */
    open class func getUsersRecordIdWithRequestBuilder(recordId: AnyCodable) -> RequestBuilder<User> {
        var localVariablePath = "/users/{recordId}"
        let recordIdPreEscape = "\(APIHelper.mapValueToPathItem(recordId))"
        let recordIdPostEscape = recordIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{recordId}", with: recordIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<User>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: true)
    }

    /**
     Search Users (WIP)
     
     - parameter query: (query) 検索ワード 
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getUsersSearch(query: AnyCodable, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: AnyCodable?, _ error: Error?) -> Void)) -> RequestTask {
        return getUsersSearchWithRequestBuilder(query: query).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Search Users (WIP)
     - GET /users/search
     - ## TODO  - [ ] フィルタリング - [ ] ページネーション
     - BASIC:
       - type: http
       - name: accessToken
     - parameter query: (query) 検索ワード 
     - returns: RequestBuilder<AnyCodable> 
     */
    open class func getUsersSearchWithRequestBuilder(query: AnyCodable) -> RequestBuilder<AnyCodable> {
        let localVariablePath = "/users/search"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        var localVariableUrlComponents = URLComponents(string: localVariableURLString)
        localVariableUrlComponents?.queryItems = APIHelper.mapValuesToQueryItems([
            "query": (wrappedValue: query.encodeToJSON(), isExplode: true),
        ])

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<AnyCodable>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: true)
    }

    /**
     Login
     
     - parameter postLoginRequest: (body)  (optional)
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func postLogin(postLoginRequest: PostLoginRequest? = nil, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: PostLogin200Response?, _ error: Error?) -> Void)) -> RequestTask {
        return postLoginWithRequestBuilder(postLoginRequest: postLoginRequest).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Login
     - POST /login
     - ## 概要 accessTokenを取得するためのエンドポイント。  初期登録フォーム未回答の場合、User情報を取得する。  ## レスポンスによるクライアント側の分岐  - 200   - レスポンスにuserがある     - 初期登録フォームに遷移、default値にuserを使用   - レスポンスにuserがない     - Homeに遷移 - 401   - 「このエラーが出たら運営に報告してね」エラーページに遷移 - 403   - 「このアカウントは無効化されてるよ」エラーページに遷移 - 404   - 「まだ運営側でデータ登録が完了していないよ」エラーページに遷移 
     - parameter postLoginRequest: (body)  (optional)
     - returns: RequestBuilder<PostLogin200Response> 
     */
    open class func postLoginWithRequestBuilder(postLoginRequest: PostLoginRequest? = nil) -> RequestBuilder<PostLogin200Response> {
        let localVariablePath = "/login"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: postLoginRequest)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<PostLogin200Response>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "POST", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Update User
     
     - parameter putUsersRequest: (body)  (optional)
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func putUsers(putUsersRequest: PutUsersRequest? = nil, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: Void?, _ error: Error?) -> Void)) -> RequestTask {
        return putUsersWithRequestBuilder(putUsersRequest: putUsersRequest).execute(apiResponseQueue) { result in
            switch result {
            case .success:
                completion((), nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Update User
     - PUT /users
     - Airtable上のUser情報をアップデートする
     - BASIC:
       - type: http
       - name: accessToken
     - parameter putUsersRequest: (body)  (optional)
     - returns: RequestBuilder<Void> 
     */
    open class func putUsersWithRequestBuilder(putUsersRequest: PutUsersRequest? = nil) -> RequestBuilder<Void> {
        let localVariablePath = "/users"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: putUsersRequest)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<Void>.Type = OpenAPIClientAPI.requestBuilderFactory.getNonDecodableBuilder()

        return localVariableRequestBuilder.init(method: "PUT", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: true)
    }
}
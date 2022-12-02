/* tslint:disable */
/* eslint-disable */
/**
 * main
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface LoginRequest
 */
export interface LoginRequest {
    /**
     * 取得方法は[こちら](https://firebase.google.com/docs/auth/admin/verify-id-tokens#retrieve_id_tokens_on_clients)
     * @type {string}
     * @memberof LoginRequest
     */
    firebaseIdToken: string;
}

/**
 * Check if a given object implements the LoginRequest interface.
 */
export function instanceOfLoginRequest(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "firebaseIdToken" in value;

    return isInstance;
}

export function LoginRequestFromJSON(json: any): LoginRequest {
    return LoginRequestFromJSONTyped(json, false);
}

export function LoginRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): LoginRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'firebaseIdToken': json['firebaseIdToken'],
    };
}

export function LoginRequestToJSON(value?: LoginRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'firebaseIdToken': value.firebaseIdToken,
    };
}


/* tslint:disable */
/* eslint-disable */
/**
 * College App API
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
import type { SocialBrand } from './SocialBrand';
import {
    SocialBrandFromJSON,
    SocialBrandFromJSONTyped,
    SocialBrandToJSON,
} from './SocialBrand';

/**
 * 
 * @export
 * @interface UserSocialLink
 */
export interface UserSocialLink {
    /**
     * 
     * @type {SocialBrand}
     * @memberof UserSocialLink
     */
    brand: SocialBrand;
    /**
     * 
     * @type {string}
     * @memberof UserSocialLink
     */
    username: string;
}

/**
 * Check if a given object implements the UserSocialLink interface.
 */
export function instanceOfUserSocialLink(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "brand" in value;
    isInstance = isInstance && "username" in value;

    return isInstance;
}

export function UserSocialLinkFromJSON(json: any): UserSocialLink {
    return UserSocialLinkFromJSONTyped(json, false);
}

export function UserSocialLinkFromJSONTyped(json: any, ignoreDiscriminator: boolean): UserSocialLink {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'brand': SocialBrandFromJSON(json['brand']),
        'username': json['username'],
    };
}

export function UserSocialLinkToJSON(value?: UserSocialLink | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'brand': SocialBrandToJSON(value.brand),
        'username': value.username,
    };
}

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
import type { Locale } from './Locale';
import {
    LocaleFromJSON,
    LocaleFromJSONTyped,
    LocaleToJSON,
} from './Locale';

/**
 * 
 * @export
 * @interface UserI18n
 */
export interface UserI18n {
    /**
     * 
     * @type {Locale}
     * @memberof UserI18n
     */
    locale: Locale;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    firstName: string;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    lastName: string;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    phoneticFirstName?: string;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    phoneticLastName?: string;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    nickName?: string;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    middleName?: string;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    phoneticMiddleName?: string;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    maidenName?: string;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    prefix?: string;
    /**
     * 
     * @type {string}
     * @memberof UserI18n
     */
    suffix?: string;
}

/**
 * Check if a given object implements the UserI18n interface.
 */
export function instanceOfUserI18n(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "locale" in value;
    isInstance = isInstance && "firstName" in value;
    isInstance = isInstance && "lastName" in value;

    return isInstance;
}

export function UserI18nFromJSON(json: any): UserI18n {
    return UserI18nFromJSONTyped(json, false);
}

export function UserI18nFromJSONTyped(json: any, ignoreDiscriminator: boolean): UserI18n {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'locale': LocaleFromJSON(json['locale']),
        'firstName': json['firstName'],
        'lastName': json['lastName'],
        'phoneticFirstName': !exists(json, 'phoneticFirstName') ? undefined : json['phoneticFirstName'],
        'phoneticLastName': !exists(json, 'phoneticLastName') ? undefined : json['phoneticLastName'],
        'nickName': !exists(json, 'nickName') ? undefined : json['nickName'],
        'middleName': !exists(json, 'middleName') ? undefined : json['middleName'],
        'phoneticMiddleName': !exists(json, 'phoneticMiddleName') ? undefined : json['phoneticMiddleName'],
        'maidenName': !exists(json, 'maidenName') ? undefined : json['maidenName'],
        'prefix': !exists(json, 'prefix') ? undefined : json['prefix'],
        'suffix': !exists(json, 'suffix') ? undefined : json['suffix'],
    };
}

export function UserI18nToJSON(value?: UserI18n | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'locale': LocaleToJSON(value.locale),
        'firstName': value.firstName,
        'lastName': value.lastName,
        'phoneticFirstName': value.phoneticFirstName,
        'phoneticLastName': value.phoneticLastName,
        'nickName': value.nickName,
        'middleName': value.middleName,
        'phoneticMiddleName': value.phoneticMiddleName,
        'maidenName': value.maidenName,
        'prefix': value.prefix,
        'suffix': value.suffix,
    };
}


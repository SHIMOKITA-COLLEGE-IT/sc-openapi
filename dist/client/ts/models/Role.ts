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


/**
 * Enum
 * @export
 */
export const Role = {
    Resident: 'resident',
    Alumni: 'alumni',
    Staff: 'staff'
} as const;
export type Role = typeof Role[keyof typeof Role];


export function RoleFromJSON(json: any): Role {
    return RoleFromJSONTyped(json, false);
}

export function RoleFromJSONTyped(json: any, ignoreDiscriminator: boolean): Role {
    return json as Role;
}

export function RoleToJSON(value?: Role | null): any {
    return value as any;
}


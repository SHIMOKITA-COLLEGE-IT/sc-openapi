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


/**
 * Enum
 * @export
 */
export const Pronoun = {
    SheHer: 'SHE_HER',
    HeHim: 'HE_HIM',
    TheyThem: 'THEY_THEM'
} as const;
export type Pronoun = typeof Pronoun[keyof typeof Pronoun];


export function PronounFromJSON(json: any): Pronoun {
    return PronounFromJSONTyped(json, false);
}

export function PronounFromJSONTyped(json: any, ignoreDiscriminator: boolean): Pronoun {
    return json as Pronoun;
}

export function PronounToJSON(value?: Pronoun | null): any {
    return value as any;
}


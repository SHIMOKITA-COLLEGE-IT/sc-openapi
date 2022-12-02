/*
 * main
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.0
 * Generated by: OpenAPI Generator (https://openapi-generator.tech)
 */

package openapi

type UpdateUserRequestI18nInner struct {

	Locale Locale `json:"locale"`

	FirstName string `json:"firstName,omitempty"`

	LastName string `json:"lastName,omitempty"`

	PhoneticFirstName string `json:"phoneticFirstName,omitempty"`

	PhoneticLastName string `json:"phoneticLastName,omitempty"`

	NickName string `json:"nickName,omitempty"`

	MiddleName string `json:"middleName,omitempty"`

	PhoneticMiddleName string `json:"phoneticMiddleName,omitempty"`

	MaidenName string `json:"maidenName,omitempty"`

	Prefix string `json:"prefix,omitempty"`

	Suffix string `json:"suffix,omitempty"`
}

// AssertUpdateUserRequestI18nInnerRequired checks if the required fields are not zero-ed
func AssertUpdateUserRequestI18nInnerRequired(obj UpdateUserRequestI18nInner) error {
	elements := map[string]interface{}{
		"locale": obj.Locale,
	}
	for name, el := range elements {
		if isZero := IsZeroValue(el); isZero {
			return &RequiredError{Field: name}
		}
	}

	return nil
}

// AssertRecurseUpdateUserRequestI18nInnerRequired recursively checks if required fields are not zero-ed in a nested slice.
// Accepts only nested slice of UpdateUserRequestI18nInner (e.g. [][]UpdateUserRequestI18nInner), otherwise ErrTypeAssertionError is thrown.
func AssertRecurseUpdateUserRequestI18nInnerRequired(objSlice interface{}) error {
	return AssertRecurseInterfaceRequired(objSlice, func(obj interface{}) error {
		aUpdateUserRequestI18nInner, ok := obj.(UpdateUserRequestI18nInner)
		if !ok {
			return ErrTypeAssertionError
		}
		return AssertUpdateUserRequestI18nInnerRequired(aUpdateUserRequestI18nInner)
	})
}

/*
 * main
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.0
 * Generated by: OpenAPI Generator (https://openapi-generator.tech)
 */

package openapi

type UserGroupBelonging struct {

	Group Group `json:"group"`

	DisplayName *interface{} `json:"displayName"`

	From *interface{} `json:"from,omitempty"`

	To *interface{} `json:"to,omitempty"`
}

// AssertUserGroupBelongingRequired checks if the required fields are not zero-ed
func AssertUserGroupBelongingRequired(obj UserGroupBelonging) error {
	elements := map[string]interface{}{
		"group": obj.Group,
		"displayName": obj.DisplayName,
	}
	for name, el := range elements {
		if isZero := IsZeroValue(el); isZero {
			return &RequiredError{Field: name}
		}
	}

	if err := AssertGroupRequired(obj.Group); err != nil {
		return err
	}
	return nil
}

// AssertRecurseUserGroupBelongingRequired recursively checks if required fields are not zero-ed in a nested slice.
// Accepts only nested slice of UserGroupBelonging (e.g. [][]UserGroupBelonging), otherwise ErrTypeAssertionError is thrown.
func AssertRecurseUserGroupBelongingRequired(objSlice interface{}) error {
	return AssertRecurseInterfaceRequired(objSlice, func(obj interface{}) error {
		aUserGroupBelonging, ok := obj.(UserGroupBelonging)
		if !ok {
			return ErrTypeAssertionError
		}
		return AssertUserGroupBelongingRequired(aUserGroupBelonging)
	})
}

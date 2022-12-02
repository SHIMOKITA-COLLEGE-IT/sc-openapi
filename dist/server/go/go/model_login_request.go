/*
 * main
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.0
 * Generated by: OpenAPI Generator (https://openapi-generator.tech)
 */

package openapi

// LoginRequest - 
type LoginRequest struct {

	// 取得方法は[こちら](https://firebase.google.com/docs/auth/admin/verify-id-tokens#retrieve_id_tokens_on_clients)
	FirebaseIdToken string `json:"firebaseIdToken"`
}

// AssertLoginRequestRequired checks if the required fields are not zero-ed
func AssertLoginRequestRequired(obj LoginRequest) error {
	elements := map[string]interface{}{
		"firebaseIdToken": obj.FirebaseIdToken,
	}
	for name, el := range elements {
		if isZero := IsZeroValue(el); isZero {
			return &RequiredError{Field: name}
		}
	}

	return nil
}

// AssertRecurseLoginRequestRequired recursively checks if required fields are not zero-ed in a nested slice.
// Accepts only nested slice of LoginRequest (e.g. [][]LoginRequest), otherwise ErrTypeAssertionError is thrown.
func AssertRecurseLoginRequestRequired(objSlice interface{}) error {
	return AssertRecurseInterfaceRequired(objSlice, func(obj interface{}) error {
		aLoginRequest, ok := obj.(LoginRequest)
		if !ok {
			return ErrTypeAssertionError
		}
		return AssertLoginRequestRequired(aLoginRequest)
	})
}

/*
 * main
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.0
 * Generated by: OpenAPI Generator (https://openapi-generator.tech)
 */

package openapi

type PutUsersRequest struct {

	I18n *interface{} `json:"i18n,omitempty"`

	ImageUrl *interface{} `json:"imageUrl,omitempty"`

	Pronoun *interface{} `json:"pronoun,omitempty"`

	Birthday *interface{} `json:"birthday,omitempty"`

	HideAge *interface{} `json:"hideAge,omitempty"`

	SocialLinks *interface{} `json:"socialLinks,omitempty"`

	RoomNumber *interface{} `json:"roomNumber,omitempty"`

	PostNumber *interface{} `json:"postNumber,omitempty"`

	SelfIntroductionSlideUrl *interface{} `json:"selfIntroductionSlideUrl,omitempty"`

	Programs *interface{} `json:"programs,omitempty"`

	Houses *interface{} `json:"houses,omitempty"`

	Committees *interface{} `json:"committees,omitempty"`

	Clubs *interface{} `json:"clubs,omitempty"`

	// 肩書き
	Position *interface{} `json:"position,omitempty"`

	// Markdown
	Bio *interface{} `json:"bio,omitempty"`

	PhotoUrls *interface{} `json:"photoUrls,omitempty"`
}

// AssertPutUsersRequestRequired checks if the required fields are not zero-ed
func AssertPutUsersRequestRequired(obj PutUsersRequest) error {
	return nil
}

// AssertRecursePutUsersRequestRequired recursively checks if required fields are not zero-ed in a nested slice.
// Accepts only nested slice of PutUsersRequest (e.g. [][]PutUsersRequest), otherwise ErrTypeAssertionError is thrown.
func AssertRecursePutUsersRequestRequired(objSlice interface{}) error {
	return AssertRecurseInterfaceRequired(objSlice, func(obj interface{}) error {
		aPutUsersRequest, ok := obj.(PutUsersRequest)
		if !ok {
			return ErrTypeAssertionError
		}
		return AssertPutUsersRequestRequired(aPutUsersRequest)
	})
}
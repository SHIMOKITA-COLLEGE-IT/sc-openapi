//
// UserSocialLink.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

public struct UserSocialLink: Codable, JSONEncodable, Hashable {

    public var brand: SocialBrand
    public var username: AnyCodable?

    public init(brand: SocialBrand, username: AnyCodable?) {
        self.brand = brand
        self.username = username
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case brand
        case username
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(brand, forKey: .brand)
        try container.encode(username, forKey: .username)
    }
}


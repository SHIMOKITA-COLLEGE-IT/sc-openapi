//
// UserGroupBelonging.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

public struct UserGroupBelonging: Codable, JSONEncodable, Hashable {

    public var group: Group
    public var displayName: String
    public var from: Date?
    public var to: Date?

    public init(group: Group, displayName: String, from: Date? = nil, to: Date? = nil) {
        self.group = group
        self.displayName = displayName
        self.from = from
        self.to = to
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case group
        case displayName
        case from
        case to
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(group, forKey: .group)
        try container.encode(displayName, forKey: .displayName)
        try container.encodeIfPresent(from, forKey: .from)
        try container.encodeIfPresent(to, forKey: .to)
    }
}

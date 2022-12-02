//
// Group.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

/** &#x60;Committee&#x60;, &#x60;Club&#x60;のモデル */
public struct Group: Codable, JSONEncodable, Hashable {

    /** Airtable's Record ID */
    public var id: String
    public var type: GroupType
    public var displayName: String
    public var createdAt: Date?
    public var archivedAt: Date?
    public var emoji: String?
    public var title: String?
    /** Markdown */
    public var description: String?
    public var slackChannel: String?
    public var photoUrls: [String]?

    public init(id: String, type: GroupType, displayName: String, createdAt: Date? = nil, archivedAt: Date? = nil, emoji: String? = nil, title: String? = nil, description: String? = nil, slackChannel: String? = nil, photoUrls: [String]? = nil) {
        self.id = id
        self.type = type
        self.displayName = displayName
        self.createdAt = createdAt
        self.archivedAt = archivedAt
        self.emoji = emoji
        self.title = title
        self.description = description
        self.slackChannel = slackChannel
        self.photoUrls = photoUrls
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case id
        case type
        case displayName
        case createdAt
        case archivedAt
        case emoji
        case title
        case description
        case slackChannel
        case photoUrls
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(id, forKey: .id)
        try container.encode(type, forKey: .type)
        try container.encode(displayName, forKey: .displayName)
        try container.encodeIfPresent(createdAt, forKey: .createdAt)
        try container.encodeIfPresent(archivedAt, forKey: .archivedAt)
        try container.encodeIfPresent(emoji, forKey: .emoji)
        try container.encodeIfPresent(title, forKey: .title)
        try container.encodeIfPresent(description, forKey: .description)
        try container.encodeIfPresent(slackChannel, forKey: .slackChannel)
        try container.encodeIfPresent(photoUrls, forKey: .photoUrls)
    }
}


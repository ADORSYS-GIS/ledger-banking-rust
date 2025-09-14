use serde::{Deserialize, Serialize};

/// Database model for messaging type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "messaging_type", rename_all = "PascalCase")]
pub enum MessagingType {
    Email,
    Phone,
    Sms,
    WhatsApp,
    Telegram,
    Skype,
    Teams,
    Signal,
    WeChat,
    Viber,
    Messenger,
    LinkedIn,
    Slack,
    Discord,
    Other,
}

// Serialization functions for MessagingType
pub fn serialize_messaging_type<S>(messaging_type: &MessagingType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let type_str = match messaging_type {
        MessagingType::Email => "email",
        MessagingType::Phone => "phone",
        MessagingType::Sms => "sms",
        MessagingType::WhatsApp => "whatsapp",
        MessagingType::Telegram => "telegram",
        MessagingType::Skype => "skype",
        MessagingType::Teams => "teams",
        MessagingType::Signal => "signal",
        MessagingType::WeChat => "wechat",
        MessagingType::Viber => "viber",
        MessagingType::Messenger => "messenger",
        MessagingType::LinkedIn => "linkedin",
        MessagingType::Slack => "slack",
        MessagingType::Discord => "discord",
        MessagingType::Other => "other",
    };
    serializer.serialize_str(type_str)
}

pub fn deserialize_messaging_type<'de, D>(deserializer: D) -> Result<MessagingType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "email" => Ok(MessagingType::Email),
        "phone" => Ok(MessagingType::Phone),
        "sms" => Ok(MessagingType::Sms),
        "whatsapp" => Ok(MessagingType::WhatsApp),
        "telegram" => Ok(MessagingType::Telegram),
        "skype" => Ok(MessagingType::Skype),
        "teams" => Ok(MessagingType::Teams),
        "signal" => Ok(MessagingType::Signal),
        "wechat" => Ok(MessagingType::WeChat),
        "viber" => Ok(MessagingType::Viber),
        "messenger" => Ok(MessagingType::Messenger),
        "linkedin" => Ok(MessagingType::LinkedIn),
        "slack" => Ok(MessagingType::Slack),
        "discord" => Ok(MessagingType::Discord),
        "other" => Ok(MessagingType::Other),
        _ => Err(serde::de::Error::custom(format!("Unknown messaging type: {s}"))),
    }
}

// Serialization functions for Option<MessagingType>
pub fn serialize_messaging_type_option<S>(messaging_type: &Option<MessagingType>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match messaging_type {
        Some(msg_type) => serialize_messaging_type(msg_type, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_messaging_type_option<'de, D>(deserializer: D) -> Result<Option<MessagingType>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => match s.as_str() {
            "email" => Ok(Some(MessagingType::Email)),
            "phone" => Ok(Some(MessagingType::Phone)),
            "sms" => Ok(Some(MessagingType::Sms)),
            "whatsapp" => Ok(Some(MessagingType::WhatsApp)),
            "telegram" => Ok(Some(MessagingType::Telegram)),
            "skype" => Ok(Some(MessagingType::Skype)),
            "teams" => Ok(Some(MessagingType::Teams)),
            "signal" => Ok(Some(MessagingType::Signal)),
            "wechat" => Ok(Some(MessagingType::WeChat)),
            "viber" => Ok(Some(MessagingType::Viber)),
            "messenger" => Ok(Some(MessagingType::Messenger)),
            "linkedin" => Ok(Some(MessagingType::LinkedIn)),
            "slack" => Ok(Some(MessagingType::Slack)),
            "discord" => Ok(Some(MessagingType::Discord)),
            "other" => Ok(Some(MessagingType::Other)),
            _ => Err(serde::de::Error::custom(format!("Unknown messaging type: {s}"))),
        },
        None => Ok(None),
    }
}
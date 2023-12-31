use crate::macros::properties;
use crate::parser::Encode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Str(String),
    Int(i32),
    Bool(bool),
}

impl Encode for PropertyType {
    fn encode(&self, buf: &mut String) -> std::fmt::Result {
        match self {
            PropertyType::Str(val) => val.encode(buf),
            PropertyType::Int(val) => val.encode(buf),
            PropertyType::Bool(val) => val.encode(buf),
        }
    }
}

properties! {
    ChannelProperty {
        ParentId: i32 = "cpid",

        Name: str = "channel_name",
        Topic: str = "channel_topic",
        Description: str = "channel_description",
        Password: str = "channel_password",

        Codec: i32 = "channel_codec",
        CodecQuality: i32 = "channel_codec_quality",

        MaxClients: i32 = "channel_maxclients",
        MaxFamilyClients: i32 = "channel_maxfamilyclients",

        Order: i32 = "channel_order",

        FlagPermanent: bool = "channel_flag_permanent",
        FlagSemiPermanent: bool = "channel_flag_semi_permanent",
        FlagDefault: bool = "channel_flag_default",

        CodecIsUnencrypted: bool = "channel_codec_is_unencrypted",
        DeleteDelay: i32 = "channel_delete_delay",

        FlagMaxClientsUnlimited: bool = "channel_flag_maxclients_unlimited",
        FlagMaxFamilyClientsUnlimited: bool = "channel_flag_maxfamilyclients_unlimited",
        FlagMaxFamilyClientsInherited: bool = "channel_flag_maxfamilyclients_inherited",

        NeededTalkPower: i32 = "channel_needed_talk_power",
        NamePhonetic: str = "channel_name_phonetic",

        IconId: i32 = "channel_icon_id",

        BannerUrl: str = "channel_banner_gfx_url",
        BannerMode: i32 = "channel_banner_mode"
    }
}

#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

#[cfg(feature = "serde")]
impl serde::Serialize for ChannelProperty {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Permission", 2)?;
        let (id, value) = self.contents();

        state.serialize_field("id", id.as_ref())?;

        let value = match value {
            PropertyType::Str(val) => val,
            PropertyType::Int(val) => val.to_string(),
            PropertyType::Bool(val) => val.to_string()
        };

        state.serialize_field("value", &value)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ChannelProperty {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<ChannelProperty, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Id,
            Value,
        }

        struct ChannelPropertyVisitor;

        impl<'de> serde::de::Visitor<'de> for ChannelPropertyVisitor {
            type Value = ChannelProperty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct ChannelProperty")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
                let id = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &"struct PermissionValue with 2 elements"))?;
                let value = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &"struct PermissionValue with 2 elements"))?;

                ChannelProperty::parse(id, value, true).map_err(serde::de::Error::custom)
            }

            fn visit_map<V: serde::de::MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let mut id: Option<String> = None;
                let mut value: Option<String> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }

                            id = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }

                            value = Some(map.next_value()?);
                        }
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let value = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;

                ChannelProperty::parse(&id, &value, true).map_err(serde::de::Error::custom)
            }
        }

        const FIELDS: &[&str] = &["id", "value"];
        deserializer.deserialize_struct("ChannelProperty", FIELDS, ChannelPropertyVisitor)
    }
}

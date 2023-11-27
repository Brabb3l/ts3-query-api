use crate::macros::properties;
use crate::parser::Encode;

pub enum PropertyType<'a> {
    Str(&'a str),
    Int(i32),
    Bool(bool),
}

impl Encode for PropertyType<'_> {
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
        Cpid: i32 = "cpid",

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
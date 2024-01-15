use crate::definitions::{Badges, ChannelInfo, Codec, CodecEncryptionMode, HostBannerMode, HostMessageMode, ServerInfo, ServerStatus};
use crate::macros::properties;
use crate::parser::Encode;

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyType {
    Str(String),
    Int(i32),
    I64(i64),
    U64(u64),
    Float(f32),
    Bool(bool),
}

impl Encode for PropertyType {
    fn encode(&self, buf: &mut String) -> std::fmt::Result {
        match self {
            PropertyType::Str(val) => val.encode(buf),
            PropertyType::Int(val) => val.encode(buf),
            PropertyType::I64(val) => val.encode(buf),
            PropertyType::U64(val) => val.encode(buf),
            PropertyType::Float(val) => val.encode(buf),
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

        Codec: Codec = "channel_codec",
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

properties! {
    ServerProperty {
        Port: i32 = "virtualserver_port",

        MinClientVersion: str = "virtualserver_min_client_version",
        MinAndroidVersion: str = "virtualserver_min_android_version",
        MinIosVersion: str = "virtualserver_min_ios_version",

        Name: str = "virtualserver_name",
        NamePhonetic: str = "virtualserver_name_phonetic",
        WelcomeMessage: str = "virtualserver_welcome_message",
        IconId: i32 = "virtualserver_icon_id",

        Status: ServerStatus = "virtualserver_status",
        AutoStart: bool = "virtualserver_autostart",
        WeblistEnabled: bool = "virtualserver_weblist_enabled",

        Password: str = "virtualserver_password",

        MaxClients: i32 = "virtualserver_maxclients",
        ReservedSlots: i32 = "virtualserver_reserved_slots",

        NeededIdentitySecurityLevel: i32 = "virtualserver_needed_identity_security_level",
        CodecEncryptionMode: CodecEncryptionMode = "virtualserver_codec_encryption_mode",

        DefaultServerGroup: i32 = "virtualserver_default_server_group",
        DefaultChannelGroup: i32 = "virtualserver_default_channel_group",
        DefaultChannelAdminGroup: i32 = "virtualserver_default_channel_admin_group",

        HostMessage: str = "virtualserver_hostmessage",
        HostMessageMode: HostMessageMode = "virtualserver_hostmessage_mode",

        HostBannerMode: HostBannerMode = "virtualserver_hostbanner_mode",
        HostBannerUrl: str = "virtualserver_hostbanner_gfx_url",
        HostBannerGfxUrl: str = "virtualserver_hostbanner_url",
        HostBannerGfxInterval: i32 = "virtualserver_hostbanner_gfx_interval",

        HostButtonTooltip: str = "virtualserver_hostbutton_tooltip",
        HostButtonGfxUrl: str = "virtualserver_hostbutton_gfx_url",
        HostButtonUrl: str = "virtualserver_hostbutton_url",

        ComplainAutoBanCount: i32 = "virtualserver_complain_autoban_count",
        ComplainAutoBanTime: i32 = "virtualserver_complain_autoban_time",
        ComplainRemoveTime: i32 = "virtualserver_complain_remove_time",

        MinClientsInChannelBeforeForcedSilence: i32 = "virtualserver_min_clients_in_channel_before_forced_silence",
        PrioritySpeakerDimmModificator: f32 = "virtualserver_priority_speaker_dimm_modificator",

        AntiFloodPointsTickReduce: i32 = "virtualserver_antiflood_points_tick_reduce",
        AntiFloodPointsNeededCommandBlock: i32 = "virtualserver_antiflood_points_needed_command_block",
        AntiFloodPointsNeededPluginBlock: i32 = "virtualserver_antiflood_points_needed_plugin_block",
        AntiFloodPointsNeededIpBlock: i32 = "virtualserver_antiflood_points_needed_ip_block",

        LogClient: bool = "virtualserver_log_client",
        LogQuery: bool = "virtualserver_log_query",
        LogChannel: bool = "virtualserver_log_channel",
        LogPermissions: bool = "virtualserver_log_permissions",
        LogServer: bool = "virtualserver_log_server",
        LogFileTransfer: bool = "virtualserver_log_filetransfer",

        DownloadQuota: u64 = "virtualserver_download_quota",
        UploadQuota: u64 = "virtualserver_upload_quota",

        MaxDownloadTotalBandwidth: u64 = "virtualserver_max_download_total_bandwidth",
        MaxUploadTotalBandwidth: u64 = "virtualserver_max_upload_total_bandwidth",
    }
}

properties! {
    ClientProperty {
        Badges: Badges = "client_badges",
        Description: str = "client_description",
        IconId: i32 = "client_icon_id",
        IsChannelCommander: bool = "client_is_channel_commander",
        IsTalker: bool = "client_is_talker",
        Nickname: str = "client_nickname"
    }
}

impl ChannelInfo {
    pub fn into_properties_vec(self, mut dst: Vec<ChannelProperty>) -> Vec<ChannelProperty> {
        dst.push(ChannelProperty::ParentId(self.parent_id));
        dst.push(ChannelProperty::Name(self.name));

        if let Some(topic) = self.topic {
            dst.push(ChannelProperty::Topic(topic));
        }

        if let Some(description) = self.description {
            dst.push(ChannelProperty::Description(description));
        }

        if let Some(password) = self.password {
            dst.push(ChannelProperty::Password(password));
        }

        dst.push(ChannelProperty::Codec(self.codec));
        dst.push(ChannelProperty::CodecQuality(self.codec_quality));

        if !self.flag_max_clients_unlimited {
            dst.push(ChannelProperty::MaxClients(self.max_clients));
        }

        if !self.flag_max_family_clients_unlimited && !self.flag_max_family_clients_inherited {
            dst.push(ChannelProperty::MaxFamilyClients(self.max_family_clients));
        }

        dst.push(ChannelProperty::Order(self.order));

        dst.push(ChannelProperty::FlagPermanent(self.flag_permanent));
        dst.push(ChannelProperty::FlagSemiPermanent(self.flag_semi_permanent));
        dst.push(ChannelProperty::FlagDefault(self.flag_default));

        dst.push(ChannelProperty::CodecIsUnencrypted(self.codec_is_unencrypted));

        if !self.flag_permanent && !self.flag_semi_permanent && !self.flag_default {
            dst.push(ChannelProperty::DeleteDelay(self.delete_delay));
        }

        dst.push(ChannelProperty::FlagMaxClientsUnlimited(self.flag_max_clients_unlimited));
        dst.push(ChannelProperty::FlagMaxFamilyClientsUnlimited(
            self.flag_max_family_clients_unlimited,
        ));
        dst.push(ChannelProperty::FlagMaxFamilyClientsInherited(
            self.flag_max_family_clients_inherited,
        ));

        dst.push(ChannelProperty::NeededTalkPower(self.needed_talk_power));

        if let Some(name_phonetic) = self.name_phonetic {
            dst.push(ChannelProperty::NamePhonetic(name_phonetic));
        }

        if self.icon_id != 0 {
            dst.push(ChannelProperty::IconId(self.icon_id));
        }

        if let Some(banner_url) = self.banner_gfx_url {
            dst.push(ChannelProperty::BannerUrl(banner_url));
        }

        dst.push(ChannelProperty::BannerMode(self.banner_mode));

        dst
    }

    pub fn to_properties_vec(self) -> Vec<ChannelProperty> {
        self.into_properties_vec(Vec::new())
    }
}

impl ServerInfo {
    pub fn into_properties_vec(self, mut dst: Vec<ServerProperty>) -> Vec<ServerProperty> {
        dst.push(ServerProperty::Port(self.port));

        dst.push(ServerProperty::MinClientVersion(self.min_client_version));
        dst.push(ServerProperty::MinAndroidVersion(self.min_android_version));
        dst.push(ServerProperty::MinIosVersion(self.min_ios_version));

        dst.push(ServerProperty::Name(self.name));

        if let Some(name_phonetic) = self.name_phonetic {
            dst.push(ServerProperty::NamePhonetic(name_phonetic));
        }

        if let Some(welcome_message) = self.welcome_message {
            dst.push(ServerProperty::WelcomeMessage(welcome_message));
        }

        dst.push(ServerProperty::IconId(self.icon_id));

        if self.icon_id != 0 {
            dst.push(ServerProperty::IconId(self.icon_id));
        }

        dst.push(ServerProperty::Status(self.status));
        dst.push(ServerProperty::AutoStart(self.auto_start));
        dst.push(ServerProperty::WeblistEnabled(self.weblist_enabled));

        if let Some(password) = self.password {
            dst.push(ServerProperty::Password(password));
        }

        dst.push(ServerProperty::MaxClients(self.max_clients));
        dst.push(ServerProperty::ReservedSlots(self.reserved_slots));

        dst.push(ServerProperty::NeededIdentitySecurityLevel(self.needed_identity_security_level));
        dst.push(ServerProperty::CodecEncryptionMode(self.codec_encryption_mode));

        dst.push(ServerProperty::DefaultServerGroup(self.default_server_group));
        dst.push(ServerProperty::DefaultChannelGroup(self.default_channel_group));
        dst.push(ServerProperty::DefaultChannelAdminGroup(self.default_channel_admin_group));

        if let Some(host_message) = self.host_message {
            dst.push(ServerProperty::HostMessage(host_message));
        }

        dst.push(ServerProperty::HostMessageMode(self.host_message_mode));

        dst.push(ServerProperty::HostBannerMode(self.host_banner_mode));

        if let Some(host_banner_url) = self.host_banner_url {
            dst.push(ServerProperty::HostBannerUrl(host_banner_url));
        }

        if let Some(host_banner_gfx_url) = self.host_banner_gfx_url {
            dst.push(ServerProperty::HostBannerGfxUrl(host_banner_gfx_url));
        }

        dst.push(ServerProperty::HostBannerGfxInterval(self.host_banner_gfx_interval));

        if let Some(host_button_tooltip) = self.host_button_tooltip {
            dst.push(ServerProperty::HostButtonTooltip(host_button_tooltip));
        }

        if let Some(host_button_url) = self.host_button_url {
            dst.push(ServerProperty::HostButtonUrl(host_button_url));
        }

        if let Some(host_button_gfx_url) = self.host_button_gfx_url {
            dst.push(ServerProperty::HostButtonGfxUrl(host_button_gfx_url));
        }

        dst.push(ServerProperty::ComplainAutoBanCount(self.complain_autoban_count));
        dst.push(ServerProperty::ComplainAutoBanTime(self.complain_autoban_time));
        dst.push(ServerProperty::ComplainRemoveTime(self.complain_remove_time));

        dst.push(ServerProperty::MinClientsInChannelBeforeForcedSilence(
            self.min_clients_in_channel_before_forced_silence,
        ));
        dst.push(ServerProperty::PrioritySpeakerDimmModificator(
            self.priority_speaker_dimm_modificator,
        ));

        dst.push(ServerProperty::AntiFloodPointsTickReduce(
            self.antiflood_points_tick_reduce,
        ));
        dst.push(ServerProperty::AntiFloodPointsNeededCommandBlock(
            self.antiflood_points_needed_command_block,
        ));
        dst.push(ServerProperty::AntiFloodPointsNeededPluginBlock(
            self.antiflood_points_needed_plugin_block,
        ));
        dst.push(ServerProperty::AntiFloodPointsNeededIpBlock(
            self.antiflood_points_needed_ip_block,
        ));

        dst.push(ServerProperty::LogClient(self.log_client));
        dst.push(ServerProperty::LogQuery(self.log_query));
        dst.push(ServerProperty::LogChannel(self.log_channel));
        dst.push(ServerProperty::LogPermissions(self.log_permissions));
        dst.push(ServerProperty::LogServer(self.log_server));
        dst.push(ServerProperty::LogFileTransfer(self.log_file_transfer));

        dst.push(ServerProperty::DownloadQuota(self.download_quota));
        dst.push(ServerProperty::UploadQuota(self.upload_quota));

        dst.push(ServerProperty::MaxDownloadTotalBandwidth(
            self.max_download_total_bandwidth,
        ));
        dst.push(ServerProperty::MaxUploadTotalBandwidth(
            self.max_upload_total_bandwidth,
        ));

        dst
    }

    pub fn to_properties_vec(self) -> Vec<ServerProperty> {
        self.into_properties_vec(Vec::new())
    }
}
use crate::definitions::{CodecEncryptionMode, HostBannerMode, HostMessageMode, ServerStatus};
use crate::macros::{flag_builder, opt_builder};

opt_builder! {
    BanParams {
        ip(with_ip): String,
        name(with_name): String,
        uid(with_uid): String,
        my_teamspeak_id(with_my_teampeak_id): String,
        time(with_time): u64,
        reason(with_reason): String,
        last_nickname(with_last_nickname): String,
    }
}

opt_builder! {
    ServerEditParams {
        port(with_port): u16,

        min_client_version(with_min_client_version): String,
        min_android_version(with_min_android_version): String,
        min_ios_version(with_min_ios_version): String,

        name(with_name): String,
        name_phonetic(with_name_phonetic): String,
        welcome_message(with_welcome_message): String,
        icon_id(with_icon_id): u32,

        status(with_status): ServerStatus,
        auto_start(with_auto_start): bool,
        weblist_enabled(with_weblist_enabled): bool,

        password(with_password): String,

        max_clients(with_max_clients): i32,
        reserved_slots(with_reserved_slots): i32,

        needed_identity_security_level(with_needed_identity_security_level): u8,
        codec_encryption_mode(with_codec_encryption_mode): CodecEncryptionMode,

        default_server_group(with_default_server_group): i32,
        default_channel_group(with_default_channel_group): i32,
        default_channel_admin_group(with_default_channel_admin_group): i32,

        host_message(with_host_message): String,
        host_message_mode(with_host_message_mode): HostMessageMode,

        host_banner_mode(with_host_banner_mode): HostBannerMode,
        host_banner_url(with_host_banner_url): String,
        host_banner_gfx_url(with_host_banner_gfx_url): String,
        host_banner_gfx_interval(with_host_banner_gfx_interval): i32,

        host_button_tooltip(with_host_button_tooltip): String,
        host_button_gfx_url(with_host_button_gfx_url): String,
        host_button_url(with_host_button_url): String,

        complain_autoban_count(with_complain_autoban_count): i32,
        complain_autoban_time(with_complain_autoban_time): i32,
        complain_remove_time(with_complain_remove_time): i32,

        min_clients_in_channel_before_forced_silence(with_min_clients_in_channel_before_forced_silence): i32,
        priority_speaker_dimm_modificator(with_priority_speaker_dimm_modificator): f32,

        antiflood_points_tick_reduce(with_antiflood_points_tick_reduce): i32,
        antiflood_points_needed_command_block(with_antiflood_points_needed_command_block): i32,
        antiflood_points_needed_plugin_block(with_antiflood_points_needed_plugin_block): i32,
        anti_flood_points_needed_ip_block(with_anti_flood_points_needed_ip_block): i32,

        log_client(with_log_client): bool,
        log_query(with_log_query): bool,
        log_channel(with_log_channel): bool,
        log_permissions(with_log_permissions): bool,
        log_server(with_log_server): bool,
        log_file_transfer(with_log_file_transfer): bool,

        download_quota(with_download_quota): i64,
        upload_quota(with_upload_quota): i64,

        max_download_total_bandwidth(with_max_download_total_bandwidth): i64,
        max_upload_total_bandwidth(with_max_upload_total_bandwidth): i64,
    }
}

flag_builder! {
    ChannelListFlags {
        topic(with_topic),
        flags(with_flags),
        voice(with_voice),
        limits(with_limits),
        icon(with_icon),
        seconds_empty(with_seconds_empty),
        banners(with_banners),
    }
}

flag_builder! {
    ClientListFlags {
        uid(with_uid),
        away(with_away),
        voice(with_voice),
        times(with_times),
        groups(with_groups),
        info(with_info),
        country(with_country),
        ip(with_ip),
        icon(with_icon),
        badges(with_badges),
    }
}

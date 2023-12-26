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
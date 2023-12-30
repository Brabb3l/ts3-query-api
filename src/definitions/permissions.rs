use std::borrow::Cow;
use crate::error::ParseError;
use crate::macros::permissions;
use crate::parser::{CommandListBuilder, Decode, Decoder, Encode, EncodeList};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionValue {
    Int(i32),
    Bool(bool),
}

impl Encode for PermissionValue {
    fn encode(&self, buf: &mut String) -> std::fmt::Result {
        match self {
            PermissionValue::Int(val) => val.encode(buf),
            PermissionValue::Bool(val) => val.encode(buf),
        }
    }
}

pub struct PermissionPair<'a> {
    pub id: &'a str,
    pub value: PermissionValue,
}

permissions! {
    Permission {
        /* Retrieve information about ServerQuery commands */
        b_serverinstance_help_view: bool,

        /* Retrieve global server information */
        b_serverinstance_info_view: bool,

        /* List virtual servers stored in the database */
        b_serverinstance_virtualserver_list: bool,

        /* List active IP bindings on multi-homed machines */
        b_serverinstance_binding_list: bool,

        /* List permissions available available on the server instance */
        b_serverinstance_permission_list: bool,

        /* Search permission assignments by name or ID */
        b_serverinstance_permission_find: bool,

        /* Create virtual servers */
        b_virtualserver_create: bool,

        /* Delete virtual servers */
        b_virtualserver_delete: bool,

        /* Start any virtual server in the server instance */
        b_virtualserver_start_any: bool,

        /* Stop any virtual server in the server instance */
        b_virtualserver_stop_any: bool,

        /* Change a virtual servers machine ID */
        b_virtualserver_change_machine_id: bool,

        /* Edit virtual server default template values */
        b_virtualserver_change_template: bool,

        /* Login to ServerQuery */
        b_serverquery_login: bool,

        /* Create a new server query login */
        b_serverquery_login_create: bool,

        /* Delete a server query login */
        b_serverquery_login_delete: bool,

        /* List server query logins */
        b_serverquery_login_list: bool,

        /* Send text messages to all virtual servers at once */
        b_serverinstance_textmessage_send: bool,

        /* Retrieve global server log */
        b_serverinstance_log_view: bool,

        /* Write to global server log */
        b_serverinstance_log_add: bool,

        /* Shutdown the server process */
        b_serverinstance_stop: bool,

        /* Edit global settings */
        b_serverinstance_modify_settings: bool,

        /* Edit global ServerQuery groups */
        b_serverinstance_modify_querygroup: bool,

        /* Edit global template groups */
        b_serverinstance_modify_templates: bool,

        /* Select a virtual server */
        b_virtualserver_select: bool,

        /* Retrieve virtual server information */
        b_virtualserver_info_view: bool,

        /* Retrieve virtual server connection information */
        b_virtualserver_connectioninfo_view: bool,

        /* List channels on a virtual server */
        b_virtualserver_channel_list: bool,

        /* Search for channels on a virtual server */
        b_virtualserver_channel_search: bool,

        /* List clients online on a virtual server */
        b_virtualserver_client_list: bool,

        /* Search for clients online on a virtual server */
        b_virtualserver_client_search: bool,

        /* List client identities known by the virtual server */
        b_virtualserver_client_dblist: bool,

        /* Search for client identities known by the virtual server */
        b_virtualserver_client_dbsearch: bool,

        /* Retrieve client information */
        b_virtualserver_client_dbinfo: bool,

        /* Find permissions */
        b_virtualserver_permission_find: bool,

        /* Find custom fields */
        b_virtualserver_custom_search: bool,

        /* Start own virtual server */
        b_virtualserver_start: bool,

        /* Stop own virtual server */
        b_virtualserver_stop: bool,

        /* List privilege keys available */
        b_virtualserver_token_list: bool,

        /* Create new privilege keys */
        b_virtualserver_token_add: bool,

        /* Use a privilege keys to gain access to groups */
        b_virtualserver_token_use: bool,

        /* Delete a privilege key */
        b_virtualserver_token_delete: bool,

        /* Create a new API key */
        b_virtualserver_apikey_add: bool,

        /* Manage existing API keys */
        b_virtualserver_apikey_manage: bool,

        /* Retrieve virtual server log */
        b_virtualserver_log_view: bool,

        /* Write to virtual server log */
        b_virtualserver_log_add: bool,

        /* Join virtual server ignoring its password */
        b_virtualserver_join_ignore_password: bool,

        /* Register for server notifications */
        b_virtualserver_notify_register: bool,

        /* Unregister from server notifications */
        b_virtualserver_notify_unregister: bool,

        /* Create server snapshots */
        b_virtualserver_snapshot_create: bool,

        /* Deploy server snapshots */
        b_virtualserver_snapshot_deploy: bool,

        /* Reset the server permission settings to default values */
        b_virtualserver_permission_reset: bool,

        /* Modify server name */
        b_virtualserver_modify_name: bool,

        /* Modify welcome message */
        b_virtualserver_modify_welcomemessage: bool,

        /* Modify servers max clients */
        b_virtualserver_modify_maxclients: bool,

        /* Modify reserved slots */
        b_virtualserver_modify_reserved_slots: bool,

        /* Modify server password */
        b_virtualserver_modify_password: bool,

        /* Modify default Server Group */
        b_virtualserver_modify_default_servergroup: bool,

        /* Modify default Channel Group */
        b_virtualserver_modify_default_channelgroup: bool,

        /* Modify default Channel Admin Group */
        b_virtualserver_modify_default_channeladmingroup: bool,

        /* Modify channel force silence value */
        b_virtualserver_modify_channel_forced_silence: bool,

        /* Modify individual complain settings */
        b_virtualserver_modify_complain: bool,

        /* Modify individual antiflood settings */
        b_virtualserver_modify_antiflood: bool,

        /* Modify file transfer settings */
        b_virtualserver_modify_ft_settings: bool,

        /* Modify file transfer quotas */
        b_virtualserver_modify_ft_quotas: bool,

        /* Modify individual hostmessage settings */
        b_virtualserver_modify_hostmessage: bool,

        /* Modify individual hostbanner settings */
        b_virtualserver_modify_hostbanner: bool,

        /* Modify individual hostbutton settings */
        b_virtualserver_modify_hostbutton: bool,

        /* Modify server port */
        b_virtualserver_modify_port: bool,

        /* Modify server autostart */
        b_virtualserver_modify_autostart: bool,

        /* Modify required identity security level */
        b_virtualserver_modify_needed_identity_security_level: bool,

        /* Modify priority speaker dimm modificator */
        b_virtualserver_modify_priority_speaker_dimm_modificator: bool,

        /* Modify log settings */
        b_virtualserver_modify_log_settings: bool,

        /* Modify min client version */
        b_virtualserver_modify_min_client_version: bool,

        /* Modify server icon */
        b_virtualserver_modify_icon_id: bool,

        /* Modify web server list reporting settings */
        b_virtualserver_modify_weblist: bool,

        /* Modify codec encryption mode */
        b_virtualserver_modify_codec_encryption_mode: bool,

        /* Modify temporary serverpasswords */
        b_virtualserver_modify_temporary_passwords: bool,

        /* Modify own temporary serverpasswords */
        b_virtualserver_modify_temporary_passwords_own: bool,

        /* Modify default temporary channel delete delay */
        b_virtualserver_modify_channel_temp_delete_delay_default: bool,

        /* Modify server nicknames */
        b_virtualserver_modify_nickname: bool,

        /* Modify integrations */
        b_virtualserver_modify_integrations: bool,

        /* Min channel creation depth in hierarchy */
        i_channel_min_depth: i32,

        /* Max channel creation depth in hierarchy */
        i_channel_max_depth: i32,

        /* Stop inheritance of channel group permissions */
        b_channel_group_inheritance_end: bool,

        /* Modify channel permission power */
        i_channel_permission_modify_power: i32,

        /* Needed modify channel permission power */
        i_channel_needed_permission_modify_power: i32,

        /* Retrieve channel information */
        b_channel_info_view: bool,

        /* Create sub-channels */
        b_channel_create_child: bool,

        /* Create permanent channels */
        b_channel_create_permanent: bool,

        /* Create semi-permanent channels */
        b_channel_create_semi_permanent: bool,

        /* Create temporary channels */
        b_channel_create_temporary: bool,

        /* Create channels with a topic */
        b_channel_create_with_topic: bool,

        /* Create channels with a description */
        b_channel_create_with_description: bool,

        /* Create password protected channels */
        b_channel_create_with_password: bool,

        /* Create channel with a banner */
        b_channel_create_with_banner: bool,

        /* Create channels using OPUS (voice) codec */
        b_channel_create_modify_with_codec_opusvoice: bool,

        /* Create channels using OPUS (music) codec */
        b_channel_create_modify_with_codec_opusmusic: bool,

        /* Create channels with custom codec quality */
        i_channel_create_modify_with_codec_maxquality: i32,

        /* Create channels with minimal custom codec latency factor */
        i_channel_create_modify_with_codec_latency_factor_min: i32,

        /* Create channels with custom max clients */
        b_channel_create_with_maxclients: bool,

        /* Create channels with custom max family clients */
        b_channel_create_with_maxfamilyclients: bool,

        /* Create channels with custom sort order */
        b_channel_create_with_sortorder: bool,

        /* Create default channels */
        b_channel_create_with_default: bool,

        /* Create channels with needed talk power */
        b_channel_create_with_needed_talk_power: bool,

        /* Create new channels only with password */
        b_channel_create_modify_with_force_password: bool,

        /* Max delete delay for temporary channels */
        i_channel_create_modify_with_temp_delete_delay: i32,

        /* Move channels */
        b_channel_modify_parent: bool,

        /* Make channel default */
        b_channel_modify_make_default: bool,

        /* Make channel permanent */
        b_channel_modify_make_permanent: bool,

        /* Make channel semi-permanent */
        b_channel_modify_make_semi_permanent: bool,

        /* Make channel temporary */
        b_channel_modify_make_temporary: bool,

        /* Modify channel name */
        b_channel_modify_name: bool,

        /* Modify channel topic */
        b_channel_modify_topic: bool,

        /* Modify channel description */
        b_channel_modify_description: bool,

        /* Modify channel password */
        b_channel_modify_password: bool,

        /* Modify channel banner */
        b_channel_modify_banner: bool,

        /* Modify channel codec */
        b_channel_modify_codec: bool,

        /* Modify channel codec quality */
        b_channel_modify_codec_quality: bool,

        /* Modify channel codec latency factor */
        b_channel_modify_codec_latency_factor: bool,

        /* Modify channels max clients */
        b_channel_modify_maxclients: bool,

        /* Modify channels max family clients */
        b_channel_modify_maxfamilyclients: bool,

        /* Modify channel sort order */
        b_channel_modify_sortorder: bool,

        /* Change needed channel talk power */
        b_channel_modify_needed_talk_power: bool,

        /* Channel modify power */
        i_channel_modify_power: i32,

        /* Needed channel modify power */
        i_channel_needed_modify_power: i32,

        /* Make channel codec encrypted */
        b_channel_modify_make_codec_encrypted: bool,

        /* Modify temporary channel delete delay */
        b_channel_modify_temp_delete_delay: bool,

        /* Delete permanent channels */
        b_channel_delete_permanent: bool,

        /* Delete semi-permanent channels */
        b_channel_delete_semi_permanent: bool,

        /* Delete temporary channels */
        b_channel_delete_temporary: bool,

        /* Force channel delete */
        b_channel_delete_flag_force: bool,

        /* Delete channel power */
        i_channel_delete_power: i32,

        /* Needed delete channel power */
        i_channel_needed_delete_power: i32,

        /* Join permanent channels */
        b_channel_join_permanent: bool,

        /* Join semi-permanent channels */
        b_channel_join_semi_permanent: bool,

        /* Join temporary channels */
        b_channel_join_temporary: bool,

        /* Join channel ignoring its password */
        b_channel_join_ignore_password: bool,

        /* Ignore channels max clients limit */
        b_channel_join_ignore_maxclients: bool,

        /* Channel join power */
        i_channel_join_power: i32,

        /* Needed channel join power */
        i_channel_needed_join_power: i32,

        /* Channel subscribe power */
        i_channel_subscribe_power: i32,

        /* Needed channel subscribe power */
        i_channel_needed_subscribe_power: i32,

        /* Channel description view power */
        i_channel_description_view_power: i32,

        /* Needed channel needed description view power */
        i_channel_needed_description_view_power: i32,

        /* Group icon identifier */
        i_icon_id: i32,

        /* Max icon filesize in bytes */
        i_max_icon_filesize: i32,

        /* Enables icon management */
        b_icon_manage: bool,

        /* Group is permanent */
        b_group_is_permanent: bool,

        /* Group auto-update type */
        i_group_auto_update_type: i32,

        /* Group auto-update max value */
        i_group_auto_update_max_value: i32,

        /* Group sort id */
        i_group_sort_id: i32,

        /* Show group name in tree depending on selected mode */
        i_group_show_name_in_tree: i32,

        /* List server groups */
        b_virtualserver_servergroup_list: bool,

        /* List server group permissions */
        b_virtualserver_servergroup_permission_list: bool,

        /* List clients from a server group */
        b_virtualserver_servergroup_client_list: bool,

        /* List channel groups */
        b_virtualserver_channelgroup_list: bool,

        /* List channel group permissions */
        b_virtualserver_channelgroup_permission_list: bool,

        /* List clients from a channel group */
        b_virtualserver_channelgroup_client_list: bool,

        /* List client permissions */
        b_virtualserver_client_permission_list: bool,

        /* List channel permissions */
        b_virtualserver_channel_permission_list: bool,

        /* List channel client permissions */
        b_virtualserver_channelclient_permission_list: bool,

        /* Create server groups */
        b_virtualserver_servergroup_create: bool,

        /* Create channel groups */
        b_virtualserver_channelgroup_create: bool,

        /* Group modify power */
        i_group_modify_power: i32,

        /* Needed group modify power */
        i_group_needed_modify_power: i32,

        /* Group member add power */
        i_group_member_add_power: i32,

        /* Needed group member add power */
        i_group_needed_member_add_power: i32,

        /* Group member delete power */
        i_group_member_remove_power: i32,

        /* Needed group member delete power */
        i_group_needed_member_remove_power: i32,

        /* Permission modify power */
        i_permission_modify_power: i32,

        /* Ignore needed permission modify power */
        b_permission_modify_power_ignore: bool,

        /* Delete server groups */
        b_virtualserver_servergroup_delete: bool,

        /* Delete channel groups */
        b_virtualserver_channelgroup_delete: bool,

        /* Client permission modify power */
        i_client_permission_modify_power: i32,

        /* Needed client permission modify power */
        i_client_needed_permission_modify_power: i32,

        /* Max additional connections per client identity */
        i_client_max_clones_uid: i32,

        /* Max idle time in seconds */
        i_client_max_idletime: i32,

        /* Max avatar filesize in bytes */
        i_client_max_avatar_filesize: i32,

        /* Max channel subscriptions */
        i_client_max_channel_subscriptions: i32,

        /* Client is priority speaker */
        b_client_is_priority_speaker: bool,

        /* Ignore channel group permissions */
        b_client_skip_channelgroup_permissions: bool,

        /* Force Push-To-Talk capture mode */
        b_client_force_push_to_talk: bool,

        /* Ignore bans */
        b_client_ignore_bans: bool,

        /* Ignore antiflood measurements */
        b_client_ignore_antiflood: bool,

        /* Use an reserved slot */
        b_client_use_reserved_slot: bool,

        /* Use channel commander */
        b_client_use_channel_commander: bool,

        /* Allow to request talk power */
        b_client_request_talker: bool,

        /* Allow deletion of avatars from other clients */
        b_client_avatar_delete_other: bool,

        /* Client will be sticked to current channel */
        b_client_is_sticky: bool,

        /* Client ignores sticky flag */
        b_client_ignore_sticky: bool,

        /* Retrieve client information */
        b_client_info_view: bool,

        /* Retrieve client permissions overview */
        b_client_permissionoverview_view: bool,

        /* Retrieve clients own permissions overview */
        b_client_permissionoverview_own: bool,

        /* View client IP address and port */
        b_client_remoteaddress_view: bool,

        /* ServerQuery view power */
        i_client_serverquery_view_power: i32,

        /* Needed ServerQuery view power */
        i_client_needed_serverquery_view_power: i32,

        /* View custom fields */
        b_client_custom_info_view: bool,

        /* Client kick power from server */
        i_client_kick_from_server_power: i32,

        /* Needed client kick power from server */
        i_client_needed_kick_from_server_power: i32,

        /* Client kick power from channel */
        i_client_kick_from_channel_power: i32,

        /* Needed client kick power from channel */
        i_client_needed_kick_from_channel_power: i32,

        /* Client ban power */
        i_client_ban_power: i32,

        /* Needed client ban power */
        i_client_needed_ban_power: i32,

        /* Client move power */
        i_client_move_power: i32,

        /* Needed client move power */
        i_client_needed_move_power: i32,

        /* Complain power */
        i_client_complain_power: i32,

        /* Needed complain power */
        i_client_needed_complain_power: i32,

        /* Show complain list */
        b_client_complain_list: bool,

        /* Delete own complains */
        b_client_complain_delete_own: bool,

        /* Delete complains */
        b_client_complain_delete: bool,

        /* Show banlist */
        b_client_ban_list: bool,

        /* Add a ban */
        b_client_ban_create: bool,

        /* Delete own bans */
        b_client_ban_delete_own: bool,

        /* Delete bans */
        b_client_ban_delete: bool,

        /* Max bantime */
        i_client_ban_max_bantime: i32,

        /* Client private message power */
        i_client_private_textmessage_power: i32,

        /* Needed client private message power */
        i_client_needed_private_textmessage_power: i32,

        /* Send text messages to virtual server */
        b_client_server_textmessage_send: bool,

        /* Send text messages to channel */
        b_client_channel_textmessage_send: bool,

        /* Send offline messages to clients */
        b_client_offline_textmessage_send: bool,

        /* Client talk power */
        i_client_talk_power: i32,

        /* Needed client talk power */
        i_client_needed_talk_power: i32,

        /* Client poke power */
        i_client_poke_power: i32,

        /* Needed client poke power */
        i_client_needed_poke_power: i32,

        /* Set the talker flag for clients and allow them to speak */
        b_client_set_flag_talker: bool,

        /* Client whisper power */
        i_client_whisper_power: i32,

        /* Client needed whisper power */
        i_client_needed_whisper_power: i32,

        /* Edit a clients description */
        b_client_modify_description: bool,

        /* Allow client to edit own description */
        b_client_modify_own_description: bool,

        /* Edit a clients properties in the database */
        b_client_modify_dbproperties: bool,

        /* Delete a clients properties in the database */
        b_client_delete_dbproperties: bool,

        /* Create or modify own ServerQuery account */
        b_client_create_modify_serverquery_login: bool,

        /* Browse files without channel password */
        b_ft_ignore_password: bool,

        /* Retrieve list of running filetransfers */
        b_ft_transfer_list: bool,

        /* File upload power */
        i_ft_file_upload_power: i32,

        /* Needed file upload power */
        i_ft_needed_file_upload_power: i32,

        /* File download power */
        i_ft_file_download_power: i32,

        /* Needed file download power */
        i_ft_needed_file_download_power: i32,

        /* File delete power */
        i_ft_file_delete_power: i32,

        /* Needed file delete power */
        i_ft_needed_file_delete_power: i32,

        /* File rename power */
        i_ft_file_rename_power: i32,

        /* Needed file rename power */
        i_ft_needed_file_rename_power: i32,

        /* File browse power */
        i_ft_file_browse_power: i32,

        /* Needed file browse power */
        i_ft_needed_file_browse_power: i32,

        /* Create directory power */
        i_ft_directory_create_power: i32,

        /* Needed create directory power */
        i_ft_needed_directory_create_power: i32,

        /* Download quota per client in MByte */
        i_ft_quota_mb_download_per_client: i32,

        /* Upload quota per client in MByte */
        i_ft_quota_mb_upload_per_client: i32,
    }
}

impl<'a> Decode for Permission<'a> {
    fn decode(decoder: &mut Decoder) -> Result<Permission<'a>, ParseError> {
        let id: String = decoder.advance_or_err("permsid")?;
        let value: String = decoder.advance_or_err("permvalue")?;

        Ok(Self::parse(Cow::from(id), &value, false).unwrap())
    }
}

impl EncodeList for Permission<'_> {
    fn encode_list(&self, builder: &mut CommandListBuilder) -> Result<(), ParseError> {
        let pair = self.into_pair();

        builder.add("permsid", pair.id)?;
        builder.add("permvalue", pair.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contents() {
        let perm = Permission::b_serverinstance_help_view(true);
        let pair = perm.into_pair();

        assert_eq!(pair.id, "b_serverinstance_help_view");
        assert_eq!(pair.value, PermissionValue::Bool(true));
    }

    #[test]
    fn test_parse_i32() {
        let perm = Permission::parse(Cow::from("i_ft_quota_mb_upload_per_client"), "123", true).unwrap();

        assert_eq!(perm, Permission::i_ft_quota_mb_upload_per_client(123));
    }

    #[test]
    fn test_parse_bool() {
        let perm = Permission::parse(Cow::from("b_serverinstance_help_view"), "true", true).unwrap();

        assert_eq!(perm, Permission::b_serverinstance_help_view(true));
    }

    #[test]
    fn test_parse_unknown_i32() {
        let perm = Permission::parse(Cow::from("unknown"), "123", false);

        match perm {
            Ok(perm) => assert_eq!(perm, Permission::Custom(Cow::from("unknown"), PermissionValue::Int(123))),
            Err(e) => panic!("Expected Ok, got '{}'", e),
        }
    }

    #[test]
    fn test_parse_unknown_bool() {
        let perm = Permission::parse(Cow::from("unknown"), "true", false);

        match perm {
            Ok(perm) => assert_eq!(perm, Permission::Custom(Cow::from("unknown"), PermissionValue::Bool(true))),
            Err(e) => panic!("Expected Ok, got '{}'", e),
        }
    }

    #[test]
    fn test_parse_unknown_parse_error() {
        let perm = Permission::parse(Cow::from("unknown"), "hello", false);

        assert!(perm.is_err());
    }

    #[test]
    fn test_parse_unknown_error() {
        let perm = Permission::parse(Cow::from("unknown"), "true", true);

        assert!(perm.is_err());
    }
}
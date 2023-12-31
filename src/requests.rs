use std::borrow::Cow;
use crate::error::QueryError;
use crate::definitions::*;
use crate::definitions::builder::{BanParams, ChannelListFlags, ClientListFlags};
use crate::parser::Command;
use crate::QueryClient;

// TODO:
// [X] apikeyadd
// [X] apikeydel
// [X] apikeylist
// [X] banadd
// [X] banclient
// [X] bandel
// [ ] bandelall
// [X] banlist
// [ ] bindinglist
// [X] channeladdperm
// [ ] channelclientaddperm
// [ ] channelclientdelperm
// [ ] channelclientpermlist
// [X] channelcreate
// [X] channeldelete
// [ ] channeldelperm
// [X] channeledit
// [ ] channelfind
// [ ] channelgroupadd
// [ ] channelgroupaddperm
// [ ] channelgroupclientlist
// [ ] channelgroupcopy
// [ ] channelgroupdel
// [ ] channelgroupdelperm
// [ ] channelgrouplist
// [ ] channelgrouppermlist
// [ ] channelgrouprename
// [X] channelinfo
// [X] channellist
// [ ] channelmove
// [ ] channelpermlist
// [ ] clientaddperm
// [ ] clientaddservergroup
// [ ] clientdbdelete
// [ ] clientdbedit
// [ ] clientdbfind
// [ ] clientdbinfo
// [ ] clientdblist
// [ ] clientdelperm
// [ ] clientdelservergroup
// [ ] clientedit
// [ ] clientfind
// [ ] clientgetdbidfromuid
// [ ] clientgetids
// [ ] clientgetnamefromdbid
// [ ] clientgetnamefromuid
// [ ] clientgetuidfromclid
// [X] clientinfo
// [ ] clientkick
// [X] clientlist
// [X] clientmove
// [ ] clientpermlist
// [ ] clientpoke
// [ ] clientsetserverquerylogin
// [ ] clientupdate
// [ ] complainadd
// [ ] complaindel
// [ ] complaindelall
// [ ] complainlist
// [ ] customdelete
// [ ] custominfo
// [ ] customsearch
// [ ] customset
// [ ] ftcreatedir
// [ ] ftdeletefile
// [ ] ftgetfileinfo
// [ ] ftgetfilelist
// [ ] ftinitdownload
// [ ] ftinitupload
// [ ] ftlist
// [ ] ftrenamefile
// [ ] ftstop
// [X] gm
// [X] help
// [ ] hostinfo
// [ ] instanceedit
// [ ] instanceinfo
// [ ] logadd
// [X] login
// [X] logout
// [ ] logview
// [ ] messageadd
// [ ] messagedel
// [ ] messageget
// [ ] messagelist
// [ ] messageupdateflag
// [ ] permfind
// [ ] permget
// [ ] permidgetbyname
// [ ] permissionlist
// [ ] permoverview
// [ ] permreset
// [ ] privilegekeyadd
// [ ] privilegekeydelete
// [ ] privilegekeylist
// [ ] privilegekeyuse
// [ ] queryloginadd
// [ ] querylogindel
// [ ] queryloginlist
// [X] quit
// [ ] sendtextmessage
// [ ] servercreate
// [ ] serverdelete
// [ ] serveredit
// [ ] servergroupadd
// [ ] servergroupaddclient
// [ ] servergroupaddperm
// [ ] servergroupautoaddperm
// [ ] servergroupautodelperm
// [ ] servergroupclientlist
// [ ] servergroupcopy
// [ ] servergroupdel
// [ ] servergroupdelclient
// [ ] servergroupdelperm
// [ ] servergrouplist
// [ ] servergrouppermlist
// [ ] servergrouprename
// [ ] servergroupsbyclientid
// [ ] serveridgetbyport
// [ ] serverinfo
// [ ] serverlist
// [X] servernotifyregister
// [ ] servernotifyunregister
// [ ] serverprocessstop
// [ ] serverrequestconnectioninfo
// [ ] serversnapshotcreate
// [ ] serversnapshotdeploy
// [ ] serverstart
// [ ] serverstop
// [ ] servertemppasswordadd
// [ ] servertemppassworddel
// [ ] servertemppasswordlist
// [ ] setclientchannelgroup
// [ ] tokenadd
// [ ] tokendelete
// [ ] tokenlist
// [ ] tokenuse
// [X] use
// [X] version
// [X] whoami

#[allow(dead_code)]
impl QueryClient {

    pub async fn api_key_add(
        &self,
        scope: Scope,
        lifetime: Option<u32>,
        client_database_id: Option<u32>,
    ) -> Result<ApiKey, QueryError> {
        let command = Command::new("apikeyadd")
            .arg("scope", scope)?
            .arg_opt("lifetime", lifetime)?
            .arg_opt("cldbid", client_database_id)?;

        self.send_command(command).await
    }

    pub async fn api_key_delete(
        &self,
        id: u32,
    ) -> Result<(), QueryError> {
        let command = Command::new("apikeydel")
            .arg("id", id)?;

        self.send_command_no_response(command).await
    }

    pub async fn api_key_list(
        &self,
        client_database_id: Option<u32>,
        start: Option<u32>,
        duration: Option<u32>,
    ) -> Result<Vec<ApiKey>, QueryError> {
        self.api_key_list_into(client_database_id, start, duration, Vec::new()).await
    }

    pub async fn api_key_list_into(
        &self,
        client_database_id: Option<u32>,
        start: Option<u32>,
        duration: Option<u32>,
        dst: Vec<ApiKey>,
    ) -> Result<Vec<ApiKey>, QueryError> {
        let command = Command::new("apikeylist")
            .arg_opt("cldbid", client_database_id)?
            .arg_opt("start", start)?
            .arg_opt("duration", duration)?;

        self.send_command_into(command, dst).await
    }

    pub async fn ban_add(
        &self,
        param: BanParams,
    ) -> Result<BanId, QueryError> {
        let command = Command::new("banadd")
            .arg_opt("ip", param.ip)?
            .arg_opt("name", param.name)?
            .arg_opt("uid", param.uid)?
            .arg_opt("mytsid", param.my_teamspeak_id)?
            .arg_opt("time", param.time)?
            .arg_opt("banreason", param.reason)?
            .arg_opt("lastnickname", param.last_nickname)?;

        self.send_command(command).await
    }

    pub async fn ban_client(
        &self,
        client_id: &[u32],
        time: Option<u32>,
        reason: Option<&str>,
        continue_on_error: bool,
    ) -> Result<Vec<BanId>, QueryError> {
        self.ban_client_into(client_id, time, reason, continue_on_error, Vec::new()).await
    }

    pub async fn ban_client_into(
        &self,
        client_id: &[u32],
        time: Option<u32>,
        reason: Option<&str>,
        continue_on_error: bool,
        dst: Vec<BanId>,
    ) -> Result<Vec<BanId>, QueryError> {
        let command = Command::new("banclient")
            .arg_list("clid", client_id)?
            .arg_opt("time", time)?
            .arg_opt("banreason", reason)?
            .flag("continueonerror", continue_on_error);

        self.send_command_into(command, dst).await
    }

    pub async fn ban_delete(
        &self,
        ban_id: u32,
    ) -> Result<(), QueryError> {
        let command = Command::new("bandel")
            .arg("banid", ban_id)?;

        self.send_command_no_response(command).await
    }

    pub async fn ban_delete_all(&self) -> Result<(), QueryError> {
        let command = Command::new("bandelall");

        self.send_command_no_response(command).await
    }

    pub async fn ban_list(
        &self,
        start: Option<u32>,
        duration: Option<u32>,
    ) -> Result<Vec<Ban>, QueryError> {
        self.ban_list_into(start, duration, Vec::new()).await
    }

    pub async fn ban_list_into(
        &self,
        start: Option<u32>,
        duration: Option<u32>,
        dst: Vec<Ban>,
    ) -> Result<Vec<Ban>, QueryError> {
        let command = Command::new("banlist")
            .arg_opt("start", start)?
            .arg_opt("duration", duration)?;

        self.send_command_into(command, dst).await
    }

    // channel

    pub async fn channel_add_perm_id(
        &self,
        channel_id: u32,
        perms_id: u32,
        perms_value: u32,
    ) -> Result<(), QueryError> {
        let command = Command::new("channeladdperm")
            .arg("cid", channel_id)?
            .arg("permid", perms_id)?
            .arg("permvalue", perms_value)?;

        self.send_command_no_response(command).await
    }

    pub async fn channel_add_perm_multiple(
        &self,
        channel_id: u32,
        permissions: &[Permission],
    ) -> Result<(), QueryError> {
        let command = Command::new("channeladdperm")
            .arg("cid", channel_id)?
            .arg_multi_list(permissions)?;

        self.send_command_no_response(command).await
    }

    pub async fn channel_add_perm(
        &self,
        channel_id: u32,
        permission: &Permission,
    ) -> Result<(), QueryError> {
        let pair = permission.into_pair();

        let command = Command::new("channeladdperm")
            .arg("cid", channel_id)?
            .arg("permsid", pair.id)?
            .arg("permvalue", pair.value)?;

        self.send_command_no_response(command).await
    }

    pub async fn channel_create(
        &self,
        name: &str,
        properties: &[ChannelProperty]
    ) -> Result<u32, QueryError> {
        let mut command = Command::new("channelcreate")
            .arg("channel_name", name)?;

        for property in properties {
            let (key, value) = property.contents();

            command = command.arg(key.as_ref(), value)?;
        }

        self.send_command::<ChannelId>(command).await.map(|v| v.id)
    }

    pub async fn channel_delete(&self, channel_id: u32, force: bool) -> Result<(), QueryError> {
        let command = Command::new("channeldelete")
            .flag("force", force)
            .arg("cid", channel_id)?;

        self.send_command_no_response(command).await
    }

    pub async fn channel_edit(
        &self,
        channel_id: u32,
        properties: &[ChannelProperty],
    ) -> Result<(), QueryError> {
        let mut command = Command::new("channeledit")
            .arg("cid", channel_id)?;

        for property in properties {
            let (key, value) = property.contents();

            command = command.arg(key.as_ref(), value)?;
        }

        self.send_command_no_response(command).await
    }

    pub async fn channel_info(&self, id: u32) -> Result<ChannelInfo, QueryError> {
        let command = Command::new("channelinfo")
            .arg("cid", id)?;

        self.send_command(command).await
    }

    pub async fn channel_info_multiple(&self, ids: &[u32]) -> Result<Vec<ChannelInfo>, QueryError> {
        self.channel_info_multiple_into(ids, Vec::new()).await
    }

    pub async fn channel_info_multiple_into(
        &self,
        ids: &[u32],
        dst: Vec<ChannelInfo>
    ) -> Result<Vec<ChannelInfo>, QueryError> {
        let command = Command::new("channelinfo")
            .arg_list("cid", ids)?;

        self.send_command_into(command, dst).await
    }

    pub async fn channel_list_dynamic(
        &self,
        flags: ChannelListFlags,
    ) -> Result<Vec<ChannelListDynamicEntry>, QueryError> {
        self.channel_list_dynamic_into(
            flags,
            Vec::new()
        ).await
    }

    pub async fn channel_list_dynamic_into(
        &self,
        flags: ChannelListFlags,
        dst: Vec<ChannelListDynamicEntry>
    ) -> Result<Vec<ChannelListDynamicEntry>, QueryError> {
        let command = Command::new("channellist")
            .flag("topic", flags.topic)
            .flag("flags", flags.flags)
            .flag("voice", flags.voice)
            .flag("limits", flags.limits)
            .flag("icon", flags.icon)
            .flag("secondsempty", flags.seconds_empty)
            .flag("banners", flags.banners);

        self.send_command_custom_into(command, dst, |decoder| {
            ChannelListDynamicEntry::decode(decoder, &flags)
        }).await
    }

    pub async fn channel_list_full(&self) -> Result<Vec<ChannelListDynamicEntry>, QueryError> {
        self.channel_list_full_into(Vec::new()).await
    }

    pub async fn channel_list_full_into(
        &self,
        dst: Vec<ChannelListDynamicEntry>
    ) -> Result<Vec<ChannelListDynamicEntry>, QueryError> {
        self.channel_list_dynamic_into(ChannelListFlags::all(), dst).await
    }

    pub async fn channel_list(&self) -> Result<Vec<ChannelListEntry>, QueryError> {
        self.channel_list_into(Vec::new()).await
    }

    pub async fn channel_list_into(
        &self,
        dst: Vec<ChannelListEntry>
    ) -> Result<Vec<ChannelListEntry>, QueryError> {
        let command = Command::new("channellist");

        self.send_command_into(command, dst).await
    }

    pub async fn channel_perm_list(
        &self,
        channel_id: u32,
    ) -> Result<Vec<ChannelPermission>, QueryError> {
        self.channel_perm_list_into(channel_id, Vec::new()).await
    }

    pub async fn channel_perm_list_into(
        &self,
        channel_id: u32,
        dst: Vec<ChannelPermission>,
    ) -> Result<Vec<ChannelPermission>, QueryError> {
        let command = Command::new("channelpermlist")
            .arg("cid", channel_id)?
            .flag("permsid", true);

        self.send_command_into(command, dst).await
    }

    // client

    pub async fn client_info(&self, id: u32) -> Result<ClientInfo, QueryError> {
        let command = Command::new("clientinfo")
            .arg("clid", id)?;

        self.send_command(command).await
    }

    pub async fn client_info_multiple(&self, ids: &[u32]) -> Result<Vec<ClientInfo>, QueryError> {
        self.client_info_multiple_into(ids, Vec::new()).await
    }

    pub async fn client_info_multiple_into(
        &self,
        ids: &[u32],
        dst: Vec<ClientInfo>
    ) -> Result<Vec<ClientInfo>, QueryError> {
        let command = Command::new("clientinfo")
            .arg_list("clid", ids)?;

        self.send_command_into(command, dst).await
    }


    pub async fn client_list_dynamic(
        &self,
        flags: ClientListFlags,
    ) -> Result<Vec<ClientListDynamicEntry>, QueryError> {
        self.client_list_dynamic_into(
            flags,
            Vec::new()
        ).await
    }

    pub async fn client_list_dynamic_into(
        &self,
        flags: ClientListFlags,
        dst: Vec<ClientListDynamicEntry>
    ) -> Result<Vec<ClientListDynamicEntry>, QueryError> {
        let command = Command::new("clientlist")
            .flag("uid", flags.uid)
            .flag("away", flags.away)
            .flag("voice", flags.voice)
            .flag("times", flags.times)
            .flag("groups", flags.groups)
            .flag("info", flags.info)
            .flag("country", flags.country)
            .flag("ip", flags.ip)
            .flag("icon", flags.icon)
            .flag("badges", flags.badges);

        self.send_command_custom_into(command, dst, |decoder| {
            ClientListDynamicEntry::decode(decoder, &flags)
        }).await
    }

    pub async fn client_list_full(&self) -> Result<Vec<ClientListDynamicEntry>, QueryError> {
        self.client_list_full_into(Vec::new()).await
    }

    pub async fn client_list_full_into(
        &self,
        dst: Vec<ClientListDynamicEntry>
    ) -> Result<Vec<ClientListDynamicEntry>, QueryError> {
        self.client_list_dynamic_into(ClientListFlags::all(), dst).await
    }

    pub async fn client_list(&self) -> Result<Vec<ClientListEntry>, QueryError> {
        self.client_list_into(Vec::new()).await
    }

    pub async fn client_list_into(
        &self,
        dst: Vec<ClientListEntry>
    ) -> Result<Vec<ClientListEntry>, QueryError> {
        let command = Command::new("clientlist");

        self.send_command_into(command, dst).await
    }

    pub async fn client_move(
        &self,
        client_ids: &[u32],
        channel_id: u32,
        password: Option<&str>,
        continue_on_error: bool,
    ) -> Result<(), QueryError> {
        let command = Command::new("clientmove")
            .flag("continueonerror", continue_on_error)
            .arg_list("clid", client_ids)?
            .arg_opt("cpw", password)?
            .arg("cid", channel_id)?;

        self.send_command_no_response(command).await
    }

    pub async fn gm(&self, msg: &str) -> Result<(), QueryError> {
        let command = Command::new("gm")
            .arg("msg", msg)?;

        self.send_command_no_response(command).await
    }

    pub async fn help(&self) -> Result<String, QueryError> {
        let command = Command::new("help");
        let response = self.send_command_raw(command).await?;
        let response = response.content();

        Ok(String::from_utf8(Vec::from(response))?)
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), QueryError> {
        let command = Command::new("login")
            .arg("client_login_name", username)?
            .arg("client_login_password", password)?;

        self.send_command_no_response(command).await
    }

    pub async fn logout(&self) -> Result<(), QueryError> {
        let command = Command::new("logout");

        self.send_command_no_response(command).await
    }

    pub async fn quit(&self) -> Result<(), QueryError> {
        let command = Command::new("quit");

        self.send_command_no_response(command).await
    }

    pub async fn server_notify_register(
        &self,
        event: EventType,
    ) -> Result<(), QueryError> {
        match event {
            EventType::Channel | EventType::TextChannel => {
                self.server_notify_register_channel(event, 0).await
            }
            _ => {
                let command = Command::new("servernotifyregister")
                    .arg("event", event)?;

                self.send_command_no_response(command).await
            }
        }
    }

    pub async fn server_notify_register_channel(
        &self,
        event: EventType,
        channel_id: u32,
    ) -> Result<(), QueryError> {
        match event {
            EventType::Channel | EventType::TextChannel => {
                let command = Command::new("servernotifyregister")
                    .arg("event", event)?
                    .arg("id", channel_id)?;

                self.send_command_no_response(command).await
            },
            _ => {
                Err(QueryError::InvalidArgument {
                    name: "event",
                    message: Cow::from("Must be EventType::Channel or EventType::TextChannel"),
                })
            }
        }
    }

    pub async fn server_notify_register_all(&self) -> Result<(), QueryError> {
        self.server_notify_register(EventType::Server).await?;
        self.server_notify_register(EventType::Channel).await?;
        self.server_notify_register(EventType::TextChannel).await?;
        self.server_notify_register(EventType::TextPrivate).await?;
        self.server_notify_register(EventType::TextServer).await?;
        self.server_notify_register(EventType::TokenUsed).await?;

        Ok(())
    }

    pub async fn use_sid(&self, sid: u32) -> Result<(), QueryError> {
        let command = Command::new("use")
            .arg("sid", sid)?;

        self.send_command_no_response(command).await
    }

    pub async fn use_port(&self, port: u16) -> Result<(), QueryError> {
        let command = Command::new("use")
            .arg("port", port)?;

        self.send_command_no_response(command).await
    }

    pub async fn version(&self) -> Result<Version, QueryError> {
        let command = Command::new("version");

        self.send_command(command).await
    }

    pub async fn who_am_i(&self) -> Result<WhoAmI, QueryError> {
        let command = Command::new("whoami");

        self.send_command(command).await
    }
}

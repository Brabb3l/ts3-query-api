use crate::error::QueryError;
use crate::definitions::*;
use crate::definitions::builder::{BanParams, ChannelListFlags, ClientListFlags};
use crate::parser::{Command, CommandResponse};
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

        let mut response = self.send_command_decode(command).await?;

        ApiKey::from(&mut response)
    }

    pub async fn api_key_delete(
        &self,
        id: u32,
    ) -> Result<(), QueryError> {
        let command = Command::new("apikeydel")
            .arg("id", id)?;

        self.send_command(command).await?;

        Ok(())
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
        mut dst: Vec<ApiKey>,
    ) -> Result<Vec<ApiKey>, QueryError> {
        let command = Command::new("apikeylist")
            .arg_opt("cldbid", client_database_id)?
            .arg_opt("start", start)?
            .arg_opt("duration", duration)?;

        for mut response in self.send_command_multi_decode(command).await? {
            dst.push(ApiKey::from(&mut response)?);
        }

        Ok(dst)
    }

    pub async fn ban_add(
        &self,
        param: BanParams,
    ) -> Result<u32, QueryError> {
        let command = Command::new("banadd")
            .arg_opt("ip", param.ip)?
            .arg_opt("name", param.name)?
            .arg_opt("uid", param.uid)?
            .arg_opt("mytsid", param.my_teamspeak_id)?
            .arg_opt("time", param.time)?
            .arg_opt("banreason", param.reason)?
            .arg_opt("lastnickname", param.last_nickname)?;

        let mut response = self.send_command_decode(command).await?;

        response.get("banid")
    }

    pub async fn ban_client(
        &self,
        client_id: &[u32],
        time: Option<u32>,
        reason: Option<&str>,
        continue_on_error: bool,
    ) -> Result<Vec<u32>, QueryError> {
        self.ban_client_with(client_id, time, reason, continue_on_error, Vec::new()).await
    }

    pub async fn ban_client_with(
        &self,
        client_id: &[u32],
        time: Option<u32>,
        reason: Option<&str>,
        continue_on_error: bool,
        mut dst: Vec<u32>,
    ) -> Result<Vec<u32>, QueryError> {
        let command = Command::new("banclient")
            .arg_list("clid", client_id)?
            .arg_opt("time", time)?
            .arg_opt("banreason", reason)?
            .flag("continueonerror", continue_on_error);

        let response = self.send_command(command).await?;

        for response in response.split("\n\r") {
            let mut response = CommandResponse::decode(response, false)?;

            dst.push(response.get("banid")?);
        }

        Ok(dst)
    }

    pub async fn ban_delete(
        &self,
        ban_id: u32,
    ) -> Result<(), QueryError> {
        let command = Command::new("bandel")
            .arg("banid", ban_id)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn ban_delete_all(&self) -> Result<(), QueryError> {
        let command = Command::new("bandelall");

        self.send_command(command).await?;

        Ok(())
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
        mut dst: Vec<Ban>,
    ) -> Result<Vec<Ban>, QueryError> {
        let command = Command::new("banlist")
            .arg_opt("start", start)?
            .arg_opt("duration", duration)?;

        for mut response in self.send_command_multi_decode(command).await? {
            dst.push(Ban::from(&mut response)?);
        }

        Ok(dst)
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

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn channel_add_perm_multiple(
        &self,
        channel_id: u32,
        permissions: &[Permission<'_>],
    ) -> Result<(), QueryError> {
        let command = Command::new("channeladdperm")
            .arg("cid", channel_id)?
            .arg_multi_list(permissions)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn channel_add_perm(
        &self,
        channel_id: u32,
        permission: &Permission<'_>,
    ) -> Result<(), QueryError> {
        let (id, value) = permission.contents();
        let command = Command::new("channeladdperm")
            .arg("cid", channel_id)?
            .arg("permsid", id)?
            .arg("permvalue", value)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn channel_create(
        &self,
        name: &str,
        properties: &[ChannelProperty<'_>]
    ) -> Result<u32, QueryError> {
        let mut command = Command::new("channelcreate")
            .arg("channel_name", name)?;

        for property in properties {
            let (key, value) = property.contents();

            command = command.arg(key, value)?;
        }

        let mut response = self.send_command_decode(command).await?;

        response.get("cid")
    }

    pub async fn channel_delete(&self, channel_id: u32, force: bool) -> Result<(), QueryError> {
        let command = Command::new("channeldelete")
            .flag("force", force)
            .arg("cid", channel_id)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn channel_edit(
        &self,
        channel_id: u32,
        properties: &[ChannelProperty<'_>],
    ) -> Result<(), QueryError> {
        let mut command = Command::new("channeledit")
            .arg("cid", channel_id)?;

        for property in properties {
            let (key, value) = property.contents();

            command = command.arg(key, value)?;
        }

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn channel_info(&self, id: u32) -> Result<ChannelInfo, QueryError> {
        let command = Command::new("channelinfo")
            .arg("cid", id)?;

        let mut response = self.send_command_decode(command).await?;

        ChannelInfo::from(&mut response)
    }

    pub async fn channel_info_multiple(&self, ids: &[u32]) -> Result<Vec<ChannelInfo>, QueryError> {
        self.channel_info_multiple_into(ids, Vec::new()).await
    }

    pub async fn channel_info_multiple_into(
        &self,
        ids: &[u32],
        mut dst: Vec<ChannelInfo>
    ) -> Result<Vec<ChannelInfo>, QueryError> {
        let command = Command::new("channelinfo")
            .arg_list("cid", ids)?;

        for mut response in self.send_command_multi_decode(command).await? {
            dst.push(ChannelInfo::from(&mut response)?);
        }

        Ok(dst)
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
        mut dst: Vec<ChannelListDynamicEntry>
    ) -> Result<Vec<ChannelListDynamicEntry>, QueryError> {
        let command = Command::new("channellist")
            .flag("topic", flags.topic)
            .flag("flags", flags.flags)
            .flag("voice", flags.voice)
            .flag("limits", flags.limits)
            .flag("icon", flags.icon)
            .flag("secondsempty", flags.seconds_empty)
            .flag("banners", flags.banners);

        for mut response in self.send_command_multi_decode(command).await? {
            let base = ChannelListEntry::from(&mut response)?;
            let topic = if flags.topic { Some(ChannelListTopicEntry::from(&mut response)?) } else { None };
            let flags2 = if flags.flags { Some(ChannelListFlagsEntry::from(&mut response)?) } else { None };
            let voice = if flags.voice { Some(ChannelListVoiceEntry::from(&mut response)?) } else { None };
            let limits = if flags.limits { Some(ChannelListLimitsEntry::from(&mut response)?) } else { None };
            let icon = if flags.icon { Some(ChannelListIconEntry::from(&mut response)?) } else { None };
            let seconds_empty = if flags.seconds_empty { Some(ChannelListSecondsEmptyEntry::from(&mut response)?) } else { None };
            let banners = if flags.banners { Some(ChannelListBannerEntry::from(&mut response)?) } else { None };

            dst.push(ChannelListDynamicEntry {
                base,
                topic,
                flags: flags2,
                voice,
                limits,
                icon,
                seconds_empty,
                banners,
            });
        }

        Ok(dst)
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
        let mut channels = dst;

        for mut response in self.send_command_multi_decode(command).await? {
            channels.push(ChannelListEntry::from(&mut response)?);
        }

        Ok(channels)
    }

    // client

    pub async fn client_info(&self, id: u32) -> Result<ClientInfo, QueryError> {
        let command = Command::new("clientinfo")
            .arg("clid", id)?;

        let mut response = self.send_command_decode(command).await?;

        ClientInfo::from(&mut response)
    }

    pub async fn client_info_multiple(&self, ids: &[u32]) -> Result<Vec<ClientInfo>, QueryError> {
        self.client_info_multiple_into(ids, Vec::new()).await
    }

    pub async fn client_info_multiple_into(
        &self,
        ids: &[u32],
        mut dst: Vec<ClientInfo>
    ) -> Result<Vec<ClientInfo>, QueryError> {
        let command = Command::new("clientinfo")
            .arg_list("clid", ids)?;

        for mut response in self.send_command_multi_decode(command).await? {
            dst.push(ClientInfo::from(&mut response)?);
        }

        Ok(dst)
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
        mut dst: Vec<ClientListDynamicEntry>
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

        for mut response in self.send_command_multi_decode(command).await? {
            let base = ClientListEntry::from(&mut response)?;
            let uid = if flags.uid { Some(ClientListUidEntry::from(&mut response)?) } else { None };
            let away = if flags.away { Some(ClientListAwayEntry::from(&mut response)?) } else { None };
            let voice = if flags.voice { Some(ClientListVoiceEntry::from(&mut response)?) } else { None };
            let times = if flags.times { Some(ClientListTimesEntry::from(&mut response)?) } else { None };
            let groups = if flags.groups { Some(ClientListGroupsEntry::from(&mut response)?) } else { None };
            let info = if flags.info { Some(ClientListInfoEntry::from(&mut response)?) } else { None };
            let country = if flags.country { Some(ClientListCountryEntry::from(&mut response)?) } else { None };
            let ip = if flags.ip { Some(ClientListIpEntry::from(&mut response)?) } else { None };
            let icon = if flags.icon { Some(ClientListIconEntry::from(&mut response)?) } else { None };
            let badges = if flags.badges { Some(ClientListBadgesEntry::from(&mut response)?) } else { None };

            dst.push(ClientListDynamicEntry {
                base,
                uid,
                away,
                voice,
                times,
                groups,
                info,
                country,
                ip,
                icon,
                badges,
            });
        }

        Ok(dst)
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
        mut dst: Vec<ClientListEntry>
    ) -> Result<Vec<ClientListEntry>, QueryError> {
        let command = Command::new("clientlist");

        for mut response in self.send_command_multi_decode(command).await? {
            dst.push(ClientListEntry::from(&mut response)?);
        }

        Ok(dst)
    }

    pub async fn client_move(
        &self,
        client_ids: &[u32],
        channel_id: u32,
        password: Option<&str>,
        continue_on_error: bool,
    ) -> Result<(), QueryError> {
        let mut command = Command::new("clientmove")
            .flag("continueonerror", continue_on_error)
            .arg_list("clid", client_ids)?
            .arg("cid", channel_id)?;

        if let Some(password) = password {
            command = command.arg("cpw", password)?;
        }

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn gm(&self, msg: &str) -> Result<(), QueryError> {
        let command = Command::new("gm")
            .arg("msg", msg)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn help(&self) -> Result<String, QueryError> {
        let command = Command::new("help");
        let response = self.send_command(command).await?;

        Ok(response)
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), QueryError> {
        let command = Command::new("login")
            .arg("client_login_name", username)?
            .arg("client_login_password", password)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn logout(&self) -> Result<(), QueryError> {
        let command = Command::new("logout");

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn quit(&self) -> Result<(), QueryError> {
        let command = Command::new("quit");

        self.send_command(command).await?;

        Ok(())
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

                self.send_command(command).await?;

                Ok(())
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

                self.send_command(command).await?;

                Ok(())
            },
            _ => {
                Err(QueryError::InvalidArgument {
                    name: "event".to_owned(),
                    message: "Must be EventType::Channel or EventType::TextChannel".to_owned(),
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

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn use_port(&self, port: u16) -> Result<(), QueryError> {
        let command = Command::new("use")
            .arg("port", port)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn version(&self) -> Result<Version, QueryError> {
        let command = Command::new("version");
        let mut response = self.send_command_decode(command).await?;

        Version::from(&mut response)
    }

    pub async fn who_am_i(&self) -> Result<WhoAmI, QueryError> {
        let command = Command::new("whoami");
        let mut response = self.send_command_decode(command).await?;

        WhoAmI::from(&mut response)
    }
}

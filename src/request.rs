use crate::client::QueryClient;
use crate::error::QueryError;
use crate::responses::{ChannelInfo, ChannelListBannerEntry, ChannelListFlagsEntry, ChannelListDynamicEntry, ChannelListIconEntry, ChannelListEntry, ChannelListLimitsEntry, ChannelListSecondsEmptyEntry, ChannelListTopicEntry, ChannelListVoiceEntry, ClientListAwayEntry, ClientListDynamicEntry, ClientListGroupsEntry, ClientListEntry, ClientListTimesEntry, ClientListUidEntry, ClientListVoiceEntry, Version, ClientListInfoEntry, ClientListCountryEntry, ClientListIpEntry, ClientListIconEntry, ClientListBadgesEntry, ClientInfo, WhoAmI};
use crate::parser::Command;
use crate::properties::ChannelProperty;

// TODO:
// [ ] apikeyadd
// [ ] apikeydel
// [ ] apikeylist
// [ ] banadd
// [ ] banclient
// [ ] bandel
// [ ] bandelall
// [ ] banlist
// [ ] bindinglist
// [X] channeladdperm
// [ ] channelclientaddperm
// [ ] channelclientdelperm
// [ ] channelclientpermlist
// [X] channelcreate
// [X] channeldelete
// [ ] channeldelperm
// [ ] channeledit
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
// [ ] gm
// [ ] help
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
// [ ] servernotifyregister
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
    pub async fn quit(&self) -> Result<(), QueryError> {
        let command = Command::new("quit");

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), QueryError> {
        let command = Command::new("login")
            .key_val_str("client_login_name", username)
            .key_val_str("client_login_password", password);

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn logout(&self) -> Result<(), QueryError> {
        let command = Command::new("logout");

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

    pub async fn use_sid(&self, sid: i32) -> Result<(), QueryError> {
        let command = Command::new("use")
            .key_val_i32("sid", sid)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn use_port(&self, port: u16) -> Result<(), QueryError> {
        let command = Command::new("use")
            .key_val_i32("port", port.into())?;

        self.send_command(command).await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn channel_list_dynamic(
        &self,
        topic: bool,
        flags: bool,
        voice: bool,
        limits: bool,
        icon: bool,
        seconds_empty: bool,
        banners: bool,
    ) -> Result<Vec<ChannelListDynamicEntry>, QueryError> {
        let command = Command::new("channellist")
            .flag("topic", topic)
            .flag("flags", flags)
            .flag("voice", voice)
            .flag("limits", limits)
            .flag("icon", icon)
            .flag("secondsempty", seconds_empty)
            .flag("banners", banners);

        let mut channels = Vec::new();

        for mut response in self.send_command_multi_decode(command).await? {
            let base = ChannelListEntry::from(&mut response)?;
            let topic = if topic { Some(ChannelListTopicEntry::from(&mut response)?) } else { None };
            let flags = if flags { Some(ChannelListFlagsEntry::from(&mut response)?) } else { None };
            let voice = if voice { Some(ChannelListVoiceEntry::from(&mut response)?) } else { None };
            let limits = if limits { Some(ChannelListLimitsEntry::from(&mut response)?) } else { None };
            let icon = if icon { Some(ChannelListIconEntry::from(&mut response)?) } else { None };
            let seconds_empty = if seconds_empty { Some(ChannelListSecondsEmptyEntry::from(&mut response)?) } else { None };
            let banners = if banners { Some(ChannelListBannerEntry::from(&mut response)?) } else { None };

            channels.push(ChannelListDynamicEntry {
                base,
                topic,
                flags,
                voice,
                limits,
                icon,
                seconds_empty,
                banners,
            });
        }

        Ok(channels)
    }

    pub async fn channel_list_full(&self) -> Result<Vec<ChannelListDynamicEntry>, QueryError> {
        self.channel_list_dynamic(true, true, true, false, true, true, true).await
    }

    pub async fn channel_list(&self) -> Result<Vec<ChannelListEntry>, QueryError> {
        let command = Command::new("channellist");
        let mut channels = Vec::new();

        for mut response in self.send_command_multi_decode(command).await? {
            channels.push(ChannelListEntry::from(&mut response)?);
        }

        Ok(channels)
    }

    pub async fn channel_info(&self, id: i32) -> Result<ChannelInfo, QueryError> {
        let command = Command::new("channelinfo")
            .key_val_i32("cid", id)?;

        let mut response = self.send_command_decode(command).await?;

        ChannelInfo::from(&mut response)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn client_list_dynamic(
        &self,
        uid: bool,
        away: bool,
        voice: bool,
        times: bool,
        groups: bool,
        info: bool,
        country: bool,
        ip: bool,
        icon: bool,
        badges: bool,
    ) -> Result<Vec<ClientListDynamicEntry>, QueryError> {
        let command = Command::new("clientlist")
            .flag("uid", uid)
            .flag("away", away)
            .flag("voice", voice)
            .flag("times", times)
            .flag("groups", groups)
            .flag("info", info)
            .flag("country", country)
            .flag("ip", ip)
            .flag("icon", icon)
            .flag("badges", badges);

        let mut clients = Vec::new();

        for mut response in self.send_command_multi_decode(command).await? {
            let base = ClientListEntry::from(&mut response)?;
            let uid = if uid { Some(ClientListUidEntry::from(&mut response)?) } else { None };
            let away = if away { Some(ClientListAwayEntry::from(&mut response)?) } else { None };
            let voice = if voice { Some(ClientListVoiceEntry::from(&mut response)?) } else { None };
            let times = if times { Some(ClientListTimesEntry::from(&mut response)?) } else { None };
            let groups = if groups { Some(ClientListGroupsEntry::from(&mut response)?) } else { None };
            let info = if info { Some(ClientListInfoEntry::from(&mut response)?) } else { None };
            let country = if country { Some(ClientListCountryEntry::from(&mut response)?) } else { None };
            let ip = if ip { Some(ClientListIpEntry::from(&mut response)?) } else { None };
            let icon = if icon { Some(ClientListIconEntry::from(&mut response)?) } else { None };
            let badges = if badges { Some(ClientListBadgesEntry::from(&mut response)?) } else { None };

            clients.push(ClientListDynamicEntry {
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

        Ok(clients)
    }

    pub async fn client_list_full(&self) -> Result<Vec<ClientListDynamicEntry>, QueryError> {
        self.client_list_dynamic(true, true, true, true, true, true, true, true, true, true).await
    }

    pub async fn client_list(&self) -> Result<Vec<ClientListEntry>, QueryError> {
        let command = Command::new("clientlist");
        let mut clients = Vec::new();

        for mut response in self.send_command_multi_decode(command).await? {
            clients.push(ClientListEntry::from(&mut response)?);
        }

        Ok(clients)
    }

    pub async fn client_info(&self, id: i32) -> Result<ClientInfo, QueryError> {
        let command = Command::new("clientinfo")
            .key_val_i32("clid", id)?;

        let mut response = self.send_command_decode(command).await?;

        ClientInfo::from(&mut response)
    }

    pub async fn client_info_multiple(&self, ids: &[i32]) -> Result<Vec<ClientInfo>, QueryError> {
        let command = Command::new("clientinfo")
            .key_val_i32_list("clid", ids)?;

        let mut clients = Vec::new();

        for mut response in self.send_command_multi_decode(command).await? {
            clients.push(ClientInfo::from(&mut response)?);
        }

        Ok(clients)
    }

    pub async fn channel_create(
        &self,
        name: &str,
        properties: Vec<ChannelProperty<'_>>
    ) -> Result<(), QueryError> {
        let mut command = Command::new("channelcreate")
            .key_val_str("channel_name", name);

        for property in properties {
            let (key, value) = property.contents();

            command = command.key_val_property(key, value)?;
        }

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn client_move(
        &self,
        client_ids: &[i32],
        channel_id: i32,
        password: Option<&str>,
        continue_on_error: bool,
    ) -> Result<(), QueryError> {
        let mut command = Command::new("clientmove")
            .flag("continueonerror", continue_on_error)
            .key_val_i32_list("clid", client_ids)?
            .key_val_i32("cid", channel_id)?;

        if let Some(password) = password {
            command = command.key_val_str("cpw", password);
        }

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn channel_delete(&self, channel_id: i32, force: bool) -> Result<(), QueryError> {
        let command = Command::new("channeldelete")
            .flag("force", force)
            .key_val_i32("cid", channel_id)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn channel_edit(
        &self,
        channel_id: i32,
        properties: Vec<ChannelProperty<'_>>,
    ) -> Result<(), QueryError> {
        let mut command = Command::new("channeledit")
            .key_val_i32("cid", channel_id)?;

        for property in properties {
            let (key, value) = property.contents();

            command = command.key_val_property(key, value)?;
        }

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn channel_add_perm_id(
        &self,
        channel_id: i32,
        perms_id: i32,
        perms_value: i32,
    ) -> Result<(), QueryError> {
        let command = Command::new("channeladdperm")
            .key_val_i32("cid", channel_id)?
            .key_val_i32("permid", perms_id)?
            .key_val_i32("permvalue", perms_value)?;

        self.send_command(command).await?;

        Ok(())
    }

    pub async fn channel_add_perm_sid(
        &self,
        channel_id: i32,
        perms_id: &str,
        perms_value: i32,
    ) -> Result<(), QueryError> {
        let command = Command::new("channeladdperm")
            .key_val_i32("cid", channel_id)?
            .key_val_str("permsid", perms_id)
            .key_val_i32("permvalue", perms_value)?;

        self.send_command(command).await?;

        Ok(())
    }
}

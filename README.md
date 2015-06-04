# nl80211rs

This is a conversion of the 802.11 netlink interface public header file into a rust module. The style has been changed to CamelCase and the prefix to the enum value is removed. The reasoning is that the prefix is redundant given module namespaces.

eg.

`nl80211_commands::NL80211_CMD_START_AP`

Becomes

`nl80211::Commands::StartAp`


#### Work in progress

### Complete:
* Commands enum (Separate alias::Commands for duplicate command aliases in the enum (not ideal))
* Attr enum
* Interface types enum
* Station Flags enum,
* Station Flag update struct,
* Bitrate info enum,
* Station Bss Information enum,
* Mesh path flag enum,
* Mesh path Infor enum,
* Band attributes enum,
* Freqency attributes enum,
* Bitrate attribute enum
* Initiator of reg domain requests enum
* Regulatory domain type enum
* Regulatory rule attributes enum
* Schedule scan attributes


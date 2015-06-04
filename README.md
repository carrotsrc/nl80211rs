# nl80211rs

This is a conversion of the 802.11 netlink interface public header file into a rust module. The style has been changed to CamelCase and the prefix to the enum value is removed. The reasoning is that the prefix is redundant given module namespaces.

eg.

`nl80211_commands::NL80211_CMD_START_AP`

Becomes

`nl80211::Commands::StartAp`


### Progress

Included all enums and structures in the header. They are near 1:1 with a few exceptions.

Also included a range of constants


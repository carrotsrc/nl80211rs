# nl80211rs

This is a hand conversion of the 802.11 netlink interface public header file into a rust module. The style has been changed to conventional CamelCase and the prefix to the enum and consts values is removed; reasoning being that the prefix is redundant given crate and module namespaces.

`crate::enum::CamelCaseUid`

eg.

`nl80211_commands::NL80211_CMD_START_AP`

Becomes

`nl80211::Commands::StartAp`



The constants are held in their own module `nl80211::constants`. Constants are almost identical to the prepocessor directives, except they are not prefixed with NL80211_.


### Progress

* Included all enums and structures in the header. They are near 1:1 with a few exceptions; one being aliased commands in nl80211::Commands, which have prefix Alias, where their real values can be resolved in nl80211::alias::Commands. It is a bit of a short term hack.

* Included a range of constants, but not all define directives have been included.

* Comments are sparse-to-non-existant.


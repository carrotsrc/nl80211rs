# nl80211rs

This is a conversion of the 802.11 netlink interface public header file into a rust module. The style has been changed to CamelCase and the prefix to the enum value is removed. The reasoning is that the prefix is redundant given module namespaces.

eg.

```nl80211_commands::START_AP```

Becomes

```nl80211::Commands::StartAp```


#### Work in progress

### Complete:
* nl80211 commands enum (nl80211::Commands::{command})

**Work in Progress**

Current state:

1. Deserialization of relevant `plist` and contained structures is complete(?) and formatted is [mostly-documented](doc/plist_info.md)
2. Builds, few present tests pass (subject to inclusion of the data file which I may [or may not] want to ~sanitize, but you can do `cp ~/Library/Containers/com.apple.notificationcenterui/Data/Library/Preferences/com.apple.notificationcenterui.plist tests/static_notificationcenterui.plist` if you are so inclined)
3. Currently running `cargo run` should pretty print out the deserialized version of your present `~/Library/Containers/com.apple.notificationcenterui/Data/Library/Preferences/com.apple.notificationcenterui.plist`
   - ^facilitating that as simply as posisble means that probably what `mod`s and `struct`s are `public` will be subject to change



---

An unauthorized API (and CLI) for programmatic manipulation of macOS [desktop widgets](https://support.apple.com/en-us/108996)

Works by equivalent[^1] of [`defaults`](https://ss64.com/mac/defaults.html)` write com.apple.notificationcenterui`
[^1]: or realistically more complicated pipelines ending with that  to deal with fact of nested raw binary plists)

# The File
As of Sanoma, we are working with
```bash
c=$HOME/Library/Containers/com.apple.notificationcenterui/Data
f=$c/Library/Preferences/com.apple.notificationcenterui.plist

cd "$(dirname $f)"
```

# The Format

This is a binary `plist` that looks liks
```shell
plutil -p com.apple.notificationcenterui.plist
```
```output
{
  "fontStyle" => 0
  "last-analytics-stamp" => 738757542.0681469
  "widgets" => {
    "DesktopWidgetPlacementStorage" => {length = 829, bytes = 0x62706c69 73743030 d2010203 045f1014 ... 00000000 00000295 }
    "instances" => [
      0 => {length = 4134, bytes = 0x62706c69 73743030 d2010203 ... }
        [...]
      9 => {length = 719, bytes = 0x62706c69 73743030 d2010203 ... }
    ]
    "vers" => 1
    "widgets" => [
      0 => {length = 1425, bytes = 0x62706c69 73743030 d4010203 04050607 ... }
       [...]
      552 => {length = 3325, bytes = 0x62706c69 73743030 d4010203 04050607 ... }
    ]
   }
}
```

## `widgets`

### 🔍 meta-prelude: the "mystery" of the binary members  🔎

<details>
  <summary><h5>TL;DR / Spoilers:</h5></summary>
<pre>They are raw nested binary plists.</pre>
</details>
<details open>
  <summary><h5>full story</h5></summary>
Looking at those binary blobs, I initially anticipated more involved reverse
engineering

but (the eagle-eyed among you might notice a suspicious repeat everywhere of
prefix, & if you take the next step,)

```shell
plutil -extract widgets.DesktopWidgetPlacementStorage raw !$ \
| base64 -D | hexdump -C -n 8
```
```output
00000000  62 70 6c 69 73 74 30 30           |bplist00|
00000008
```

… as [magic numbers](https://en.wikipedia.org/wiki/Magic_number_(programming)) go, pretty self-explanitory. But to ask the experts to verify

```shell
^hexdump -C -n 8^(echo; file -)^
```
```output
plutil -extract widgets.DesktopWidgetPlacementStorage raw com.apple.notificationcenterui.plist | base64 -D | (echo; file -)

/dev/stdin: Apple binary property list
```
</details>

Indeed it is just[^1] another binary plist file!


[^1]: I spent probably altogether too long thinking these were in fact "base64 encoded nested binary plists", but in fact despite what I think the word `raw` means, Apple doesn't wanna mess up your terminal and so
```shell
man plutil | \
grep -B1 -A3 'the value printed depends'; (printf "%13s\n" "…"; !!0-1 | grep 'data')
```
```output
RAW VALUES AND EXPECTED TYPES
     With -extract keypath raw the value printed depends on its type.

     Following are the possible expect_type values and how they will be
     printed when encountered with -extract keypath raw
            …
            data         a base64-encoded string representation of the data
```


### `widgets`.`vers`
a version number, I presume. `1`, classic.

### `widgets`.`DesktopWidgetPlacementStorage` <small>(v1)</small>

This is probably what is gonna be most interesting to this project:


`{`

`CompatibilityVersion = 1 ,`

`NumberedDisplays` &rarr; `[{`

- `Number`: zero indexed (ordered how?)

- `Resolutions[0]`[^2]  `={`

    -  `Groups`: 1 or more collections of widgets on e.g. left and right of screen &rarr; `[`

        - `Items`: the individual widgets &rarr; `[{`
            - `Column`: from 0 to \[max determined by number & size of widgets\] (+1 small, +2 medium & large, +4 XL)
            - `Identifier`: a UUID
            - `Row`: from 0 to \[max determined by number & size of widgets\] (+1 small & medium, + 2 large & XL),
            - `Size`: `{( "Small"| "Medium" | "Large" | "ExtraLarge") => {} }` - one imagines this is a memberless enum serialization?
            - `ZOrder` -  0…{number of items}. Presumably this is the z-index, but whereas the layout engine doesn't let you layer via
            click and drag, in practice its a most-recently-dragged (highest most recent) `}, {…}, …]`
        - `Origin = [ $offset_from_left, $offset_from_top ]`. In (HiDPI) pixels `}`

    - `Size = [$width, $height]`, In (HiDPI) pixels. `},`

    - `{ … that for other display(s) /w widgets? … },`

  `  ]`
`}`


[^2]: the support was/is there at some point, but Apple is holding out on us! If this was utilized I probably wouldn't have gotten into any of this, as all I
really wanted is to be able to have more widgets on my ultrawide monitor than my 13" macbook air screen!

<details>
<summary><h5>command line exploration continued</h5></summary>


To get at this nested ones I'll use this:
```shell
nplutil() {
  local k; k=$1; shift;
  plutil -extract widgets.$k raw com.apple.notificationcenterui.plist \
  | base64 -D \
  | plutil "$@" -
}
```

This is all prety fixed length, so 20 lines should bring you to end of your first widget

```shell
nplutil DesktopWidgetPlacementStorage -p | head -20
```
```output
{
  "CompatibilityVersion" => 1
  "NumberedDisplays" => [
    0 => {
      "Number" => 0
      "Resolutions" => [
        0 => {
          "Groups" => [
            0 => {
              "Items" => [
                0 => {
                  "Column" => 0
                  "Identifier" => "438AD4D3-278A-4506-8417-6FF8980004AE"
                  "Row" => 0
                  "Size" => {
                    "Medium" => {
                    }
                  }
                  "ZOrder" => 4
                }
```

I have 9 widgets (despite 10 members of `widgets`.`instances` … TBD why ) and its 9 lines of head-matter then ~9 lines per widget so to pick up the remaining members of `Groups[0]` and `Resoulutions[0]`

```shell
nplutil DesktopWidgetPlacementStorage -p | tail -n +$((9 + 9*9))
```

```output
                }
              ]
              "Origin" => [
                0 => 2286
                1 => 6
              ]
            }
          ]
          "Size" => [
            0 => 3008
            1 => 822
          ]
        }
      ]
    }
  ]
}
```
</details>

### `widgets`.`instances`

this is where state lives for the widgets you are actually actively using (have instantiated). Once you extract and parse them they are actually just:

`[{`

- `uuid` : in fact a UUID. \[mostly\] matching the `Identifier` of the corresponding `Item` in `plist(widgets.DesktopWidgetPlacementStorage).NumberedDisplays.*.Resolutions.0.Groups.*.Items.*.Identifier`

  [with additional entries not matching those for widgets, if any, configured to show on notification center view]

- `widget` :  Surprise! 🎊 its actually another layer of nested raw binary plist.


`},]`

>a silly Aside:
> its interesting to me that top-level of the plist there is one camelCase attribute, one-kebab case attribute, and then one single-word one; then a layer down theres `PascalCase` key with a `PascalCase` set off values, and here its back to lowercase)

### `widgets`.`instances[*]`.`widget`

 In particular, once decoded, this inner `widget` plist is a bundles of objects output by [`NSKeyedArchiver`](https://developer.apple.com/documentation/foundation/nskeyedarchiver) ;

 layed out like:

- `$version` = all the ones I looked at it is `100000` (less classic)
- `$top = {`
  - `$root`: a `CFKeyedArchiverUID` object, with value = 1 `}`
    - that value and other UID objects with such seem to point into the index of `$objects`
- `$objects = [ <the serialized objects of the graph> ]`


There is some variance amongst my 10, but walking that graph through `$objects` by UID value, each appears to be an object of class `CHSWidget`
> apparently, going by `.tbd`s, a native of `ChronoServices.framework`
> -in `$objects` slot `1` (hereafter `$1`, after `$null` in `$0`)

with attributes:

- `activityIdentifier` : `$null` (for all mine)
- `family` = `1` |`2` | `3`  enum corresponding more-or-less to [`WidgetFamily`](https://developer.apple.com/documentation/widgetkit/widgetfamily):
    - `1` = small
    - `2` = medium
    - `3` = both large
    - `4` = extra large
- `kind`: a string  naming the widget per app devs
    (e.g. `EventListWidget`)
    (ending up in 6th slot or bumped down to 7 by the device Id)
- `extensionIdentity` [&rarr;`$2`  ] an object of class `CHSExtensionIdentity`
    with attributes:
  - `deviceIdentifier` =
    - when mac local: [`$0 =`] `$null`,
    -  when procied from iphone: [`$5`:] a class 7 UUID (presumably referring to my iPhone)
  - `containerBundleIdentifier` [&rarr;`$4`] =  e.g. `"com.flexibits.fantastical2.mac"`
  - `extensionBundleIdentifier` [&rarr; `$3` ] = e.g.` "com.flexibits.fantastical2.mac.FantasticalWidgets"`
- `intent2` ; app specific data?,
  - sometimes `$null`,
  - sometimes (apparently for both families) an an object of class (`CHSIntentReference` with a `stableHash` value and `idata` pointing at an object of class [`NSMutableData`](https://developer.apple.com/documentation/foundation/nsmutabledata/)
    > For the one I looked at the `NS.data` is _not_ a plist; does contian numerous meaningful-ish utf8 strings
    > I'm guessing this rev-eng is done and/or apple discloses more about the format BUT I doubt I have any need to go this deep


[^3]: not apparently

<details>
<summary><h5>command line exploration continued (for each of above 2 sections) </h5></summary>
`nplutil` takes us a little further

```shell
nplutil instances.0 -p
```
```output
{
  "uuid" => "6003AFEE-C0E5-4204-BEAA-08ABB4D7F71D"
  "widget" => {length = 4024, bytes = 0x62706c69 73743030 d4010203 04050607 ... 00000000 00000f18 }
}
```

we'll need a new function/pipeline to get into the `widget`, but first lets check our suspicion that these uuid's correspond with the `Identifier`s from the `Items` above

(I tried to use `nplutil` with `-extract x json`  to pipe to `jq` but no dice; so instead... )

```shell
nplutil DesktopWidgetPlacementStorage -p   | grep Identifier | cut -d'>' -f2 | sort | tee  placement_identifiers
```
```output
 "438AD4D3-278A-4506-8417-6FF8980004AE"
 "4E6811F3-74FE-4087-B633-91F11CC970A2"
 "6003AFEE-C0E5-4204-BEAA-08ABB4D7F71D"
 "680C59B4-009E-4A34-AB5C-4870DA9F31C1"
 "83D5493B-17F7-4970-97D6-60128C1089F9"
 "8DD73949-BF33-4994-81C3-66D8037FE444"
 "959287CA-C5E4-4DCA-914B-0004EFB7D97A"
 "D30E9A04-3E59-41FB-8CEE-0988367355D3"
```

and

```shell
for n in $(seq 0 9); do nplutil instances.$n -p | grep uuid | cut -d'>' -f2 ; done | sort  | tee  instance_uuids
```

```output
 "438AD4D3-278A-4506-8417-6FF8980004AE"
 "4E6811F3-74FE-4087-B633-91F11CC970A2"
 "6003AFEE-C0E5-4204-BEAA-08ABB4D7F71D"
 "680C59B4-009E-4A34-AB5C-4870DA9F31C1"
 "83D5493B-17F7-4970-97D6-60128C1089F9"
 "8DD73949-BF33-4994-81C3-66D8037FE444"
 "959287CA-C5E4-4DCA-914B-0004EFB7D97A"
 "9F580D1B-96F1-4C6E-90E5-802EBD05DAA8"
 "B42B1366-5746-40F4-9CEA-16D833302227"
 "D30E9A04-3E59-41FB-8CEE-0988367355D3"
```

to save our eyes the trouble
```shell
diff -u  placement_identifiers instance_uuids
```
```output
--- placement_identifiers	2024-06-02 16:24:22
+++ instance_uuids	2024-06-02 16:25:02
@@ -5,4 +5,6 @@
  "83D5493B-17F7-4970-97D6-60128C1089F9"
  "8DD73949-BF33-4994-81C3-66D8037FE444"
  "959287CA-C5E4-4DCA-914B-0004EFB7D97A"
+ "9F580D1B-96F1-4C6E-90E5-802EBD05DAA8"
+ "B42B1366-5746-40F4-9CEA-16D833302227"
  "D30E9A04-3E59-41FB-8CEE-0988367355D3"
```

huh. The good news is that they _mostly_ match. The bad news is that in addition to the mystery of why my instances contains one more than my total visible widgets, (presumably) that one ***AND*** a second one (of the 9) are otherwise-indexed.

I can tell you from previous work that one of them (and probably both?) are actually from the widgets that are _actually_ in NotificationCenter (when I click on clock, below the notifications). There are two there - and it's probably those

Why are there "only" 10 if I 9 widgets on Desktop and 2 there? I'm not sure yet, but I'd guess its to do with [AirBuddy](https://v2.airbuddy.app/) having shipped their own widgets _just_ before Apple announced this support, and still using their own rendering, but wanting to keep a placeholder? (implying I'm _not_ the first person to go through (some of) this). TBD.

---

Anyway, now its time to look into one of those `instance.widget`s; There are other ways probs but lets keep it simple

```shell
instwidgetp() { nplutil instances.$1 -extract widget raw | base64 -D | plutil -p - }
```

then

```shell
instwidgetp 0
```

```output
{
  "$archiver" => "NSKeyedArchiver"
  "$objects" => [
    0 => "$null"
    1 => {
      "$class" => <CFKeyedArchiverUID 0x600001192660 [0x1f30d08c0]>{value = 11}
      "activityIdentifier" => <CFKeyedArchiverUID 0x600001192640 [0x1f30d08c0]>{value = 0}
      "extensionIdentity" => <CFKeyedArchiverUID 0x6000011925e0 [0x1f30d08c0]>{value = 2}
      "family" => 2
      "intent2" => <CFKeyedArchiverUID 0x600001192600 [0x1f30d08c0]>{value = 7}
      "kind" => <CFKeyedArchiverUID 0x600001192620 [0x1f30d08c0]>{value = 6}
    }
    2 => {
      "$class" => <CFKeyedArchiverUID 0x600001192700 [0x1f30d08c0]>{value = 5}
      "containerBundleIdentifier" => <CFKeyedArchiverUID 0x6000011926c0 [0x1f30d08c0]>{value = 4}
      "deviceIdentifier" => <CFKeyedArchiverUID 0x600001192640 [0x1f30d08c0]>{value = 0}
      "extensionBundleIdentifier" => <CFKeyedArchiverUID 0x6000011926e0 [0x1f30d08c0]>{value = 3}
    }
    3 => "com.flexibits.fantastical2.mac.FantasticalWidgets"
    4 => "com.flexibits.fantastical2.mac"
    5 => {
      "$classes" => [
        0 => "CHSExtensionIdentity"
        1 => "NSObject"
      ]
      "$classname" => "CHSExtensionIdentity"
    }
    6 => "EventListWidget"
    7 => {
      "$class" => <CFKeyedArchiverUID 0x6000011927c0 [0x1f30d08c0]>{value = 10}
      "idata" => <CFKeyedArchiverUID 0x600001192780 [0x1f30d08c0]>{value = 8}
      "stableHash" => 643232281733152198
    }
    8 => {
      "$class" => <CFKeyedArchiverUID 0x6000011927e0 [0x1f30d08c0]>{value = 9}
      "NS.data" => {length = 3237, bytes = 0xe44d5f69 6e646578 696e6748 61736833 ... 7465676f 72790903 }
    }
    9 => {
      "$classes" => [
        0 => "NSMutableData"
        1 => "NSData"
        2 => "NSObject"
      ]
      "$classname" => "NSMutableData"
    }
    10 => {
      "$classes" => [
        0 => "CHSIntentReference"
        1 => "NSObject"
      ]
      "$classname" => "CHSIntentReference"
    }
    11 => {
      "$classes" => [
        0 => "CHSWidget"
        1 => "NSObject"
      ]
      "$classname" => "CHSWidget"
    }
  ]
  "$top" => {
    "root" => <CFKeyedArchiverUID 0x600001192560 [0x1f30d08c0]>{value = 1}
  }
  "$version" => 100000
}

```

... from here, as above I realized (after a bit) the graph of value = UIDs to stuff in `$objects`as described above. This might be common knowledge to an apple ecosystem dev, but was new to me

doing

```shell
for n in $(seq 0 9); do instwidgetp $n | grep '\$classname'; done | sort | uniq -c
```

```output
 10       "$classname" => "CHSExtensionIdentity"
   8       "$classname" => "CHSIntentReference"
  10       "$classname" => "CHSWidget"
   7       "$classname" => "NSMutableData"
```

confirms everything has ~fairly similar structure.

---

then I did more variations on that to elucidate further whats going on; but for a summary:

```shell
for n in $(seq 0 9); do instwidgetp $n | sed 's/0x[0-9a-f]*/$hex/g'; done | sort | uniq -c | sort -rn
  70     }
  35       ]
  35       "$classes" => [
  28         1 => "NSObject"
  10 }
  10 {
  10   }
  10   ]
  10   "$version" => 100000
  10   "$top" => {
  10   "$objects" => [
  10   "$archiver" => "NSKeyedArchiver"
  10     2 => {
  10     1 => {
  10     0 => "$null"
  10     "root" => <CFKeyedArchiverUID $hex [$hex]>{value = 1}
  10       "extensionIdentity" => <CFKeyedArchiverUID $hex [$hex]>{value = 2}
  10       "extensionBundleIdentifier" => <CFKeyedArchiverUID $hex [$hex]>{value = 3}
  10       "containerBundleIdentifier" => <CFKeyedArchiverUID $hex [$hex]>{value = 4}
  10       "activityIdentifier" => <CFKeyedArchiverUID $hex [$hex]>{value = 0}
  10       "$classname" => "CHSWidget"
  10       "$classname" => "CHSExtensionIdentity"
  10         0 => "CHSWidget"
  10         0 => "CHSExtensionIdentity"
   8     8 => {
   8     11 => {
   8     10 => {
   8       "$classname" => "CHSIntentReference"
   8       "$class" => <CFKeyedArchiverUID $hex [$hex]>{value = 11}
   8       "$class" => <CFKeyedArchiverUID $hex [$hex]>{value = 10}
   8         0 => "CHSIntentReference"
   7     9 => {
   7       "$classname" => "NSMutableData"
   7         2 => "NSObject"
   7         1 => "NSData"
   7         0 => "NSMutableData"
   6     7 => {
   6     5 => {
   6       "kind" => <CFKeyedArchiverUID $hex [$hex]>{value = 6}
   6       "family" => 2
   6       "deviceIdentifier" => <CFKeyedArchiverUID $hex [$hex]>{value = 0}
   6       "$class" => <CFKeyedArchiverUID $hex [$hex]>{value = 5}
   4     6 => {
   4     5 => "[a UUID I've redacted]"
   4       "kind" => <CFKeyedArchiverUID $hex [$hex]>{value = 7}
   4       "intent2" => <CFKeyedArchiverUID $hex [$hex]>{value = 8}
   4       "intent2" => <CFKeyedArchiverUID $hex [$hex]>{value = 7}
   4       "idata" => <CFKeyedArchiverUID $hex [$hex]>{value = 9}
   4       "idata" => <CFKeyedArchiverUID $hex [$hex]>{value = 8}
   4       "family" => 1
   4       "deviceIdentifier" => <CFKeyedArchiverUID $hex [$hex]>{value = 5}
   4       "$class" => <CFKeyedArchiverUID $hex [$hex]>{value = 9}
   4       "$class" => <CFKeyedArchiverUID $hex [$hex]>{value = 6}
   3     12 => {
   3       "$class" => <CFKeyedArchiverUID $hex [$hex]>{value = 12}
   2     4 => "com.hey.app.ios"
   2     4 => "com.apple.clock"
   2     3 => "com.hey.app.ios.widget"
   2     3 => "com.apple.clock.WorldClockWidget"
   2       "intent2" => <CFKeyedArchiverUID $hex [$hex]>{value = 0}
   2       "$class" => <CFKeyedArchiverUID $hex [$hex]>{value = 7}
   1     9 => {length = 1835, bytes = $hex ... }
   1     7 => "UpcomingWidget"
   1     7 => "TaskNUpWidget"
   1     7 => "ExpandedSummaryWidget"
   1     7 => "BoxWidget"
   1     6 => "com.apple.mobiletimer.digital.city"
   1     6 => "com.apple.mobiletimer.clock.digital"
   1     6 => "NetWorthWidget"
   1     6 => "EventListWidget"
   1     6 => "BatteriesAvocadoWidget"
   1     6 => "Batteries"
   1     4 => "me.danielgauthier.Jazzed"
   1     4 => "com.streaksapp.streak"
   1     4 => "com.flexibits.fantastical2.mac"
   1     4 => "com.copilot.production"
   1     4 => "com.apple.Batteries"
   1     4 => "codes.rambo.AirBuddyHelper"
   1     3 => "me.danielgauthier.Jazzed.Widgets"
   1     3 => "com.streaksapp.streak.widgets"
   1     3 => "com.flexibits.fantastical2.mac.FantasticalWidgets"
   1     3 => "com.copilot.production.widgets"
   1     3 => "com.apple.Batteries.BatteriesAvocadoWidgetExtension"
   1     3 => "codes.rambo.AirBuddyHelper.AirBuddyWidgetKit"
   1       "stableHash" => 8242903295471159785
   1       "stableHash" => 643232281733152198
   1       "stableHash" => 3482238442379606734
   1       "stableHash" => -5635317975281096893
   1       "stableHash" => -5460504746951996121
   1       "stableHash" => -5056586109943120591
   1       "stableHash" => -4158500553160717609
   1       "stableHash" => -384985244352148310
   1       "NS.data" => {length = 9052, bytes = $hex ... }
   1       "NS.data" => {length = 3391, bytes = $hex ... }
   1       "NS.data" => {length = 326, bytes = $hex ...}
   1       "NS.data" => {length = 3237, bytes = $hex ... }
   1       "NS.data" => {length = 2877, bytes = $hex ... }
   1       "NS.data" => {length = 2871, bytes = $hex ... }
   1       "NS.data" => {length = 1584, bytes = $hex ...}
```
</details>

### `widgets`.`widgets`

This appears to be a list of all avaliable widgets on your system(s)

As with `instance` the bulk of it is in a further nested plist, this time under key `encodedDescriptor` (oh hey, more camel case!)

There are also (seemingly unused) keys; being for all 553 entries I have:

- `localizedLocale` ==  `en_US@calendar=iso860` (this one probably actually depends on your system, but seems same across all)

- `version` = [empty string]
- `modDate` = `0001-01-01 00:00:00 +0000` (Happy First Easter?)



### `widgets`.`widgets[*]`.`encondedDescriptor`

Here again we find `NSKeyedArchiver` data; skipping over that format, it appears down here we are looking at

Objects of class `CHSWidgetDescriptor`.

\[It was only at this point that I looked into not doing this manually and grabbed [`nskeyedarchiver_converter`](https://crates.io/crates/nskeyedarchiver_converter); soooo ... you get this here, I might come back to it\]

(See above collapsed discussions for definition of `nplutil` if you care)

```shell
wdplutilp() { nplutil widgets.$1 -extract encodedDescriptor raw | base64 -D  }
```

```shell
wdplutilp 0 > ~/Downloads/widget0descriptor.bin
nskeyedarchiver_converter ~/Downloads/widget0descriptor.bin ~/Downloads/widget0descriptor.plist
plutil -p ~/Downloads/widget0descriptor.plist
```

```output
{
  "root" => {
    "$classes" => [
      0 => "CHSWidgetDescriptor"
      1 => "CHSBaseDescriptor"
      2 => "NSObject"
    ]
    "backgroundRemovable" => 1
    "backgroundStyle" => 0
    "displayName" => "Tips"
    "enablesMultipleTapTargets" => 0
    "extensionIdentity" => {
      "$classes" => [
        0 => "CHSExtensionIdentity"
        1 => "NSObject"
      ]
      "containerBundleIdentifier" => "com.apple.tips"
      "extensionBundleIdentifier" => "com.apple.tips.Widget"
    }
    "hiddenBySensitiveUI" => 0
    "kind" => "com.apple.tips"
    "localeToken" => {length = 20, bytes = 0xb44ff8341935fd26b4eb602120931f29f4731b35}
    "nativeCBI" => "com.apple.tips"
    "platform" => 1
    "sdkVersion" => "14.5"
    "supportedSizeClasses" => 14
    "supportsInteraction" => 1
    "supportsVibrantContent" => 0
    "unsupLoca" => [
    ]
    "version" => 1
    "widgetDescription" => "Get the most from your Apple devices with helpful hints and hidden gems."
    "widgetVisibility" => 0
  }
}
```

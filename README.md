# Passthrough practice

Step 4 adds **Minecraft mouse-look transfer to Skyrim's first-person view**.
  The Fabric mod sends rendered player yaw and pitch alongside position; Skyrim
sets the corresponding player angles for its normal camera to follow. New look
behavior and its comparison option are **not tested** in game or automated tests.

Step 3 added **sampled Skyrim collision and NPC positions** to the Fabric mod and
Rust SKSE DLL. Minecraft still sends its interpolated player position each render
frame. Skyrim follows it and sends nearby surface boxes and NPC records back.
Minecraft adds those boxes to its normal collision queries on both the client
and local server. Its movement and gravity code is unchanged.

The user's live run passed transport comparison: 117 received snapshots,
149,415 collision boxes and 117 NPC records matched Skyrim exactly. Skyrim's log
confirms PLUGIN_LOADED and HOOK_INSTALLED. Minecraft's collision-query logs show
nonzero matches on both client and server (12 client entries and 1 server entry
in the inspected run). In the subsequent run, the user confirmed movement in
Minecraft stopped at a Skyrim wall and NPC. The position comparison passed for
5,986 received frames out of 11,437 sent, with exact coordinates/partial ticks
and zero Skyrim target/read-back difference. The new world comparison also passed:
543 complete received snapshots, 1,354,585 collision boxes and 139 NPC records
matched exactly. These are records across snapshots, not unique objects.
The user subsequently confirmed jumping and landing work. Slopes and broad
collision coverage remain **not tested**. The agent has not modified game installations.

**Use Minecraft's controls while connected.** Skyrim's player is a puppet: its
position is overwritten by Minecraft's latest position. Keyboard/mouse forwarding
from the Skyrim window into Minecraft is not implemented, so Skyrim movement
input will not drive the connected player. Keep Minecraft focused for movement
and configure Skyrim to continue running in the background as described below.
Minecraft mouse look worked in the user's 0.3.0 test, but Skyrim's camera did not
follow. The new 0.4.0 build implements that look-direction transfer.

The plan is in [docs/DESIGN.md](docs/DESIGN.md). The narrow, version-specific
Skyrim interface is documented in [docs/ABI.md](docs/ABI.md).

## Previous step 2 checks (not rerun for step 3)

- 14 Rust tests pass: 8 unit tests, 5 connection process tests, and 1 position
  lifecycle test. They cover connection recovery, protocol layout, coordinate
  conversion, invalid/stale samples, role ownership, and simulated puppet calls.
- A separate Java 25 process calls the actual native DLL and sends 180 synthetic
  render samples. Another Windows process receives 100 of them; every received
  coordinate and partial tick matches its sender exactly.
- The simulated puppet moves the actor and clears controller velocity; it
  restores gravity on inactivity, stale data, and resumption after a pause.
- The Fabric mod compiles against Minecraft 26.3 and Fabric Loader 0.19.5. The
  render method and its injection target exist in that game's class signatures.
- The DLL exports `SKSEPlugin_Load`, `SKSEPlugin_Version`, and the Minecraft
  bridge functions. Its loader accepts only runtime 1.7.104 and validates the
  matching format-5 Address Library before registering its callback.
- The comparison script rejects mismatched coordinates and empty receive logs.

## What is not tested / not implemented

- Slopes, general player/capsule behavior,
  background execution, and interaction with other mods: **not tested**.
- The earlier missing Address Library error is resolved for the user's latest
  run: Skyrim reached PLUGIN_LOADED and exported snapshots.
- The connection's abandoned-lock crash path and other PCs: **not tested**.
- Sampled collision and NPC transport and query injection are observed in live
  logs. Wall and NPC blocking passed the user's manual check; general collision
  accuracy and coverage remain **not tested**.
- First-person look transfer is implemented but **not tested**. Full camera
  placement, third-person mirroring, FOV/bob matching, Skyrim-window input
  forwarding, graphics overlays, combat, and matched saves
  are not implemented. This step does not create a mirror world.

## Step 3 behavior and limits

The exporter samples an eight-block cube around the player, using rays in both
directions on each axis at half-block spacing. It continues past up to eight
hits per ray, includes world collision layers, and ignores actor/player ray hits.
Each surface becomes a thin box behind the hit. Thin objects between rays can
be missed, curved/sloped surfaces are approximate, and doors can lag until the
next snapshot. This is SkyCraft's ray-sampling first version, not exact mesh export.

Work is spread across frames (about 2 ms or 384 casts per update, checked between
rays; one ray and lock waiting can exceed that target). A complete scan contains
1,536 rays plus continuation casts. Scans taking over one second or completed
after the player moves outside the useful center are discarded. Collision data
expires after 2.5 seconds; a stalled Minecraft render loop stops using it after
250 ms. Ordinary Minecraft blocks remain solid too. A sparse practice world makes
the exported geometry easier to distinguish. Stale/missing data falls back to
Minecraft's ordinary world; there is no invisible emergency floor.

NPCs are loaded high-process actors in the player's current Skyrim cell and
within eight blocks per axis. Their IDs and positions are exact transported
values, but their Minecraft body boxes are a fixed 0.6 by 1.8 blocks. Creature
sizes, animated hitboxes, neighboring-cell NPCs, NPC interaction and combat are
not supported by this step. Maximum snapshot: 8,192 surface boxes and 128 NPCs;
overflow disables the adapter with an error instead of silently truncating it.

Both halves must be upgraded together to 0.4.0 / protocol 4. Old protocol-2/3
plugins use different shared-memory names and cannot connect to this version.
The existing `logs/step2` folder is retained for all live bridge logs.

## Repeat the automated checks

From PowerShell in this project folder:

```powershell
.\tools\test-link.ps1
.\tools\test-position-native.ps1
```

Expect 14 Rust tests to pass, then the Java/native check to report:

```text
PASS: Java native smoke publisher sent 180 synthetic render samples.
PASS: 100 matching received frames (synthetic fixture); all Minecraft coordinates and partial ticks match exactly.
```

These commands do not launch either game. Connection-test logs are under
`passthrough\target\link-tests`; simulated and Java/native logs are under
`logs\step2-fixtures`. They are separate from real-game logs.

Build and package both adapters using the dependencies already cached here:

```powershell
.\tools\build-step4.ps1 -Offline
```

The packages are:

- `dist\step4\Passthrough-Skyrim-0.4.0.zip`: `SKSE\Plugins\passthrough_link.dll`.
- `dist\step4\Passthrough-Minecraft-0.4.0.zip`: `mods\passthrough-fabric-0.4.0.jar`
  and `mods\passthrough_link.dll`.

Rust stable MSVC, compiler tools, Windows SDK libraries, Java 25, and Gradle 9.7.1
are needed. This PC's Rust, SDK libraries, and Gradle are cached in ignored
`.tools`. The build scripts select the installed MSVC tools and Java 25. On a
clean machine, the Fabric build needs dependency downloads before `-Offline`
works. The Rust crate has no external Rust package dependencies.

## Install and test in the games

Installation writes require your authorization under AGENTS.md rule 4. The
following is the manual procedure; it has **not been run**.

1. Install the [Address Library for Skyrim runtime **1.7.104**](https://www.nexusmods.com/skyrimspecialedition/mods/32444?tab=files) through your normal
   mod setup. SKSE must see `Data\SKSE\Plugins\versionlib-1-7-104-0.bin`.
2. Install the Skyrim zip through your mod manager, or put its `SKSE` folder
   under `E:\SteamLibrary\steamapps\common\Skyrim Special Edition\Data`.
3. Put the Minecraft zip's two `mods` files in
   `C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\mods`.
   Remove the previous passthrough JAR when upgrading; keep only the new version.
4. In this Prism instance's Java arguments, include
   `--enable-native-access=ALL-UNNAMED`. Use Java 25 and Fabric 0.19.5.
5. For a manual test with two windows, Skyrim needs to continue updating while
   Minecraft has focus. The usual Skyrim setting is `bAlwaysActive=1` under
   `[General]` in the active `Skyrim.ini` (the mod-manager profile's copy when
   applicable). This background configuration is **not tested** here. In
   Minecraft, use F3+P to disable pausing on lost focus if needed.
6. Start Skyrim through SKSE and load a practice save in an open area after the
   opening sequence. Start Minecraft through Prism and enter a local
   single-player world; keep Open to LAN off. Close both pause menus.
7. Walk a short distance in Minecraft, stop, then jump. Stop both games before
   comparing their completed logs. Collision between the two worlds is not yet
   synchronized, so use short movements in an open area for this check.

For the direct Skyrim installation, the launch command is:

```powershell
& 'E:\SteamLibrary\steamapps\common\Skyrim Special Edition\skse64_loader.exe'
```

If using a mod manager, launch its SKSE entry instead so it supplies your mods.

A successful SKSE load writes this actual message to `logs\step2\skyrim-*.log`:

```text
plugin loaded runtime=1.7.104 event=PLUGIN_LOADED
```

The Fabric initializer writes `Passthrough Fabric plugin loaded` to Minecraft's
normal `logs\latest.log`. Its position data is in this project's
`logs\step2\minecraft-*.log`. The paths are compiled into this practice build,
so this build expects the project to stay at its current location.

To compare the latest pair of real-game logs, from the project folder:

```powershell
$mcLog = Get-ChildItem .\logs\step2\minecraft-*.log | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$skyLog = Get-ChildItem .\logs\step2\skyrim-*.log | Sort-Object LastWriteTime -Descending | Select-Object -First 1
.\tools\compare-position-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName
```

Expect a nonzero number of matching received frames, exact Minecraft coordinate
and partial-tick matches, and a Skyrim target/read-back difference no larger than
0.01 Skyrim units. A missing received frame is not counted as success.

### Check step 3 in the logs

After installing both new packages, load an offline practice world and a Skyrim
save near a floor, wall, and loaded NPC. Let sampling settle, then walk slowly
against the wall, jump and land, and approach the NPC. Expect Minecraft's normal
movement to stop at exported surfaces and NPC boxes. Step 2's matching SEND/RECV
player positions must continue. These are expected results, **not tested**.

Stop both games, select their latest bridge logs as above, then run:

```powershell
.\tools\compare-world-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName
Select-String -LiteralPath 'C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\logs\latest.log' -Pattern 'event=COLLISION_QUERY'
```

Expect the comparison to report complete matching snapshots, nonzero collision
boxes, and NPC records when near an NPC. `WORLD_SEND` in Skyrim must correspond
to `WORLD_RECV` in Minecraft with the same `sender`, `world`, `epoch`, and
`snapshot`. Every `BOX_SEND`/`BOX_RECV` and `NPC_SEND`/`NPC_RECV` pair carries
identical coordinates. `WORLD_SCAN` gives ray counts, hit counts, rays reaching
the hit limit, and scan duration. In Minecraft's normal log, expect
`COLLISION_QUERY side=client` and `side=server`, with nonzero `matches` when near
a surface. Query logs are limited to one per second per side. The comparison
proves transport only; it does not prove live movement or accurate geometry.

### Check step 4 mouse look

If Minecraft produces no bridge log after upgrading, check its normal
`logs/latest.log`. The reported 0.4.0 startup failure was a mixed installation:
the new JAR was present but `mods/passthrough_link.dll` still matched 0.3.0.
That DLL lacks `pt_mc_frame4`, causing `NoSuchElementException` in NativeLink
before the bridge logger starts. The agent replaced this instance's outdated DLL
with user approval and confirmed its hash matches the 0.4.0 package. For future
upgrades, close Minecraft and replace **both** the JAR
and DLL from the 0.4.0 Minecraft package. Skyrim's protocol-4 startup succeeded
in that run. Successful camera behavior after correcting the DLL is **not tested**.

Install both 0.4.0 packages together, removing the older Fabric JAR. Keep
Minecraft in first-person view and focused; leave both pause menus closed.
Skyrim must run in the background. Slowly look left/right, up/down, then turn
through a complete circle. Expected: Skyrim's first-person view follows the same
direction without reversing axes or jumping at the angle wrap. The initial view
may turn to align with Minecraft's absolute direction. Skyrim's native camera
animation and one-frame update delay can remain; FOV and camera position are
not synchronized in this step.

Ordinary Skyrim third-person view temporarily switches to first-person while
following Minecraft. It is restored on release only if that same camera is still
in first-person. Other camera states (mounts, furniture, death, free camera and
scripted transitions) are left alone by the look adapter.

Stop both games, select `$mcLog` and `$skyLog` as above, and run:

```powershell
.\tools\compare-position-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName -RequireLook
```

Expect position PASS plus a nonzero number of matching look frames, with at most
0.00001 radians conversion/read-back error. `mc_yaw`/`mc_pitch` are degrees in
SEND and LOOK_RECV. Skyrim logs target and read-back radians plus its camera-state
rotation as `camera_qw/qx/qy/qz` (a four-number rotation representation). Those
camera values are sampled before the next normal camera update; the PASS checks
transport and actor angles, not the final rendered camera. If the view does not
follow, send both logs and report whether left/right, up/down, or both failed.

## Read the position logs

Match `sender`, `world`, and `frame` between `event=SEND` and `event=RECV`.
Both lines carry `partial`, `mc_x`, `mc_y`, and `mc_z`. Skyrim also records
`target_x/y/z` and `actual_x/y/z`, read from the actor immediately after its move.

An `ANCHOR` line records how the initial Minecraft sample was aligned with the
current Skyrim position. After that, one Minecraft block moves 70 Skyrim units:
Minecraft X maps to Skyrim X, Minecraft Z to negative Skyrim Y, and Minecraft Y
to Skyrim Z. This avoids an initial teleport to an unrelated part of Skyrim.

The connection holds the latest frame, so Skyrim may skip intermediate Minecraft
frames. It never interpolates the received position again. Samples older than
250 milliseconds, inactive worlds, loading, or paused updates release the puppet;
resuming establishes a fresh alignment. The two adapters use a dedicated
`Local\PassthroughPractice_link2_...` connection, separate from SkyCraft and the
old step-1 protocol. Use logs to check behavior; no visual game inspection is
required.

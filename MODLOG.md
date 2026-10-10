## 2026-10-11 Handover rules for Codex and Claude Code; first checkpoint commit

**Changed:** AGENTS.md: new section "Handover between agents" (what must
always be true, how to pick up, where things are, things that have cost time);
rule 3 now carries the user's standing instruction to commit when they say all
testing passed, on branch `dev`, without pushing; the log permission is marked
as applying to both agents; the Environment line names both agents; the "Keep
these files updated" list gains STATUS.md and the Backlog. New
templates/HANDOVER-note.md. STATUS.md: its first section is now a handover note
in that shape. README.md: the one remaining Windows-1252 dash byte (in "bytes
240-255", 0.8.8 section) replaced by a UTF-8 dash, so the file is valid UTF-8
like every other tracked file. No code or build change. Done by Claude Code.

**Why:** The user asked how best to hand over between the two agents, then
asked for documented rules so that either can continue the other's work at any
state, and for a commit each time they report that all testing passed.

**Tested how:** `git add -A --dry-run` before the first commit: 84 files, only
.rs, .java, .json, .md, .ps1, .toml, .lock, .properties and .gitignore; a
credential-pattern scan of those files found nothing; largest file 201 KB.
Checked every tracked file for encoding and line endings. No build or game run
was needed for these documents.

**Result:** Branch `dev` created from `master`. Commit 28bf33d holds everything
from 0.5.0 through the accepted 0.10.18 (84 files). A second commit holds these
handover documents. `master` and GitHub are unchanged; nothing was pushed. All
tracked files are UTF-8; all use Windows line endings.

**Still broken / not tested:** The handover rules have not yet been exercised
by a real switch to Codex; the first one will show what they miss. Codex has
not been told about branch `dev` except through AGENTS.md. Notes that Claude
Code keeps in its own memory folder outside the project are not visible to
Codex; what mattered from them is now in AGENTS.md.

**Commit:** 28bf33d (dev) Passthrough 0.5.0 to 0.10.18; the handover documents
are in the commit that follows it.

**Next:** To check: `git log --oneline -3` should show two new commits on
`dev` above a9fc0fd, and `git status --short` should print nothing. At the
next switch, the incoming agent follows "Picking up from the other agent" in
AGENTS.md.

## 2026-10-11 0.10.18 test session verified

**Changed:** README and STATUS record the result; DESIGN backlog gains the
user's wish for Skyrim's E to take precedence everywhere. No code or build
change. Done by Claude Code.

**Why:** User ran the 0.10.18 test list and reports all tests passed.

**Tested how:** User gameplay. Ran `.\tools\collect-scene-logs.ps1` (report
20261011-101205) and counted crosshair kinds, interaction and menu sends
against receipts, recoveries and error lines.

**Result:** Protocol 29 on both sides; no ERROR, DISABLED or fallback line.
Crosshair kinds logged: container 13, living 11, body 4, door 9, other 2.
Activations: 10 from the inventory key, 8 from the Interact key; 17 dispatched,
1 dropped with reason=cooldown. Menu commands: 75 sent, 75 received, 75
returned, including 25 true clicks from the left button and 5 confirms from
the inventory key. Recoveries 3 requested, 3 accepted.

**Still broken / not tested:** The optional map click was not reported. The
logs show that E activated a container or body, not what Skyrim then showed;
that part rests on the user's report. Findings from the 0.10.16 session
(rapid small hits mostly lost; no protection while Minecraft is inactive) are
unchanged.

**Next:** Nothing to install. Commit as the user asked, then write the
handover rules.

## 2026-10-11 E opens Skyrim containers (0.10.18)

**Changed:** Planned in DESIGN first. Rust: world.rs adds target_kind (u32 at
TARGET_AT=397,440, 8 bytes inserted before the damage section; size 398,024;
magic PTWORL13) with kind constants, a maximum and a test; attack.rs classifies
the crosshair reference (classify() pure, kind() reads the game) and logs
SCENE_CROSSHAIR on change; lib.rs protocol 29; windows.rs link29. Java:
NativeLink WORLD_SIZE and TARGET_KIND_AT; SkyrimAttack keeps the kind and
offers lootTarget(); SkyrimInteraction.beforeKeybinds makes the inventory key
confirm in Skyrim menus and activate a container or body under the crosshair,
and logs source=inventory_key; SkyrimSavesMixin calls it. Versions 0.10.18.
ABI, README, STATUS updated. Done by Claude Code.

**Why:** User asked for E to open the Skyrim container. Activating whatever
the crosshair is on would have stopped E opening the Minecraft inventory in a
fight, so Minecraft has to know what the target is.

**Tested how:** Read CommonLib FormTypes.h and TESObjectREFR.h. Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first), twice: once
more after a review change. Checked the packaged DLL carries PTWORL13. No game
run, install, log read or commit.

**Result:** 28 tests pass (22 unit, 5 connection, 1 position). Packages
0.10.18: Minecraft 344563 bytes, Skyrim 219302 bytes. Review found one hazard
before release: a press from E could, in a rare timing, have been sent as the
Interact key's "take all"; a press from E can now only activate or confirm.

**Still broken / not tested:** All gameplay is **not tested**. The reads on
arbitrary Skyrim references (base object at +0x40, its type, life state on
NPCs) come from CommonLib and have not run. E does nothing special for doors or
living characters, by design. If Skyrim reports no crosshair target at the
distance the user stands, E opens the Minecraft inventory. Java has no
automated tests.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.18). Follow README "Test 0.10.18": E on a chest, E on open ground,
E on a living NPC, E on a body, left click on a dialogue choice. Report what
happened; logs are collected directly.

## 2026-10-11 Left click is a real Skyrim click (0.10.17)

**Changed:** Planned in DESIGN first. SkyrimMenus.java: in a Skyrim menu the
left mouse button sends command 10 (true click) instead of command 5
(confirm). Versions 0.10.17. Also recorded: the user confirms R inside a
container takes everything (the unconfirmed item from the 0.10.16 session).
Done by Claude Code.

**Why:** User reported the experimental middle-button click "acts like left
mouse in Skyrim" and chose this as the next step. A real click reacts to where
the pointer is, which the confirm key cannot do.

**Tested how:** Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`
(tests first). No game run.

**Result:** Build passed; 26 Rust tests passed at this step. Packages 0.10.17:
Minecraft 343326 bytes, Skyrim 218772 bytes. No wire change.

**Still broken / not tested:** **Not tested** with the left button itself. A
left click with the pointer over nothing no longer confirms the highlighted
entry (Enter and, from 0.10.18, E do). Map markers are untested.

**Next:** Covered by README "Test 0.10.18".

## 2026-10-11 0.10.16 test session verified

**Changed:** README and STATUS record the user's test results and the log
check; DESIGN backlog updated with two findings and the true-click result. No
code or build change. Done by Claude Code.

**Why:** User ran the 0.10.16 test list: A, C, D and E passed; for B, "middle
mouse acts like left mouse in skyrim".

**Tested how:** User gameplay. Ran `.\tools\collect-scene-logs.ps1` (report
20261011-094530; about 167,000 lines) and counted events on both sides with
grep and awk: errors, waits, draws, recoveries, menu sends against receipts,
attack requests against Skyrim health changes, wear, damage applications by
armour and blocking, deaths.

**Result:** No ERROR, DISABLED or fallback_generic line. Fix: 6
mesh_from_another_place waits, then 6410 draws; depth usable in 2221 of 2238
probes. Travel: 9 recoveries requested, 9 accepted, 0 rejected, one fast
travel. Menus: 115 SEND = 115 RECV dispatch = 115 RESULT returned, identical
sequence sets; wheel 25, right_click 14, interact_key (action 9) 5,
pointer_click (action 10) 27. Pointer baseline remembered=true with gain 2,2
from the fourth menu. Attacks: 102 sent, 90 dispatched, 11 dropped
range_or_cell, 1 not seen by Skyrim; max |loss - requested| 0.00001; scale
1.75; 37 critical requests. Wear: 59 events of 1, broken=true at damage 58. Incoming:
110 applications as mob attack; larger hits 11 (9 taken, 2 shield-blocked);
armour 14 turned 13.578 into 8.43. Death: cause=mob, kill, reload, clear,
recovery.

**Still broken / not tested:** Small rapid hits: 99 under 2 Skyrim damage, 7
taken (0.69 of 12.76 requested). This is Minecraft's hurt cooldown combined
with the bridge using up refused hits; it existed before tonight. For about
19 seconds Minecraft sent no active frames (paused or unfocused; not
established which); 113 hits then reached Skyrim's health natively (52.6) and
22 queued hits were dropped. Whether R takes all from a container was not
reported. One attack request never appeared on the Skyrim side; reason not
established. Axes, enchantments, other armour sets, walled cities and places
outside the height range remain untested.

**Next:** Nothing to install. To review: `rg --no-ignore -c "SCENE_DAMAGE_APPLY"
logs/scene-report-20261011-094530` (expect 110) and
`rg --no-ignore -c "event=ERROR" logs/scene-report-20261011-094530` (expect no
output). Choose the next step.

## 2026-10-11 Review correction: button latches cannot stick (0.10.16)

**Changed:** SkyrimMenus.java: the two held-button latches added in 0.10.11
(right click swallowed until released, confirming left click not allowed to
start mining) now count only while the per-frame poll that releases them has
run in the last half second. Versions 0.10.16. Done by Claude Code while the
user was away.

**Why:** Re-reading tonight's Java changes, which have no automated tests. The
latches are released by code that runs once per rendered frame as part of the
bridge sender. If that sender ever stopped after an error while a button was
held, right click would have been swallowed until Minecraft restarted.

**Tested how:** Code review only, then
`.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first). No game run.

**Result:** Build passed; 26 Rust tests pass. Final packages 0.10.16:
Minecraft 343307 bytes, Skyrim 218760 bytes. This is the build to install; it
contains every step from 0.10.10 on.

**Still broken / not tested:** **Not tested** in game. The review found no
other defect, which is not proof that there is none.

**Next:** Install 0.10.16 and follow the combined test list at the top of
README.md.

## 2026-10-11 Armour and shields now work against Skyrim hits (0.10.15)

**Changed:** Planned in DESIGN first. BridgeDamage.java: forwarded Skyrim hits
are delivered as Minecraft's "mob attack" damage type instead of "generic",
coming from the position of the nearest Skyrim character in the snapshot (none
listed: no direction). Lookup failure falls back to generic and logs
SCENE_DAMAGE_SOURCE state=fallback_generic once. SCENE_DAMAGE_APPLY also logs
source, attacker, armor and blocking. Versions 0.10.15. Done by Claude Code
while the user was away.

**Why:** Found while checking Minecraft's own tag data in the cached 26.3 jar:
"generic" is listed in bypasses_armor, and bypasses_shield includes that list.
So Minecraft armour and shields did nothing against Skyrim hits. The 0.9.8
entry and DESIGN said Minecraft armour "remains native"; that was never tested
and was wrong.

**Tested how:** Read data/minecraft/tags/damage_type/bypasses_armor.json,
bypasses_shield.json and no_knockback.json from the cached jar (read only,
nothing copied into the project). javap for DamageSource constructors,
DamageTypes, Registries.DAMAGE_TYPE, LivingEntity.getArmorValue/isBlocking.
Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first). No
game run, install, log read or commit.

**Result:** Build passed. Packages 0.10.15: Minecraft 343229 bytes, Skyrim
218769 bytes. Minecraft side only; no wire change.

**Still broken / not tested:** **Not tested** in game. Side effect: a mob
attack knocks the Minecraft player back a little, which generic damage did not,
so Skyrim hits will now push you. Only Skyrim characters within the collision
snapshot (about 8 blocks, same cell) give a direction; hits from further away
are reduced by armour but cannot be shield-blocked. The nearest character is
assumed to be the attacker, which can be wrong in a crowd. If you die to such a
hit, Minecraft's death line in chat may read oddly because there is no
Minecraft attacker to name. This changes a behaviour accepted in 0.9.9 (damage
arriving in full), on purpose.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Weapons wear out when hitting Skyrim enemies (0.10.14)

**Changed:** Planned in DESIGN first. SkyrimAttack.java: when a bridge attack
is sent, the held item's own "weapon" data decides the wear (swords 1, axes 2)
and it is applied on the integrated server with Minecraft's hurtAndBreak, so
Unbreaking, creative mode and breaking behave as usual. Logs SCENE_ATTACK_WEAR
with item, amount, damage before/after and whether it broke. Versions 0.10.14.
Done by Claude Code while the user was away.

**Why:** Continue gameplay features; "no durability" was in the backlog.

**Tested how:** javap for ItemStack.hurtAndBreak, DataComponents.WEAPON and the
Weapon record. Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`
(tests first). No game run, install, log read or commit.

**Result:** Build passed. Packages 0.10.14: Minecraft 341918 bytes, Skyrim
218767 bytes. First draft used ItemStack.is(Item), which does not exist in
26.3; javap showed that before the build and it compares getItem() instead.

**Still broken / not tested:** **Not tested** in game. Wear is charged when the
attack is sent, so a swing that Skyrim then drops (target moved, out of range)
still costs durability. Bare hands and items without weapon data are unaffected.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Critical hits on Skyrim enemies (0.10.13)

**Changed:** Planned in DESIGN first. SkyrimAttack.java: a bridge attack is
critical under Minecraft's own conditions (charge above 0.9, falling, not on
the ground, not climbing, in water, sprinting, riding or blinded); the requested
damage is then multiplied by 1.5 (new constant CRITICAL_SCALE), Minecraft's
critical-hit sound is played, and SCENE_ATTACK_SEND logs critical=true/false.
Versions 0.10.13. Done by Claude Code while the user was away.

**Why:** Continue gameplay features: jump attacks should matter against Skyrim
enemies as they do in Minecraft. Listed in the backlog as "no critical hits".

**Tested how:** javap on the cached 26.3 jar for Entity.fallDistance, onGround,
isInWater, isSprinting, isPassenger, LivingEntity.onClimbable/hasEffect,
MobEffects.BLINDNESS, SoundEvents.PLAYER_ATTACK_CRIT and LocalPlayer.playSound.
Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first). No
game run, install, log read or commit.

**Result:** Build passed; 26 Rust tests still pass. Packages 0.10.13: Minecraft
340755 bytes, Skyrim 218717 bytes. Minecraft side only; no wire change. A full
wooden-sword critical asks for 7 x 1.5 = 10.5 damage.

**Still broken / not tested:** **Not tested** in game. No critical-hit
particles on the Skyrim enemy (it is not a Minecraft entity). The rule is
checked on the client at click time. Java has no automated tests.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Pointer speed remembered between menus (0.10.12)

**Changed:** Planned in DESIGN first. menu_input.rs: the measured pointer
pixels-per-count is kept for the session (State.learned) and used as the
starting value for the next menu; the two rules are now pure functions
starting_gain() and measured_gain() with two tests using logged numbers. The
baseline log line reports gain_x, gain_y and remembered=true/false. Versions
0.10.12. Done by Claude Code while the user was away.

**Why:** Backlog defect found on 2026-10-11: every menu started from Skyrim's
stored sensitivity (1) while the real value is 2, so the first small pointer
movements in each menu travelled double.

**Tested how:** Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`
(tests first). No game run, install, log read or commit.

**Result:** 26 tests pass (20 unit, 5 connection, 1 position). Packages
0.10.12: Minecraft 340333 bytes, Skyrim 218725 bytes. No wire change.

**Still broken / not tested:** **Not tested** in game. The first menu of each
Skyrim session still starts from the stored sensitivity until one larger
movement has been measured. Pointer speed scaling for a small Minecraft window
stays in the backlog.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 More Skyrim menu controls from Minecraft (0.10.11)

**Changed:** Planned in DESIGN first. Minecraft: SkyrimMenus gains request(),
scroll() (wheel = Up/Down), back() (right click = Back, once per press, press
swallowed until released), an optional "Skyrim menu: click at pointer" key
(default middle mouse button) and a latch so a held confirming click cannot
start mining after the menu closes; SkyrimInteraction sends Skyrim's R-key menu
action when a Skyrim menu is open; SkyrimCursorMixin hooks MouseHandler.onScroll;
SkyrimAttackMixin hooks Minecraft.startUseItem; SkyrimSavesMixin also drops
"pick block" while a menu is open and it shares the pointer-click key; the new
key is registered and named. Rust menu_input.rs: command table event(), pure
verdict() rule, new commands 9 ("XButton", keyboard R) and 10 ("Click", mouse
left), three tests. Versions 0.10.11. Done by Claude Code while the user was
away.

**Why:** Containers, shops and other Skyrim list menus could only be driven
with arrows, Enter and Backspace. Right click also still used the held
Minecraft item during a Skyrim menu.

**Tested how:** javap on the cached 26.3 jar for MouseHandler.onScroll,
Minecraft.startUseItem, KeyMapping constructors and the mouse constants; read
CommonLib UserEvents.h for the event names. Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly` (tests first). No game run,
install, log read or commit.

**Result:** 24 tests pass (18 unit, 5 connection, 1 position). Packages
0.10.11: Minecraft 339916 bytes, Skyrim 218307 bytes. No wire layout change,
protocol 28. Left click, arrows, Enter and Backspace are unchanged in code.

**Still broken / not tested:** All of it is **not tested** in game. Wheel,
right click and the R action reuse the dispatch path accepted for arrows and
Backspace, but whether Skyrim's item menus act on "XButton" from a forwarded
event is unknown. The true click ("Click" as a mouse button event) is an
experiment and may do nothing. Fast wheel spins can drop steps because only the
newest command is kept. The mixins are checked against method signatures only;
a wrong target would stop Minecraft at start-up.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Invisible blocks after a door fixed (0.10.10)

**Changed:** scene.rs: new mesh_origin(), which refuses a block mesh whose
origin is more than 64 blocks from the Minecraft player on any axis, plus a
test using the real numbers from the failing run. scene_draw.rs: uses it and,
for such a mesh, waits for the next one (SCENE_WAIT
reason=mesh_from_another_place) instead of returning an error. Versions
0.10.10. Done by Claude Code while the user was away.

**Why:** User tested 0.10.9: doors and interiors work and play continues, but
placed blocks were invisible indoors, and blocks placed earlier outdoors were
invisible (though still solid) after coming back out.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1` (report 20261011-012218)
and read it. Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`, which
ran the tests first. No game run, install or commit.

**Result:** One cause for both symptoms. At 189587250 ms, the same millisecond
the interior's origin became ready, Skyrim logged `SCENE_DISABLED
message=invalid scene origin`. The newest mesh was still the outdoor one
(origin about 122,26,-580) while the player was already at about
-1245187,129,-4149243; mapped through the interior's origin it exceeded the
position limit, the drawing code treated that as fatal, and block drawing stayed
off for the rest of the session. Minecraft kept sending correct indoor meshes
(origin -1245195,121,-4149252) that were never read. Everything else in that
run worked: door in and out, recovery in 47 ms and 31 ms, 154 indoor collision
scans with 1242 to 2573 boxes each, block placement indoors accepted on both
Minecraft sides. This was a defect in my 0.10.9 change: I did not consider data
left over from the previous place. 21 tests pass (15 unit, 5 connection, 1
position). Packages 0.10.10: Minecraft 338541 bytes, Skyrim 218255 bytes.

**Still broken / not tested:** The fix is **not tested** in game. Whether
Skyrim's depth information is usable for hiding blocks behind walls indoors is
unknown: drawing was already off, so no indoor depth probe was logged. Hands
and HUD drawing use a separate pass and were not affected.

**Next:** Covered by the combined test list at the top of README.md.

## 2026-10-11 Doors and interiors: a Minecraft region per Skyrim place (0.10.9)

**Changed:** Planned in DESIGN first. saved_origin.rs: Origin::for_location and
the fixed square formula; restore uses it and returns 5 (recover) instead of 2
when the Minecraft position is too far to express. save_identity.rs: origin(),
recovery_target() and menu_authorized() use the place's origin; a derived one
is logged as SCENE_ORIGIN state=derived. skse.rs test: simulated loading door.
Java: BridgeRecovery reports a target outside the Minecraft world in chat and
log; BridgePairing reason-5 text reworded. Versions 0.10.9. DESIGN (plan and
backlog), ABI, README, STATUS updated. Done by Claude Code.

**Why:** User asked for doors and interiors, which also unblocks fast travel
and walled cities. The bridge refused every place except the saved one.

**Tested how:** Read saved_origin.rs, save_identity.rs and the skse.rs frame
flow. Wrote 5 unit tests for the formula and restore rule and extended the
simulated-player test with a door, recovery request, simulated teleport and a
step inside. `.\tools\build-step5b.ps1 -Offline -PackageOnly` ran the tests
then built. No game run, install or commit; no new logs read.

**Result:** 20 tests pass (14 unit, 5 connection, 1 position). Writing the
restore test exposed a real defect before any build: a Minecraft position more
than about 1.4 million blocks from a place's region made restore return 2
("invalid origin"), which never recovers; it now returns 5. Packages:
Passthrough-Minecraft-0.10.9.zip 338385 bytes, Passthrough-Skyrim-0.10.9.zip
218099 bytes. No wire, record or protocol change (28).

**Still broken / not tested:** All gameplay is **not tested**: R on a door
while the bridge steers the player, loading-screen and cell-change timing,
collision sampling inside an interior, Minecraft teleports of millions of
blocks and finding ground there, the way back out, walled cities, fast travel.
The tests use fake objects and prove only the bridge's own arithmetic and
hold/recover logic. Known limits: Minecraft height covers Skyrim z of about
-11,600..15,000 with the current saved origin; two different places whose IDs
share the same low bits would share a region (not detected); light-plugin
places can move if load order changes; a fast-travel marker may not be
selectable from Minecraft. 0.10.5 damage values still unverified in combat.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.9). F5, then R on a building door, walk inside, place a block, R
on the door to leave, go back in. Expect short holds and the block still there.
Report the result; logs will be collected directly. Expect SCENE_ORIGIN
state=derived, SCENE_RECOVERY request/accepted, origin_ready, no ERROR.

## 2026-10-11 Death and respawn verified (0.10.8)

**Changed:** README, STATUS, DESIGN and ABI record the accepted result and the
confirmed life-state reading. No code or build change. Done by Claude Code.

**Why:** User installed 0.10.8 and reports the respawn works and the bridge
reconnects.

**Tested how:** User gameplay. Ran `.\tools\collect-scene-logs.ps1` (report
20261011-005940) and read the death, recovery, alignment, link and error lines
on both sides.

**Result:** Minecraft SCENE_DEATH_SEND sequence=1 cause=genericKill with
immediate_respawn=true keep_inventory=true, then hold reason 7 shown. Skyrim:
RECV age_ms=19 decision=kill; RESULT health 100 -> -200; SCENE_DEATH_LIFE 1 at
+15 ms, 2 at +2.5 s; save reverted/loaded at +6.3 s; SCENE_DEATH clear;
recovery token 2 requested and accepted 31 ms apart; origin_ready; damage
ready. Minecraft reported ready 7115 ms after the death. Zero ERROR, DISABLED
or sender-disabled lines in the three files. Second attempt succeeded.

**Still broken / not tested:** Only `/kill` was exercised. Death from Skyrim
enemy hits, falls or other Minecraft damage, death during a Skyrim menu, and
death with no recent Skyrim save are **not tested**. A Skyrim-only death still
leaves Minecraft health unchanged. No attacks in this report, so the 0.10.5
damage scale is still unverified. F5/F9 not pressed in this run.

**Next:** Nothing to install. To review: `rg --no-ignore "SCENE_DEATH|SCENE_RECOVERY"
logs/scene-report-20261011-005940` (one line); expect kill, life 1 then 2,
clear, and an accepted recovery. Choose the next feature.

## 2026-10-11 Death reconnect repair, tests before build (0.10.8)

**Changed:** world.rs: hold reasons are constants (HOLD_DEAD=7,
MAX_HOLD_REASON) and the packet check uses them; new test encodes a reply for
every reason. death.rs: hold depends on health <= 0 plus grace, life state only
logged (SCENE_DEATH_LIFE). skse.rs: uses HOLD_DEAD; test fixture gains the
missing Addresses fields and is updated to current rules, with a simulated
death, reload and no second kill. save_identity.rs: test-only publish helper.
tools/build-step5b.ps1 runs `cargo test --offline` before building.
tools/collect-scene-logs.ps1 also keeps ERROR, LINK, RELEASED, ANCHOR, LOAD_,
PLUGIN_LOADED, HOOK_INSTALLED, NATIVE_READY and overlay error lines, and the
first 16 lines after a Minecraft "sender disabled" message. AGENTS.md: new
"Test before build" rule. Versions 0.10.8. DESIGN, ABI, README, STATUS updated.
Done by Claude Code.

**Why:** User tested 0.10.7: F5/F9 worked; after `/kill` both characters died
but the bridge never reconnected. User also asked for the Rust unit tests to be
fixed and run before every build, and granted standing log-collection access.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1` twice (reports
20261011-005050 and, after extending the collector, 20261011-005252) and read
them. Ran `.\tools\cargo.ps1 test --offline` until green, then
`.\tools\build-step5b.ps1 -Offline -PackageOnly`, which now runs the tests
first. No game run, install or commit.

**Result:** Cause found in the second report: `event=ERROR message=invalid
world snapshot` at the kill frame. The packet check allowed hold reasons up to 6
and I had added reason 7 in 0.10.6 without changing it, so the Skyrim adapter
rejected its own reply and disabled itself. The first report did not contain
that line because the collector kept only SCENE_ lines; my first reading of it
could not explain the silence. Evidence that the rest worked: death received
age_ms=5, health 100 -> -200, Skyrim save reverted/loaded 6.3 s later; 2
quicksaves and 6 quickloads dispatched once each; 4 recoveries accepted.
Tests: the stale skse.rs test first failed to compile, then failed at its first
gravity assertion because it predates save pairing; after the update all 15
pass (9 unit, 5 connection, 1 position). Packages 0.10.8: Minecraft 338052
bytes, Skyrim 218109 bytes.

**Still broken / not tested:** The reconnect after a death reload is **not
tested**; this is the second build of the feature. Life-state offset is
unconfirmed (read 0 right after the lethal hit). Whether Skyrim keeps calling
the player update while dead is unknown. A Skyrim-only death still does not
reset Minecraft health. 0.10.5 damage values remain untested in combat. The
simulated tests use fake objects, not Skyrim; they check the bridge's own
logic, not game behaviour. The separate background task for the test fixture
worked from the old commit and is superseded by this change.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.8). F5, then `/kill`. Expect the "You died" chat line, Skyrim's
death and reload, then movement again at the saved spot. Report the result;
logs will be collected directly. If it fails again, write the handoff.

## 2026-10-11 Skyrim quicksave/quickload keys (0.10.7)

**Changed:** New SkyrimSaves.java (keys F5/F9, remappable) and SkyrimSavesMixin
(Minecraft.handleKeybinds HEAD drains perspective-toggle clicks in bridge worlds
when the keys coincide). SkyrimMenus.frame sends commands 7/8 with context 0
while no Skyrim menu is open. InteractionOptionsMixin registers the two keys;
en_us.json names them. menu_input.rs accepts 7/8 through a gameplay gate and
dispatches "Quicksave"/"Quickload" key events via MenuControls. Versions 0.10.7.
DESIGN, ABI, README, STATUS updated. Done by Claude Code.

**Why:** User asked, before testing 0.10.6, for F5 to quicksave and F9 to
quickload Skyrim from Minecraft.

**Tested how:** Read CommonLib UserEvents.h, QuickSaveLoadHandler.h,
MenuControls.h and javap output for KeyMapping, Options and InputConstants in
the cached 26.3 jar. Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`.
Checked the packaged DLL and JAR contain the new names. No game run, install,
log read, unit tests or commit.

**Result:** Rust and Fabric builds passed. Passthrough-Minecraft-0.10.7.zip
(337938 bytes) and Passthrough-Skyrim-0.10.7.zip (217997 bytes) in dist/step5b.
No wire layout change; protocol 28 as 0.10.6.

**Still broken / not tested:** Everything in game is **not tested**: whether
Skyrim's quicksave handler accepts a forwarded event, key registration and
rebinding, the perspective-toggle suppression, and recovery after F9. The
DESIGN section for this step was written alongside the code rather than before
it. Minecraft's world is neither saved nor rolled back by these keys. Requires
the paired save to be loaded, so F9 cannot be used to get out of an unpaired
save. Rust unit tests still do not compile.

**Next:** Install and test together with 0.10.6 using README "Test 0.10.7".

## 2026-10-11 Minecraft death becomes a Skyrim death and reload (0.10.6)

**Changed:** Planned in DESIGN first, after the user chose the rule. New
passthrough/src/death.rs (decide/hold/kill, vitals read), wired into skse.rs
frame() after lineage admission; damage.rs gains a pass-through flag for the
bridge's own lethal hit; position.rs/lib.rs/windows.rs/native.rs carry a death
number and time (record 456 bytes, protocol 28, link28, pt_mc_frame14). New
BridgeDeath.java and BridgeDeathMixin (ServerPlayer.die observer);
BridgeRulesMixin forces immediate respawn and keep inventory in bridge worlds;
BridgePairing shows hold reason 7; NativeLink sends the two new values.
Versions 0.10.6. Done by Claude Code.

**Why:** Death was undefined: Minecraft could die while Skyrim's character
lived on unprotected. User chose "Skyrim-style: reload the last save".

**Tested how:** Read CommonLib Actor.h/Actor.cpp/ActorState.h for DoDamage,
life state and offsets, and javap for ServerPlayer.die and GameRules. Read the
existing recovery and frame flow. Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly`. No game run, install, log
read, unit tests or commit.

**Result:** Rust and Fabric builds passed. 0.10.6 packages were produced
(Minecraft 334356 bytes, Skyrim 216643 bytes) and then superseded in dist by
0.10.7, which contains this change.

**Still broken / not tested:** All of it is **not tested** in game. Specific
unknowns: DoDamage on the player with a null attacker (may not kill, may
crash); life state at Actor+0xC8 bits 21-24; whether death is visible to the
bridge inside the call; Skyrim's reload while its window is in the background;
recovery after that reload. A Skyrim-only death does not reset Minecraft
health. No death screen or message in Skyrim's window. Keep inventory and
immediate respawn are forced on every load of a bridge world, overriding the
player's own setting. Protection while the bridge is merely "not ready" is
unchanged. 0.10.5 damage balance is still untested too.

**Next:** Make a named Skyrim save first. Close games, run
`.\tools\install-build.ps1` (three INSTALLED lines, Fabric 0.10.7), then follow
README "Test 0.10.7": F5, walk, F9, then `/kill`. Expect Skyrim's death and
reload and a return to the saved spot. If Skyrim crashes or the character
survives, stop and report; one more designed attempt only.

## 2026-10-11 0.10.5 left untested; death/respawn investigation started

**Changed:** README and STATUS record that the user chose not to test 0.10.5
and what code reading shows about death today. No code or build change.
Done by Claude Code.

**Why:** User: "I won't test this. It sounds like it should feel right.
Continue with the next feature." Next feature per the earlier recommendation is
death and respawn.

**Tested how:** Read BridgeDamage.java, BridgeRecovery.java, PassthroughClient,
OverlayCapture conditions, damage.rs, recovery.rs and the skse.rs frame flow.
Nothing built, run or installed; no logs read.

**Result:** From code only: Minecraft death makes samples inactive; Skyrim
suspends the puppet and its character then takes native damage (not_ready
passthrough). After a Minecraft respawn the position mismatch should trigger the
existing recovery teleport back to Skyrim's position. No death-specific logic
exists, and the overlay is not shown while dead.

**Still broken / not tested:** 0.10.5 damage balance **not tested** (user
choice). Every statement about death above is from reading code, **not tested**.
Death behaviour is undefined by design until the user chooses a rule.

**Next:** Ask the user what dying should do, then write the plan in DESIGN
before any code. Nothing to install or test for this entry.

## 2026-10-11 Outgoing damage balance (0.10.5) and fuller backlog

**Changed:** Planned in DESIGN first. SkyrimAttack.java: new constant
DAMAGE_SCALE=1.75 replaces the x10 factor; `scale` added to the
SCENE_ATTACK_SEND log line. Versions 0.10.5 in Cargo.toml, gradle.properties,
fabric.mod.json and the SKSE plugin version field. ABI, README and STATUS
updated. Separately, as the user asked, the Backlog section in docs/DESIGN.md
now lists the other deferred and untested items gathered from README, STATUS,
MODLOG and DESIGN, grouped by area. Done by Claude Code.

**Why:** User said damage on Skyrim enemies is far too high (wooden sword 40
killed a 22-health wolf in one hit) and asked for the balance pass. 1.75 puts
Minecraft sword tiers level with Skyrim one-handed swords.

**Tested how:** Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`.
No game run, install, log read, unit tests or commit.

**Result:** Rust and Fabric builds passed (existing linker and deprecation
notes only). Packages Passthrough-Minecraft-0.10.5.zip (330059 bytes) and
Passthrough-Skyrim-0.10.5.zip (216222 bytes) are in dist/step5b. Protocol 27
and all layouts unchanged; the DLL changed only in its version field.

**Still broken / not tested:** New damage values are **not tested** in game.
The Skyrim and Minecraft weapon base values behind 1.75 are from memory, not
read from game files. Fist at 1.75 may feel useless. Axes and other weapons,
enchantments, criticals and durability remain as before. Incoming damage is
unchanged. The backlog list is a gathering of existing notes, not a plan. The
user wrote "the new section in MODLOG"; the Backlog section lives in
docs/DESIGN.md, so the items went there. Rust unit tests still do not compile.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.5). Hit a surviving hostile enemy with a rested wooden sword twice,
then one rested punch. Expect about 7 and 1.75 health lost per hit; logs should
show scale=1.75 and matching native before/after. Report how it feels.

## 2026-10-11 Dialogue pointer accepted; pointer scaling to backlog

**Changed:** Added a Backlog section at the top of docs/DESIGN.md with pointer
speed scaling and the first-movement doubling defect, and marked the 0.10.4
pointer plan as accepted. README and STATUS record the acceptance. No code or
build change. Done by Claude Code.

**Why:** User confirmed the mouse works inside an NPC conversation and asked to
put mouse scaling in the backlog.

**Tested how:** User gameplay report. No new logs read, no tests or builds run.

**Result:** 0.10.4 dialogue pointer accepted by the user (menus and NPC
conversation), with the numeric check from report 20261011-001910 on record.

**Still broken / not tested:** Pointer speed scaling deferred. First small
movements per menu still travel double (no fix decision). Punching an enemy
after a menu not tried. Mouse wheel, right click and true click absent. Rust
unit tests still do not compile.

**Next:** Nothing to install or test. Choose the next gameplay increment.

## 2026-10-11 0.10.4 pointer verified from logs

**Changed:** README and STATUS record the numeric result. No code or build
change. Done by Claude Code.

**Why:** User ran the collector, supplied report scene-report-20261011-001910
and allowed reading it; also reported dialogue keys and after-menu block
placing/breaking work.

**Tested how:** Searched the three report files for SCENE_CURSOR and SCENE_MENU
events with grep; compared requested counts with pointer before/after values
and matched Minecraft command sequences against Skyrim receipts.

**Result:** 3 baselines, 65 logged moves (counters reach 380 and 51). All 92
logged axis movements: (after-before)/request = exactly 2, same sign. Screen
2560x1440, stored sensitivity 1. Minecraft logged 57 pointer sends (window
854x480) and 24 menu sends (8 from mouse, 16 from keys); Skyrim shows 24
dispatches and 24 returned results with identical sequence sets and no
duplicates. Zero ERROR/DISABLED lines. Attempt 1 works for the tested menus.

**Still broken / not tested:** Initial gain 1 is wrong (real 2), so the first
small movements in each menu go double until an 8+ count move; not fixed.
Pointer use inside NPC dialogue is not shown: sessions with pointer movement
had no key presses and the key sessions had no pointer movement, and logs do
not name the open menu. Punching after a menu not tried. Wheel, right click and
true click still absent. Unit tests still do not compile.

**Next:** No reinstall needed. To repeat the check:
`rg --no-ignore "SCENE_CURSOR state=move" logs/scene-report-20261011-001910`
(one line); expect after values differing from before by twice the request.
Ask user whether to fix the first-move doubling and whether speed feels right.

## 2026-10-11 User confirms 0.10.4 menu pointer

**Changed:** README and STATUS record the user's result. No code or build change.
Done by Claude Code.

**Why:** User installed 0.10.4 and reported back.

**Tested how:** User gameplay report only. No logs collected or read (permission
not granted to Claude Code). No tests or builds run for this note.

**Result:** While adding this note I found the one Windows-1252 dash byte in
STATUS.md ("bytes 240-255", 0.8.8 section) had become a replacement character
since my last check. I did not knowingly edit that line and the cause is not
established. Replaced it with a proper UTF-8 dash; README.md's byte is intact.

User: "That seems to work." With Skyrim's Tab menu opened from the
Skyrim window and Minecraft focused again, the pointer followed the mouse and
left click moved through the menu. Backspace left the menu; the bridge
reconnected and movement resumed.

**Still broken / not tested:** Not verified by numbers: pointer read-back,
learned speed factor, vertical direction learning, event counts. Hover and click
on NPC dialogue choices were not described separately from the Tab menu. Mouse
look, block breaking and punching after a menu were not reported. Mouse wheel,
right click and a true Skyrim click event are still not implemented. Rust unit
tests still do not compile (stale skse.rs fixture; separate task started by user).

**Next:** No reinstall or repeat test needed. If log permission is given, check
with `rg --no-ignore SCENE_CURSOR logs` after `.\tools\collect-scene-logs.ps1`;
expect one baseline then move lines whose after values differ from before.

## 2026-10-10 Mouse pointer in Skyrim dialogue (0.10.4)

**Changed:** Planned in DESIGN first. Minecraft: new SkyrimCursorMixin captures
mouse motion at MouseHandler.turnPlayer while a Skyrim menu is open; SkyrimMenus
keeps a running total and sends it with the menu context; SkyrimAttackMixin now
turns a left click in a Skyrim menu into the existing confirm action and skips
vanilla attack/mining for it. NativeLink calls pt_mc_frame13. Rust: position.rs
adds pointer context/totals at 424..440, lib.rs record 440 bytes/protocol 27,
windows.rs link27, native.rs frame13, menu_input.rs builds a MouseMoveEvent for
MenuControls and learns pointer speed from read-back, skse.rs resolves MenuCursor
403551 and MouseMoveEvent vtable 236703. Versions 0.10.4. Updated ABI, README,
STATUS. Done by Claude Code.

**Why:** User asked to make Skyrim's mouse work when talking to an NPC. Skyrim's
window has no focus, so it never receives the mouse; only forwarded keys worked.

**Tested how:** Read local CommonLib MenuCursor/MouseMoveEvent/MenuControls/
MenuEventHandler/IMenu sources and javap signatures for MouseHandler and
Minecraft in the cached 26.3 jar. Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly` (twice, see mistakes below).
Ran `.\tools\cargo.ps1 test --offline --lib`. No game run, install, log read
or commit.

**Result:** Rust release and Fabric builds passed (existing linker note and Java
deprecation note only). Packages Passthrough-Minecraft-0.10.4.zip and
Passthrough-Skyrim-0.10.4.zip are in dist/step5b. The unit-test command FAILED
to compile: the skse.rs test fixture is missing Addresses fields hud_vtable and
interaction. That line is unchanged by this work and already lacked the fields;
no tests ran, including the new record round-trip check.

Mistakes made and corrected this session:
- My first build and test invocations failed only because I added output
  redirection, which PowerShell 5.1 turns into an error. Rerun without it.
- A scripted edit wrote a raw NUL byte instead of the two characters `\0` inside
  the "Cursor" name in menu_input.rs. Same compiled value, wrong source text.
  Corrected and rebuilt.
- Claude Code's edit tool converted Windows-1252 dash bytes (0x96) into the
  replacement character in 14 places in docs/ABI.md (0.8.7 to 0.8.9 byte ranges)
  and 2 places in MODLOG.md (0.8.8 entry) while adding text elsewhere in those
  files. Each sat between two digits of a byte range. Repaired by writing a
  proper UTF-8 dash in those 16 places. README.md
  and STATUS.md each still hold one such byte; their new sections were added by
  prepending bytes so that byte is untouched.

**Still broken / not tested:** All new gameplay is **not tested**: whether Skyrim
accepts the forwarded mouse move, the "Cursor"/0xA event naming (from memory of
Skyrim's control map, not a local source), pointer speed and vertical direction,
hover highlight, click-to-confirm, mixin application at runtime, and regressions
to mouse look, block breaking and punching. No mouse wheel, right click or true
Skyrim click event; right click still acts in Minecraft during a Skyrim menu.
Unit tests remain broken until the stale fixture is repaired (separate task).

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric 0.10.4). Talk to an NPC with R, move the mouse, point at a choice, left
click. Expect Skyrim's pointer to follow, the choice to highlight and the click
to select it, with Minecraft's view still. Report what happened and whether logs
may be collected; expect SCENE_CURSOR baseline then move lines with changing
after_x/after_y. If the pointer never moves, stop: one more designed attempt
only, then a STATUS handoff.

## 2026-10-10 Point rules and whitelist at templates folder

**Changed:** AGENTS.md now names `templates/MODLOG-template.md` and
`templates/STATUS-handoff.md`. `.gitignore` whitelist drops the two root-level
template entries and allows `templates/` and `templates/*.md` instead. Still a
whitelist. No code, build or game-file change. Done by Claude Code (handover
from Codex while its usage is on cooldown).

**Why:** Both templates live in `templates/`, but the rules and whitelist named
the project root, so Git ignored the real files and the rule paths led nowhere.

**Tested how:** `git check-ignore` on both template paths before and after;
`git status --short` on the touched paths; spot-checked `logs`, `dist` and
`notes.md` are still ignored. No build or game run needed.

**Result:** Before: both templates matched the ignore-all rule (`*`). After:
`git check-ignore` prints nothing and exits 1 (not ignored) and `templates/`
shows as untracked (`??`). `logs`, `dist` and `notes.md` remain ignored.

**Still broken / not tested:** Nothing staged or committed. Git still records
the old root `MODLOG-template.md` as deleted (` D`); that clears when the
deletion and the new `templates/` files are committed together. AGENTS.md and
STATUS.md still say "Agent: Codex"; left unchanged, not asked to edit.

**Next:** To check, run `git status --short templates` (expect `?? templates/`)
and `git check-ignore templates/MODLOG-template.md` (expect no output).
Commit only when requested.

## 2026-10-10 Recharge combat trace accepted

**Changed:** README/STATUS record accepted0.10.3 sword/fist recharge. No code change.

**Why:** User identified final15seconds as relevant combat after enemy setup.

**Tested how:** Collected report20261010-225957; searched ignored log directory
explicitly, matched attack sequences and compared requests with before/after health.

**Result:**19 sends/receipts/results across9781ms. Sword intervals687/454/250ms
produce40/26.483198/14.195199 damage. Fist156/141ms gives5.92; rested fist10.
Native losses match within0.00005 from printed rounding; no duplicate results.

**Still broken / not tested:** Axe, block-priority regression and HUD recharge
not verified here. Full damage balance remains deferred per user feedback.

**Next:** No install/repeat fight needed. Review via `rg --no-ignore SCENE_ATTACK
logs/scene-report-20261010-225957` (one line); expect19 complete attack traces.
Continue next scoped feature when requested.

## 2026-10-10 Weapon attack recharge (0.10.3)

**Changed:** Recorded user feedback that damage is incredibly high; future10x
balance pass in DESIGN/README. Planned then implemented held-component ATTACK_SPEED
and elapsed-client-tick recharge in SkyrimAttack. Added diagnostic charge fields.
Lowered both spam guards500ms to100ms, protocol26 namespace and versions. Updated
README/STATUS/ABI. Full-strength damage unchanged.

**Why:** Continue gameplay features with weaker rapid attacks and item speed.

**Tested how:** Read local javap tickCount/ATTACK_SPEED signatures, reviewed code,
built using build-step5b.ps1 -Offline -PackageOnly. Rust/Fabric succeeded; existing
linker informational/Java deprecation warnings. No tests/game run or installation;
Gradle test NO-SOURCE. No commit.

**Result:**0.10.3 packages in dist/step5b. Gameplay **not tested**. Logs carry
speed, elapsed ticks, charge, multiplier, full damage and requested damage.

**Still broken / not tested:** HUD indicator not synchronized. Only emitted bridge
requests consume recharge; misses/real MC targets use vanilla behaviour separately.
No durability, enchantments, effects or criticals. Damage balance deferred as asked.

**Next:** Close games; `.\tools\install-build.ps1` (three INSTALLED/Fabric0.10.3).
Try sword attacks0.2-0.3s apart then wait two seconds/hit; expect lower quick-hit
request and full rested hit. Use a target surviving first hit; optional axe test,
then normal block break. Collect and search logs with rg --no-ignore.

## 2026-10-10 Sword and punch damage confirmed

**Changed:** README and STATUS record successful0.10.2 comparison. No source change.

**Why:** User reports one-hit wolf kill with sword and mudcrab kill with fist.

**Tested how:** Collected report20261010-224713; read SCENE_ATTACK events using
rg --no-ignore. Compared item computation, request and native before/after.

**Result:** Sword seq1 attribute4/request40, health22 ->-18; fist seq3 attribute1/
request10, health5 ->-5. Second calculation attempt succeeds. Additional fist
seq2 sent against earlier handle has no dispatch/result; reason not established.
User confirms visible kills; species and kill attribution not independently logged.

**Still broken / not tested:** Other weapon types, effects, durability, recharge
and block-priority regression **not tested** by this report.

**Next:** No rebuild/reinstall required. Review with `rg --no-ignore SCENE_ATTACK
logs/scene-report-20261010-224713` (one line); expect40 sword and10 fist with
matching native health changes. Continue next increment when requested.

## 2026-10-10 Held-item calculation repair (0.10.2, attempt 2)

**Changed:** Planned in DESIGN; SkyrimAttack uses player base attack and held
stack ATTRIBUTE_MODIFIERS.compute for MAINHAND instead of client total attribute.
Added base/observed/component-count diagnostics; versions and README/STATUS updated.

**Why:**0.10.1 wooden sword and fist both returned1 and dealt10 in report223504.

**Tested how:** Read local javap signatures for component compute and player base
attribute API. Built with build-step5b.ps1 -Offline -PackageOnly; Rust/Fabric pass.
No game run or test suite; Gradle test NO-SOURCE. No game writes or commits.

**Result:**0.10.2 packages built. Gameplay **not tested**. Protocol25/frame12
unchanged. Existing linker informational/Java deprecation warnings remain.

**Still broken / not tested:** Runtime weapon differentiation awaits confirmation.
Temporary player modifiers, enchantments, durability, recharge and criticals not
implemented. If attempt2 fails in game, stop and produce template-based handoff.

**Next:** Close games, run `.\tools\install-build.ps1` (three INSTALLED lines,
Fabric0.10.2), compare fist then unenchanted wooden sword with one second between
hits. Expect sword damage greater than fist. Collect logs using rg --no-ignore.

## 2026-10-10 Sword versus fist comparison failed

**Changed:** README and STATUS record the failed0.10.1 runtime result; no code changes.

**Why:** User asked whether wooden sword and subsequent punch did different damage.

**Tested how:** Collected scene-report-20261010-223504 while games running;
searched SCENE_ATTACK_SEND/RECV/RESULT with rg --no-ignore.

**Result:** Wooden sword sequence1 attribute1.0 requested10, health22 ->12;
empty hand sequence2 attribute1.0 requested10, health12 ->2. Both dealt10.
Transport matches. Client attribute value did not include a sword increase.

**Still broken / not tested:** Weapon damage differentiation failed its first
runtime test; underlying attribute behaviour not yet investigated. No fix tested.

**Next:** Investigate held-item calculation when continuing. No reinstall needed.
Review with `rg --no-ignore SCENE_ATTACK logs/scene-report-20261010-223504`;
expect two sends and matching native dispatch/results showing equal damage.

## 2026-10-10 Held-item base damage (0.10.1)

**Changed:** Planned in DESIGN before code. SkyrimAttack captures Minecraft
ATTACK_DAMAGE*10 and logs item/attribute/request. NativeLink/frame12 transports
float at connection412; attack.rs validates it and passes it to DoDamage.
Updated protocol25, namespace, Rust/Fabric/SKSE versions, README/STATUS/ABI.

**Why:** Next scoped combat increment after verified fixed-strength punches:
make held weapons affect damage while retaining the empty-hand baseline.

**Tested how:** Read local javap API signatures for getAttributeValue and compiled
Rust/Fabric using build-step5b.ps1 -Offline -PackageOnly. First inspection command
had PowerShell brace syntax error; corrected. First build passed but review found
hardcoded Fabric manifest still0.10.0; corrected and rebuilt successfully.
No test suite run; Gradle test NO-SOURCE. No game installs or commits.

**Result:** 0.10.1 packages staged in dist/step5b. Native and Java builds pass.
Existing linker informational/deprecation warnings remain. Gameplay **not tested**.

**Still broken / not tested:** No weapon durability, enchantment-specific damage,
criticals or weapon-speed cooldown;500ms and210-unit range retained. Initial10x
scale may need balancing. Runtime held-item attribute and health results await
user test. Existing real Minecraft block/entity priority unchanged in code.

**Next:** Close games, run `.\tools\install-build.ps1`; expect three INSTALLED
lines and Fabric0.10.1. Compare empty hand vs unenchanted wooden sword on hostile
enemies with one second between clicks; sword should hit harder. Break a normal
Minecraft block. Collect logs and compare requests/results with rg --no-ignore.

## 2026-10-10 Controlled punch trace and correction

**Changed:** README and STATUS now record complete outgoing attack evidence and
correct the previous false missing-log diagnosis. No code or architecture change.

**Why:** Verify loaded build and trace punches following the user's controlled test.

**Tested how:** Ran collect-scene-logs.ps1 (report20261010-222744), read startup
records and searched using rg --no-ignore. Rechecked previous report221938.

**Result:** Three sends/dispatches/results, sequences1..3, target000EBC4D,
health22 ->12 ->2 ->-8 (10 per punch). Five incoming captures match five accepted
Minecraft applications. Fabric0.10.0/native protocol24. Earlier report also has
three complete punch traces. Previous rg directory searches silently skipped
ignored logs, causing my incorrect conclusion; logging itself was working.

**Still broken / not tested:** Kill processing/attribution, range rejection,
empty aim and block priority are **not tested** by these results. No new tests run.

**Next:** No reinstall/repeat fight needed. To reproduce the log review run
`rg --no-ignore SCENE_ATTACK logs/scene-report-20261010-222744`; expect three
SEND, three dispatch RECV and three RESULT lines. Continue combat when requested.

## 2026-10-10 User confirms outgoing punch

**Changed:** README and STATUS record successful user gameplay. No code/build change.

**Why:** User confirms punching damages a Skyrim enemy.

**Tested how:** Collected scene-report-20261010-221938; searched collected logs
and raw logs/step2 for attack events. Collector includes all event=SCENE_ lines.

**Result:** Gameplay confirmed by user. No SCENE_ATTACK events in report or
SEND/RESULT in raw bridge logs; numeric damage not independently verified.
README update succeeded; atomic STATUS replacement failed with access denied,
then completed by writing the already prepared bytes directly.

**Still broken / not tested:** Exact damage, range/empty aim rejection, block
priority and regressions are **not tested** by this report. Missing attack logs
unexplained. Existing weapon/animation limitations remain.

**Next:** No rebuild/reinstall needed. For another report run
`.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines and report path.
Choose next gameplay increment when requested.

## 2026-10-10 Basic outgoing punch (0.10.0)

**Changed:** Planned in DESIGN, added SkyrimAttack/startAttack mixin without
cancelling vanilla input. Real MC blocks/entities take priority. Snapshot publishes
crosshair handle; frame request includes sequence/handle/first-send timestamp.
Native attack.rs validates actor handle/type/range/cell/health and calls DoDamage
37335 with player source, fixed10 damage and Skyrim difficulty enabled. Reused
existing reference-counted ActorRef lifetime wrapper. Added post-attack load/menu
checks, protocol24, versions and documentation.

**Why:** User approved attacking Skyrim enemies from Minecraft while preserving
block breaking. User clarified the previous not-ready hit happened after closing
Minecraft while chased; it is expected native behaviour, not an active defect.

**Tested how:** Read local CommonLib DoDamage signature/Address Library ID, form
type and reference layout; local javap confirms Minecraft.startAttack returns
boolean. Built twice using build-step5b.ps1 -Offline -PackageOnly, including final
post-attack guards. Rust/Fabric passed. No test suite/game run; Gradle test NO-SOURCE.

**Result:** 0.10.0 packages ready. Protocol24, connection424, world size unchanged,
magic PTWORL12, pt_mc_frame11. No installs or commits. New gameplay **not tested**.
Existing linker informational and Java deprecation warnings remain.

**Still broken / not tested:** No weapon/enchantment/durability stats, Skyrim swing,
critical hits or knockback. Same-cell feet range is conservative; stale snapshot
handles may discard quick aiming changes. Native targeting/DoDamage health effect,
block-priority behaviour and combat reactions require game verification.

**Next:** Install, click a close hostile NPC twice, try out-of-range/empty aim,
break a real MC block, and collect outgoing attack/target-health logs. Confirm
incoming damage and dialogue regressions separately in the same session.

## 2026-10-10 Active incoming damage accepted (0.9.9)

**Changed:** Documentation only: README, DESIGN and STATUS record accepted live
nonlethal forwarding and the observed fallback limitation. No source/build change.

**Why:** User reports damage worked with the second interception attempt.

**Tested how:** Collected report20261010-220115 and compared native clamp/capture/
ack with Minecraft server apply events and health samples.

**Result:** Generation3 sequences1..3 captured once, applied once (accepted=true)
and acknowledged once. Raw damage6.789/4.526/4.526, max100; Minecraft requested
1.3578/0.9052/0.9052, health20 ->18.6422 ->17.737 ->16.831799. Native health100
at all three clamps and all exported health samples. Later Minecraft samples
rise to17.831799. No duplicate applications in these records.

**Still broken / not tested:** A fourth hit at177713671 passed through not_ready,
reducing native health to93.211 after coverage loss/save activity. Do not claim
unconditional native health protection. Exact cause of readiness loss not proven.
Other damage sources, lethal/death/respawn and full healing remain unverified.

**Next:** Stabilize policy for not-ready/disconnect protection and then death/
respawn, before broader combat. No new test required for this documentation update.

## 2026-10-10 Failed mutation hook and second clamp-hook attempt (0.9.9)

**Changed:** Replaced ModActorValue slot6 interception with primary player
CheckClampDamageModifier slot0x127. Prior clamp invoked once; health input/output
and current health logged. Negative returned health delta is queued under the
existing lease and replaced by zero when captured. Queue/ack/scaling/MC damage
unchanged. Updated DESIGN, version metadata, README/ABI/STATUS.

**Why:** Report 20261010-215421 shows readiness true/max100, two -6.789 probes,
but no mutation capture/passthrough records. Enemy damage bypassed our old virtual
entry. This is the second attempt at incoming damage forwarding, not a proven fix.

**Tested how:** Collected logs and inspected CommonLib Actor.h/Actor.cpp:
CheckClampDamageModifier returns float, takes ActorValue and delta, non-VR slot
0x127. Local capstone unavailable; no disassembly performed. Built package using
build-step5b.ps1 -Offline -PackageOnly. Runtime path/effect **not tested**.
No test suite or game run. No game-install writes or commits.

**Result:** First forwarding attempt failed despite readiness. Second interception
uses a documented interface with diagnostic logs to establish actual invocation.
No claim that the game uses it for these hits until logs confirm.

**Still broken / not tested:** Shared damage remains unverified, as does returning
zero from this callback. Death/respawn/scripted setters remain unsupported.

**Next:** Install 0.9.9 and take one nonlethal hit. Collect logs. If it fails,
stop hook variations and prepare STATUS-handoff per AGENTS.md two-attempt rule.

## 2026-10-10 Forward incoming damage to Minecraft (0.9.8)

**Changed:** Added damage.rs mutation hook and bounded shared queue; Minecraft
BridgeDamage validates context and applies each event once on integrated server.
Hook targets the actual player's ActorValueOwner+0xB8 slot 6, negative health
modifier 2/value 24. Suppresses only after enqueue with fresh server readiness.
Maximum-health fraction uses permanent health plus temporary modifier (38469).
Minecraft hurtServer(generic) keeps native armour/cooldown/absorption/death rules.
Added acknowledgement/readiness fields, versioned world damage payload, logs,
protocol 23 and package/docs updates. Existing mutation handler retained outside
readiness and for other actor values/modifiers.

**Why:** User explicitly requested building damage forwarding and preventing
separate native health loss after selecting Minecraft health authority. The
observed HandleHealthDamage callback is too late for suppression.

**Tested how:** Read CommonLib ActorValueOwner damage/restore implementation,
modifier enum, maximum-health computation; inspected Minecraft hurtServer API
using local javap. Built twice with build-step5b.ps1 -Offline -PackageOnly;
Rust/Fabric passed, Gradle test NO-SOURCE. No test suite or game run.

**Result:** Final packages Minecraft 326169 bytes / Skyrim 220335 bytes. Source
review caught damage count shadowing collision count; corrected before final
package and confirmed separate loops. Added serialized hook installation and
readiness diagnostics. Previous read attempt was blocked by approval-service usage
limit, resumed successfully. No game-install writes or commits.

**Still broken / not tested:** All new runtime behaviour is **not tested**.
Actual enemy mutation interception and maximum-health API need game confirmation.
Full death/respawn presentation, direct health setters and attack forwarding remain
out of scope. No durable event delivery through crashes; pending hits can expire
or be cancelled. Native damage resumes outside readiness/queue capacity. Existing
linker informational and Java deprecation warnings remain.

**Next:** Install, verify stable load/movement, take two nonlethal hits separated
by one second, retreat and heal using Minecraft food. Compare capture/apply/ack
and both health values before claiming the authority path works.

## 2026-10-10 Damage callback ordering established for two enemy hits

**Changed:** Documented 0.9.7 probe results in README, DESIGN and STATUS. No code
or package changes.

**Why:** User completed two enemy hits followed by regeneration.

**Tested how:** Collected report 20261010-120922 with collect-scene-logs.ps1;
read probe events and compared health snapshots by sender/world/epoch/snapshot.

**Result:** Hook installed successfully. Sequence 1 at 142166187: supplied=-6.789,
before=after=93.211. Sequence 2 at 142168406: supplied=-4.526,
before=after=89.694016. Both attacker 000EBC4D, on different callback threads.
measured_loss=0 inside both original calls: health mutation precedes this callback.
Sampled health drops from 100 to 93.256, then 94.16464 to 89.73967, returning to
100 at 142190703. All 587 received snapshots match out of 588 exports; client
health remains 20. No native ERROR/LOAD_FAILED events found.

**Still broken / not tested:** Callback-only suppression cannot implement Minecraft
health authority. Other damage sources, lethal hits and an earlier mutation path
are not tested. No damage forwarding or health writes implemented.

**Next:** Investigate the actual health-mutation entry point before suppression;
use the signed callback data only with verified semantics and duplicate guards.
No new test or build required for this documentation update.

## 2026-10-10 Minecraft health policy and individual damage probe (0.9.7)

**Changed:** User selected Minecraft-owned health and food/healing. DESIGN now
specifies staged damage-event verification, acknowledged transport, and eventual
Skyrim health suppression/death integration. Implemented only native damage probe
in skse.rs: PlayerCharacter vtable slot 0x104, original invoked once, before/after
health through corrected +0xB8 accessor, unique sequence/attacker/load revision/
thread diagnostics. Expanded vtable address bound and updated versions/docs.

**Why:** Sampled health deltas can combine damage/healing or loads. Verify actual
callback semantics before writing hearts or suppressing Skyrim damage.

**Tested how:** Read local CommonLib Actor::HandleHealthDamage declaration and
RelocateVirtual implementation (non-VR slot 0x104). Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly`: Rust/Fabric passed. No test
suite or game run; Gradle test NO-SOURCE.

**Result:** 0.9.7 packages built, protocol remains 22. Original damage behaviour
preserved intentionally. No installs, commits or health writes. New probe runtime
behaviour **not tested**; ownership policy is not yet implemented end-to-end.

**Still broken / not tested:** Callback ordering/amount semantics, shared damage,
maximum-health scaling, death/respawn, disconnect authority and attack forwarding.

**Next:** Install, verify loading, receive two enemy hits and inspect probe logs.
Only after evidence should the damage transport and health authority be enabled.

## 2026-10-10 Plan health ownership before damage writes

**Changed:** Added common damage, duplication, load-baseline and scaling rules to
DESIGN. Asked user to choose Minecraft or Skyrim health/healing authority.
Documentation only; no source, package, or game-file changes.

**Why:** Movement authority does not determine how food, healing and death should
work. Applying sampled health differences directly risks missed or duplicate hits.

**Tested how:** Read current design and lifecycle code references. No tests run.

**Result:** Common rules documented; health ownership decision pending. Existing
0.9.6 read-only build remains current. No new gameplay to test for this update.

**Still broken / not tested:** Damage writes, attack forwarding and death/respawn
synchronization remain unimplemented. No new runtime behaviour claimed.

**Next:** Resolve the requested gameplay preference, then plan and implement one
scoped nonlethal damage increment with diagnostics.

## 2026-10-10 Health observation accepted under damage and regeneration

**Changed:** Documentation only: README and STATUS record the successful health
test. No source, architecture or package changes.

**Why:** User reports travelling to an enemy, receiving two hits and waiting
for regeneration after each.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1`; parsed report
20261010-120304 by sender/world/epoch/snapshot and compared native health values.

**Result:** 1043 send and receive snapshots, all matching, zero mismatches or
missing send keys. Observed 100 -> 93.22922 -> 100 (14.953 seconds to full from
first sampled low), then 100 -> 93.28407 -> 100 (14.407 seconds). 1043 client
records keep Minecraft health at 20. No native ERROR/origin-block events found.
Sampling does not establish exact hit damage or exact regeneration start time.

**Still broken / not tested:** Cross-game damage/death and attacks are not
implemented. This confirms read-only health transport, not health synchronization.
No test suite run; no extra gameplay test required for the documentation update.

**Next:** Design damage ownership and death/load baselines before health writes.

## 2026-10-10 Verify 0.9.6 load stability and baseline health

**Changed:** Documentation only: README and STATUS record the user-run stability
check and numeric health comparison. No code or package changes.

**Why:** Verify the crash correction before requesting damage/regeneration tests.

**Tested how:** Collected `.\tools\collect-scene-logs.ps1` report
20261010-115741. Parsed native health records by sender/world/epoch/snapshot,
compared values, and inspected collected errors and final save events.

**Result:** 329 exports across 69719 ms; 327 received snapshots all match exactly,
zero mismatches or unmatched receive keys. 327 client records. Skyrim health 100,
Minecraft health 20. Later Skyrim save events show continued operation. No logged
ERROR/LOAD_FAILED/menu-disabled or Minecraft sender-disabled events found.
The load-time crash was not reproduced during this run; not a general stability
guarantee. Two intermediate exports were not received in latest-snapshot transport.

**Still broken / not tested:** Damage/regeneration and cross-game health writes
remain untested/unimplemented respectively. No automated test suite run.

**Next:** Use installed 0.9.6 for ordinary Skyrim damage/healing outdoors, then
collect logs again. No install or rebuild needed for this documentation update.

## 2026-10-10 Correct health-read offset after load crash (0.9.6)

**Changed:** Corrected world_export ActorValueOwner subobject offset from 0xB0
to 0xB8 for Skyrim 1.7.104. Updated version metadata, corrected DESIGN/ABI offset
claims and recorded failed 0.9.5 gameplay in README/STATUS. Protocol unchanged.

**Why:** User reports save-load crashes and could not test damage/regeneration.
I incorrectly treated the older Actor layout as valid for all SE/AE runtimes.
Local Actor.h AsActorValueOwner uses RUNTIME_CAST_ACCESSOR_VERSIONED with
threshold 1.6.629, old offset 0xB0, new offset 0xB8. The macro selects the new
member offset for this runtime. This is a concrete ABI error in the new call.

**Tested how:** Collected scene-report-20261010-115402, which ends after save
admission/alignment before health export. Read versioned accessor/macro and
built `.\tools\build-step5b.ps1 -Offline -PackageOnly`; Rust/Fabric passed.
No crash dump was inspected; logs alone do not establish the faulting instruction.
No game test or test suite run; Gradle test NO-SOURCE.

**Result:** 0.9.6 packages built. Runtime crash repair **not tested**. No install
writes or commits. Damage/healing remains untested, not a failed gameplay test.

**Still broken / not tested:** Must confirm save loading and first health read
survive. Then assess damage/healing. Existing informational linker warning remains.

**Next:** Install patch, load same save with paired Minecraft active, wait 30
seconds and walk briefly. Collect logs before asking for further damage testing.

## 2026-10-10 Read-only Skyrim health feed (0.9.5)

**Changed:** Planned combat in stages in DESIGN; implemented health observation
only. world_export reads ActorValueOwner::GetActorValue(kHealth) on the existing
player update thread. world snapshot carries f32 current health at reserved
header offset 68. Native SEND/RECV and Minecraft client logs include snapshot
identity and health; client log also records Minecraft health. Updated package
versions, protocol 22, PTWORL10 magic, README/ABI/STATUS. Packet size unchanged.

**Why:** Establish damage readings before choosing health/death authority or
applying damage across games. Dialogue/menu repairs were accepted in 0.9.4.

**Tested how:** Read local CommonLib Actor/ActorValueOwner/ActorValues definitions
(ActorValueOwner +0xB0 for SE/AE, vtable slot 1, health enum 24). Built with
`.\tools\build-step5b.ps1 -Offline -PackageOnly`; Rust/Fabric passed. No test
suite or gameplay run; Gradle test NO-SOURCE. New game behaviour **not tested**.

**Result:** 0.9.5 packages ready in dist/step5b. No game installs, commits, health
writes or attack dispatch. Nonfinite readings fail snapshot validation; health
logs require active admitted snapshots to avoid false disconnect zeroes.

**Still broken / not tested:** No cross-game damage, death synchronization or
attacks. Health readings under damage/healing need user verification. Sampling
follows collision exports and may miss short-lived changes; paused menus suspend it.

**Next:** User installs and experiences Skyrim damage/healing, then collect and
compare health logs. Decide health/death rules before implementing health writes.

## 2026-10-10 Dialogue and interior return acceptance

**Changed:** Updated README and STATUS with user-confirmed 0.9.4 behaviour.
No code, architecture or package changes.

**Why:** User confirms single-step dialogue selection, Enter and Backspace; also
reports intentional interior blocking and successful outdoor reconnection.

**Tested how:** User played both games. Collected logs directly with
`.\tools\collect-scene-logs.ps1`; read report 20261010-114651 and counted native
menu dispatch sequences. No automated test suite run.

**Result:** 14 dispatches, zero duplicate sequences, contexts stable across
callback threads. Interior cell 000133C8/worldspace 0 blocked reason 4 at
140786765; origin_ready resumes at 140792812 with unchanged saved origin.
User confirms the visible dialogue actions and outdoor return behaviour.

**Still broken / not tested:** Interior bridge play remains unsupported. Other
menu types, rebinding and held-key suppression are not tested in this report.

**Next:** The three scoped recovery repairs now have user acceptance. Choose the
next gameplay increment; no implementation started in this documentation update.

## 2026-10-10 Shared menu state repair (0.9.4)

**Changed:** menu_input.rs now keeps context/fingerprint/last-command in one
process-wide mutex. UI callbacks use a nonblocking lock, including dispatch, so
reentrant/contending callbacks skip safely. Only mapping handles remain thread-local.
No unsafe Send implementations added. State/receive/result logs include callback
thread ID. Updated versions, DESIGN plan, README, ABI and STATUS.

**Why:** 0.9.1 report showed oscillating context numbers and duplicate dispatch.
Different callback threads had independent state. This is the next scoped menu
repair; event routing is unchanged until runtime evidence warrants another change.

**Tested how:** Collected report 20261010-114219; recovery token 1 server/client
positions match and Skyrim accepted with error_squared=0 (47 ms after request).
Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`: Rust/Fabric passed.
No new tests or test suite run; Gradle test NO-SOURCE. Menu gameplay **not tested**.

**Result:** 0.9.4 packages built. Protocol remains 21, PTWORLD9; no save format
changes. No game install writes or commits.

**Still broken / not tested:** Actual dialogue highlight/confirm/back remains
unverified. Ordinary menu events might need a dialogue-specific path; do not
claim fixed merely from a returned dispatch. Save quickload/marker stability
beyond the observed recovery handshake is not independently established by logs.

**Next:** User tests R, single Down/Up, Enter and Backspace, then movement resume.
Collect logs; expect stable context and no repeated dispatch for the same sequence.

## 2026-10-10 Save-position recovery handshake (0.9.3)

**Changed:** Added native recovery.rs and Minecraft BridgeRecovery, using the
saved origin to convert the loaded Skyrim position into a Minecraft teleport.
Integrated-server connection performs the teleport; server/client agreement
produces a token acknowledgement. Movement/rendering remain held until native
acceptance and fresh coverage. Added timeout/session/load/location guards and
teleport deduplication across transient missing snapshots. Added logs, versions,
README/ABI/STATUS and implementation plan in DESIGN. PBOR mapping is unchanged.

**Why:** Step 2 of the recovery plan. Step 1 accepted by the user; collected report
20261010-113517 confirms outdoor cells 9731 and 9712 accepted with worldspace 3C
and mapping preserved. Previously, matching save lineage could fail reason 5
because current Minecraft and loaded Skyrim positions disagreed.

**Tested how:** Read collected logs, local javap teleport API signatures and source.
Built twice with `.\tools\build-step5b.ps1 -Offline -PackageOnly`, including a
final rebuild after transient-gap deduplication. No test suite or game run;
Gradle test NO-SOURCE. Both builds passed with existing informational warnings.

**Result:** 0.9.3 packages: Minecraft 311896 bytes, Skyrim 212826 bytes. Protocol 21,
PTWORLD9 397488 bytes, pt_mc_frame9; connection remains 384 bytes. No installs,
commits or game-file writes. New game behaviour **not tested**.

**Still broken / not tested:** Dialogue arrows/context duplication remain pending.
Recovery, quickload, marker stability and stale-world guards need game testing.
No historical inventory rollback, different-worldspace/interior travel, or height
outside Minecraft's build limits. After timeout the bridge holds for diagnosis.

**Next:** User installs 0.9.3 and tests the previously failing paired load plus
quicksave/walk/quickload. Collect token/position logs, then address menu state.

## 2026-10-10 Outdoor cell continuity workaround (0.9.2)

**Changed:** Planned three independent recovery steps in docs/DESIGN.md. Implemented
step 1 only: Origin.same_location permits distinct nonzero cells within the same
nonzero worldspace; interiors require exact cell. Used by saved-origin restore and
menu eligibility. Added outdoor_cell_accepted diagnostic, bumped package versions,
and updated README/ABI/STATUS. No saved-origin format, coordinates or protocol change.

**Why:** Collected scene-report-20261010-112905 directly. Walking from cell 00009732
to 00009731, worldspace 0000003C, failed reason 4 at time_ms=139650046. The apparent
range limit was an exact-cell check. Initial load separately failed reason 5 after
successful lineage admission. Menu tokens alternate and sequence 10 dispatches
twice, consistent with separate thread-local state across UI callback threads.

**Tested how:** Read collected logs and source. Ran
`.\tools\build-step5b.ps1 -Offline -PackageOnly`; Rust and Fabric build passed.
No new tests or test suite run; Gradle test NO-SOURCE. Game behaviour **not tested**.

**Result:** 0.9.2 packages ready in dist/step5b. Existing coordinate mapping,
Minecraft world UUID/lineage checks and 140-unit position guard are preserved.
Cell transitions still clear/rebuild collision coverage through the existing path.

**Still broken / not tested:** Save-position recovery and dialogue arrows remain
known failures. Native menu dispatch has duplicate/context problems; use Skyrim
controls for choices until step 3. Outdoor crossing and marker stability need user
verification. Doors, different worldspaces and fast travel remain unsupported.

**Next:** Install and cross the previously failing outdoor boundary in both
directions; collect logs. Then implement explicit save-position resynchronization
without shifting existing blocks, followed by one shared menu command state.

## 2026-10-10 Menu navigation from Minecraft (0.9.1)

**Changed:** Added SkyrimMenus key bindings and native menu_input UI tasks; extended
NativeLink, position transport, connection status and save-load revision tracking.
Protocol 20 uses a 384-byte connection. Minecraft holds movement and suppresses
scene export during fresh Skyrim menu status. Added menu send/receive/state logs.
Updated DESIGN before implementation, plus README, ABI, STATUS and package versions.

**Why:** NPC activation works; the next gameplay increment is selecting dialogue
choices and leaving Skyrim menus without switching controls to Skyrim.

**Tested how:** Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`.
Read local CommonLib ABI definitions. No game testing or test suite was run;
Gradle reported test NO-SOURCE.

**Result:** Rust and Fabric builds passed. Packages: Minecraft 304808 bytes,
Skyrim 209951 bytes. Existing linker informational warning and Java deprecated-API
note remain. Post-build inspection was temporarily blocked by automatic approval
review's usage limit; it resumed successfully on continuation. No install writes.

**Still broken / not tested:** All new menu behaviour is **not tested** in game.
No mouse/text input, held-key repeat or shared item inventory. Rapid/simultaneous
commands may be dropped because the transport retains only the latest command.
Different Skyrim menus may handle these ordinary menu events differently.

**Next:** Install 0.9.1, test NPC dialogue arrows/Enter/Backspace and a pausing
container, then collect logs and compare sequence/action/context and movement hold.

## 2026-10-09 Confirm NPC interaction (0.9.0)

**Changed:** Updated README and STATUS with confirmed NPC interaction. No code or package changes.

**Why:** User reports they can talk to Skyrim NPCs using the new interaction key.

**Tested how:** Collected report logs/scene-report-20261009-214457 and compared SCENE_INTERACT events.

**Result:** Minecraft sent sequence=1 at 21:44:01. Skyrim dispatched that sequence once to target_handle=1058908 with age_ms=0 and logged return. Menu blocked=true followed 16 ms later; blocked=false followed later. Three additional Minecraft input presses were discarded as inactive_or_menu. User observation confirms NPC conversation opened; the native return alone would not prove that.

**Still broken / not tested:** Menu navigation from Minecraft is not implemented. Non-NPC activation, rebinding persistence, held-key suppression and empty-target handling are not confirmed by this report.

**Next:** Implement Skyrim menu input routing as a separate increment when requested. No rebuild needed for this evidence update.

## 2026-10-09 Add Minecraft-to-Skyrim interaction key (0.9.0)

**Changed:** Planned gameplay-first priority in DESIGN. Added remappable R binding through an Options.load mixin and English label. Each eligible press sends a short-lived sequence with native issue timestamp and duplicate/expiry/cooldown/resume checks. Calls Skyrim ActivatePickRef using its crosshair target after pairing/origin admission. Added SCENE_INTERACT_SEND/RECV/RESULT/INPUT/MENU logs. Menu counters suspend bridge movement/look/render control, including dialogue; menus/load/cell are rechecked after activation. Bumped protocol 19/link19, C pt_mc_frame7 and packages to 0.9.0; updated README/ABI/STATUS. No menu navigation, combat or travel work bundled.

**Why:** User wants practical gameplay interaction before further travel/save infrastructure.

**Tested how:** Read local CommonLib activation/singleton/UI definitions and Minecraft signatures with javap; inspected Options constructor ordering. Built tools/build-step5b.ps1 -Offline -PackageOnly. No automated tests added/run (Gradle NO-SOURCE), visual game inspection or agent writes to game installs.

**Result:** Final Rust/Fabric builds passed; both 0.9.0 packages produced. First Java build failed because org.lwjgl.glfw is not exposed by the current Minecraft classpath. Replaced it with verified Minecraft InputConstants.KEY_R; second build succeeded. Existing MSVC informational link and Java deprecated-API notices remain. A documentation write using Windows default encoding failed on an arrow character and truncated README. Recovered its full 0.8.9 contents from Git blob 653cfab86a44a601354d0f512072104332a12e4d, reapplied the known restart-acceptance update, then added 0.9.0 instructions using a byte-preserving temporary-file replacement. No commits, staging or Git resets performed.

**Still broken / not tested:** Binding/rebinding, runtime mixin loading, activation/reach, held-key suppression, menu gating/recovery and gameplay regressions are **not tested**. Function return is not object-action confirmation. Native menu input still requires Skyrim's window. Cross-cell doors remain blocked by saved-origin location checks. Target is Skyrim's current pick, not a Minecraft-selected object. Rapid presses can be dropped by latest-slot/cooldown rules.

**Next:** Install both adapters; test a nearby in-place lever/object with R, empty aim, held R, inventory suppression and rebinding. Collect logs directly and compare sequences to native dispatch plus user-observed effects. Menu navigation is the next increment.

## 2026-10-09 Accept scoped 0.8 restart milestone

**Changed:** Updated README, DESIGN and STATUS to record acceptance of the same-location restart milestone. No gameplay code or package changes.

**Why:** User followed the save/quit/restart procedure and reports the placed block and inventory remained present when using Resume Bridge.

**Tested how:** Collected report logs/scene-report-20261009-212809. Compared SCENE_ORIGIN state=saved from skyrim-19424-89071390.log with state=restored from skyrim-17024-89233968.log. Exact text comparison of world ID, cell/worldspace and origin fields returned True. Read current Minecraft pairing/inventory diagnostics.

**Result:** Saved, loaded and restored Minecraft origin [131.30122659251012, 100.0, -45.740949485334895] and Skyrim origin [17405.072265625, -47335.90234375, -141.60723876953125] match exactly. Cell 00009732/worldspace 0000003C and Minecraft UUID bytes match. Minecraft reports paired lineage and 62 dirt on join and later saves. User confirms the block/inventory remained. This meets the narrowed 0.8 restart acceptance scope; visual landmark placement was not inspected by the agent.

**Still broken / not tested:** The earlier pre-save session logged reason=5 after Minecraft changed from below-world Y to Y=100, then reloaded a legacy Skyrim save and created the final persisted origin. Cause of that coordinate change is not established; no claim that every setup attempt passed. Unrestricted altitude/travel, interiors, rollback/save revisions and auto-selection remain 0.9 work. Wrong-world, corrupt-origin and form-resolution failure cases are not gameplay-tested. This does not prove complete save-state equality.

**Next:** Begin 0.9 planning when requested, keeping 0.8.9 as the accepted same-location practice baseline. No rebuild needed for this documentation-only update.

## 2026-10-09 Implement scoped 0.8 restart alignment (0.8.9)

**Changed:** Updated DESIGN before code to finish 0.8 with same-location restart alignment; moved save revision rollback, doors/interiors, location construction and coordinate remapping to 0.9. Added saved_origin.rs and PBOR SKSE serialization with Minecraft world identity, Skyrim cell/worldspace IDs and both origin vectors. Restores after resolving saved form IDs, exact world/location checks and a 140-unit proximity guard. Invalid origin data remains blocked and is marked on subsequent saves. Added world UUID transport (from the existing bridge folder UUID), nonoverlapping 288-byte connection record, protocol 18/link18, pt_mc_frame6, PTWORLD8 hold reason and Minecraft messages. Updated the existing layout fixture header constant (not run), versions, README/ABI/STATUS.

**Why:** Re-anchoring current player positions on every restart shifts construction. A persisted mapping shared by movement/collision/rendering should keep the practice world fixed without implementing full save-history synchronization.

**Tested how:** Inspected current source, callback lifecycle, complete wire offsets, decoding bounds and admission order. Built tools/build-step5b.ps1 -Offline -PackageOnly successfully twice (final build after replacing a zero-length invalid marker with a nonempty marker). No automated tests added/run (Gradle NO-SOURCE), gameplay inspection, game-install writes or save-file edits by the agent.

**Result:** Rust and Fabric builds passed; both 0.8.9 packages created. No compile failures. Existing informational MSVC linker and Java deprecated-API notices remain. A prior attempted plan/edit was stopped by automatic approval review's usage limit before execution; after user continuation, source inspection confirmed no partial implementation and the work resumed.

**Still broken / not tested:** Restart-origin equality, SKSE origin form resolution, legacy first initialization, context/distance rejection and regression gameplay are **not tested in game**. Same-lineage older saves are not revision-matched. One practice cell/origin only; cell changes hold. Minecraft folder renames/copies are not independently identified (copies with the same folder UUID share identity). Save write success is not atomic two-game persistence. Legacy saves without PBOR establish a new initial mapping and cannot recover earlier mapping. 0.8 remains pending acceptance, not declared complete.

**Next:** Install both adapters. Place a marker by a landmark, stop, save Skyrim to a new slot with both games connected, Save and Quit Minecraft, close both and reopen those exact saves. Check marker stays fixed and gameplay resumes; collect logs directly and compare PBOR origin coordinates across the two sessions. Keep previous Skyrim slot for rollback.

## 2026-10-09 Confirm missing/mismatched identity rejection and recovery

**Changed:** Updated README and STATUS with completed 0.8.8 admission-test evidence. No code/package changes.

**Why:** User reports that saving an old unidentified Skyrim slot produced a mismatch, and loading the original paired slot restored the bridge as expected.

**Tested how:** Ran the collector and read SCENE_PAIRING, SCENE_ADMISSION and SCENE_SAVE_ID in report logs/scene-report-20261009-205719.

**Result:** Minecraft reports matched at 20:55:51, missing_skyrim_identity at 20:56:19, mismatch at 20:56:25 and matched at 20:56:47. Skyrim admission is true for paired ID 1E111A315F021C4B82760DF261418370, false for zero ID, false for newly saved ID 4280A21499D6A440A2173B97DDA9308D, then true again for the original ID. Expected Minecraft ID stayed unchanged. User confirms observed blocking and recovery.

**Still broken / not tested:** Save revision equality, persistent origin restoration and automatic world selection remain unimplemented. New unpaired-world first admission is not covered by this test.

**Next:** Continue persistent origin/restored-coordinate design. No rebuild needed for this evidence update.

## 2026-10-09 Confirm matched-save admission repair (0.8.8)

**Changed:** Updated README/STATUS with successful retest evidence. No code/package changes.

**Why:** User reports movement and rendering work normally again.

**Tested how:** Ran the collector and read report logs/scene-report-20261009-205054.

**Result:** Three Minecraft world openings report eligible=true and matched lineage 1E111A315F021C4B82760DF261418370. Skyrim reports SCENE_ADMISSION allowed=true with identical expected/loaded bytes. Combined with user feedback, matched-save startup and normal movement/rendering are confirmed.

**Still broken / not tested:** Mismatched-lineage rejection/recovery and new-world admission remain **not tested**. Persistent origin restoration and save revision matching remain unimplemented.

**Next:** Test a different Skyrim lineage followed by returning to the paired slot. No rebuild needed.

## 2026-10-09 Repair admission startup and identity encoding (0.8.8)

**Changed:** Recorded repair plan. BridgeWorld.marked no longer caches results before server.overworld exists; inactive position sends no longer request pairing metadata. Moved pairing ID from overlapping bytes 216–231 to unused bytes 240–255. Bumped protocol 17/link17 and packages to 0.8.8. Updated README/ABI/STATUS.

**Why:** User's 0.8.7 matched-save test failed. Collected report 20261009-204741 shows eligible=false three times and no pairing/admission events, while Skyrim loaded the correct lineage. Source tracing found new expected-ID lookup running during loading and permanently caching false. Also found Record.encode overwrites ID bytes with render_ms/render_session, which would prevent matching after eligibility was fixed. Both are agent-introduced 0.8.7 bugs.

**Tested how:** Collected logs directly, inspected cache call order and complete Record encode/decode field ownership. Built tools/build-step5b.ps1 -Offline -PackageOnly: Rust/Fabric passed. No automated tests added/run (Gradle NO-SOURCE) or save/install writes.

**Result:** Replacement 0.8.8 packages built; no compile failures. Existing informational MSVC and Java deprecated-API warnings remain. This is the first repair attempt after the failed 0.8.7 gameplay test.

**Still broken / not tested:** Runtime activation/admission recovery is **not tested**. No mismatch test was performed by the user; defer it until matched-save activation works. No save or pairing migration required.

**Next:** Install both adapters and retest only existing paired saves. Collect eligible/matched/admission logs before further changes.

## 2026-10-09 Verify pairing and enforce native admission (0.8.7)

**Changed:** Collected report 20261009-204217 and confirmed user-tested pairing. Planned native admission in DESIGN, then added expected ID to position samples, native pre-control comparison, identity-only rejected replies and admission flag in world snapshots. Java discards non-admitted geometry and suppresses scene/overlay export until fresh admitted coverage. Updated protocol to 16/PTWORLD7/link16, C entry to pt_mc_frame5, versions and documentation.

**Why:** Diagnostic pairing warned on mismatch but previously allowed mismatched Minecraft worlds to control Skyrim. Native checks must precede movement, look and rendering while still permitting first-time pairing discovery.

**Tested how:** Read SCENE_PAIRING in collected logs: unpaired at 20:38:44, paired/matched at 20:39:21, matched again at 20:39:36, all ID 1E111A315F021C4B82760DF261418370. Inspected native and Java transport/admission paths; built with tools/build-step5b.ps1 -Offline -PackageOnly. No automated tests added/run (Gradle NO-SOURCE) or game-install writes.

**Result:** Pairing/reopen confirmed by user and logs. Rust and Fabric builds passed and 0.8.7 packages were produced. Runtime 0.8.7 admission is **not tested**. Existing informational MSVC linker and Java deprecated-API warnings remain.

**Still broken / not tested:** Matching, mismatch, recovery and initial pairing admission need gameplay testing. Same-lineage older save revisions still pass. No save revision pairing, restored origin, automatic world selection or construction remapping. Released Skyrim can move normally under its own control.

**Next:** Install both adapters, check matched movement, mismatched hold and recovery to the original paired Skyrim save; collect logs directly when testing completes.

## 2026-10-09 Add explicit Minecraft save pairing (0.8.6)

**Changed:** Planned diagnostic pairing before code. Added published Skyrim lineage bytes to world snapshots, changed protocol to 15/PTWORLD6/link15 and size to 397456 bytes. Added BridgePairing confirmation, versioned world-root metadata, create-only writes, validation and matched/mismatch/missing/error logs and messages. Updated versions, README, DESIGN, ABI and STATUS.

**Why:** Associate the Minecraft bridge world with the Skyrim save lineage before implementing automatic save selection or restored coordinate origins.

**Tested how:** Inspected serialization, transport and metadata paths; built `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Rust and Fabric passed. No automated tests added/run (Gradle NO-SOURCE), no agent writes to game installs or existing saves.

**Result:** Both 0.8.6 packages built. Existing informational MSVC link and Java deprecated-API warnings remain. No compile failures. Earlier source read was blocked by automatic approval review's usage limit, not an unsafe-action determination; it executed successfully after the user resumed on Oct 9.

**Still broken / not tested:** Pair/decline/reopen/mismatch and IO failure paths are **not tested in game**. Confirmation uses the most recently received snapshot and does not provide a native handshake. Mismatch is a warning, not a movement gate. No origin restoration, save revision matching, plugin fingerprint, automatic save selection or pairing-file repair. Runtime writes only the new metadata file after user acceptance, never overwriting it. SKSE WriteRecord success is not whole-save durability proof.

**Next:** Install both adapters, accept pairing with the identified Skyrim save, quit/reopen Minecraft and check matched without re-prompting. Collect logs directly after the user finishes. Implement admission handshake before claiming mismatched worlds are prevented from controlling the puppet.

## 2026-10-08 Verify Skyrim save identity across restart (0.8.5)

**Changed:** Updated README and STATUS with confirmed save-identity persistence. No code or package changes.

**Why:** User saved an existing Skyrim game to a new slot, fully restarted Skyrim through SKSE, loaded that slot and saved to another new slot. Minecraft was not run and is unnecessary for this identity test.

**Tested how:** Ran the collector, producing scene-report-20261008-213256. The collector includes only the latest native session, so also read SCENE_SAVE_ID entries in the two most recent project-local Skyrim logs: skyrim-27040-3073265.log and skyrim-14360-3129937.log. Minecraft files in the report are older and were not treated as evidence for this test.

**Result:** Process 27040 logged missing_record, then created/saved ID 1E111A315F021C4B82760DF261418370. Process 14360 logged loaded/saved with the exact same ID. Identity persistence across a full process restart and subsequent Save As is confirmed.

**Still broken / not tested:** Minecraft pairing, restart-origin restoration, save revision matching and malformed-record handling remain unimplemented or not tested, as applicable. This confirms lineage identity only.

**Next:** Pair Minecraft save metadata to the Skyrim lineage and define revision/origin restoration checks. No rebuild needed for this documentation update.

## 2026-10-08 Add Skyrim save-lineage identity (0.8.5)

**Changed:** Planned save identity in DESIGN before code. Added save_identity.rs using SKSE serialization version 4 callbacks, a versioned 16-byte bridge ID generated by Windows CoCreateGuid, and SCENE_SAVE_ID diagnostics. Revert clears state; load restores identity; save creates one only when absent. Invalid/duplicate/unsupported records block writing our record for that loaded state. Registered callbacks after messaging succeeds, updated package versions and previously stale SKSE plugin version metadata, and updated README/ABI/STATUS. Protocol remains 14.

**Why:** Cell/worldspace IDs cannot distinguish Skyrim playthroughs. Persistent pairing needs a save-lineage identity before origin restoration can be safe.

**Tested how:** Inspected local CommonLib SKSE interface IDs, callback signatures and struct field order. Built `.\tools\build-step5b.ps1 -Offline -PackageOnly`: Rust and Fabric passed. No automated tests added/run (Gradle NO-SOURCE), game-install writes or save-file edits by the agent.

**Result:** 0.8.5 packages created. Existing informational MSVC linker warning remains; no compile failures. Disk writes are performed by SKSE only when the user saves in-game.

**Still broken / not tested:** Save/restart/load identity persistence and regression gameplay are **not tested**. This does not yet pair Minecraft, restore origins, fingerprint plugin load order or identify a save revision. Save copies/branches inherit identity; saves predating the record get a new one. Invalid record handling does not abort the game's overall save or preserve bad record bytes. No migration of existing saves by the agent.

**Next:** Install, save Skyrim to a new slot, fully restart and load that slot, save again, then directly collect logs to compare saved/loaded IDs. Keep the previous slot for rollback.

## 2026-10-08 Record pause-alignment gameplay feedback

**Changed:** Updated README and STATUS with user-confirmed behaviour and collected evidence. No gameplay code or package changes.

**Why:** User reports 0.8.4 behaves exactly as expected in the pause test.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1` under standing authorization and read SCENE_ALIGNMENT events from report `logs/scene-report-20261008-212546`.

**Result:** Three suspended origins were reused with reason=same_session_and_position. Two were rejected with reason=skyrim_moved_while_suspended: one with substantial horizontal movement, another with a 5.55-unit vertical change. Initial connection created an origin as expected; the final suspension has no recorded resume.

**Still broken / not tested:** User confirms the observed result, but logs do not prove every menu pause retains alignment. Cause of vertical movement during suspension is not established. Full-restart alignment, save binding and door travel remain unimplemented/not tested.

**Next:** Continue persistent alignment planning while retaining the documented movement-during-suspension limitation. No rebuild needed for this documentation update.

## 2026-10-08 Preserve alignment across temporary pauses (0.8.4)

**Changed:** Added a DESIGN plan before code. Skyrim retains an in-memory suspended origin while releasing normal camera/gravity/render/collision control. Resume requires matching Minecraft session/world, no Skyrim cell/load reset, and Skyrim player displacement at most one unit since suspension. Logs SCENE_ALIGNMENT suspended/resumed/new_origin with reasons. Bumped packages to 0.8.4, retained protocol 14, and updated README/STATUS.

**Why:** Previously every menu or missing sample discarded the origin, allowing a new relative alignment on resume and potential construction drift.

**Tested how:** Inspected release/reset/sample lifecycle and built `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Rust/Fabric builds succeeded; no automated tests added/run (Gradle NO-SOURCE), no game-install writes.

**Result:** 0.8.4 packages built. Existing informational MSVC linker warning remains. No compile failures.

**Still broken / not tested:** Pause/resume stability and regression gameplay are **not tested**. Movement while disconnected rejects reuse and can still realign construction. No restart persistence, save binding, door travel or construction remapping. Sprint unchanged.

**Next:** Install, place a block beside a landmark, pause/resume each game while stationary, and check it remains fixed. Collect SCENE_ALIGNMENT decisions directly when the user finishes testing.

## 2026-10-08 Collect and compare 0.8.3 location logs

**Changed:** Recorded user authorization in AGENTS.md to run the collector and read collected logs whenever needed. Updated README with measured location-feed results; no code/package changes.

**Why:** User completed testing and authorized direct log collection.

**Tested how:** Ran `.\tools\collect-scene-logs.ps1`; read report `logs/scene-report-20261008-212004` and compared SCENE_LOCATION_SEND/RECV pairs.

**Result:** Both sides match cell_id=00009732 and worldspace_id=0000003C at epoch=1 snapshot=1 and epoch=3 snapshot=17. Movement logs show an initial 6444 ms wait for a snapshot and a later 140 ms wait, both returning to ready. The cause of the later reset is not established by this report.

**Still broken / not tested:** Full-restart stability and location transitions are **not tested** by these same-session pairs. Persistent alignment/save binding remains unimplemented. No rebuild needed for documentation changes.

**Next:** Use direct collection for subsequent diagnostics; continue stable alignment work in a separate increment.

## 2026-10-08 Export Skyrim location identity (0.8.3)

**Changed:** Planned the first stable-location increment in DESIGN. Added cell/worldspace form IDs to world snapshots and Java receipt diagnostics, using reserved header bytes 112/116. Added SCENE_LOCATION_SEND/RECV transition logs. Bumped protocol to 14/PTWORLD5/link14 and package versions to 0.8.3. Updated README, ABI and STATUS.

**Why:** Persistent construction alignment needs game location identifiers rather than process addresses before save binding or coordinate remapping can be implemented.

**Tested how:** Inspected local CommonLib TESForm/TESObjectCELL layout, including the AE >=1.6.629 offset change. Built `.\tools\build-step5b.ps1 -Offline -PackageOnly` successfully for Rust and Fabric. No automated tests added/run (Gradle test NO-SOURCE), game inspection or game-install writes.

**Result:** Both 0.8.3 packages built in dist/step5b. Existing MSVC informational link warning and Java deprecated-API notice remain. No compilation failures. Snapshot size stays unchanged; incompatible older adapters are isolated by the new namespace.

**Still broken / not tested:** Runtime location reads, matching IDs across both logs and restarts, and regression gameplay are **not tested**. No save binding, construction remapping, door travel or stable alignment yet. Form IDs depend on plugin load order and cannot identify a save. Sprint work is deferred at the user's request.

**Next:** Install both adapters, collect a report in the same Skyrim cell, restart and compare location IDs. Then design persistent binding with plugin/save identity and alignment metadata before moving construction.

## 2026-10-08 Sprint report analysis (0.8.2)

**Changed:** Recorded analysis only; no gameplay code or package changes.

**Why:** User reported no apparent sprint speed increase and supplied scene-report-20261008-211142 after installing the current build.

**Tested how:** Read the three supplied report files. Matched Minecraft SCENE_BOUNDS snapshot IDs to native SCENE_SEND timestamps and calculated horizontal displacement divided by elapsed time across ten-snapshot windows. Read coverage transitions and Skyrim scan timings. No game inspection or automated tests.

**Result:** The 21:11:17–18 window measured 4.329 blocks/s. Later windows at 21:11:22–26 measured 5.261, 5.800, 5.627 and 5.716 blocks/s, showing faster movement consistent with sprinting. The only logged movement hold was the initial 548 ms wait for the first snapshot; no later coverage holds were recorded. Skyrim logged 256 scans, averaging 142.13 ms (109–364 ms). These are snapshot-window averages, not sprint-state measurements.

**Still broken / not tested:** The report does not record sprint key/state, so activation on every attempt is not proven. Sprint FOV and Skyrim sprint animation are not bridged. Some slower windows could reflect turns, obstacles or input changes; their cause is not established. The earlier supplied 210742 report used an older movement format and also showed faster movement near its end. A first read-only calculation command had a PowerShell brace error; corrected calculation succeeded.

**Next:** Explain the measured speed increase; investigate explicit sprint-state/input diagnostics if speed still fails on a repeatable attempt.

## 2026-10-08 Predict collision coverage ahead of movement (0.8.2)

**Changed:** Planned bounded predictive scan placement in DESIGN before code. Native exporter derives velocity from distinct rendered samples, rejects gaps/teleport-like motion, and shifts each new scan ahead using prior scan duration plus 150 ms (horizon 200–500 ms; lead capped at 1.5 horizontal/.75 vertical blocks). Keeps the scan fixed until completion and retains radius/density/work/freshness limits. Added collected SCENE_WORLD_SCAN timing/lead/publication diagnostics. Split Java coverage failure reasons and added hold duration/requested-bounds/coverage diagnostics. Updated versions to 0.8.2, retaining protocol 13, README/ABI/STATUS and recorded confirmed save resuming.

**Why:** Refreshing only around the player's scan-start position lets sustained motion approach stale coverage edges. A bounded lead can provide more forward coverage without increasing the ray count before measuring performance. Logs must distinguish bounds holds from stale/missing snapshots.

**Tested how:** Inspected exporter, sample timestamps, snapshot lifetime and Java movement gate. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added/run (Gradle NO-SOURCE), gameplay-log reads, visual game inspection or game-install writes. Final package files confirmed present.

**Result:** Rust release and Fabric build passed; both 0.8.2 ZIPs are in dist/step5b. Existing informational MSVC and Java deprecated-API warnings remain. No compile failures. Normal shell setup failed before execution; reviewed escalated project-only execution succeeded. Existing SCENE_ collector includes the new diagnostics without script changes.

**Still broken / not tested:** Smoother travel, sharp turns, jumps/drops, lead stability and hold reductions are **not tested in game**. Radius remains four blocks and sampling can miss thin shapes. Sudden motion and missing coverage still hold. No larger region, snapshot merging, exact shape export, scale/reach change, fluid containment or persistent Skyrim save/coordinate mapping. Hold durations measure check-state transitions, not rendered input latency.

**Next:** Close games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Resume the same bridge and follow README's walking/sprinting/turning/slope/stair/jump/short-drop route. Briefly pause/resume Skyrim and verify safe hold behavior. Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines) and supply the path plus observed pauses. Compare SCENE_WORLD_SCAN lead/timing with client/server SCENE_BRIDGE_MOTION reasons and hold_ms before expanding coverage further.

## 2026-10-08 Resume existing bridge saves and inventory diagnostics (0.8.1)

**Changed:** Planned this save-workflow increment in DESIGN. Added Resume bridge on Singleplayer, asynchronously reading Minecraft save summaries and offering the latest marked save by name/folder before opening through the normal loader. Added a new-world confirmation explaining separate inventory. Updated bridge messages. Added server join/save-return inventory logs with save/player identity, slot item IDs/counts, six equipment slots and XP. Updated versions to 0.8.1, retaining protocol 13, and updated README/ABI/STATUS. Recorded user-confirmed basic empty-world success.

**Why:** User raised inventory persistence when starting a world. New Skyrim bridge is first-time setup; existing Minecraft saves already own their player data and builds. Resuming must not silently create a fresh save or transfer inventory between unrelated saves.

**Tested how:** Inspected cached Minecraft save-summary, world-loader, confirmation-screen and player/inventory APIs. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added/run (Gradle NO-SOURCE), gameplay-log reads, game inspection or game-install writes. Confirmed both 0.8.1 ZIP package files exist.

**Result:** Final Rust/Fabric builds passed. Initial Java compile failed because screen handling moved from Minecraft to Minecraft.gui; corrected using inspected signatures and rebuilt successfully. Existing informational MSVC and Java deprecated-API warnings remain. Two normal shell calls failed before execution with sandbox setup-refresh errors; reviewed escalated project-only commands succeeded. No save migration or independent inventory writer was introduced.

**Still broken / not tested:** Resume UI, runtime hooks, missing/locked/multiple-save cases and inventory persistence round trip are **not tested in game**. Diagnostics do not compare item components, enchantments, ender chests or disk durability. Resume selects the most recently played Minecraft bridge, not the matching Skyrim save. Stable construction alignment, paired save/rollback, long-travel coverage and height-window mapping remain pending. Marked saves still rely on their generated display-name marker.

**Next:** Close games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Resume the existing bridge, note distinctive inventory/equipment/XP, Save and Quit to Title, then Resume bridge again and compare. Keep Minecraft open and run .\tools\collect-scene-logs.ps1 (three COLLECTED lines); provide that report. Compare save_return and subsequent join_return for the same save/player. New bridge > Cancel should leave the existing save intact. Travel coverage remains the next separate implementation increment.

## 2026-10-08 Empty bridge world and local movement gate (0.8.0)

**Changed:** Planned step 1 details before code. Added New Skyrim bridge to Singleplayer, creating a unique marked save via Minecraft world creation with 128 air layers, void biome, no structures/decoration/lakes/chest and no natural mob spawning. Existing saves are not converted; active position and world suppression are gated to eligible saves. Added startup-ground and fresh-coverage movement hold on client/server, clearing velocity/fall distance while continuing stationary alignment samples. Added scan centre/radius to world snapshot headers, protocol 13 / PTWORLD4, coverage validation and SCENE_BRIDGE_WORLD/CHUNK/COVERAGE/MOTION logs. Packaged 0.8.0; updated README, DESIGN, ABI and STATUS.

**Why:** Minecraft generated terrain must not constrain Skyrim travel. The first implementation removes that second landscape entirely using a new save, while preserving player construction and Skyrim collision. An empty world needs a bootstrap hold so it does not drop the player before collision is available.

**Tested how:** Inspected cached 26.3 world-creation, flat generator/spawn, movement, gamerule and message APIs. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added or run; Gradle test tasks NO-SOURCE. No game visual inspection, gameplay-log reads or game-install writes. Confirmed final 0.8.0 package files exist. Collector already includes SCENE_BRIDGE_ via its existing filter.

**Result:** Rust release and final Fabric builds passed. First Java build failed on the removed displayClientMessage method; corrected using inspected sendSystemMessage/sendOverlayMessage APIs, then rebuilt successfully and rebuilt final diagnostics. Existing MSVC informational and Java deprecated-API warnings remain. Packages are dist/step5b/Passthrough-Minecraft-0.8.0.zip and Passthrough-Skyrim-0.8.0.zip. Matching JAR and both DLLs are required.

**Still broken / not tested:** UI/mixin application, creation/empty chunks, spawn height, readiness/hold/resume, server movement correction and actual Skyrim-only terrain traversal are **not tested in game**. The scan radius is still four blocks; fast movement/drop edges may hold. No proactive coverage expansion, infinite altitude, coordinate-window remap, stable construction across Skyrim origin/cell changes, old-build migration or matched saves. Normal-axis placement gaps, scale/reach calibration and fluid containment remain pending. Initial chunk counts run after terrain generation; expected zero is not yet observed in gameplay.

**Next:** Close both games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Stand on Skyrim solid ground; Minecraft Singleplayer > New Skyrim bridge. Expect initial waiting then ready; walk/jump a local route without Minecraft terrain, place/break stone to check deliberate collision, and test holding when Skyrim pauses/stops. Check ordinary saves show bridge inactive and retain normal Minecraft rendering. Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines) and supply the report. Check eligible=true, generated non_air_blocks=0, coverage bounds and both-side motion transitions before moving to travel coverage improvements.
## 2026-10-08 Plan removal of generated Minecraft terrain from bridge gameplay

**Changed:** Replaced the practice-terrain note in DESIGN with the next-priority plan: dedicated empty bridge save, bridge-world gating and safe startup, travel-ready collision coverage, stable Skyrim-coordinate persistence with bounded local windows, then construction verification and resumed fluid/scale work. Added README limitation/priority notice. Documentation only.

**Why:** User explicitly requires Skyrim travel to be independent of Minecraft caves/mountains. Retaining generated terrain was never the intended finished behavior. Removing only rendering or one collision query leaves other Minecraft terrain interactions in place.

**Tested how:** Read current design, collision mixin and bridge lifecycle references. No tests, build, game inspection or gameplay-log reads. User says buckets appear to work but this remains tentative.

**Result:** Concrete implementation order and acceptance criteria are recorded. Existing worlds are preserved; a separate empty save eliminates generated terrain across physics, rendering, targeting and fluids. Skyrim collisions and deliberately placed blocks still constrain movement. Coverage/lifecycle and coordinate limits are explicit follow-up requirements rather than a claim that a void world alone solves every travel issue.

**Still broken / not tested:** Current binaries still contain the reported restriction. New empty-world setup, startup hold, coverage changes, coordinate-window remapping and persistent world/cell binding are unimplemented and **not tested**. Scale/reach and fluid containment remain pending.

**Next:** Implement only step 1 of the new plan when requested. No rebuild/reinstall or gameplay test needed for this plan. Future implementation must provide exact world-setup controls and use .\tools\install-build.ps1 / .\tools\collect-scene-logs.ps1 with expected three INSTALLED / three COLLECTED lines, plus logged empty chunks and correct Skyrim grounding before movement resumes.
## 2026-10-08 Filled bucket placement on Skyrim surfaces (0.7.3)

**Changed:** Planned bucket targeting in DESIGN before code. Added water/lava buckets to the existing contact-preserving target path. Added an Item.useOn hook restricted to those exact items, air destinations and offline integrated-server sessions. Reused intent validation, then called Minecraft's bucket emptying/extra-content and item replacement helpers at the destination. Added SCENE_BUCKET diagnostics. Updated versions to 0.7.3, retaining protocol 12. Updated README/ABI/STATUS and recorded scale/reach feedback.

**Why:** The user accepts block placement for this iteration and requests the next design feature. Buckets need their own item-use path. Scale looks large and reach feels short to the user; calibration is pending rather than mixed into this feature.

**Tested how:** Read cached 26.3 BucketItem, ItemStack and client item-use signatures/call paths to preserve bucket inventory replacement and avoid double item-use statistics. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added or run (Gradle NO-SOURCE). No gameplay-log reads, visual game inspection or game-install writes.

**Result:** Rust release and Fabric build succeeded; both 0.7.3 ZIPs are packaged in dist/step5b. Existing informational MSVC and deprecated Java API warnings remain. No build failures. Normal Minecraft bucket targets, empty and special buckets remain on vanilla paths. Passing null hit to emptyContents prevents fallback into a different adjacent cell.

**Still broken / not tested:** Bucket terrain targeting, runtime hook, survival empty-bucket return, creative retention, source pickup, prediction/server correction and regressions are **not tested in game**. Fluids still do not interact with Skyrim terrain; no invisible support or simulation changes. Scale remains 70 Skyrim units/block, normal reach and collision radius four. True surface normals, fluid containment and previous rendering issues remain pending.

**Next:** Close both games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Follow README 0.7.3: give water/lava buckets and pour on separate Skyrim floor targets without starter blocks; expect fluid at the selected cell but still flowing through Skyrim. Check ordinary bucket use, waterlogging, creative retention, survival empty-bucket return and source pickup, plus stone placement/movement/HUD. Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines); supply the report and compare placement intent IDs with SCENE_BUCKET results. Terrain-aware fluid simulation is the next separate step.
## 2026-10-08 Separate terrain contact from placement packet (0.7.2)

**Changed:** Wrote the resumed design before code. Removed ray/grid-plane tangential shifting. Anchored destination cells to actual contact and clamped a separate packet point inside the cell. Added an offline intent mailbox (16 entries, 500 ms expiry, one-time server consumption, reset clearing) carrying contact/origin and player/session/hand/item identity. Revalidate actual surface on both sides; vanilla still owns placement, prediction and inventory. Added intent IDs and mapping=contact_cell logs. Bumped packages to 0.7.2, retaining protocol 12. Updated README/ABI and rewrote STATUS using templates/STATUS-handoff.md supplied by the user.

**Why:** The user resumed after two inadequate iterations. Earlier logs showed that solving packet geometry by moving the destination caused placement near the player. The contact and packet point now have separate roles.

**Tested how:** Inspected cached 26.3 prediction/item-use call paths; performUseItemOn runs before the normal packet is sent. Built .\tools\build-step5b.ps1 -Offline -PackageOnly. No automated tests added or run (Gradle NO-SOURCE), game inspection, new gameplay-log reads or game-install writes.

**Result:** Rust release and Fabric builds succeeded. Both 0.7.2 ZIPs exist in dist/step5b. Existing MSVC informational and Java deprecated-API warnings remain. No build failures. An apply_patch delete/add combination was rejected and replaced by a direct write; a README heading lookup failed and was corrected using existing headings. The handoff template was under templates, not missing as previously reported.

**Still broken / not tested:** In-game targeting, client/server intent association, prediction/rollback, expiry and inventory behavior are **not tested**. Sampled box-edge faces, four-block coverage and conservative overlap rejections remain. Normal-axis rounding still leaves sub-block gaps. Buckets, fluid containment and true surface normals are not added. This build is not claimed to solve all placement failures.

**Next:** Close games and run .\tools\install-build.ps1 (three INSTALLED lines; no rebuild). Follow README's near/far floor and wall procedure plus ordinary adjacency and survival consumption checks. Run .\tools\collect-scene-logs.ps1 (three COLLECTED lines) and supply the report. Compare intent IDs and contact-anchored cell coordinates against SCENE_PLACE_RESULT.
## 2026-10-08 Confirm displaced terrain placement; stop and hand off

**Changed:** Created STATUS.md and marked the placement blocker in README/DESIGN. No code change, build or install. STATUS-handoff.md was missing; a file search found no handoff template.

**Why:** User reports normal Minecraft adjacency works, but direct Skyrim placement is pulled toward the player. Two real placement iterations (0.7.0/0.7.1) have not met the goal; AGENTS.md requires stopping and documenting rather than trying more variations.

**Tested how:** Read only the three explicitly supplied scene-report-20261008-161512 files. Summarized placement decisions and compared native received scene metadata with sent metadata. No automated code tests, game inspection or other log reads.

**Result:** At snapshot 514 the sampled contact is 3.007263 blocks from the eye but the selected cell centre is only 0.872835 blocks away, with 1.589780 blocks horizontal displacement from the contact. Snapshot 707 records client/server successful dirt placement into the same displaced destination. Server decisions: 13 accepted, 5 occupied_terrain, 1 no_fresh_surface. All 875 received scene records match sent metadata/checksums. Source and logs identify 0.7.1's rounded grid-plane projection as a cause of destination movement; matching transport/server decisions do not establish correct placement.

**Still broken / not tested:** Direct terrain placement remains incorrect. Other misses, sampled-box edge normals and vanilla failures remain separate uncertainties. No replacement fix is implemented or tested. Wall placement is unconfirmed. Buckets/fluid containment remain pending.

**Next:** Review STATUS.md's proposal to separate terrain contact, destination cell and packet point, with explicit server validation and preserved surface normals where needed. No further patch under this investigation. No rebuild/reinstall needed. Future authorized reproductions: .\tools\collect-scene-logs.ps1, expecting three COLLECTED lines; read only user-supplied reports.
## 2026-10-08 Correct shallow-angle floor targeting (0.7.1)

**Changed:** Planned and corrected grid targeting: intersect the aiming ray with the outward rounded grid plane before choosing tangential cell coordinates. Previously, a shallow ray could miss the cell directly above the original contact and silently lose its target. Added throttled SCENE_TARGET_MISS and target decision/eye/reach/distance logs. Updated versions, README, DESIGN and ABI; protocol and collision-feed coverage are unchanged.

**Why:** User reports floor placement works only when very close. Source inspection found a concrete geometric rejection that can cause this; it is not established as the sole cause.

**Tested how:** Read the latest Minecraft log from the installation path named in AGENTS.md. Initial sandbox read was denied; escalated read succeeded. Logs show client/server occupied_terrain rejections, accepted targets followed by vanilla Fail, and matching successful dirt placement. Inspected the four-block-radius exporter and Java targeting. Built `.\tools\build-step5b.ps1 -Offline -PackageOnly`; no automated tests added or run (Gradle NO-SOURCE). No game visual inspection or installation writes.

**Result:** Rust release and Fabric builds succeeded, packaging both 0.7.1 ZIPs. Existing informational MSVC and Java deprecated-API warnings remain. No build failure. Removed the requirement that a shallow ray cross the cell vertically above the original floor hit, while retaining reach and obstruction checks against the actual surface.

**Still broken / not tested:** The range improvement is **not tested in game**. Four-block collision coverage still limits targeting; increasing it is not part of this correction. Terrain-overlap and vanilla placement failures remain possible, with the cause of the observed vanilla Fail results unconfirmed. Sampled tile edges, uneven ground and fractional-height grid gaps remain limitations. Wall placement has not been confirmed by the user. Buckets and fluid containment remain pending.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Follow README's 0.7.1 floor procedure, aiming at progressively shallower angles within coverage. Hold a failed aim steady for two seconds and click once, then run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines). Compare SCENE_TARGET/MISS with client/server placement decisions and final states before attributing any remaining short range to a particular cause.
## 2026-10-08 Direct placement against Skyrim terrain (0.7.0)

**Changed:** Expanded the step 6a plan before implementation. Tagged terrain and NPC collision shapes separately. Added terrain ray targeting after Minecraft's normal pick, preserving nearer vanilla hits. Selected the first whole grid cell outside the sampled surface, with a packet click point at the ray's entry into that cell. Added client/server air-target validation for fresh surfaces, reach, cell/face agreement, vanilla occlusion and terrain overlap, then retained normal BlockItem placement. Added SCENE_TARGET/SCENE_PLACE/SCENE_PLACE_RESULT logs. Updated package versions, README and ABI; retained protocol 12.

**Why:** User requested the next step after observing that block placement and water still only interact with Minecraft blocks. This increment implements solid-block placement first; fluid interaction remains separate.

**Tested how:** Read existing collision code and cached Minecraft signatures/call paths for targeting, BlockPlaceContext, item use and server packet validation. Built and packaged twice with `.\tools\build-step5b.ps1 -Offline -PackageOnly`, after the final call-path corrections. No automated tests added or run; Gradle test tasks were NO-SOURCE. No game inspection, game-install writes or extracted assets saved.

**Result:** Both builds succeeded. Final dist/step5b packages are Passthrough-Minecraft-0.7.0.zip and Passthrough-Skyrim-0.7.0.zip. Existing Java deprecated-API and informational MSVC warnings remain. Inspection caught two initial design mistakes before release: Entity.pick is not the current crosshair call path, and using the actual terrain contact as the packet point can violate vanilla's destination-cell distance check. Replaced the hook with Minecraft.pick and separated actual surface from the packet point. Initial shell rg wildcard paths produced path errors; subsequent searches used directory/glob arguments. No build failures.

**Still broken / not tested:** Runtime mixin application, direct terrain placement, packet acceptance, server correction, lifecycle changes and gameplay regressions are **not tested in game**. Grid alignment can leave gaps under one block. Thin/irregular sampled surfaces may be missed or cause occupied_terrain rejection. Support-dependent blocks retain vanilla restrictions. Buckets, water containment, terrain mining and Skyrim NPC collision with placed blocks are not added. Deferred cracks/camera issues remain.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Use `/gamemode creative` and `/give @s minecraft:stone 64`; place on Skyrim ground and walls without a Minecraft starter block. Check existing block placement, breaking, movement, NPC exclusion, reach, compass and drops. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines). Compare SCENE_TARGET with client/server SCENE_PLACE decisions and SCENE_PLACE_RESULT state. Bucket targeting and terrain-aware fluid simulation follow separately.
## 2026-10-08 Record terrain placement and fluid interaction limits

**Changed:** Recorded the user's 0.6.8 observations in README and added the next interaction sequence to DESIGN: terrain targeting/solid-block placement, bucket targeting, then fluid flow against terrain. Documentation only.

**Why:** Skyrim geometry currently supplements entity collision queries. It does not replace the Minecraft block data used by placement targeting and fluid simulation.

**Tested how:** User reports visible water flowing through Skyrim terrain and placement requiring existing Minecraft blocks. Read current collision mixin, source references and design. No new tests, build or game inspection.

**Result:** Both observations match unimplemented interaction paths. This is not evidence that movement collision has stopped working. No code fix has been made.

**Still broken / not tested:** Direct placement on Skyrim surfaces and fluid containment by Skyrim terrain are unimplemented. Lava, waterlogging and broader fluid rendering remain **not tested** by this report.

**Next:** Implement surface targeting and solid-block placement first, with matching client/server validation and logs. Buckets and terrain-aware fluid simulation follow separately. No rebuild or reinstall for these documentation changes. For additional evidence run `.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines.
## 2026-10-08 Step 5b water and lava surfaces (0.6.8)

**Changed:** Planned fluid surfaces in DESIGN before code. Added FluidCapture using Minecraft's FluidRenderer for still/flowing/waterlogged cells, section-coordinate rebasing, atlas remapping, tint/light and opaque/translucent routing. Added fluid counts at scene offsets 120/124, validation and SCENE_FLUID/SCENE_FLUID_DRAW plus send/receive diagnostics. Bumped adapters to 0.6.8 and protocol 12. Updated README and ABI. Glass/ice cracks remain deferred per the request to move to other features.

**Why:** Add another kind of world content using the existing geometry and pre-HUD rendering paths.

**Tested how:** Inspected local SkyCraft reference and cached Minecraft FluidRenderer/VertexConsumer signatures and vertex call. Built with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No game inspection, installation writes or tracked game assets.

**Result:** Rust release and Fabric builds succeeded; dist/step5b contains both 0.6.8 ZIP packages. Existing informational MSVC import-library and Java deprecated-API warnings remain. Initial lookup of a nonexistent BlockRenderDispatcher class failed; direct FluidRenderer inspection supplied the needed API. No build failures.

**Still broken / not tested:** Fluid appearance, flow/waterlogging, negative-coordinate alignment, transparency, depth, timing and regressions are **not tested in game**. Textures use static frames. Fluids follow Minecraft blocks, not Skyrim geometry; no underwater effects or Skyrim-water import. Existing glass/ice cracks and camera/world-bleed issues remain deferred.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Follow README's creative water/lava basin and flowing-water procedure, checking Skyrim walls and compass, waterlogging, drops and opaque cracks. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines). Compare fluid counts across matching send/receive/draw snapshots. Both adapters must use protocol 12.
## 2026-10-08 Confirm 0.6.7 and record missing glass/ice cracks

**Changed:** Updated README and DESIGN with user-confirmed drop-extension success and missing glass/ice breaking animation. Documentation only.

**Why:** Separate the working transparent/non-full drops from the remaining crack-rendering limitation.

**Tested how:** User gameplay report and inspection of CrackCapture's MODEL/isSolidRender/full-collision-shape filter. No new logs supplied or automated tests run.

**Result:** Drop extension is user-confirmed. Cracks still deliberately skip non-opaque/non-full blocks; glass and ice are outside the implemented crack path. No code fix has been made for that limitation.

**Still broken / not tested:** Glass/ice cracks are absent. Numerical 0.6.7 comparison and broader rendering/regression cases are **not tested** in this report.

**Next:** Extend crack geometry and transparent-surface ordering as a separate rendering increment. No rebuild or reinstall needed for these notes. Future logs: `.\tools\collect-scene-logs.ps1`, expecting three COLLECTED lines.

## 2026-10-08 Extend transparent and non-full drops (0.6.7)

**Changed:** Assessed this as a bounded extension of the existing model/transparency paths and planned it in DESIGN. BlockDropCapture now gathers directional and unculled default-model quads (maximum 96) without the opaque/full-cube restriction. Opaque/cutout faces remain in the item prefix; translucent drop faces join the sorted placed-face collection. Added dual-buffer rollback and removal of old dynamic faces on publication retries. Added explicit model and translucent-item counts at header offsets 112/116, revised validation/logging, and bumped adapters to 0.6.7 / protocol 11. Updated README and ABI.

**Why:** User authorized extending glass/pane/ice drops if it did not require a difficult redesign. Existing shaders, pre-HUD rendering and blending can handle the new geometry; no renderer redesign was needed.

**Tested how:** Inspected existing geometry, transparency, capacity and retry paths. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No game visual inspection or game-folder writes.

**Result:** Rust release and Fabric builds succeeded; packages are dist/step5b/Passthrough-Minecraft-0.6.7.zip and Passthrough-Skyrim-0.6.7.zip. Existing Java deprecated-API and informational MSVC import-library warnings remain. No build failures. The confirmed HUD hook and shader passes are retained; transparent drops sort together with placed transparent faces rather than using an independent layer.

**Still broken / not tested:** Glass/pane/ice model appearance, transparency, pickup, mixed-scene sorting, buffer rollback/capacity and regressions are **not tested in game**. Default-state models can differ from dedicated item models; panes use default-state geometry. Special block-entity renderers, item-specific components, out-of-block geometry and models over 96 quads use the icon fallback when available. Existing approximate sorting, both-sided opacity, roughly 10 Hz updates and work-budget limits remain. No broader rendering completeness is claimed.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Give/drop glass, glass panes, ice, stained glass and red sand using README's commands. Expect 3D drop geometry, transparent ice/stained glass, red sand still 3D, normal pickup and the compass staying visible. Check apple icons, placed glass and cracks. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines); compare models/items, opaque item_vertices and translucent_item_vertices on matching native snapshots. Client SCENE_ITEM should report block_model geometry and translucent counts where applicable. Both adapters must use protocol 11.

## 2026-10-08 Confirm HUD layering and record drop-model limits

**Changed:** Updated README and DESIGN with the user's confirmation that the HUD is no longer blocked, glass/panes/ice drops remain 2D, and red sand drops remain 3D. Documentation only.

**Why:** Distinguish the successful compass fix from the existing limits of dropped-block rendering.

**Tested how:** User gameplay report and inspection of BlockDropCapture eligibility checks. No new logs supplied or automated tests run.

**Result:** HUD layering is user-confirmed. The 3D drop exporter requires solid rendering, full-cube collision, one quad per direction and no translucent faces; excluded models use the icon fallback. Placed-block transparency does not change those eligibility rules. Reported glass/pane/ice behavior is consistent with this implementation, rather than evidence of a new regression.

**Still broken / not tested:** Translucent/non-full 3D drops remain unimplemented. Broader menu/HUD-hidden cases and numerical hook logs are **not tested** in this report.

**Next:** A separate increment can extend dropped models to transparent and non-full geometry. No rebuild, reinstall or additional gameplay test needed for these notes. For future reports run `.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines.

## 2026-10-08 Move world drawing before Skyrim HUD (0.6.6)

**Changed:** Recorded user-confirmed transparency and the compass overlap. Planned the correction in DESIGN before code. Added a HUDMenu PostDisplay vtable hook (AE ID 215362, slot 6), drawing world geometry before chaining the original HUD display. Reused the Present renderer's device/context with isolated state restoration and fresh back-buffer references. Removed world drawing from Present; hands/HUD remain there. Added SCENE_STAGE_HOOK/STAGE/WAIT/DISABLED diagnostics. Updated both package versions to 0.6.6, retaining protocol 10. Updated README and ABI.

**Why:** The previous Present world pass ran after Skyrim UI and could cover the compass. This changes the timing of world drawing without altering projection, blending or geometry transport.

**Tested how:** Inspected current Present path, local SkyCraft render hooks and pinned CommonLib HUDMenu/IMenu vtable declarations and IMenu::PostDisplay implementation. The latter invokes uiMovie->Display. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No game inspection or installation writes.

**Result:** Rust and Fabric builds succeeded; dist/step5b contains Passthrough-Minecraft-0.6.6.zip and Passthrough-Skyrim-0.6.6.zip. Only the existing informational MSVC import-library warning appeared in Rust; Java compile was up-to-date. No build failures. Geometry has one call site in the pre-HUD world method and no Present fallback. The original HUD method is called even if the new stage fails.

**Still broken / not tested:** Compass visibility, HUD hook invocation/render-thread timing, whether the back buffer at that point receives the final world image, menu transitions, transparency and other regressions are **not tested in game**. A first frame can skip world drawing until Present initializes its renderer. HUD-hidden modes or other HUD hooks can suppress this stage; repeated SCENE_STAGE_WAIT records identify absence on the Present thread. Unsupported multisampling or errors disable the new stage. There is no late fallback, because that would repeat the compass bug. This is not a verified ordering guarantee for every Skyrim menu or UI mod. Previously deferred camera/world-bleed issues remain.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild needed). With Skyrim HUD visible, put a tall Minecraft wall across the compass on screen. Expect compass above the blocks; repeat with stained glass and check drops, cracks, depth, hands/HUD and menu open/close. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines). Expect SCENE_STAGE_HOOK installed=1, recurring SCENE_STAGE stage=before_skyrim_hud present_world_draw=0 and SCENE_DRAW; one startup wait is possible. Send all three files and the visual result, especially if geometry disappears or still covers the compass.

## 2026-10-08 Step 5b translucent placed blocks (0.6.5)

**Changed:** After the user selected more step-5b rendering, wrote the translucent-block increment in DESIGN. Added a bounded TranslucentMesh buffer with quad-centre far-to-near sorting at publication. SceneCapture now exports translucent MODEL faces with texture/vertex alpha, reserves their capacity before drops, and sends their vertex count at header offset 108. Skyrim draws opaque blocks/items, cracks, then the translucent slice with alpha blending, depth testing and no depth writes. Added SCENE_TRANSLUCENT timing/count/eye diagnostics and matching send/receive/draw counts. Bumped both adapters to 0.6.5 and protocol/namespace to 10; updated README and ABI.

**Why:** Add placed stained-glass/ice rendering while retaining the working opaque/drop/crack paths and Skyrim wall occlusion. The user chose to continue rendering before step 6 interactions. Ground setup and deferred camera/world-bleed issues remain separate.

**Tested how:** Inspected existing Minecraft material-layer/quad export and D3D blend/depth code. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No game visual inspection or game-folder writes.

**Result:** Rust release and Fabric builds succeeded; packages are dist/step5b/Passthrough-Minecraft-0.6.5.zip and Passthrough-Skyrim-0.6.5.zip. Existing informational MSVC import-library and Java deprecated-API warnings remain. No build failures. Total geometry retains the 262140-vertex limit; items/cracks cannot consume reserved translucent space. Rendering order places glass over the cracks of opaque blocks behind it.

**Still broken / not tested:** Runtime HLSL behavior, glass/ice appearance, face order, occlusion, capacity behavior and regressions are **not tested in game**. Sorting is by quad centroid at roughly 10 Hz using Minecraft's eye; intersecting surfaces and camera differences may sort incorrectly. Disabled face culling can increase opacity for closed translucent models. Sorting is vertex-bounded but has no separate time budget; timing is logged. Fluids, refraction, colored lighting, animated textures and translucent 3D dropped-block models remain unsupported. Previously deferred issues are unchanged.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild). Give/place glass, blue/red stained glass and ice using README's exact commands. Expect Skyrim/blocks visible through translucent pixels, opaque surfaces hiding glass behind them, and Skyrim walls hiding both. Check overlapping colored blocks, cracks through glass, drops and hands/HUD. Run `.\tools\collect-scene-logs.ps1` (three COLLECTED lines), then compare translucent_vertices on matching SEND/RECV/TRANSLUCENT_DRAW snapshots. Stained glass/ice should produce nonzero counts; plain glass may use the cutout pass. Remove translucent blocks to check the count clears when none remain nearby. Install both adapters for protocol 10.

## 2026-10-08 Record successful 0.6.4 drop appearance

**Changed:** Updated README and DESIGN with the user's confirmation that dropped items and blocks appear as they should. Documentation only.

**Why:** Preserve the gameplay result before selecting the next implementation increment.

**Tested how:** User gameplay report. No new log files supplied or agent-run tests.

**Result:** Item/block appearance is user-confirmed. The earlier 0.6.3 pickup and disappearance confirmation remains recorded.

**Still broken / not tested:** Numerical mixed model/icon counts, broader occlusion/resource-pack/regression cases are **not tested**. Deferred camera/world-bleed issues and remaining rendering/interaction features are unchanged.

**Next:** No rebuild or reinstall needed for these notes. Future log collection uses `.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines. The design has remaining 5b rendering work and step 6 NPC interactions; direction is being clarified before code.

## 2026-10-08 Confirm icon gameplay and add 3D full-block drops (0.6.4)

**Changed:** Recorded the user's confirmation that 0.6.3 drops spin, can be picked up and disappear. Planned the next increment in DESIGN, then added BlockDropCapture for supported default-state full opaque blocks: six directional model quads, original face UVs, default block tint, quarter-block scale and existing bob/spin. Other items and unsupported geometry retain the icon fallback. Added an explicit item entity count at scene header offset 104; validated and logged mixed six-/36-vertex items on both sides. Updated versions to 0.6.4 and protocol/namespace to 9. Updated README, DESIGN and ABI.

**Why:** Improve the successful first iteration's flat block drops one step at a time, while preserving Minecraft-owned item motion and pickup. Camera/world-bleed/terrain changes remain deferred.

**Tested how:** Read local SkyCraft's full-block drop approach and cached Minecraft model/quad/tint API signatures. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No games launched or visually inspected, no game-folder writes. No fresh item logs were supplied for numerical comparison of the user-confirmed 0.6.3 behavior.

**Result:** Both Rust release and Fabric builds succeeded; dist/step5b contains Passthrough-Minecraft-0.6.4.zip and Passthrough-Skyrim-0.6.4.zip. Existing Java deprecated-API and informational MSVC import-library warnings remain. An initial API lookup used a nonexistent top-level QuadMaterialInfo name; the returned BakedQuad signature identified the nested BakedQuad.MaterialInfo class and the corrected lookup succeeded. No build failures.

**Still broken / not tested:** New 3D appearance, face orientation, tint, pickup/occlusion and regressions are **not tested in game**. Only six-face opaque full-block models are supported; additional/unculled quads and non-full/transparent blocks fall back to icons. Default block state ignores item-specific components. Item lighting/glint, arbitrary 3D item models, animated textures and interpolation between roughly 10 Hz exports remain unimplemented. Existing scan/item/time budgets can omit items; individual model errors roll back that item and log an error. The 0.6.3 user report confirms appearance/spin/pickup only, not all timing/occlusion cases.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines; no rebuild needed). Give dirt, oak logs and apples with `/give @s minecraft:dirt 8`, `/give @s minecraft:oak_log 8`, `/give @s minecraft:apple 8`; drop each using Q. Expect small 3D dirt/log blocks with distinct log bark/end faces and a flat apple icon. Check pickup, wall/block occlusion, cracks, textures, hands/HUD and movement/look. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft (three COLLECTED lines). Client SCENE_ITEM should report geometry=block_model vertices=36 or geometry=icon vertices=6; matching native records should agree on items and item_vertices, with model/icon counts in SCENE_ITEM_DRAW.

## 2026-10-08 Step 5b first dropped-item icons (0.6.3)

**Changed:** Added the dropped-item increment to DESIGN before code. Added DropCapture using real ItemEntity instances, interpolated positions, Minecraft bob/spin and the item model's particle sprite. Packed block/item atlases plus crack textures in memory; added identity-based sprite UV mapping and remapped block UVs for either combined layout. Appended item vertices before cracks in each completed mesh. Added header offset 100 item count, validation and item send/receive/draw diagnostics. Bumped adapters to 0.6.3 and protocol/namespace to 8. Updated README and ABI with limits and manual procedure.

**Why:** Continue step 5b with visible drops and pickup, using SkyCraft's simple icon fallback as the first bounded increment. Minecraft retains all item gameplay. Terrain/world setup, camera orbit and world bleed remain separate.

**Tested how:** Read the local SkyCraft item fallback and runtime atlas approach, inspected Minecraft 26.3 public ItemEntity, item-model, sprite and entity-iteration signatures, and built both adapters with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No games launched or visually inspected, and no game-folder writes.

**Result:** Rust release and Fabric builds succeeded. Packages are dist/step5b/Passthrough-Minecraft-0.6.3.zip and Passthrough-Skyrim-0.6.3.zip. Existing Java deprecated-API and informational MSVC import-library warnings remain. No build failures. Geometry stays in the existing solid depth/alpha-cutout pass; cracks remain last. New snapshots replace dynamic geometry rather than appending it again on retries.

**Still broken / not tested:** Dropping, falling, pickup, merging, wall occlusion, combined atlas reload and existing-feature regressions are **not tested in game**. These are flat particle-sprite icons, not full 3D models; no item lighting/tint/glint or animated textures. One icon represents a whole stack. Updates remain roughly 10 Hz and can look stepped. The export checks a 3 ms deadline between entities, scans at most 4096 entities and attempts at most 64 nearby items; limits may omit items, and one model-resolution call can exceed the budget. Unsupported sprite atlases are skipped, individual runtime errors are logged, and an oversized combined atlas disables scene export. Neither correctness nor performance under resource packs/crowded scenes is claimed. Both adapters must be installed together for protocol 8.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines, no rebuild needed). In offline first-person practice worlds, use `/give @s minecraft:apple 8` and `/give @s minecraft:dirt 8`, then drop each with Q. Expect bobbing/spinning apple and dirt-face icons in Skyrim, hidden by walls/blocks and removed on Minecraft pickup. Mine dirt in survival to create a natural drop; check cracks, ordinary textures, hands/HUD, movement and look. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft; expect three COLLECTED lines. Compare SCENE_SEND/RECV/ITEM_DRAW item_vertices (six per icon; zero when none remain) and client SCENE_ITEM positions/IDs/counts.

## 2026-10-08 Confirm 0.6.2 crack logs and explain ground rendering

**Changed:** Updated README and DESIGN with the user's successful gameplay report, numerical crack results and expected Minecraft ground inclusion. Documentation only.

**Why:** User supplied logs/scene-report-20261008-084544 and asked whether Minecraft ground should render.

**Tested how:** Parsed all three report files. Matched every received record to its sent kind/sender/world/snapshot/atlas and compared vertex count, dimensions, byte count, checksum, origins and crack count. Matched Skyrim crack-draw counts and solid-plus-crack totals to received snapshots. Inspected SceneCapture's region/block selection.

**Result:** All 400 received records match (399 meshes and one atlas); all 399 crack-draw counts match. There are 45 snapshots/draws with 36 crack vertices, 354 with zero, all stages 0–9, and four transitions from active cracks to zero. All 87 depth probes are usable; only one startup wait, no reported SCENE_ERROR/DISABLED/BUILD_DROPPED, and no skipped crack shapes in this run. User reports gameplay works as expected. Ground rendering follows current code: all supported nearby blocks are included, with no distinction between generated and player-placed terrain.

**Still broken / not tested:** Logs alone do not distinguish cancellation from completion for each clear, or prove pixel-level occlusion. Exhaustive graphics/resource-pack cases are **not tested**. A mostly empty/void Minecraft world is not set up by this mod; generated floor and its collision remain. Previously deferred camera/world-bleed issues and unimplemented dropped items remain.

**Next:** No rebuild, reinstall or additional gameplay test needed for this documentation update. For future reproductions, run `.\tools\collect-scene-logs.ps1` before restarting Minecraft; expect three COLLECTED lines. Ground removal would require a separate world-setup change, not just hiding its graphics.

## 2026-10-08 Step 5b breaking cracks on full opaque blocks (0.6.2)

**Changed:** Wrote the next increment in DESIGN before implementation. Added CrackCapture using Minecraft's existing destructionProgress map and resource manager. Appended ten runtime crack textures to the in-memory atlas, remapped block V coordinates, and appended up to 32 full-cube crack overlays to complete mesh snapshots. Added a trailing crack-vertex count at header offset 96, validation, transparent darkening after the solid draw, and depth testing without crack depth writes. Bumped both adapters to 0.6.2 and the shared namespace/protocol to 7. Added client stage/count and native send/receive/draw count logs. Updated README and ABI with procedure and limits.

**Why:** Continue step 5b one piece at a time, addressing the previously missing breaking animation. Camera orbit and world bleed remain deferred; dropped items are a separate increment.

**Tested how:** Inspected the local SkyCraft crack-export approach and Minecraft 26.3 public method signatures; wrote original implementation using those APIs. Built and packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests added or run; Gradle test was NO-SOURCE. No games launched or visually inspected, and no game-folder writes.

**Result:** Rust release and Fabric builds succeeded. Packages are dist/step5b/Passthrough-Minecraft-0.6.2.zip and Passthrough-Skyrim-0.6.2.zip. Existing Java deprecated-API and informational MSVC import-library warnings remain. An initial rg wildcard-path read produced Windows path syntax error; explicit paths and directory globs were used afterward. Implementation refreshes cracks on publication retries, sends zero crack vertices after damage clears, and retains the existing expiry/world checks.

**Still broken / not tested:** Runtime HLSL compilation, crack progression/cancellation/completion, depth occlusion, resource reload and regressions are **not tested**. Only full opaque MODEL cubes are supported; other shapes and capacity overflow are counted/skipped. Textures are resampled to 16x16, colored crack packs become grayscale, and a block atlas already 4096 pixels high cannot fit the strip and disables scene export. Mesh publication remains roughly 10 Hz, so fast damage stages can be skipped. Missing/invalid crack resources fail scene export with a logged error. Dropped items, camera orbit, world bleed and placement reach remain as previously recorded. Both game adapters must be installed together for protocol 7.

**Next:** Close both games and run `.\tools\install-build.ps1`; expect three INSTALLED lines (no rebuild needed). Start offline first-person worlds, run `/setblock ~2 ~1 ~ minecraft:stone` and `/gamemode survival` in Minecraft, and mine the stone bare-handed. Expect cracks to advance in Skyrim; stopping early should clear cracks, completing should remove the block, and a Skyrim wall should hide both block and cracks. Check ordinary block textures, HUD and movement/look. Run `.\tools\collect-scene-logs.ps1`; expect three COLLECTED lines. Matching snapshots should have crack_vertices=36 for one damaged block and 0 after clearing, with SCENE_CRACK stages in the client and SCENE_CRACK_DRAW in Skyrim. In-game outcome is not yet known.

## 2026-10-08 Compare 0.6.1 logs and correct the cutoff assessment

**Changed:** Updated README and DESIGN with the numerical comparison and the user's corrected eight-block observation. Recorded close placement reach, perceived scale, camera orbit and world bleed; the last two are deferred as requested. No code, builds, deployment or game configuration changes.

**Why:** The user supplied logs/scene-report-20261008-081526 and clarified that the original three-block estimate was mistaken.

**Tested how:** Parsed all three report files and compared every SCENE_RECV with its SCENE_SEND by kind/sender/world/snapshot/atlas, checking counts, dimensions, payload sizes, checksum and origins. Summarized bounds and projection/depth diagnostics. Read position/mesh mapping code and ANCHOR/RELEASED events in logs/step2/skyrim-39800-163086750.log. No games were visually inspected.

**Result:** All 1,444 received records match (1,443 meshes and one atlas), with 1,443 draws, 317 usable depth probes and only one startup depth wait. No SCENE_ERROR, SCENE_DISABLED or SCENE_BUILD_DROPPED appeared in the report. All 1,443 bounds describe a 16x16x16 cube. Maximum depth-formula disagreement at 500 units is 0.00000114; maximum projection-W error is 0.00046 units. Both movement and mesh rendering use 70 Skyrim units/block. An initial whole-run scale comparison showed a 518.2307-unit offset change; reading the full log revealed release/re-anchor events, so a single unchanged anchor cannot be assumed. Between offset changes, consecutive mesh-origin deltas agree with 70 units/block within 0.0004 Skyrim units. This does not verify every movement sample or establish the cause of re-anchoring. Two rg wildcard-path reads failed with Windows path syntax; corrected directory/glob and direct file reads supplied the needed data.

**Still broken / not tested:** Close placement reach and perceived scale are unresolved. Camera orbit and world bleed are user-reported and deferred; their causes are **not tested**. Correct near/far arithmetic does not prove every pixel has correct occlusion. Breaking cracks and dropped-item rendering remain unimplemented. The earlier three-block-cutoff diagnosis is superseded by the user's corrected observation, not fixed through code.

**Next:** No rebuild or reinstall needed. For any later reproduction, run `.\tools\collect-scene-logs.ps1` immediately afterward and before restarting Minecraft; expect three COLLECTED lines. Keep the full bridge logs too, because the scene-only report omits alignment lifecycle events.

## 2026-10-08 Investigate short range and add 0.6.1 diagnostics

**Changed:** Wrote the investigation plan in DESIGN before code. Added SCENE_BOUNDS in Minecraft's client log (export limits, player position and aimed block), SCENE_PROJECTION in Skyrim (depth at known distances versus the shader's near/far formula), and an additional centre-screen depth sample with its sampling timestamp. Preserved the original 16 samples used to infer the depth convention. Added tools/collect-scene-logs.ps1 to stream rendering-only lines into an ignored project report folder. Updated README and package versions to 0.6.1; protocol remains 6.

**Why:** Investigate the user's roughly three-block disappearance without guessing at an occlusion fix or removing wall occlusion.

**Tested how:** Read the latest recorded bridge logs (minecraft-29756-152557406.log and skyrim-35748-152539546.log), local SkyCraft reference, CommonLib camera layouts, and Minecraft tessellation signatures. Built Rust release and Fabric with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. No automated tests or game inspection. Gradle's test task has no test sources.

**Result:** The recorded run reaches mesh snapshot 2204, continues drawing nonempty meshes, and has only one SCENE_WAIT at startup, with no SCENE_DISABLED/ERROR entries found in Skyrim. Logged projection W is approximately 500 for a point 500 units ahead; near/far are 15/353840. These observations do not establish why pixels disappear. Both adapters compile. A diagnostic sample initially shadowed the new centre value; a compiler warning exposed this and the duplicate was removed before final packaging. The usual linker import-library message and Java deprecated-API warning remain.

**Still broken / not tested:** The cutoff is **unresolved**. The added runtime shader/diagnostics, report collector, installation and gameplay are **not tested**. Breaking cracks and dropped items remain unimplemented. The collector selects the newest bridge log for each game; collect immediately after a shared run, before restarting Minecraft. No game installation was changed.

**Next:** Close both games and run `.\tools\install-build.ps1` (three INSTALLED lines). Start offline first-person worlds, place a stone block with `/setblock ~2 ~1 ~ minecraft:stone`, keep looking at it while backing away until it disappears, then approach again. Run `.\tools\collect-scene-logs.ps1` before restarting Minecraft; expect three COLLECTED lines. Supply those three report files so exported coverage, aimed block, camera projection and centre depth can be compared. This diagnostic build may still show the cutoff.

## 2026-10-08 Record 0.6.0 gameplay results and short render range

**Changed:** Updated README and DESIGN with the user's 0.6.0 results. No code changes.

**Why:** Record passed behavior and the unexpected rendering cutoff separately from missing features.

**Tested how:** User gameplay report: spawned and broke a block, moved around it and a Skyrim wall, checked wall occlusion and older features. No agent-run tests or visual inspection.

**Result:** Block appearance/removal, stable world placement, Skyrim-wall occlusion, and prior features passed the user's checks. The user reports blocks disappear at roughly three blocks away. Breaking animation and block drops are not visible.

**Still broken / not tested:** Short render range is unresolved; no cause is established. Intended export-range coverage and numerical log comparison are **not tested**. Breaking crack overlays and dropped-item rendering are unimplemented; absence of a visible drop does not establish whether Minecraft created an item entity.

**Next:** Inspect this run's SCENE_CAPTURE/SEND/RECV/DEPTH/DRAW/WAIT logs to distinguish missing geometry from depth rejection or stale snapshots. The existing README step-5b Select-String commands should show corresponding snapshot identifiers, nonzero vertex counts and depth state while reproducing the cutoff. No rebuild is needed for this documentation update.

## 2026-10-08 Fix installer blocking VS Code Java processes

**Changed:** Replaced the blanket java/javaw process block in tools/install-build.ps1 with Minecraft launch-class/instance-path detection and the existing Skyrim check. Added an exclusive read/write-open preflight for all existing destination files before backup/copy. It does not write file contents during that check. Process-query failures warn and fall back to file checks. Command lines are inspected in memory and never printed. Updated README and DESIGN.

**Why:** The user reported installation refused with only VS Code's Java extension processes running. Blocking every Java process was incorrect.

**Tested how:** Read the supplied error/screenshot and installer source. No script execution, installation, builds or automated tests were run for this fix.

**Result:** Corrected the source so unrelated Java processes do not trigger the game-process guard. Existing backup, checksum and restoration behavior remains.

**Still broken / not tested:** Updated process detection, file-access checks and deployment are **not tested**. Process details may be unavailable or nonstandard launchers may be unidentified; both games must remain closed, and file-access checks cannot prevent a game starting afterward.

**Next:** Leave VS Code open, close both games, and run `.\tools\install-build.ps1`. Expect three INSTALLED lines and "Install complete: 3 files copied and verified." No rebuild is needed.

## 2026-10-08 Step 5b first world-mesh build (0.6.0)

**Changed:** Updated DESIGN before code after inspecting SkyCraft's actual mesh-rendering path. Added SceneCapture, atlas/sprite accessors, native scene mailboxes/FFI and scene_draw.rs. Minecraft tessellates nearby opaque/cutout block models and sends runtime atlas pixels; Skyrim maps triangles through the movement Anchor, projects through its rendered camera, probes its scene depth and clips hidden pixels. A private depth buffer resolves block-on-block overlap; the existing hands/HUD pass follows. Added SCENE_CAPTURE/SEND/RECV/UPLOAD/DEPTH/DRAW/WAIT/ERROR diagnostics. Bumped both adapters to 0.6.0 / protocol 6. Added build-step5b.ps1 with matching version checks; earlier step builders delegate to it and the installer defaults to dist/step5b. Updated README and ABI with exact commands, numerical checks, manual wall-occlusion procedure and limitations.

**Why:** The user confirmed the 5a arms/items/interface appear inside Skyrim and requested 5b. SkyCraft's current implementation sends world meshes and textures to Skyrim so its own camera/depth can position and hide them correctly; this build follows that approach rather than superimposing a flat world image.

**Tested how:** Compiled Rust release and Fabric, then packaged with `.\tools\build-step5b.ps1 -Offline -PackageOnly`. Inspected cached Minecraft 26.3 tessellation signatures and private atlas fields, pinned CommonLib renderer/camera layouts, SkyCraft's fallback depth sampling, and Microsoft's D3D11 depth/Map documentation. Read the source changes and ran git diff --check. No automated tests were added or run; Gradle's test task was NO-SOURCE. No games were started or visually inspected.

**Result:** Rust and Fabric builds succeeded; dist/step5b contains Passthrough-Minecraft-0.6.0.zip and Passthrough-Skyrim-0.6.0.zip. Java reported use of a deprecated Minecraft API; Rust emitted the informational MSVC import-library linker warning. Initial WMI process classification was denied by the sandbox; the approved retry returned classification flags but did not establish that Minecraft was closed. The installer was then invoked and refused before any file writes because it detected two Java processes. No game installation changed. Two patch-context mismatches were corrected before those patches applied.

**Still broken / not tested:** New HLSL runtime compilation, mesh/atlas transfer, rendered camera alignment, depth inference/occlusion, resource reloads, pause/reconnection/resizing, installation and performance are **not tested**. This first 5b build handles a 16-cubed nearby region, up to 262,140 vertices, atlas up to 4096x4096, first sprite animation frames only, and simplified Minecraft brightness. Translucent faces are counted/skipped; fluids, dropped items, block-entity models, mobs and the third-person player are not exported. Skyrim lighting/shadow/post-processing integration is not implemented. Uniform/unavailable depth suppresses drawing instead of claiming valid occlusion; depth convention is inferred from samples. The installer remains blocked while Java is running.

**Next:** Close the games/Java applications, then run `.\tools\install-build.ps1` (no rebuild needed). Expect three INSTALLED lines. Start offline first-person practice worlds and use `/setblock ~2 ~1 ~ minecraft:stone` in Minecraft. Expect the nearby block to stay fixed in Skyrim and disappear behind a Skyrim wall; hands/HUD should remain visible. Run README's step-5b log commands and compare SCENE_SEND/RECV identities/counts/checksums, SCENE_DEPTH usable=true and SCENE_DRAW. Send both bridge logs plus Minecraft latest.log if drawing waits or fails. Further world-object classes remain later rendering work.

## 2026-10-07 Confirm step 5a overlay appears in Skyrim

**Changed:** Updated README, DESIGN and ABI status with the user's explicit confirmation that first-person arms/punching, held items, HUD and inventory also appear inside Skyrim's window. No code changes.

**Why:** Resolve the destination-window ambiguity in the previous gameplay report.

**Tested how:** User gameplay confirmation. No agent-run tests or visual inspection.

**Result:** Basic hand/item/interface composition over Skyrim is confirmed by the user.

**Still broken / not tested:** Numerical image-log comparison, resize/reconnection/error paths and broad graphics compatibility are **not tested**. Invisible Minecraft terrain still collides; third-person and world/depth rendering remain unsupported.

**Next:** No rebuild needed. To compare transport numerically, run the existing README step-5a log commands; expect matching OVERLAY_SEND/RECV frame identifiers and checksums plus OVERLAY_DRAW. Step 5b remains pending.

## 2026-10-07 Record first step 5a gameplay observations

**Changed:** Updated README and DESIGN with the user's report of first-person arms/punching, held items, hotbar, health, hunger and inventory rendering, a black Minecraft world, remaining Minecraft terrain collisions, and absent third-person rendering. No code changes.

**Why:** Keep observed behavior and current rendering limits explicit. Suppressing level drawing does not remove Minecraft's normal collisions. Superflat reduces obstacles but still supplies a colliding floor.

**Tested how:** User gameplay report only. Asked whether the visible elements appeared over Skyrim or only in Minecraft. No games, scripts or automated tests were run by the agent.

**Result:** Recorded the observations without claiming verified Skyrim composition or image transport. Third-person models/world drawing remain outside 5a.

**Still broken / not tested:** Destination-window confirmation and image-log comparison pending. Superflat workaround is **not tested**. World/depth rendering remains unimplemented; invisible Minecraft terrain can block movement.

**Next:** Confirm the destination window. For numerical evidence, run README's step-5a Select-String commands after closing both games; expect matching OVERLAY_SEND/RECV frame identifiers and checksums plus OVERLAY_DRAW. No rebuild or installation is needed for these documentation changes.

## 2026-10-07 Install matching files after package builds

**Changed:** Added tools/install-build.ps1 for the two game folders named in AGENTS.md. It checks package completeness and matching DLL hashes, requires Skyrim/Java to be closed, backs up previous mod files in ignored dist/install-backups, copies the Minecraft JAR plus both DLLs, verifies copied hashes, removes only older passthrough JARs, and attempts restoration on failure. Connected it to successful step-5 package builds by default; added -PackageOnly through older build wrappers and -WhatIf on the installer. Updated DESIGN before implementation and README with commands and expected output.

**Why:** The user requested a PowerShell script to place each new build in the mod folders and avoid repeated manual copies or mixed JAR/DLL versions.

**Tested how:** Not tested. No deployment, build or automated test was run for this script change. Read the existing scripts and installation paths while implementing.

**Result:** Script and build integration are written. No game folder was modified in this turn.

**Still broken / not tested:** Installation, checksum checks and failure restoration are **not tested**. Direct Skyrim installation can be overridden by a mod manager; use -PackageOnly and the manager in that case. The closed-game guard conservatively blocks all Java processes.

**Next:** Close both games and run `.\tools\install-build.ps1` from the project folder to install the existing build. Expect three INSTALLED lines and "Install complete: 3 files copied and verified." Future `.\tools\build-step5.ps1 -Offline` runs install after packaging; add -PackageOnly to skip that step.

## 2026-10-07 Step 5a: Minecraft hands and HUD overlay

**Changed:** Recorded the user's successful camera test. Wrote the 5a/5b split in DESIGN before implementation. Added Minecraft transparent level suppression and three asynchronous image-readback buffers, new native overlay exports and a bounded RGBA mailbox, protocol 5 readiness/generation checks, and a Skyrim D3D11 Present hook that uploads and blends hands/HUD while restoring graphics state. Added OVERLAY capture/send/receive/draw/error logs with pixel checksums, alpha counts and CPU timings. Versioned both adapters 0.5.0; added tools/build-step5.ps1 and made the step-4 builder delegate. Updated README and ABI with installation, limits and exact manual/log commands. Game pixel data stays in runtime memory. No game installation writes or Git commits were made for this change.

**Why:** The user confirmed Minecraft now turns Skyrim's camera and asked to proceed with step 5. Its first build step is hands/interface; world objects and depth follow after this part is validated.

**Tested how:** Built Rust release and Fabric against the cached Minecraft 26.3 APIs, then ran `.\tools\build-step5.ps1 -Offline` to package both. Read SkyCraft's capture/composition implementation, CommonLib's published renderer pointer/offset, cached Minecraft method signatures, and Microsoft's D3D11 context-state documentation. No automated tests were added or run; Gradle's test task reported NO-SOURCE. No games were launched or visually inspected.

**Result:** Final Rust release build and Fabric build succeeded; created dist/step5/Passthrough-Skyrim-0.5.0.zip and Passthrough-Minecraft-0.5.0.zip. Rust emitted only the existing informational MSVC import-library linker warning. Initial dependency fetch failed under sandbox network restrictions; approved fetch succeeded. First Rust build caught an incorrect string-replacement argument; that was fixed. The following build caught a separate u32/typed-flag mismatch in device creation flags; that was fixed and subsequent builds passed. API inspection also caught the texture-format enum name: changed the guard to the actual RGBA8_UNORM enum. The initial TextureFormat class lookup failed because this API calls it GpuFormat. The local MODLOG-template.md is absent; used its tracked template format without restoring the user's deleted file.

**Still broken / not tested:** Live shader compilation, Minecraft mixin application, image capture/transport/composition, orientation/transparency, resizing, reconnection, pause handling, performance and graphics-mod compatibility are **not tested**. Builds do not prove visible rendering. Step 5b world/depth drawing is not implemented. Paused menus, destination-aware crosshair inversion, exact color matching and GPU device replacement are not handled. Input remains in Minecraft's window. No pixel capture files were produced.

**Next:** Install both 0.5.0 packages, replacing Minecraft's JAR and DLL together. Run both games offline at 1280x720 with Minecraft focused, first person and HUD visible. Follow README's "Check step 5a hands and HUD" commands. Expect matching OVERLAY_SEND/RECV identity, dimensions, checksum and alpha counts, plus Skyrim OVERLAY_DRAW submissions; manually confirm the hand/hotbar appear over Skyrim. Validate this before implementing step 5b.

## 2026-10-07 Prepare initial GitHub publication

**Changed:** Configured origin as https://github.com/StevenLum/SkyCraftPassthrough.git for the user's requested commit and push. Prepared the current 0.4.0 project source, documentation and build scripts as the initial repository snapshot on the existing master branch. Removed extra trailing blank lines in tools/build-step4.ps1 flagged by the staged whitespace check.

**Why:** The user asked to push the accumulated project changes to their GitHub repository.

**Tested how:** Read AGENTS and the whitelist, enumerated all 42 publishable files, checked for common credential patterns without printing potential matches, and fetched the remote branch list. No builds or gameplay tests were rerun for publication.

**Result:** No existing local commits or remote branches; no credential-pattern matches. The whitelist excludes game data, reference checkouts, downloaded tools, logs and compiled packages. The sandbox initially blocked GitHub access; the approved fetch succeeded. Git author identity was resolved from the user's configured name and GitHub noreply address after the temporary safe-directory configuration hid those settings. Created initial commit b8119b46ac93ff67910be9d23bc508db43821785. The first push was rejected for authentication; retrying with the normal user Git configuration included succeeded. GitHub's master ref was read back and matched that commit exactly; local master now tracks origin/master.

**Still broken / not tested:** Camera behavior after the corrected Minecraft DLL installation remains **not tested**. Source publication does not verify gameplay, and compiled packages are not included in Git.

**Next:** Publish this outcome record, then continue the pending in-game camera verification. To check synchronization, compare `git rev-parse HEAD` with `git ls-remote origin refs/heads/master`; expect the same commit ID.

## 2026-10-07 Replace outdated Minecraft bridge DLL with approval

**Changed:** With explicit tool approval under AGENTS.md rule 4, replaced `C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\mods\passthrough_link.dll` with the prepared 0.4.0 DLL. Updated README status. No other game files changed.

**Why:** The installed 0.3.0 DLL could not supply the function required by the 0.4.0 JAR.

**Tested how:** Copied the packaged DLL and read the installed file's SHA256.

**Result:** Installed hash now matches the 0.4.0 package exactly: 75D5F9066EEDE11C3F203AACF25C5A70FC5D40E06A0497903E7E64F2A5C3F1DF. Copy command completed successfully.

**Still broken / not tested:** Minecraft restart and live camera following after replacement are **not tested**. No games were launched by the agent.

**Next:** Restart the Prism instance, enter the offline world and expect a new Minecraft bridge log containing NATIVE_READY protocol=4 and SEND lines. Run the README position comparison with -RequireLook after testing mouse look with both games running.

## 2026-10-07 Diagnose missing Minecraft logs after 0.4.0 upgrade

**Changed:** Added upgrade troubleshooting to README. No implementation or game installation changes.

**Why:** The user reported 0.4.0 did not work and Minecraft created no bridge log.

**Tested how:** Read Minecraft's latest.log, the two latest Skyrim bridge logs, and inspected the installed Minecraft DLL's SHA256 and exports. Installation reads initially failed in the sandbox and succeeded with approval. No automated tests were run.
**Result:** Minecraft loaded passthrough_practice 0.4.0, then threw NoSuchElementException in NativeLink.java:26 while resolving pt_mc_frame4. The installed DLL hash is 6563BD8D6FA3AF8EF22B79870873B949EAF94751ED07576CB0A3A5EC377FEA9E, exactly matching the 0.3.0 package; it exports pt_mc_frame but not pt_mc_frame4. The correct 0.4.0 packaged DLL hash is 75D5F9066EEDE11C3F203AACF25C5A70FC5D40E06A0497903E7E64F2A5C3F1DF. Both Skyrim runs loaded successfully, installed the hook and reached NATIVE_READY protocol=4, waiting for Minecraft. This establishes a mixed Minecraft JAR/DLL installation as the startup failure.


**Still broken / not tested:** Corrected Minecraft installation and live mouse-look behavior are **not tested**. No code fix is required for this diagnosed mismatch.

**Next:** Close Minecraft and replace its mods/passthrough_link.dll with dist/step4/minecraft/mods/passthrough_link.dll, with game-folder write authorization required by AGENTS.md rule 4. Restart and expect Passthrough Fabric plugin loaded in latest.log, a new project Minecraft bridge log with NATIVE_READY protocol=4, and SEND lines. Then repeat the README look comparison with -RequireLook.

## 2026-10-07 Step 4: Minecraft mouse-look transfer

**Changed:** Recorded the user's successful jumping/landing test. Updated DESIGN before implementation. Added yaw/pitch to rendered position samples, protocol 4 and `pt_mc_frame4`, and `look.rs` to convert Minecraft angles and drive Skyrim's player angles for its normal first-person camera. Added ordinary third-person switching/restoration and special-camera guards. Extended SEND/RECV logs with source angles and added LOOK_RECV target/read-back and camera-state quaternion diagnostics. Added `-RequireLook` to the position comparison script, using streaming file reads for large logs. Updated README/ABI, versioned packages 0.4.0, and added `tools/build-step4.ps1`; older build commands delegate to the current package builder.

**Why:** The user confirmed Minecraft's mouse turns its own camera but Skyrim's view does not follow, and requested Minecraft-to-Skyrim mouse translation. Send the resulting view angles so Minecraft retains its normal mouse sensitivity and controls.

**Tested how:** Inspected SkyCraft's angle conversion/player-angle writes and the public CommonLib camera interfaces; checked Minecraft 26.3 view-angle method signatures. Compiled and packaged using `.\tools\build-step4.ps1 -Offline`. No automated tests were added or run. Existing fixture code was kept source-compatible with the extended sample/API.

**Result:** Rust and Fabric compilation and packaging succeeded. Builds contain finite/range checks, same-frame yaw/pitch, normalized heading and matching log frame identities. These are compiled changes, not a confirmed live camera result. The user reports jumping and landing worked in the previous build, adding to previously confirmed wall/NPC blocking. The linker still prints its informational import-library creation warning.

**Still broken / not tested:** New camera response, axis direction, camera-state transitions/restoration, timing and `-RequireLook` are **not tested**. Camera quaternion logs precede the next native camera update; actor read-back is not proof of the rendered camera. Third-person Minecraft mirroring, exact eye position, FOV/bob matching and Skyrim-window input forwarding remain unimplemented. No game installation files changed.

**Next:** Install both 0.4.0 packages together, keep Minecraft focused in first-person and Skyrim running in the background, then look left/right, up/down and through a full turn. Expect Skyrim's first-person view to follow. Stop both games, select the latest `$mcLog`/`$skyLog` as in README, and run `.\tools\compare-position-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName -RequireLook`; expect position PASS and nonzero look frames matching within 0.00001 radians. Supply logs if the camera does not follow.

## 2026-10-07 Position and world comparisons; manual wall and NPC blocking

**Changed:** Updated README and DESIGN with the new comparisons, the user's manual collision result, and the clarified missing camera synchronization. No code or game files changed.

**Why:** Verify the user's supplied live logs and distinguish working Minecraft mouse look from unimplemented Skyrim camera alignment.

**Tested how:** Ran `.\tools\compare-position-logs.ps1 -MinecraftLog 'D:\Documents\ChatGPT\Game modding\logs\step2\minecraft-32128-107636093.log' -SkyrimLog 'D:\Documents\ChatGPT\Game modding\logs\step2\skyrim-15308-107625921.log'` and `.\tools\compare-world-logs.ps1` with the same arguments. Both scripts completed with exit code 0. The user tested movement and reported stopping at a Skyrim wall and NPC; the agent did not visually inspect either game.

**Result:** Position PASS: 5986 received frames matched all Minecraft coordinates and partial ticks exactly; 11437 frames sent; maximum Skyrim target/read-back difference 0 units. World PASS: 543 complete received snapshots, 1354585 collision-box records and 139 NPC records matched exactly (record counts across snapshots, not unique objects). The world comparison took several minutes for the approximately 335 MB logs. User clarified the camera turns inside Minecraft but not Skyrim: Minecraft mouse input works; view synchronization is absent.

**Still broken / not tested:** Look-direction transfer and Skyrim camera alignment are unimplemented, as is Skyrim-window input forwarding. Jumping, landing, slopes and general collision coverage remain **not tested**. Transport equality does not establish complete geometric accuracy.

**Next:** The comparison commands above should continue to print PASS on these logs. Next proposed implementation: transmit Minecraft look direction and align Skyrim's camera, then handle Skyrim-window input forwarding separately. No camera or input code was added in this comparison task.

## 2026-10-07 Live step 3 transport and control clarification

**Changed:** Updated README and DESIGN with the user's live result and clarified that movement currently uses Minecraft's controls. No code or game installation changes.

**Why:** The user reported matching world logs but could not move using Skyrim's controls. Minecraft owns position; Skyrim-window input forwarding is not implemented.

**Tested how:** Used the user's reported comparison result for the named Minecraft/Skyrim logs. Read the start of `skyrim-24140-106825984.log` and Minecraft's normal latest.log collision-query entries. The initial Minecraft read was denied by the sandbox; an approved elevated read succeeded. No automated tests were run.

**Result:** User comparison: 117 complete received snapshots, 149415 boxes, 117 NPC records matched exactly. Skyrim recorded PLUGIN_LOADED, HOOK_INSTALLED and Connected. Minecraft logged 28 client queries (12 with nonzero matches) and 28 server queries (1 with nonzero matches). The missing Address Library problem is resolved for this run. These counters confirm transport and query injection, not correct physical response; queries can belong to entities other than the player.

**Still broken / not tested:** Skyrim keyboard/mouse forwarding is unimplemented. Accurate movement, wall blocking, landing and NPC collision are **not tested**. Skyrim input being overwritten while Minecraft supplies positions is expected under the current design.

**Next:** Focus Minecraft, leave its pause menu closed, walk and jump near the exported surfaces, with Skyrim configured to update in the background. Use the README position/world comparisons after stopping both games; expect matching coordinates and snapshots, then verify movement behavior. Input forwarding can be a separate requested step.

## 2026-10-07 Step 3: sampled Skyrim geometry and NPC positions

**Changed:** Updated `docs/DESIGN.md` before implementing the requested step. Added `world.rs` (bounded snapshot format) and `world_export.rs` (Havok ray sampling and nearby loaded actors), extended the Rust/Java native bridge, and added `SkyCollision.java` and the common `BlockCollisionsMixin.java`. Added protocol-3 shared memory, immutable collision snapshots shared by the Minecraft client and integrated server, world/origin/age guards, per-snapshot geometry/NPC logs, and collision-query logging. Added `tools/build-step3.ps1` and `tools/compare-world-logs.ps1`. Versioned both packages 0.3.0; the previous build command now delegates to the current one to avoid mixing old Java and new DLLs. Updated README and ABI documentation. No game installation writes, commits, or staging.

**Why:** Keep Minecraft authoritative for movement while its unchanged physics queries include nearby Skyrim surfaces and NPC body boxes; log the sent and received data for comparison.

**Tested how:** Inspected SkyCraft's documented ray-sampling approach, public CommonLib headers and implementation at the existing pinned checkout, and Minecraft 26.3 collision iterator signatures/bytecode in the ignored build cache. Compiled the Rust release DLL and Fabric JAR and packaged with `.\tools\build-step3.ps1 -Offline`. No new tests were added or run; the previous 14 tests and Java smoke check were not rerun for step 3.

**Result:** Native and Java compilation and packaging succeeded. The exporter samples 1,536 grid rays per completed scan (plus continuation rays, up to eight hits each), with a target budget of 2 ms / 384 casts per update checked between rays. Complete snapshots carry up to 8,192 surface boxes and 128 NPC records. SEND/RECV logs include matching world, epoch, snapshot, indices/IDs and coordinates; Minecraft's normal log reports client/server COLLISION_QUERY use once per second per side. These are implementation details, not observed gameplay results. The linker still prints its informational import-library creation warning.

**Still broken / not tested:** Step 3 runtime behavior, Havok/native interfaces, mixin application, collision accuracy, performance, and the new log-comparison script are **not tested**. Successful Skyrim loading after installing its missing Address Library has not been reported. Ray sampling is approximate (half-block spacing, four-block radius); it can miss thin objects and lags moving surfaces. NPCs are restricted to loaded high-process actors in the player's cell within eight blocks per axis, with fixed 0.6-by-1.8-block approximate collision boxes. No exact shape decoding, NPC interactions/combat, camera/input work, or emergency floor on missing geometry. Incomplete/slow scans are discarded, stale snapshots expire, and overflow reports an error instead of truncating data.

**Next:** Install both 0.3.0 packages together using the README procedure and confirm Skyrim's required Address Library is installed. Run both games offline, approach a floor, wall and NPC, stop both games, and run `.\tools\compare-world-logs.ps1 -MinecraftLog $mcLog.FullName -SkyrimLog $skyLog.FullName` after selecting the latest logs as documented. Expect complete matching snapshots with nonzero boxes/NPCs and COLLISION_QUERY for both client and server. Recheck position logs as well. Game-folder installation by the agent still requires authorization under AGENTS.md rule 4.

## 2026-10-07 Identify missing Skyrim Address Library


**Changed:** Added the full Address Library path and required runtime to the file-read error in `passthrough/src/skse.rs`. Updated README with the actual load failure and dependency download link. Rebuilt the step 2 packages.

**Why:** The user's Skyrim log reported only Windows error 2, without identifying the missing file.

**Tested how:** Read `logs/step2/skyrim-11832-84680734.log`, inspected the loader source, and listed the named Skyrim install's `Data/SKSE/Plugins` directory. Ran `.\tools\build-step2.ps1 -Offline` to rebuild. No new automated tests were run.

**Result:** The log contains LOAD_BEGIN followed by LOAD_FAILED. The plugin directory contains only `passthrough_link.dll`; the required `versionlib-1-7-104-0.bin` is absent. The release build and packaging succeeded. The user reports Minecraft position logging.

**Still broken / not tested:** Skyrim remains blocked by the missing dependency. The improved message and successful loading/movement in the real game are **not tested**. No game files were changed by the agent.

**Next:** Install the Address Library for runtime 1.7.104.0, then run `& 'E:\SteamLibrary\steamapps\common\Skyrim Special Edition\skse64_loader.exe'` (or the mod manager's SKSE entry). Expect `plugin loaded runtime=1.7.104 event=PLUGIN_LOADED` in the new Skyrim log, then compare position logs after loading both single-player worlds.

## 2026-10-07 Step 2: rendered position sender and Skyrim puppet adapter

**Changed:** Updated `docs/DESIGN.md` for the user's requested step 2. Added the Fabric render callback and Java/native bridge in `minecraft/`; extended the Rust connection to protocol 2 with a latest-frame position record; added a Rust SKSE loader and player-update adapter in `passthrough/src/skse.rs`. Added per-frame SEND/RECV logs, origin alignment, coordinate conversion, actor/controller movement, temporary gravity suspension and restoration, and stale/inactive-world guards. Added build/package, Java/native smoke-test, and log-comparison scripts, plus `docs/ABI.md`. Updated README and the source whitelist. Built the two packages under ignored `dist/step2`.

**Why:** Minecraft must determine position using the partial-tick rendered value, and matching frame identities must let the user compare what was sent, received, and applied.

**Tested how:** `tools/test-link.ps1`: 8 unit tests, 5 connection process tests, and 1 position lifecycle test. `tools/test-position-native.ps1`: Java 25 loaded the real DLL, sent synthetic render samples, and compared them with a separate Windows receiver. Built the Fabric JAR against 26.3/Fabric 0.19.5, inspected the render-hook target signatures and Java 25 mixin compatibility, and inspected the DLL exports. Built both packages with `tools/build-step2.ps1 -Offline`. Fed the log checker a deliberately altered coordinate to verify rejection.

**Result:** All 14 Rust checks passed. The Java/native test sent 180 synthetic frames; 100 consumed frames matched coordinates and partial ticks exactly. Example fixture frame 2: partial=0.3333333333333333, mc_x=12.033333333333333, mc_y=64.04166666666667, mc_z=-5.066666666666666 on both sides. Simulated puppet movement produced (170, -60, 335) from anchor (100, -200, 300), then restored gravity from 0 to 9.8. Both native release and Fabric builds passed; the DLL exports SKSEPlugin_Load, SKSEPlugin_Version, pt_mc_init and pt_mc_frame. The checker rejected the deliberately changed mc_x=999 value. The first Gradle attempt picked the pre-existing JAVA_HOME pointing at Java 15; the script now verifies and selects installed Java 25, and the retry passed. Source review caught SKSE's pointer-encoded post-load success flag and corrected its handling before game testing. Linker output includes an informational import-library creation warning.

**Still broken / not tested:** Actual Fabric/SKSE loading, live Skyrim movement/capsule behavior, background execution, and compatibility with other mods are **not tested**. The standalone and simulated checks are not game logs. The approved read of Minecraft's installation now succeeds; its manifest confirms 26.3/Fabric 0.19.5, resolving the earlier access issue. Skyrim's executable confirms 1.7.104 and SKSE is present, but the direct install lacks Data/SKSE/Plugins and the required Address Library there. No game installation files were changed. Camera/input/collision export, rendering composition, combat and save synchronization remain unimplemented.

**Next:** With authorization for game-folder writes (AGENTS.md rule 4) and the matching Address Library installed, install the prepared packages, run both games in offline single-player, and compare real SEND/RECV logs using the README procedure. Stay on this step until the in-game result is verified.

## 2026-10-07 Step 1: standalone connection prototype

**Changed:** Wrote the requested three-part plan in `docs/DESIGN.md`. Implemented only step 1: the reusable Rust connection and two-role probe in `passthrough/`, process tests, and `tools/cargo.ps1` / `tools/test-link.ps1`. Added `README.md` with tested behavior, limitations, and exact commands. Added the missing whitelist `.gitignore`; downloaded tools, logs, build output, and `reference/` remain ignored. Added `rust-toolchain.toml` and Cargo build files. Game installations and the SkyCraft reference source were not modified.

**Why:** Prove the shared-memory connection independently before adding SKSE/Fabric adapters or controlling either player.

**Tested how:** Ran `.\tools\test-link.ps1` on Windows x64 using Rust stable 1.99.0 and MSVC. This ran 2 protocol unit tests and 5 process tests covering both startup orders, heartbeat progress, version rejection, timeout and fresh-session recovery, duplicate-role rejection, orderly shutdown, and isolated sessions. Read the generated logs and confirmed Git excludes reference files, downloaded tools, and build output.

**Result:** All 7 tests passed. In the recovery run, `skyrim.log` recorded `Connected` at 100 ms, `TimedOut` at 3511 ms with `peer_age_ms=2078`, and a fresh `Connected` at 3614 ms. The duplicate logged `duplicate role Minecraft: process 25320 still owns it`. Initial Rust download was blocked by the network sandbox and succeeded after approval. The first build failed because `link.exe` was not discoverable and Windows SDK libraries were absent; the build wrapper locates the installed linker, and project-local Microsoft SDK libraries completed the next build. Rust and SDK downloads stay in ignored `.tools`, with no permanent PATH change. Git's normal ownership check still rejects this repository; inspection used a temporary project-local trusted-directory config, leaving the user's Git config unchanged. Git also emitted a filesystem-monitor daemon warning during inspection.

**Still broken / not tested:** These are standalone probes, not loaded game mods. Actual SKSE/Fabric integration and all gameplay are not implemented and **not tested**. A read of the named Minecraft installation folder was denied, so its contents were not inspected. The abandoned-lock defensive path and other PCs are **not tested**. Manual gameplay was not run.

**Next:** When requested, implement step 2: thin SKSE and Fabric adapters calling the same Rust connection, and verify their handshake in offline single-player logs.

## 2026-10-06 SkyCraft architecture review

**Changed:** Cloned SkyCraft into `reference/SkyCraft` and read its `README.md` and `docs/DESIGN.md`. No mod implementation code was changed.

**Why:** Study the requested reference before planning this project's architecture.

**Tested how:** Read the two requested documents and checked the local clone's top-level layout. No tests were run.

**Result:** SkyCraft keeps each game's native logic and connects them with a Fabric mod, an SKSE plugin, a shared-memory protocol, and shared GPU textures. Minecraft owns player physics and inventory; Skyrim owns the world, NPCs, quests, and saves. Skyrim world collision is translated into Minecraft collision shapes; Minecraft player state and hits are sent back to Skyrim; Minecraft-rendered layers are composited over Skyrim. The design proceeds in phases, starting with a handshake and player movement before rendering, combat, blocks, and persistence.

**Still broken / not tested:** This project's own architecture is not yet planned or implemented. No in-game behavior or compatibility has been tested.

**Next:** Use the reference to plan the first practice milestone in this project's `docs/DESIGN.md` before writing code.

## 2026-10-06 Documentation baseline

**Changed:** Initialized the project guidance and the plain-language design baseline (`AGENTS.md`, `docs/DESIGN.md`), and recorded this entry using the new log format (`MODLOG.md`).

**Why:** Keep project rules, design notes, and change history in the files requested by the user.

**Tested how:** Read the documentation and checked the workspace file list.

**Result:** Documentation is in place. No mod implementation files were present in the workspace at the time of review.

**Still broken / not tested:** The mod itself is not implemented; no game run or automated tests were performed.

**Next:** Before any substantial code change, write the plan in `docs/DESIGN.md` and implement it one step at a time.










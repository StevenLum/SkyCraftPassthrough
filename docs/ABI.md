# Native interface notes

The game adapter is written in Rust and supports Windows x64 MSVC and Skyrim
1.7.104.0 only. It uses a small subset of the public SKSE and CommonLib interface
definitions. CommonLib is a source reference, not a linked runtime dependency.
The reference checkout is ignored under `.tools/CommonLibSSE-NG`.

## Sources used

- [SKSE plugin interface](https://github.com/ianpatt/skse64/blob/master/skse64/PluginAPI.h):
  loader structures, messaging interface 5, messages, and plugin metadata.
- [SKSE load result dispatch](https://github.com/ianpatt/skse64/blob/master/skse64/Hooks_SaveLoad.cpp):
  post-load success is encoded as a non-null pointer value, not a bool to dereference.
- CommonLib reference commit `94faaed0c60eddd8347767f2d4d29a97c93bde8c`:
  [player vtable ID](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/RE/Offsets_VTABLE.h),
  [actor interface](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/RE/A/Actor.h),
  [reference position](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/RE/T/TESObjectREFR.h),
  [controller](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/RE/B/bhkCharacterController.h),
  [address format](https://github.com/alandtse/CommonLibSSE-NG/blob/94faaed0c60eddd8347767f2d4d29a97c93bde8c/include/REL/IDDB.h).
- SkyCraft's local `skse/src/Game.cpp` uses the player update virtual slot `0xAD`
  and `Actor::SetPosition(position, true)`. Its Minecraft render callback uses
  `getGameTimeDeltaPartialTick(false)` and `player.getPosition(partial)`.

## Skyrim boundary

| Item | Value for this runtime |
| --- | --- |
| Packed runtime | `0x01070680` |
| Plugin metadata size | `0x350` bytes |
| Player vtable Address Library ID | `208040` |
| UI singleton pointer Address Library ID | `400327` |
| Player update virtual slot | `0xAD`; call original before applying position |
| Actor SetPosition virtual slot | `0xA9`; pass `true` to update its controller |
| Reference position | `0x54`, three floats |
| Actor process pointer | `0xF8` for post-1.6.629 layout |
| Process middle-high pointer | `0x08` |
| Middle-high character controller pointer | `0x250` |
| Controller set-velocity virtual slot | `7`, pointer to an aligned four-float vector |
| Controller fall-start height / fall time / gravity | `0x240` / `0x244` / `0x248` |
| UI pause counter | `0x160` |

The adapter reads the matching format-5 Address Library at runtime. It checks
format, version, pointer size, file length and image-relative address bounds.
It changes no saved control-enable flags. While following Minecraft, it clears
controller velocity and fall timers and saves/suspends controller gravity. It
restores gravity on release when that same controller still exists. A newly
created controller gets its own saved gravity value.

Earlier tests covered metadata layout, synthetic address-library validation, and
simulated actor/controller objects. Subsequent live logs confirmed loading and
position read-back; the user confirmed movement, wall/NPC blocking, jumping and
landing. The new step 4 look adapter is **not tested** in the games.

## Position transport

Protocol 4 uses the `Local\PassthroughPractice_link4_` namespace and retains the
256-byte position record and its layout-2 header. Integers and
floating-point bit patterns are stored little-endian, protected by the existing
named mutex. Bytes 0–127 retain the connection fields; bytes 128 onward hold:

| Offset | Field |
| --- | --- |
| 128 / 136 / 144 / 152 | sender generation / world generation / frame / boot-relative send time, each u64 |
| 160 | active u32; 164–167 reserved |
| 168 | partial-tick fraction, f64 |
| 176 / 184 / 192 | interpolated Minecraft X / Y / Z, each f64 |
| 200 / 208 | rendered player yaw / pitch in degrees, f64 |
| 216–255 | reserved |

Only Minecraft can publish positions. Skyrim requires a mutual handshake,
matching protocol and source generation, finite coordinates, and a sample no
older than 250 ms. Inactive samples clear movement authority immediately on
the next Skyrim update. Frame IDs must increase within each sender generation.

Fabric calls `pt_mc_init()` and then `pt_mc_frame4(world, frame, partial, x, y, z,
yaw, pitch, active)` on the render thread using Java 25's foreign-function API. No position
is calculated in a 20-Hz client-tick callback. Rendering rates can differ, so the
receiver consumes the newest frame instead of replaying a backlog. Tests use
their own named sessions and keep their logs in `logs/step2-fixtures`.

## Step 3 world export

The same pinned CommonLib checkout supplies these additional definitions:
`TESObjectCELL.cpp`, `hkpWorld.h`, `bhkWorld.h`, `BSAtomic.cpp`,
`ProcessLists.h/.cpp`, `Misc.cpp`, `BSHandleRefObject.cpp`, `NiRefObject.h`,
`hkpWorldRayCastInput.h`, `hkpWorldRayCastOutput.h`, and `hkpCollidable.h`.
No Skyrim binaries or assets are copied into source control.

| Interface | AE Address Library ID or offset |
| --- | --- |
| TESObjectCELL::GetbhkWorld | 18995 |
| hkpWorld::CastRay | 61399 |
| Havok world scale float | 188105 |
| ProcessLists singleton | 400315 |
| LookupReferenceByHandle | 12332 |
| BSReadWriteLock read lock/unlock | 68233 / 68239 |
| Reference parent cell / form ID | 0x60 / 0x14 |
| bhkWorld::GetWorld1 virtual slot | 0x27 |
| bhkWorld worldLock | 0xC598 |
| ProcessLists highActorHandles data / count | 0x30 / 0x40 |
| TESObjectREFR BSHandleRefObject base / reference count | 0x20 / base+8 |
| NiRefObject::DeleteThis virtual slot | 1 on the adjusted base pointer |
| Ray input size / alignment | 0x30 / 16 |
| Ray output size / alignment | 0x60 / 16 |
| Ray fraction / root collidable | 0x10 / 0x50 |
| Collidable filter info | 0x2C, layer mask 0x7F |

The exporter runs on the existing Skyrim player-update thread, holds the Havok
read lock during a sampling batch, and releases each acquired actor reference.
It keeps numeric copied data between frames, not Havok object pointers. NPCs
come from the same cell as the player. Shapes come from the cell's Havok world.
Layer 30 is used for ray filtering, and returned layers 1, 2, 3, 9, 10, 13, 17,
27, and 31 are included. Up to eight closest-hit casts continue through each ray.
The sampled boxes are an approximation; no Havok shape decoder is implemented.

The separate `_world_memory` mapping has 397,440 bytes, under its own mutex:

| Offset | Contents |
| --- | --- |
| 0 | Eight bytes `PTWORLD3` |
| 8 / 16 / 24 | Skyrim generation / recipient Minecraft generation / Minecraft world, u64 |
| 32 / 40 / 48 | origin epoch / snapshot sequence / Windows boot-relative send milliseconds, u64 |
| 56 / 60 / 64 | active / box count / NPC count, u32 |
| 72 | Java copy only: remaining snapshot lifetime in milliseconds, u64 |
| 128 | 8,192 slots of six f64 values: min XYZ, max XYZ (48 bytes each) |
| 393344 | 128 NPC slots: form ID u64, position XYZ f64 (32 bytes each) |

Only Skyrim publishes snapshots. Minecraft checks the connection handshake,
generations, active state, world and age before copying through
`pt_mc_world(pointer, capacity)`. Both receivers reject oversized counts or
invalid coordinates. Java constructs a spatially indexed immutable copy and
publishes it to the integrated server through a volatile reference. It restricts
queries to the current client level or its own integrated server and dimension.
Snapshots expire rather than retaining geometry indefinitely after a failure.

The common `BlockCollisions.computeNext` mixin supplies extra elements before
letting the vanilla iterator continue. It uses the existing result provider and
world-space shapes, skips suffocation-only queries, and does not change movement
resolution. Minecraft 26.3 method signatures and iterator behavior were inspected
from the ignored build cache. Source compilation passed; the new native ABI,
mixin runtime application was subsequently observed in live logs. The user has
confirmed basic wall/NPC blocking, jumping and landing; broad coverage remains
**not tested**.

## Step 4 look direction

Minecraft uses `player.getViewYRot(partial)` and `getViewXRot(partial)` with the
same partial tick used for position. Yaw is normalized to [-180,180) degrees;
pitch is bounded to [-90,90]. The decoder rejects nonfinite/out-of-range values.
The new native export name `pt_mc_frame4` prevents the older Java argument list
from silently calling the changed native signature. Protocol 4 isolates old
adapters with a new shared-memory namespace; the world-snapshot format stays 3.

Skyrim follows SkyCraft's `McYawToHeading` convention: heading equals yaw minus
180 degrees, normalized to [0,360), then converted to radians. Pitch retains its
sign and is converted to radians. The look adapter writes only player
`data.angle.x` (0x48) and `data.angle.z` (0x50), after the normal player update.
Skyrim's next normal first-person camera update consumes the new angles. This
does not override camera matrices or duplicate the camera update.

Additional published CommonLib interfaces, from the existing pinned checkout:

| Item | ID/offset |
| --- | --- |
| PlayerCamera singleton | AE ID 400802 (`PlayerCamera.cpp`) |
| ForceFirstPerson / ForceThirdPerson | AE IDs 50790 / 50796 |
| TESCamera currentState | 0x28 (`TESCamera.h`) |
| TESCameraState ID | 0x18 (`TESCameraState.h`) |
| First-person / ordinary third-person state IDs | 0 / 9 |
| TESCameraState GetRotation virtual slot | 4, writes NiQuaternion (w,x,y,z) |

`LOOK_RECV` logs the source angles and converted target/read-back angles, plus
camera-state quaternion. The quaternion is explicitly sampled before the next
camera update and is not claimed to be this frame's final rendered view.
The comparison tool's `-RequireLook` checks the source values bit-for-bit and
the conversion/read-back within 0.00001 radians. It does not prove rendered
camera alignment. Automated angle checks and live camera behavior are **not tested**.

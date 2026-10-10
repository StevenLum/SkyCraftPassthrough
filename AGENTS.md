# AGENTS.md

## Project
Passthrough mod from minecraft to skyrim

## Hard rules, never break these

1. **Never write game assets, decompiled code, or extracted game files into this
   repository.** They stay on this machine, untracked. If you need to read game
   data, read it from the install path at runtime or extract it to a gitignored
   folder.
2. **`.gitignore` is a whitelist.** It ignores everything and includes only
   source files. Do not switch it to a normal ignore list.
3. **Do not run `git commit` unless I asked.** Stage nothing beyond what the
   current task requires. Standing instruction since 2026-10-11, for both
   agents: commit each time I say that all the testing passed. Commit on the
   `dev` branch. `master` is what was last published to GitHub; do not push,
   merge or change `master` unless I ask.
4. **Do not touch anything outside this project folder** unless I explicitly name
   the path. This includes my game installs: read them, never write to them.
5. **Single-player and offline only.** If this touches online play, anti-cheat, or
   DRM, stop and tell me. Offline play with anti-cheat switched off through the
   game's own option is fine. Bypassing anti-cheat is never fine.
6. **Never put credentials in the repo or in any file you can read:** no API keys,
   no tokens, no passwords.

## How to work

- **Plan before code.** For anything more than a small fix, write the plan to
  `docs/DESIGN.md` first, then implement one step at a time.
- **One thing at a time.** Do not bundle unrelated changes. I want to be able to
  revert a single step.
- **Log, don't look.** You cannot see the game. Instrument instead: write
  positions, counts, timings, and state transitions to a log file so you can
  verify from the numbers. Do not try to visually inspect the game.
- **Collect logs directly.** The user authorizes running
  `.\tools\collect-scene-logs.ps1` and reading its collected game logs whenever
  needed for this project, without asking the user to collect or supply paths.
  This applies to both agents (given to Codex on 2026-10-08 and to Claude Code
  on 2026-10-11).
- **Test before build.** Run the Rust tests before every build and do not build
  on a failure. `.\tools\build-step5b.ps1` runs them first by itself. When a
  change adds a rule the packets or adapters must obey, add a test for it.
- **Tell me how to test it.** After every change, say the exact command to run and
  what I should see. "Done" without a test procedure is not done.
- **Ask before large refactors.** If you think the architecture is wrong, say so
  and explain, then wait for me.
- **Explain in plain language.** I am not the programmer here. If you use a term,
  explain it the first time.

## Honesty

- If something is not tested, write **"not tested"**. Never imply you verified
  something you didn't.
- If you are not sure, say you are not sure. A confident wrong answer costs me
  hours.
- If you hit something you cannot solve after two real attempts, stop and write
  it up as the first section of `STATUS.md` (see `templates/STATUS-handoff.md`)
  instead of trying variations at random.
- Record failures, not just successes. A dead end I can see is worth more than a
  dead end I have to watch you repeat.

## Keep these files updated

- `MODLOG.md`: add an entry after every change. Template in
  `templates/MODLOG-template.md`.
- `docs/DESIGN.md`: how the project works, in plain language. Update when the
  architecture changes, not on every commit.
- `README.md`: the "what works / what doesn't work" list. Test before you claim
  something works.
- `STATUS.md`: its first section is the handover note. See the next section.
- The "Backlog" section in `docs/DESIGN.md`: work I deferred and problems found
  but not fixed. Add to it when I defer something; update it when one is done.

## Handover between agents

Codex and Claude Code both work on this project, one at a time, in this same
folder. I may switch at any moment, including in the middle of a step, for
example when one of them runs out of usage. Work so that the other agent can
continue from the files alone. Nothing said only in chat, and nothing kept only
in an agent's private memory or notes, counts as handed over.

### What must be true at all times

- **The first section of `STATUS.md` is the handover note.** It follows
  `templates/HANDOVER-note.md` and describes the present moment. Everything
  below it is history: do not rely on it and do not edit it.
- **Update the handover note before you start changing code, not only after.**
  Say what you are about to do and which files you will touch, so that a change
  cut off halfway can be recognised. Update it again when the step is built,
  when it is waiting for my test, and when I accept or reject it. Replace the
  note's contents in place for these updates; start a new first section when a
  new step begins.
- **The plan is in `docs/DESIGN.md` before the code**, newest plan first, so an
  interrupted step can be finished from its plan.
- **Leave the project building.** If you have to stop with it broken, say so in
  the handover note with the failing command and its output.
- **Rules and permissions I give in chat go into this file, and state goes into
  the handover note, in the same session.** Say which agent a permission is
  for. Unless I say otherwise it is for both.

### Picking up from the other agent

1. Read this file, then the first section of `STATUS.md`.
2. Run `git status --short` and `git log --oneline -5`. Uncommitted changes are
   either the step named in the handover note or something to ask me about.
   Never discard them.
3. Read the newest three `MODLOG.md` entries, the newest plan and the Backlog in
   `docs/DESIGN.md`, and in `docs/ABI.md` only the parts for the area you will
   touch. You do not need to read the older history to continue.
4. Run `.\tools\cargo.ps1 test --offline`. If it fails and the handover note
   does not explain why, tell me before changing anything.
5. Tell me in a few lines what state you found and what you will do next, then
   continue the step in the handover note. Do not redo finished work or ask me
   to re-test accepted work.

If the handover note and the files disagree (for example the note says "built"
but the version numbers or packages say otherwise), trust the files, tell me
what you found, and correct the note first.

### Where things are

| File | Holds | Order |
| --- | --- | --- |
| `AGENTS.md` | rules and standing permissions | by topic |
| `STATUS.md` | the handover note, then older notes | newest first |
| `MODLOG.md` | one entry per change, with what was and was not tested | newest first |
| `docs/DESIGN.md` | plans, and one "Backlog" section | newest first |
| `docs/ABI.md` | data formats and facts about the two games' interfaces | newest first |
| `README.md` | what works, what does not, and how to test it | newest first |
| `templates/` | shapes for a MODLOG entry, the handover note, and the write-up after two failed attempts | |

In a newest-first file, add at the top and leave older text as it is.

### Things that have already cost time

- The version number lives in four places that must match:
  `passthrough/Cargo.toml`, `minecraft/gradle.properties`,
  `minecraft/src/main/resources/fabric.mod.json`, and `plugin_version` in
  `passthrough/src/skse.rs`.
- A change to a data format also needs: the protocol number in
  `passthrough/src/lib.rs`, the matching name in `passthrough/src/windows.rs`,
  the sizes in `minecraft/src/main/java/dev/passthrough/NativeLink.java`, and
  every range check on the changed field. Add a test for the new value.
- When a change adds a new state (a place, a menu context, a generation), look
  for code that may still hold data from the old state. Left-over data must be
  skipped, not treated as an error that switches a feature off.
- Every tracked file is UTF-8; keep it that way and check after editing (an
  older Windows-1252 dash character once got damaged by an edit). Line endings
  are a mix of Windows and Unix style in the working folder and that is
  harmless: Git stores them all one way (`core.autocrlf=true`). Do not "fix"
  line endings. In Git Bash, `grep $'\r$'` reports Windows endings on every
  file whether or not they are there; count bytes with a script instead.
- Game logs are in the ignored `logs` folder. Search them with
  `rg --no-ignore`; a plain search silently skips them.
- The collector keeps scene, error and connection lines only. When the bridge
  goes silent, look for `event=ERROR` and `LINK` lines first.
- In Windows PowerShell 5.1 do not add `2>&1` or `*>&1` to the build or cargo
  scripts; it turns their normal progress output into an error.

## Environment

- OS: Windows 11
- Game A: Minecraft version 26.3, installed at C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3
- Game B: Skyrim version 1.7.104, installed at E:\SteamLibrary\steamapps\common\Skyrim Special Edition
- Loader: SKSE and Fabric
- Which game is authoritative for the player: Minecraft
- Language and version: Rust stable with MSVC
- Agents: Codex and Claude Code, one at a time (see "Handover between agents")

# amane config (JAQC rewrite)

This folder is the config for **amane**, a Rust Wayland layer-shell toolkit (niri, NixOS). It is a rewrite of my Quickshell shell "JAQC" into amane.
- Library repo: `~/Projects/amane`. Config repo: `~/.config/amane` (this folder, own git repo).
- Run cargo inside `nix develop`.
- Test with `amane dev` only. Release compile is too slow.
- After any change to the amane library, reinstall the CLI in `~/Projects/amane`: `cargo install --debug --path cli`. The CLI bundles its own copy of amane.
- CLI commands: startup, dev, compile, run, ipc call.
- `~/CLAUDE.md` also loads here because it sits in the home folder. It holds the amane **library's** internal rules (Renderer, glyphs, etc.). Ignore it unless a task truly needs a new generic primitive in `~/Projects/amane`. Work in this repo is shell config.

## Rules

**Never add, track or commit `CLAUDE.local.md`. Never list it in `.gitignore` either. Leave it untracked.**

- Be token-efficient: use the ponytail plugin and agent skills, read only the files you need, never read crate source (use documented APIs), build only what's asked, one build at the end, short replies.
- Don't treat todo items as approved work. If the ask is vague, confirm which item first.
- Pick sensible defaults. Ask only on real design forks. A question is not a go-ahead.
- Check `src/` before claiming amane lacks something.
- One commit per finished feature, in my style: short, lowercase, match `git log`. No Co-Authored-By or Claude-Session trailers. No push unless asked.
- Performance matters. Measure fps and RAM against Quickshell before calling something good (current: panels 53-61 fps, ~97 MB vs Quickshell 240 MB).

## Code rules

- Shell-specific things stay in this config. Only generic primitives the config needs go into the amane library.
- Configs are always Rust. Never copy Quickshell/QML conventions.
- One concern per small file.
- Every third-party lib lives behind one module (vello/wgpu in graphics/gpu, zbus in dbus/, serde_json in niri/, libpulse in services/audio/pulse.rs, PAM FFI in services/lock/pam.rs).
- New crates must be raised with an alternative before adding.
- Use libraries through their documented API only.
- Colors are plain hex; opacity is its own method.
- Markers are bare names (Full, Parent, Start, Pointer, Text), not Enum::Variant.
- `on_click` stays `Fn(Button)`.
- Comments describe the code only. Never mention JAQC or Quickshell, and never write "like X's ...".
- Size changes must animate. Animations use `src/motion/` presets rather than ad-hoc curves.
- Don't redraw or allocate per frame without need; the known per-frame costs are listed in REWRITE.md.

## Layout

- Liquid surface: `src/liquid.rs` + `liquid.wgsl`. A band past the screen edges counts as liquid so panels melt into edges.
- Animation: `src/motion/` (Spring, Glide, follow/fade, presets).
- Panels share one overlay window, always mapped (1px when empty, which also warms up shaders), sized to what open panels reach.
- Font: `JetBrainsMono Nerd Font`.
- `REWRITE.md` (gitignored) lists known gaps and costs.

## amane API

- **App**: window, window_per_monitor(fn(&Monitor)), normal_window(Window), lock(fn(&Monitor)), ipc, font, run.
- **LayerWindow**: anchor, margin, layer, keyboard, space, visible, input_region/click_through, on_key.
- **Rectangle**: fill (Color | hex | Image | Mask | Gradient::linear/radial), radius, border, blur, opacity, shadow, padding, align_child, clip, cursor, rotate/scale/translate, shader(wgsl|glsl path), shader_values (up to 16 [f32;4]), on_click(Button), on_hover, on_scroll, on_drag/on_move(Point).
- **Row/Column**: gap, justify, align. **Stack**: layered children, last on top, place with .translate. **Scroll**. **Canvas**.
- **Text**: size, color, font, weight, wrap, elide, max_lines, tight. **TextInput**: placeholder, password, on_change, on_submit, focused, set_text.
- **Image**: loaded, .thumbnail(w,h). **Animation/Easing**. `request_frame()`.
- **Services**: Audio (+mic level/mute), Media, Network (scan, access_points, connect, disconnect, set_wifi, saved/connecting), Notifications (list, dismiss, invoke, click, actions, arrival time, clear all), Workspaces, Apps (list, launch, icon_path), Lock (unlock, checking, failed), Battery, Memory, Cpu, Brightness (logind), Bluetooth (bluez).
- **Bus** (D-Bus client+server), spawn/output/lines, watch_file.

## Done

- Bar: full black rect + bar background with 16px rounded top corners.
- Power menu (NixOS logo).
- Launcher: `amane ipc call launcher toggle`.
- Utility center (right panel; opens from the tray pill or `amane ipc call utility show wifi`): tabs, wifi, calendar, notifications, bluetooth pages, brightness slider, light/dark switch.
- Notification server starts at amane launch.
- Wallpaper layer + screen mask, wallpaper picker (`amane ipc call wallpaper toggle`; writes Quickshell's `wallpaper-selection`).
- Liquid: blend radius 24, soft shadow in liquid.wgsl (#50000000, 10px blur, 4px down, panels only).

## Next

Tick each item off here when its commit lands.

- [x] Notification popups (0329d27).
- [ ] Font fallback (library, `~/Projects/amane`): characters missing from the chosen font (Japanese etc.) render as tofu. Handed to the library session.
- [ ] Control center (top edge, JAQC `widgets/controlcenter/`): media card with controls and wavy progress bar, volume and mic faders.
- [ ] Persist the light/dark choice across restarts.
- [ ] Point niri's Ctrl+Space and Mod+Shift+W at amane instead of qs. Only when I say so.

Known library gaps (see REWRITE.md): SVG icons, icon font fallback, tray, rectangles sizing to content, timers, color transitions, triggering the lock screen from a running shell, .desktop descriptions/keywords, a wrong wifi password leaving a saved profile. Known costs: canvas texture created every frame, every window redraws on any service change, switched wallpapers stay in memory until restart.

## Usage

Start each session with: "do the next unchecked item in CLAUDE.local.md, then tick it off".

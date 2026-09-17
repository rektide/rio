RIO(5)

# NAME

Rio - TOML configuration file format.

# SYNTAX

Rio's configuration file uses the TOML format. The format's specification
can be found at _https://toml.io/en/v1.0.0_.

# LOCATION

Rio doesn't create the config file for you, but it looks for one in the
following locations on UNIX systems:

. _$RIO_CONFIG_HOME/config.toml_
. _$XDG_CONFIG_HOME/rio/config.toml_
. _$HOME/.config/rio/config.toml_

On Windows, the config file will be looked for in:

. _%USERPROFILE%\\AppData\\Local\\rio\\config.toml_

# GENERAL

*theme* = _"<string>"_

	Theme name to use. When set, colors defined in the configuration file will be ignored.

	Default: _None_

*confirm-before-quit* = _true_ | _false_

	Require confirmation before quitting.

	Default: _true_

*draw-bold-text-with-light-colors* = _true_ | _false_

	When _true_, bold text is drawn using the bright color variants.

	Default: _false_

*env-vars* = [_"<string>"_,]

	Environment variables to set for spawned processes.

	Example:
		env-vars = [_"TERM=xterm-256color"_, _"COLORTERM=truecolor"_]

# WINDOW

This section documents the *[window]* table of the configuration file.

*width* = _<integer>_

	Window width in pixels.

	Default: _600_

*height* = _<integer>_

	Window height in pixels.

	Default: _400_

*mode* = _"Windowed"_ | _"Maximized"_ | _"Fullscreen"_

	Window startup mode.

	Default: _"Windowed"_

*opacity* = _<float>_

	Background opacity as a floating point number from _0.0_ to _1.0_.

	Default: _1.0_

*blur* = _true_ | _false_

	Request compositor to blur content behind transparent windows.

	Default: _false_

*decorations* = _"Enabled"_ | _"Disabled"_ | _"Transparent"_ | _"Buttonless"_

	Window decorations.

	*Enabled*
		Normal window decorations.
	*Disabled*
		No window decorations.
	*Transparent* _(macOS only)_
		Transparent title bar.
	*Buttonless* _(macOS only)_
		Title bar without buttons.

	Default: _"Enabled"_

*background-image* = { path = _"<string>"_, opacity = _<float>_ }

	Window background image. The image is uploaded once into a dedicated GPU
	texture and stretched to cover the full window.

	*path*
		Path to the background image file.
	*opacity*
		Image opacity from _0.0_ to _1.0_. Default _1.0_.

# FONTS

This section documents the *[fonts]* table of the configuration file.

*size* = _<float>_

	Font size in points.

	Default: _18.0_

*family* = _"<string>"_

	Font family name. When set, overrides all font variant families.

	Default: _"cascadiacode"_

*features* = [_"<string>"_,]

	OpenType font features to enable.

	Example:
		features = [_"ss02"_, _"ss03"_, _"liga"_]

*use-drawable-chars* = _true_ | _false_

	Use built-in drawable characters for box drawing.

	Default: _true_

*disable-warnings-not-found* = _true_ | _false_

	Disable warnings when fonts are not found.

	Default: _false_

*additional-dirs* = [_"<string>"_,]

	Additional directories to search for fonts.

*regular* = { family = _"<string>"_, style = _"<string>"_, width = _"<string>"_, weight = _<integer>_ }

	Regular font configuration.

	Default: { family = _"cascadiacode"_, style = _"Normal"_, width = _"Normal"_, weight = _400_ }

*bold* = { family = _"<string>"_, style = _"<string>"_, width = _"<string>"_, weight = _<integer>_ }

	Bold font configuration.

	Default: { family = _"cascadiacode"_, style = _"Normal"_, width = _"Normal"_, weight = _800_ }

*italic* = { family = _"<string>"_, style = _"<string>"_, width = _"<string>"_, weight = _<integer>_ }

	Italic font configuration.

	Default: { family = _"cascadiacode"_, style = _"Italic"_, width = _"Normal"_, weight = _400_ }

*bold-italic* = { family = _"<string>"_, style = _"<string>"_, width = _"<string>"_, weight = _<integer>_ }

	Bold italic font configuration.

	Default: { family = _"cascadiacode"_, style = _"Italic"_, width = _"Normal"_, weight = _800_ }

*extras* = [{ family = _"<string>"_ },]

	Additional fonts to load.

*emoji* = { family = _"<string>"_ }

	Emoji font configuration.

	Default: Built-in Twemoji font

*hinting* = _true_ | _false_

	Enable font hinting.

	Default: _true_

*symbol-map* = [{ chars = _"<string>"_, family = _"<string>"_ },]

	Map specific character ranges to fonts.

# COLORS

This section documents the *[colors]* table of the configuration file.

Colors are specified using their hexadecimal values with a _#_ prefix: _#RRGGBB_ or _#RRGGBBAA_.

*background* = _"<string>"_

	Default background color.

*foreground* = _"<string>"_

	Default foreground color.

*cursor* = _"<string>"_

	Cursor color.

*vi-cursor* = _"<string>"_

	Vi mode cursor color.

*black*, *red*, *green*, *yellow*, *blue*, *magenta*, *cyan*, *white* = _"<string>"_

	Standard ANSI colors.

*light-black*, *light-red*, *light-green*, *light-yellow*, *light-blue*, *light-magenta*, *light-cyan*, *light-white* = _"<string>"_

	Bright ANSI colors.

*dim-black*, *dim-red*, *dim-green*, *dim-yellow*, *dim-blue*, *dim-magenta*, *dim-cyan*, *dim-white* = _"<string>"_

	Dim ANSI colors.

*tabs* = _"<string>"_

	Inactive tab title color. Tab island backgrounds are derived from
	the window background color automatically.

*tabs-active* = _"<string>"_

	Active tab title color.

*split* = _"<string>"_

	Split separator color.

*search-match-background* = _"<string>"_

	Search match background color.

*search-match-foreground* = _"<string>"_

	Search match text color.

*search-focused-match-background* = _"<string>"_

	Focused search match background color.

*search-focused-match-foreground* = _"<string>"_

	Focused search match text color.

*selection-background* = _"<string>"_

	Selection background color.

*selection-foreground* = _"<string>"_

	Selection text color.

# CURSOR

This section documents the *[cursor]* table of the configuration file.

*shape* = _"block"_ | _"underline"_ | _"beam"_

	Cursor shape.

	Default: _"block"_

*blinking* = _true_ | _false_

	Enable cursor blinking.

	Default: _false_

*blinking-interval* = _<integer>_

	Cursor blinking interval in milliseconds. Range: 350-1200.

	Default: _800_

# SCROLL

This section documents the *[scroll]* table of the configuration file.

*multiplier* = _<float>_

	Scroll speed multiplier.

	Default: _3.0_

*divider* = _<float>_

	Scroll divider for fine-tuning.

	Default: _1.0_

# NAVIGATION

This section documents the *[navigation]* table of the configuration file.

*mode* = _"NativeTab"_ | _"Tab"_ | _"Plain"_

Navigation display mode.

Default: _"Tab"_

*clickable* = _true_ | _false_

	Enable clickable navigation elements.

	Default: _false_

*use-current-path* = _true_ | _false_

	Show current working directory in navigation.

	Default: _false_

*color-automation* = [{ program = _"<string>"_, path = _"<string>"_, color = _"<string>"_ },]

	Automatic color assignment based on program or path.

# ISLAND

This section documents the *[island]* table of the configuration file: the
geometry and palette of the tab strip drawn in *Tab* navigation mode. All
sizes are logical pixels; every field is optional.

*height* = _<float>_

	Height of the whole strip. The terminal grid starts below it and
	mouse routing treats y < height as strip chrome.

	Default: _38.0_

*title-font-size* = _<float>_

	Font size of tab titles.

	Default: _12.0_

*tab-padding-x* = _<float>_

	Horizontal padding between a tab's edge and its (centred) title.

	Default: _27.0_

*tab-gap* = _<float>_

	Space between tab slots; the pill is inset by half the gap on each
	side.

	Default: _6.0_

*tab-inset-y* = _<float>_

	Vertical inset of a tab pill from the top and bottom of the strip.
	Kept below half of *height*.

	Default: _7.0_

*tab-radius* = _<float>_

	Corner radius of a tab pill. Clamped to half the pill at draw time,
	so large values round into a capsule.

	Default: _6.0_

*margin-right* = _<float>_

	Right margin kept after the last tab slot.

	Default: _8.0_

*margin-left-macos* = _<float>_

	Left margin on macOS clearing the traffic-light buttons. Ignored on
	other platforms.

	Default: _76.0_

*progress-bar-height* = _<float>_

	Thickness of the OSC 9;4 progress bar.

	Default: _3.0_

*progress-bar-timeout-secs* = _<integer>_

	Seconds without an OSC 9;4 report before the progress bar is
	dismissed.

	Default: _15_

*picker-swatch-size* = _<float>_

	Size of one color-picker swatch.

	Default: _18.0_

*picker-swatch-gap* = _<float>_

	Gap between color-picker swatches.

	Default: _4.0_

*picker-padding* = _<float>_

	Inner padding of the color-picker dropdown.

	Default: _6.0_

*picker-input-height* = _<float>_

	Height of the rename input inside the color picker.

	Default: _26.0_

*picker-input-font-size* = _<float>_

	Font size of the rename input.

	Default: _12.0_

*picker-input-margin-top* = _<float>_

	Space between the swatch row and the rename input.

	Default: _8.0_

*picker-top-padding* = _<float>_

	Extra padding above the first swatch row.

	Default: _4.0_

*picker-colors* = _<list of hex colors>_

	Swatches offered by the tab color picker (right-click or
	ctrl-click a tab), as hex strings. The picker's width follows the
	list length.

	Default: red, orange, yellow, green, blue, purple

*close-margin-right* = _<float>_

	Distance from a pill's right edge to the close glyph center.

	Default: _14.0_

*close-glyph-half* = _<float>_

	Half-size of the × glyph.

	Default: _3.5_

*close-min-island-width* = _<float>_

	Pills narrower than this hide the close button.

	Default: _64.0_

*close-hover-half* = _<float>_

	Half-size of the close-button hover highlight box.

	Default: _10.0_

*close-hover-corner-radius* = _<float>_

	Corner radius of the close-button hover highlight.

	Default: _5.0_

*close-hit-half-width* = _<float>_

	Half-width of the close-button hit area.

	Default: _10.0_

*close-alpha-idle* = _<float>_

	Idle alpha of the close glyph.

	Default: _0.55_

*close-alpha-hover* = _<float>_

	Hover alpha of the close glyph.

	Default: _0.95_

*close-stroke-width* = _<float>_

	Stroke width of the close glyph lines.

	Default: _1.2_

*inactive-custom-mute* = _<float>_

	Alpha multiplier applied to a custom-colored tab when it is not
	the active one.

	Default: _0.55_

*drag-threshold* = _<float>_

	Pointer travel before a tab press becomes a drag.

	Default: _4.0_

*drag-animation-length* = _<float>_

	Slide-animation length (seconds) when tabs settle after reorder.

	Default: _0.15_

*drag-max-dt* = _<float>_

	Upper bound on a single animation step (seconds).

	Default: _0.05_

*bell-dot-size* = _<float>_

	Edge length of the bell dot drawn beside the title of a
	background tab that rang.

	Default: _6.0_

*bell-dot-order* = _<integer>_

	Rect draw order of the bell dot; must sit above the tab
	backgrounds, which are submitted at order 0.

	Default: _1_

*bell-gap* = _<float>_

	Gap between the bell dot and the tab title.

	Default: _4.0_

Values are clamped at load time so a degenerate setting degrades to a
bound instead of drawing a broken strip.

# SHELL

This section documents the *[shell]* table of the configuration file.

*program* = _"<string>"_

	Shell program to execute.

	Default: System default shell

*args* = [_"<string>"_,]

	Arguments to pass to the shell.

# EDITOR

This section documents the *[editor]* table of the configuration file.

*program* = _"<string>"_

	Editor program for opening configuration files.

	Default: _"vi"_

*args* = [_"<string>"_,]

	Arguments to pass to the editor.

# BINDINGS

This section documents the *[bindings]* table of the configuration file.

*keys* = [{ key = _"<string>"_, with = _"<string>"_, action = _"<string>"_ },]

	Key bindings configuration.

	*key*
		Key name (e.g., _"q"_, _"Enter"_, _"F1"_).

	*with*
		Modifier keys: _"super"_, _"alt"_, _"ctrl"_, _"shift"_.
		Multiple modifiers can be combined with _|_.

	*action*
		Action to perform. Available actions include:
		_"Quit"_, _"CreateWindow"_, _"CloseWindow"_, _"Copy"_, _"Paste"_,
		_"ScrollUp"_, _"ScrollDown"_, _"ToggleViMode"_, _"OpenConfigEditor"_,
		and many others.

# RENDERER

This section documents the *[renderer]* table of the configuration file.

*performance* = _"High"_ | _"Low"_

	Rendering performance mode.

	Default: _"High"_

*backend* = _"Automatic"_ | _"GL"_ | _"Vulkan"_ | _"DX12"_ | _"Metal"_

	Graphics backend to use.

	Default: _"Automatic"_

*disable-renderer-when-unfocused* = _true_ | _false_

	Disable rendering when window is unfocused to save power.

	Default: _false_

*level* = _<integer>_

	Rendering level (0-3).

	Default: _3_

# DEVELOPER

This section documents the *[developer]* table of the configuration file.

*log-level* = _"OFF"_ | _"ERROR"_ | _"WARN"_ | _"INFO"_ | _"DEBUG"_ | _"TRACE"_

	Log level for debugging.

	Default: _"OFF"_

*enable-log-file* = _true_ | _false_

	Write logs to file.

	Default: _false_

*enable-fps-counter* = _true_ | _false_

	Display FPS counter.

	Default: _false_

# SEE ALSO

*rio*(1)

# BUGS

Found a bug? Please report it at _https://github.com/raphamorim/rio/issues_.

# MAINTAINERS

- Raphael Amorim <rapha850@gmail.com>

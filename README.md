## cybercmd

Command-line argument loader for Cyberpunk 2077.

### usage

- if you use CET:
  - download and extract [cybercmd.zip from the latest release](https://github.com/jac3km4/cybercmd/releases) into the main game directory
- if you don't use CET:
  - download and extract [cybercmd-standalone.zip from the latest release](https://github.com/jac3km4/cybercmd/releases) into the main game directory
    - if prompted, do not overwrite any files except for `cybercmd.asi`

### Vortex

Use the regular `cybercmd.zip` (the versioned CI/release package), with CET installed.
It contains only `bin/x64/plugins/cybercmd.asi`. Do not import
`cybercmd-standalone.zip` into Vortex: its additional ASI loader files in `bin/x64`
are rejected by the Cyberpunk 2077 ASI installer with `Too many files in ASI Mod! 1`.
For installations without CET, extract the standalone package manually as described above.

### usage from mods

When the game starts, it'll look for arguments to load in TOML files placed inside `Cyberpunk 2077\r6\config\cybercmd\`.
You can see a reference file [here](https://github.com/jac3km4/cybercmd/blob/master/reference/cmd.toml).

# Vendored detour-rs (ec23632)

Upstream: https://github.com/veeenu/detour-rs @ `ec23632`

Local change vs upstream:
- Gate `extern "thiscall"` hookable impls with `target_arch = "x86"`.
  Modern rustc rejects `thiscall` on `x86_64` (E0570); cybercmd only needs
  `system`/`C` ABIs on 64-bit Windows.

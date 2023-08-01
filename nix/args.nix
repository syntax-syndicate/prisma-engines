{ flakeInputs, system, ... }:
{
  config._module.args =
    let
      overlays = [
        flakeInputs.rust-overlay.overlays.default
        (self: super:
          let toolchain = super.rust-bin.nightly."2023-06-20"; in
          { cargo = toolchain.minimal; rustc = toolchain.minimal; rustToolchain = toolchain; })
      ];
    in
    { pkgs = import flakeInputs.nixpkgs { inherit system overlays; }; };
}

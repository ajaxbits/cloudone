{
  description = "A basic flake with a shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        get-openapi-spec = pkgs.writeShellScriptBin "get-openapi-spec" ''
          ${pkgs.jq}/bin/jq 'del(.components.securitySchemes.Legacy_API_Key)' swagger.json > openapi-spec.json
        '';
      in
      with pkgs; {
        devShell = mkShell {
          buildInputs = [
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
            cargo-edit
            openssl
            pkgconfig
            openapi-generator-cli
            get-openapi-spec
          ];
        };
      });
}

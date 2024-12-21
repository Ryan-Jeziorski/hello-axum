{
  description = "A Nix-flake-based Rust development environment";

  nixConfig.bash-prompt = "\n\\[\\033[$PROMPT_COLOR\\](nix-dev)[\\[\\e]0;\\u@\\h: \\w\\a\\]\\u@\\h:\\w]\\$\\[\\033[0m\\] ";
  # nixConfig.bash-prompt = "\n[\\u@\\h";
  # nixConfig.bash-prompt-suffix = "(dev) $";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.*.tar.gz";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixvim = {
      url = "github:Ryan-Jeziorski/nix-vim-config/more_plugins";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, nixvim }:
    let
      overlays = [
        rust-overlay.overlays.default
        (final: prev: {
          rustToolchain =
            let
              rust = prev.rust-bin;
            in
            if builtins.pathExists ./rust-toolchain.toml then
              rust.fromRustupToolchainFile ./rust-toolchain.toml
            else if builtins.pathExists ./rust-toolchain then
              rust.fromRustupToolchainFile ./rust-toolchain
            else
              rust.stable.latest.default.override {
                extensions = [ "rust-src" "rustfmt" ];
              };
          nixvim = nixvim;
        })
      ];
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit overlays system nixvim; };
      });
    in
    {
      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            openssl
            pkg-config
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer

            # Other Tools
            lsd
            tree
            helix
            nixvim.legacyPackages.${system}.nixvim
            #nixvim.legacyPackages.x86_64-linux.nixvim

          ];
        };
      });
    };
}

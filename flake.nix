{
    description = "A flake for getting bevy dependecies set up";
    
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs, }: {
        devShells.x86_64-linux =
        let
            pkgs = import nixpkgs { system = "x86_64-linux"; };
        in 
        {
            default = pkgs.mkShell rec {
                nativeBuildInputs = with pkgs; [
                    pkg-config
                ];
                buildInputs = with pkgs; [
		                neovim tmux
                    rustup
                    rust-analyzer
                    udev alsa-lib vulkan-loader
                    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
                    libxkbcommon wayland # To use the wayland feature
                    neovim
                    tmux
                    rust-analyzer
                    udev
                    alsa-lib
                    libdrm
                    libxkbcommon
                    codex
                    wayland
                ];
                shellHook = ''
                  rustup default stable
                  rustup component add rust-analyzer
                  #export CARGO_MANIFEST_DIR=$(realpath ./)
                  set -a
                  source ./API.secret.env
                  set +a
                  exec zsh
                '';
                LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
                RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            };
        };
    };
}

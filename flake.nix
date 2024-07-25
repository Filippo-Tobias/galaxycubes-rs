{
    description = "A flake for getting bevy dependecies set up";
    
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs }: {
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
                    rustup
                    rust-analyzer
                    udev alsa-lib vulkan-loader
                    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
                    libxkbcommon wayland # To use the wayland feature
                ];
                shellHook = ''
                  rustup default stable
                  rustup component add rust-analyzer
                '';
                LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
                RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            };
        };
    };
}

{
  description = "A Nix-flake-based Node.js development environment";

  inputs.nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.*.tar.gz";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit system; overlays = [ self.overlays.default ]; };
      });
    in
    {
      overlays.default = final: prev: rec {
        nodejs = prev.nodejs;
        yarn = (prev.yarn.override { inherit nodejs; });
      };

      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          packages = with pkgs; [ node2nix nodejs nodePackages.pnpm yarn ];
          shellHook = ''
            export PATH=$PATH:node_modules/vite/bin/
            NODE_DIR="node_modules"
            echo "Checking node modules..."
            if [ -d "$NODE_DIR" ]; then
              echo "Node modules already installed (skipping installation)"
            else
              ${pkgs.nodejs}/bin/npm install
              echo "Installing node modules..."
            fi
          '';
        };
      });
    };
}

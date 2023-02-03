with import <nixpkgs> {};
mkShell {

    buildInputs = [
        clangStdenv
        cmake

    ]
}
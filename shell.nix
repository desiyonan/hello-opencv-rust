# use `nix-shell`
with import <nixpkgs> {};

mkShell {

    buildInputs = [
    ];

    nativeBuildInputs = [
        # rustup
        cargo
        llvmPackages.clangUseLLVM
        llvmPackages.libclang
        cmake
        opencv
    ];

    LIBCLANG_PATH="${llvmPackages.libclang.lib}/lib";
}

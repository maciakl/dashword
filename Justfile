PROJ := "dashword"
VER := `sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml`

all: release

version:
    @echo "{{VER}}"

bump part:
    bmpv Cargo.toml {{part}}

build:
    cargo build --release

zip:
    cd target/release && zip {{PROJ}}_{{VER}}_Windows_x86_64.zip {{PROJ}}.exe
    cd target/release && tar -czf {{PROJ}}_{{VER}}_Linux_x86_64.tar.gz {{PROJ}}

hash: zip
    cd target/release && sha256sum {{PROJ}}_{{VER}}_Windows_x86_64.zip > checksums-{{VER}}.txt
    cd target/release && sha256sum {{PROJ}}_{{VER}}_Linux_x86_64.tar.gz >> checksums-{{VER}}.txt
    cat target/release/checksums-{{VER}}.txt

release: hash
    git tag -a "v{{VER}}" -m "Release v{{VER}}"
    git push origin "v{{VER}}"
    gh release create "v{{VER}}" target/release/{{PROJ}}_{{VER}}_Windows_x86_64.zip target/release/{{PROJ}}_{{VER}}_Linux_x86_64.tar.gz target/release/checksums-{{VER}}.txt --title "v{{VER}}" --generate-notes

publish:
    cargo publish

clean:
    cargo clean

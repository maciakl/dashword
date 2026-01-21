PROJ := `sed -n 's/^name *= *"\(.*\)"/\1/p' Cargo.toml`
VER := `sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml`

all: release

version:
    @echo "{{VER}}"

info:
    @echo "Project: {{PROJ}}"
    @echo "Version: {{VER}}"
    @echo Git Branch: $(git rev-parse --abbrev-ref HEAD)
    @echo Git HEAD: $(git rev-parse --short HEAD)
    @echo Git Tag: $(git describe --tags --abbrev=0) \($(git rev-parse --short $(git describe --tags --abbrev=0)^{commit})\)
    @echo Git Status: $(git status --porcelain | wc -l) uncommitted
    @echo Git Origin: $(git config --get remote.origin.url)
    @echo Recent commits:
    @git --no-pager log --oneline --graph --decorate -10
    @git --no-pager log --oneline --graph --decorate -10

bump part:
    bmpv Cargo.toml {{part}}

build:
    cargo build --release

[windows]
zip: build
    cd target/release && zip {{PROJ}}_{{VER}}_Windows_x86_64.zip {{PROJ}}.exe

[linux]
zip: build
    cd target/release && tar -czf {{PROJ}}_{{VER}}_Linux_x86_64.tar.gz {{PROJ}}
    
[macos]
zip: build
    cd target/release && tar -czf {{PROJ}}_{{VER}}_Darwin_arm64.tar.gz {{PROJ}}

[windows]
hash: zip
    cd target/release && sha256sum {{PROJ}}_{{VER}}_Windows_x86_64.zip >> checksums-{{VER}}.txt
    cat target/release/checksums-{{VER}}.txt

[linux]
hash: zip
    cd target/release && sha256sum {{PROJ}}_{{VER}}_Linux_x86_64.tar.gz >> checksums-{{VER}}.txt
    cat target/release/checksums-{{VER}}.txt

[macos]
hash: zip
    cd target/release && sha256sum {{PROJ}}_{{VER}}_Darwin_arm64.tar.gz >> checksums-{{VER}}.txt
    cat target/release/checksums-{{VER}}.txt

release: hash
    git tag -a "v{{VER}}" -m "Release v{{VER}}"
    git push origin "v{{VER}}"
    gh release create "v{{VER}}" \
        target/release/{{PROJ}}_{{VER}}_Windows_x86_64.zip \
        target/release/{{PROJ}}_{{VER}}_Linux_x86_64.tar.gz \
        target/release/{{PROJ}}_{{VER}}_Darwin_arm64.tar.gz \
        target/release/checksums-{{VER}}.txt \
        --title "v{{VER}}" --generate-notes

publish:
    cargo publish

clean:
    cargo clean

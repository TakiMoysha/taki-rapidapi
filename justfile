
bin-clown:
    RUST_LOG=info cargo run --bin clown-bin

py-dev *args:
    uv run dev {{args}}

build *args:
    uv run maturin build {{args}}

dev-install *args:
    uv run maturin develop --uv {{args}}

granian-dev:
    uv run granian --interface rsgi py.prototype.main:rsgi_app

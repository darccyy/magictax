# Name of program (current folder)
name := `basename "$(pwd)"`

# Run program as debug
run:
  cargo run

# Install program to system
install:
  cargo install --path .

# Cross compile to x86 Windows, and compress to zip
cross:
  cargo install cross &&\
  cross build --release --target x86_64-pc-windows-gnu &&\
  cd target/x86_64-pc-windows-gnu/release/ &&\
  zip -v {{name}} {{name}}.exe

# Build program with all targets: debug, release (installed), and x86 Windows release
build-all:
  cargo build &&\
  just install &&\
  just cross


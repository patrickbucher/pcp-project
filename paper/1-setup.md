# Installation

Eine Rust-Installation beinhaltet folgende Werkzeuge:

- `rustc`: Compiler
- `cargo`: Package- und Build-Management
- `rustdoc`: Erstellung von Dokumentation

## Voraussetzungen

Unter macOS und Linux wird ein Linker und ein C-Compiler benötigt. `gcc` und
`llvm` sind weit verbreitete Optionen. Für Windows wird das [Microsoft Visual
C++ Redistributable for Visual Studio
2017](https://www.visualstudio.com/downloads) benötigt:

- [Version für 64-Bit-Architektur](https://aka.ms/vs/15/release/VC_redist.x64.exe)
- [Version für 32-Bit-Architektur](https://aka.ms/vs/15/release/VC_redist.x86.exe)

## macOS and Linux: Mittels `rustup`

Die einfachste Variante ist die Installation mit `rustup`, wozu man folgende
Befehlszeile mit der Shell ausführen muss:

```bash
$ curl https://sh.rustup.rs -sSf | sh 
```

Anschliessend kann man den Instruktionen folgen. Die `curl`-Parameters sind
nötig um zu verhindern, dass `sh` Zeichen ausgaben erhält, womit es nichts
anfangen kann:

- `-s`: silent (keine Statusmeldungen ausgeben)
- `-S`: show errors (nur Fehler anzeigen)
- `-f`: fail silently (bei serverseitigen Fehlern keine  Ausgabe produzieren)

Damit die Umgebungsvariablen nach der Installation aktualisiert werden kann man
entweder eine neue Shell öffnen oder folgende Befehlszeile ausführen:

```bash
$ source $HOME/.cargo/env
```

Anschliessend kann man die Installation folgendermassen überprüfen:

```bash
$ rustc --version
rustc 1.30.1 (1433507eb 2018-11-07)
$ cargo version
cargo 1.30.0 (a1a4ad372 2018-11-02)
$ rustdoc --version
rustdoc 1.30.1 (1433507eb 2018-11-07)
```

Aktualisierungen können mit `rustup` durchgeführt werden:

```bash
$ rustup update
```

Rust und `rustup` könne folgendermassen wieder deinstalliert werden:

```bash
$ rustup self uninstall
```

## macOS and Linux: Mittels Package Manager

macOS mit Homebrew und viele Linux-Distributionen bieten eigene Packages für
Rust an. Nach der Installation sollte man wie gerade beschrieben sicherstellen,
dass zumindest die Programme `rustc`, `cargo` und `rustdoc` installiert worden
sind.

Homebrew (macOS):

```bash
$ brew install rust
```

aptitude (Debian, Ubuntu):

```bash
$ aptitude install rustc cargo rust-doc
```

pacman (Arch Linux):

```bash
$ pacman -S rust rust-docs
```

## Windows: Mittels `rustup-init`

Für Windows lässt sich Rust über ein Setup-Programm installieren:

1. `rustup-init.exe` von [win.rustup.rs](https://win.rustup.rs/) herunterladen.
2. Das Programm ausführen und den Anweisungen folgen.
3. Eine längere Pause einplanen, da die Installation der Dokumentation auf
   Windows [bekanntermassen wesentlich länger
   dauert](https://github.com/rust-lang/rustup.rs/issues/763) als auf macOS and
   Linux.
4. Die Installation mittels `cmd.exe` oder Powershell validieren (siehe oben).

# Memory Safety Demo — C vs Rust

Begeleidingsmateriaal bij de tech talk *"Rust is geen hype: waarom de industrie herschrijft"*.

Deze repository bevat de live demo uit de presentatie: dezelfde bug — een classic use-after-free — in zowel C als Rust. Het doel is om met eigen ogen het verschil te zien tussen *veilig by discipline* en *veilig by design*.

## Structuur

```
memory-safety-demo/
├── README.md
├── Dockerfile          ← optionele container-route
├── run-all.sh          ← alle stappen achter elkaar
├── c/
│   ├── demo.c          ← C-versie met use-after-free
│   └── Makefile
└── rust/
    ├── broken/         ← Rust-versie met dezelfde bug (compileert NIET)
    │   ├── Cargo.toml
    │   └── src/main.rs
    └── fixed/          ← Rust-versie correct
        ├── Cargo.toml
        └── src/main.rs
```

## Installatie — twee routes

Kies de route die past bij je werkomgeving en voorkeur.

### Route A — Lokale installatie *(aanbevolen als je Rust/GCC al hebt)*

**Rust toolchain:**
Installeer via [rustup.rs](https://rustup.rs). Eén commando, ongeveer vijf minuten, geen administrator-rechten vereist — de toolchain wordt in je gebruikersmap geplaatst.

**C-compiler:**
- macOS: `xcode-select --install`
- Ubuntu / Debian: `sudo apt install build-essential`
- Windows: WSL2 met Ubuntu, of MinGW

**Verificatie:**
```bash
rustc --version
gcc --version
```

### Route B — Docker *(aanbevolen als je geen lokale install wil)*

Vereiste: een werkende Docker-installatie (Docker Desktop op macOS/Windows, of `docker` op Linux).

**Build de image:**
```bash
docker build -t memory-safety-demo .
```

Eenmalig nodig, duurt ongeveer twee minuten. De image bevat Rust, GCC en AddressSanitizer.

**Run de container interactief:**
```bash
docker run -it --rm memory-safety-demo
```

Vanuit de container kun je de stappen hieronder uitvoeren alsof het je lokale shell is.

**Of run alle stappen in één keer:**
```bash
docker run --rm memory-safety-demo ./run-all.sh
```

## Walkthrough — vier stappen

### Stap 1 — C zonder sanitizer

```bash
cd c
make run
```

Verwachte output:
```
Eerste read:  user_42
Tweede read:  iets_anders_999
```

De buffer voor `name` is vrijgegeven, en de daaropvolgende `malloc` voor `other` krijgt dezelfde geheugen-slot terug. De tweede `printf` leest dus de inhoud van een vreemde allocatie. In productie kan dit leiden tot informatie-lekken, crashes, of security-exploits, afhankelijk van wat er in die geheugen-slot terechtkomt.

### Stap 2 — C met AddressSanitizer

```bash
make asan
```

Verwachte output:
```
Eerste read:  user_42
==12345==ERROR: AddressSanitizer: heap-use-after-free on address ...
READ of size 2 at ... thread T0
    #0 in printf_common
    #1 in main /demo.c:20
freed by thread T0 here:
    #0 in __interceptor_free
    #1 in main /demo.c:15
previously allocated by thread T0 here:
    #0 in __interceptor_malloc
    #1 in get_username /demo.c:8
```

ASAN vangt de bug runtime en levert een gedetailleerd rapport: waar het geheugen werd geallocateerd, waar het werd vrijgegeven, en waar de illegale toegang plaatsvond. Krachtig — maar uitsluitend voor code-paden die je daadwerkelijk uitvoert, en niet bruikbaar in productie (twee tot drie keer trager).

### Stap 3 — Rust met dezelfde bug *(compileert niet)*

```bash
cd ../rust/broken
cargo build
```

Verwachte output:
```
error[E0515]: cannot return reference to local variable `buffer`
 --> src/main.rs:3:5
  |
3 |     &buffer
  |     ^^^^^^^ returns a reference to data owned by the current function

error: could not compile `broken` due to 1 previous error
```

De compiler weigert te bouwen. Niet runtime, niet onder een sanitizer — bij `cargo build` op de laptop van de developer, voordat de code zelfs gecommit is.

### Stap 4 — Rust correct

```bash
cd ../fixed
cargo run
```

Verwachte output:
```
Naam: user_42
```

De fix retourneert een owned `String` in plaats van een referentie naar lokaal geheugen. Identieke functionaliteit, geen onveilige patronen, memory safety gegarandeerd door de compiler.

## Wat dit aantoont

| Aspect | C | Rust |
|--------|---|------|
| Bug detectie | Runtime, met optionele tooling | Compile-time, altijd |
| Tooling overhead | ASAN: 2-3× trager | Geen |
| Dekking | Alleen uitgevoerde paden | Alle paden |
| Productie-risico | Bug shipt regelmatig ongezien | Bug bereikt productie niet |

Het verschil is niet dat Rust geen bugs heeft. Het verschil is *wanneer* een bug ontdekt wordt — en daarmee hoeveel hij kost om te herstellen.

## Verder lezen

- [Google Security Blog — *Eliminating Memory Safety Vulnerabilities at the Source*](https://security.googleblog.com/2024/09/eliminating-memory-safety-vulnerabilities-Android.html)
- [White House ONCD — *Back to the Building Blocks*](https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/memory-safe-back-to-the-building-blocks/)
- [CISA — *The Case for Memory Safe Roadmaps*](https://www.cisa.gov/resources-tools/resources/case-memory-safe-roadmaps)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)

## Licentie

MIT — vrij te gebruiken voor eigen presentaties, workshops en educatie.

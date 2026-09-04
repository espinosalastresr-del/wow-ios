# wow-ios

Cliente **World of Warcraft 3.3.5a (WotLK)** orientado a iOS, escrito en **Rust** con host **Swift/Metal**.

> Uso personal / servidores privados 3.3.5a. Los datos del cliente (MPQ) son tuyos: no se incluyen en el repo.

## Estado

- Red: auth SRP6, realm, personajes, mundo, movimiento, casts, chat
- UI táctil: login, realms, chars, settings (ruta de datos), HUD, action bar
- Renderer: wgpu → Metal, terreno debug, entidades, overlay UI
- Assets: cadena MPQ + probe de `Data/` cuando configuras la ruta
- iOS: bridge C ABI, Documents/`WoW335`, pensado para **LiveContainer**

## Datos del juego (iPhone)

1. Copia tu carpeta `Data/` (MPQs 3.3.5a) a  
   `Archivos → En mi iPhone → wow-ios → WoW335/Data/`
2. En la app: **Settings → Data path → Probe → Apply**

Ver `docs/DATOS_IOS_LIVECONTAINER.md`.

## Desarrollo (Linux)

```bash
cargo check -p wow-client
cargo run -p wow-client          # demo offline sin MPQs
export WOW_DATA_PATH=/ruta/a/Data
export WOW_USERNAME=... WOW_PASSWORD=... WOW_AUTO_LOGIN=1
cargo run -p wow-client
```

## IPA / LiveContainer

GitHub Actions (macOS) genera un artefacto IPA orientado a sideload/LiveContainer.
Sin certificado de Apple Developer el firmado es **ad-hoc**; LiveContainer suele aceptar ese tipo de empaquetado según tu setup.

Workflow: `.github/workflows/ios-ipa.yml` — ejecuta en push a `main` o manualmente (*Actions → iOS IPA → Run workflow*).

## Estructura

```
crates/     # Rust workspace
ios/        # Host Swift (MTKView, bridge)
docs/       # Guías
```

## Legal

Código: proyecto personal. Assets de Blizzard: no redistribuir; usa solo copias legítimas.

# Estado de subida del codigo

El repositorio se esta poblando por commits via API.

**Fuente completa local del proyecto:** workspace de desarrollo `wow-ios` (~19k LOC).

Si faltan archivos `.rs` grandes (`wow-net/src/world.rs`, formats, etc.) en GitHub,
se seguiran subiendo en commits posteriores. El workflow de IPA ya esta en:

`.github/workflows/ios-ipa.yml`

## Como lanzar el IPA

1. GitHub → **Actions** → **iOS IPA (LiveContainer)** → **Run workflow**
2. Descarga el artifact `wow-ios-ipa`
3. Instala con LiveContainer / sideload segun tu jailbreak o firmante

## Limitacion importante

Sin certificado Apple Developer el IPA es **ad-hoc / stub de empaquetado**.
El binario nativo completo requiere enlazar `libwow_client.a` + Swift host en Xcode
en un runner macOS (el workflow prepara esa ruta).

## Datos del juego

No van en el repo. Configura ruta en Settings de la app → Documents/WoW335/Data.

# Datos del cliente en iPhone (LiveContainer / Documents)

## Donde van los archivos

1. Crea en el iPhone (app **Archivos**):
   ```
   En mi iPhone / <NombreDeTuApp> / WoW335 / Data /
   ```
2. Copia ahi los `.MPQ` del cliente 3.3.5a.

## En la app

1. Options → **Settings** (o Login → Settings)
2. Data path → Probe → Apply

## Info.plist

UIFileSharingEnabled = YES
LSSupportsOpeningDocumentsInPlace = YES

## Linux debug

```
export WOW_DATA_PATH=/ruta/a/Data
export WOW_LOCALE=enUS
```

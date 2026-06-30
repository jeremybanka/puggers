# Importer and Formatter Compatibility

The HTML importer and Pug formatter share formatting options when the option
means the same thing for generated Pug and existing Pug source. HTML extraction
policy stays importer-owned so structural simplification does not leak into the
formatter.

| Option | Status | Notes |
| --- | --- | --- |
| `indentWidth` / `--indent-width` | Shared | Controls space indentation width when tabs are disabled. |
| `lineWidth` / `--line-width` | Shared | Controls prose wrapping and width-aware layout decisions that apply to both surfaces. |
| `useTabs` / `--use-tabs` | Shared | Controls indentation units. Tabs count as one display column for width checks. |
| `quoteStyle` / `--quote-style` | Shared | Controls rendered quoted attribute values. Defaults to double quotes. |
| `--allow-attr` | Importer-only | Determines which HTML attributes survive extraction. |
| id/class shorthand preference | Importer-only | Chooses whether imported `id` and `class` values become Pug shorthand. |
| `--trim-outer-document` | Importer-only | Selects body children instead of the parsed outer document shell. |
| `--collapse-single-nested` | Importer-only | Collapses anonymous wrapper structure during import. |
| `--preserve-text-whitespace` | Importer-only | Changes how HTML text payload whitespace is normalized. |
| `--drop-comments` | Importer-only | Removes HTML comments during import. |
| Pug syntax normalization | Formatter-only / pending | Parser recovery, statement-head normalization, and Pug-only syntax cleanup do not currently have HTML-import equivalents. |

Shared behavior lives in `puggers-core::PugFormatOptions`. Surface-specific
configuration should translate into that type instead of reimplementing
indentation, quote rendering, display width, or prose wrapping independently.

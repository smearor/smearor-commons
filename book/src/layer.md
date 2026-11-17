# smearor-commons-layer

Layer-shell vocabulary: `SmearorLayer` is the shared name for the four
layer-shell layers (`Background`, `Bottom`, `Top`, `Overlay`). It parses
from CLI arguments (`FromStr`), deserializes from configuration files
(`serde`) and converts into `gtk4_layer_shell::Layer` behind the
`gtk4-layer-shell` feature. `ParseLayerError` reports unknown names.

`LayerConfig` is the config quartet apps repeat across arguments, files
and state: `layer`, `namespace`, `monitor` and `exclusive_zone`
(`None` = auto-exclusive-zone, `0` = overlay mode).
`merge()` applies overrides; `effective_exclusive_zone()` resolves the
value to apply.

## Orientation to edge

With `gtk4-layer-shell` enabled, `EdgeOf` maps the snapped
four-orientation vocabularies — geometry's `QuarterTurn`/`Rotation` and,
behind the `layout` feature, layout's `TableSide` — onto the
layer-shell `Edge` a surface anchors to:

```mermaid
flowchart LR
    cli["CLI args (FromStr)"] --> sl[SmearorLayer]
    file["config file (serde)"] --> lc[LayerConfig]
    sl -->|"gtk4-layer-shell"| layer["gtk4_layer_shell::Layer"]
    orient["QuarterTurn / Rotation / TableSide"] -->|EdgeOf| edge["gtk4_layer_shell::Edge"]
    lc --> zone["exclusive_zone: Pixels&lt;i32&gt;"]
```

See the crate's rustdoc for the full API.

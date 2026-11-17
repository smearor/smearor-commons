# smearor-commons-buffer

Pixel and graphics buffer types: ownership, format, extent and stride.

`PixelBuffer<F>` owns a packed pixel buffer; `BufferView` and
`BufferViewMut` borrow strided foreign memory (Wayland SHM, framework
surfaces). `BufferMetadata` carries the format-agnostic
width/height/stride triple. The pixel format is a type-level marker
(`Rgba`, `Bgra`, `Rgb`, `Bgr`, `Rgbx`, `Bgrx`, `Gray`) implementing
`PixelFormat`, so format mismatches are compile errors rather than
runtime checks.

```mermaid
classDiagram
    class PixelBuffer~F~ {
        <<owned>>
        +new(size)
        +from_raw(size, pixels)
        +view()
        +view_mut()
        +into_bgra()
        +into_rgba()
    }
    class BufferView~'a, F~ {
        <<borrowed>>
        +from_raw(data, metadata)
        +from_packed(data, size)
    }
    class BufferViewMut~'a, F~ {
        <<borrowed mut>>
        +from_raw(data, metadata)
    }
    class BufferMetadata {
        +width, height, stride
    }
    class PixelFormat {
        <<trait marker>>
        +Rgba, Bgra, Rgb, Bgr, ...
    }
    PixelBuffer --> BufferView : view()
    PixelBuffer --> BufferViewMut : view_mut()
    BufferView --> BufferMetadata
    BufferViewMut --> BufferMetadata
    PixelBuffer ..> PixelFormat : F
```

Drawing operations live downstream (`pixel-drawing`); framework bridges
(`image`, `cairo`, `gtk4`, `smithay`) are added behind dependency-named
features. See the crate's rustdoc for the full API.

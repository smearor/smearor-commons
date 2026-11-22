# Sampling

`BufferView`'s sampling methods read texels the way effect-style
consumers (the `fx` crate) sample textures:

| Method | Behavior |
|---|---|
| `view.sample_nearest(uv)` | the texel whose center is closest to `uv` (pixelate-style) |
| `view.sample_bilinear(uv)` | weighted mean of the four surrounding texels |
| `view.sample_radial_blur(uv, radius, directions, taps)` | mean of `taps` samples along `directions` rays (glow/blur shapes) |

The contract mirrors GLSL texture reads:

- **uv space**: origin top-left, y down, `0..1` spans the texture
  (GNOME's `iTexCoord` convention)
- **clamp-to-edge**: out-of-range uv reads the nearest edge texel -
  no wrapping, no border color
- **alpha**: `PixelFormat::read_pixel` decodes to straight alpha, so
  premultiplied formats are unpremultiplied on read and every sample
  returns straight-alpha `Color<f64>` with normalized `0..=1` channels

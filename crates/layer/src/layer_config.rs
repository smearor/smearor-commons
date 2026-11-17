use crate::SmearorLayer;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Pixels;

/// Layer-shell surface configuration shared by Smearor apps.
///
/// The `layer`/`namespace`/`monitor`/`exclusive_zone` quartet the
/// ecosystem repeats across CLI arguments, configuration files and
/// runtime state. Consumers needing extra fields wrap it with
/// `#[serde(flatten)]` on a `LayerConfig` member.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LayerConfig {
    /// The layer-shell layer; `None` leaves the surface on the
    /// compositor's default layer.
    #[serde(default)]
    pub layer: Option<SmearorLayer>,

    /// Layer-shell namespace - compositors match window rules on it.
    #[serde(default)]
    pub namespace: Option<String>,

    /// Monitor index the surface appears on; `None` uses the
    /// primary monitor.
    #[serde(default)]
    pub monitor: Option<u32>,

    /// Exclusive zone: `None` enables the auto-exclusive-zone,
    /// [`Pixels::new(0)`] disables it (overlay mode), other values
    /// reserve that many pixels for the surface.
    ///
    /// `i32`-backed because [`gtk4_layer_shell`] consumes `i32`;
    /// serde keeps the bare-number wire format.
    #[serde(default)]
    pub exclusive_zone: Option<Pixels<i32>>,
}

impl LayerConfig {
    /// The exclusive zone to apply: meaningful only when `layer`
    /// selects a layer-shell layer - `None` otherwise.
    pub fn effective_exclusive_zone(&self) -> Option<Pixels<i32>> {
        match &self.layer {
            Some(_) => self.exclusive_zone,
            None => None,
        }
    }

    /// Returns `self` with every field present in `overrides`
    /// replacing the local value - the CLI-over-config merge used
    /// by Smearor apps.
    ///
    /// `Option` fields cannot distinguish "unset" from "explicitly
    /// unset": an `overrides` field that is `None` never clears a
    /// configured value.
    pub fn merge(mut self, overrides: &Self) -> Self {
        if overrides.layer.is_some() {
            self.layer = overrides.layer;
        }
        if overrides.namespace.is_some() {
            self.namespace = overrides.namespace.clone();
        }
        if overrides.monitor.is_some() {
            self.monitor = overrides.monitor;
        }
        if overrides.exclusive_zone.is_some() {
            self.exclusive_zone = overrides.exclusive_zone;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::LayerConfig;
    use crate::SmearorLayer;
    use smearor_commons_geometry::Pixels;

    /// Overrides replace only the fields they set.
    #[test]
    fn merge_keeps_unset_fields() {
        let config = LayerConfig {
            layer: Some(SmearorLayer::Top),
            namespace: Some("app".to_string()),
            monitor: Some(1),
            exclusive_zone: Some(Pixels::new(64)),
        };
        let overrides = LayerConfig {
            layer: Some(SmearorLayer::Overlay),
            exclusive_zone: Some(Pixels::new(0)),
            ..Default::default()
        };
        let merged = config.merge(&overrides);
        assert_eq!(merged.layer, Some(SmearorLayer::Overlay));
        assert_eq!(merged.namespace.as_deref(), Some("app"));
        assert_eq!(merged.monitor, Some(1));
        assert_eq!(merged.exclusive_zone, Some(Pixels::new(0)));
    }

    /// The exclusive zone applies only when a layer is selected.
    #[test]
    fn exclusive_zone_requires_a_layer() {
        let config = LayerConfig {
            exclusive_zone: Some(Pixels::new(64)),
            ..Default::default()
        };
        assert_eq!(config.effective_exclusive_zone(), None);

        let config = LayerConfig {
            layer: Some(SmearorLayer::Top),
            exclusive_zone: Some(Pixels::new(64)),
            ..Default::default()
        };
        assert_eq!(config.effective_exclusive_zone(), Some(Pixels::new(64)));
    }

    /// Missing fields deserialize to `None` via the serde defaults.
    #[test]
    fn empty_toml_deserializes_to_defaults() {
        let config: LayerConfig = serde_json::from_str("{}").ok().unwrap_or_default();
        assert_eq!(config, LayerConfig::default());
    }
}

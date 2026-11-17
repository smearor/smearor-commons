use crate::SmearorLayer;
use gtk4_layer_shell::Layer;

impl From<SmearorLayer> for Layer {
    fn from(layer: SmearorLayer) -> Self {
        match layer {
            SmearorLayer::Background => Layer::Background,
            SmearorLayer::Bottom => Layer::Bottom,
            SmearorLayer::Top => Layer::Top,
            SmearorLayer::Overlay => Layer::Overlay,
        }
    }
}

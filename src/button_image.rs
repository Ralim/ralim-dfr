use cairo::ImageSurface;
use librsvg_rebind::Handle;
pub enum ButtonImage {
    Svg(Handle),
    Bitmap(ImageSurface),
}

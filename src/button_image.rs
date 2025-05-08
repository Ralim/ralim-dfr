use cairo::ImageSurface;
use librsvg_rebind::Handle;
pub enum ButtonImage {
    Text(String),
    Svg(Handle),
    Bitmap(ImageSurface),
    Time(String, String),
    Processor(),
}

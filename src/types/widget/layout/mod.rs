use super::WidgetSize;

#[derive(Clone, Copy)]
pub struct ScreenSize {
    pub width: u16,
    pub height: u16,
}

impl std::fmt::Debug for ScreenSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} x {}", self.width, self.height))
    }
}

#[derive(Clone, Copy)]
pub struct GroupOrigin {
    /// (In a css layout sense) the (hidpi) pixels right from the left edge of the screen
    pub left: u16,
    /// (In a css layout sense) the (hidpi) pixels down from the top edge of the screen
    pub top: u16,
}

impl std::fmt::Debug for GroupOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[→  {}, ↓ {}]", self.left, self.top))
    }
}

pub fn rows_used(w: &WidgetSize) -> u8 {
    match w {
        WidgetSize::Small => 1,
        WidgetSize::Medium => 1,
        WidgetSize::Large => 2,
        WidgetSize::ExtraLarge => 2,
        WidgetSize::Invalid(_) => 0,
    }
}

pub fn columns_used(w: &WidgetSize) -> u8 {
    match w {
        WidgetSize::Small => 1,
        WidgetSize::Medium => 2,
        WidgetSize::Large => 4,
        WidgetSize::ExtraLarge => 8,
        WidgetSize::Invalid(_) => 0,
    }
}

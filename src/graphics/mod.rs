mod renderer;
mod macroquad_renderer;
mod mock_renderer;

/// Trait to render objects to the user.
pub use renderer::Renderer;

/// Uses the `Macroquad` library to render objects to the screen.
pub use macroquad_renderer::MacroquadRenderer;

/// Test renderer for unit tests.
pub use mock_renderer::MockRenderer;
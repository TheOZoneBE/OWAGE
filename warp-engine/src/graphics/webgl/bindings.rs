//! Bindings for all objects and method associated with WebGL2
//!
//! Documentation taken straight from https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext
//! and https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext
//!
//! Would like to place this in a separate crate but at the moment wasm bindgen
//! does not like this idea.

use glenum_bind::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method, js_name = getElementById)]
    fn get_element_by_id(this: &HTMLDocument, id: &str) -> HTMLCanvasElement;

    pub type HTMLCanvasElement;
    #[wasm_bindgen(method)]
    fn get_context(this: &HTMLCanvasElement, context: &str) -> WebGL2RenderingContext;
}

impl WebGL2RenderingContext {
    pub fn new(id: &str) -> WebGL2RenderingContext {
        document.get_element_by_id(id).get_context("webgl2")
    }

    /// Maps to `get_buffer_parameter` when return type is `i32`
    pub fn get_buffer_size(&self, target: BufferKind) -> i32 {
        self.get_buffer_parameter_size(target, BufferParameter::Size)
    }

    /// Maps to `get_buffer_parameter` when return type is `DataHint`
    pub fn get_buffer_usage(&self, target: BufferKind) -> DataHint {
        self.get_buffer_parameter_usage(target, BufferParameter::Usage)
    }
}

/// WebGL2RenderingContext
#[wasm_bindgen]
extern "C" {
    /// The WebGL2RenderingContext interface provides the OpenGL ES 3.0 rendering context
    /// for the drawing surface of an HTML <canvas> element.
    pub type WebGL2RenderingContext;

    /// The `WebGLRenderingContext.canvas` property is a read-only reference to the `HTMLCanvasElement`
    /// or `OffscreenCanvas` object that is associated with the context. It might be null if it is not
    /// associated with a <canvas> element or an `OffscreenCanvas` object.
    #[wasm_bindgen(method, getter)]
    pub fn canvas(this: &WebGL2RenderingContext) -> HTMLCanvasElement;

    /// The read-only `WebGLRenderingContext.drawingBufferWidth` property represents the actual width
    /// of the current drawing buffer. It should match the width attribute of the `<canvas>` element
    /// associated with this context, but might differ if the implementation is not able to provide
    /// the requested width.
    #[wasm_bindgen(method, getter = drawingBufferWidth)]
    pub fn drawing_buffer_width(this: &WebGL2RenderingContext) -> u32;

    /// The read-only `WebGLRenderingContext.drawingBufferHeight` property represents the actual height
    /// of the current drawing buffer. It should match the height attribute of the `<canvas>` element
    /// associated with this context, but might differ if the implementation is not able to provide
    /// the requested height.
    #[wasm_bindgen(method, getter = drawingBufferHeight)]
    pub fn drawing_buffer_height(this: &WebGL2RenderingContext) -> u32;

    /// The `WebGLRenderingContext.getContextAttributes()` method returns a `WebGLContextAttributes`
    /// object that contains the actual context parameters. Might return `null`, if the context is lost.
    #[wasm_bindgen(method, js_name = getContextAttributes)]
    pub fn get_context_attributes(this: &WebGL2RenderingContext) -> WebGLContextAttributes;

    /// The WebGLRenderingContext.isContextLost() method returns a Boolean indicating whether or not
    /// the WebGL context has been lost.
    #[wasm_bindgen(method, js_name = isContextLost)]
    pub fn is_context_lost(this: &WebGL2RenderingContext) -> bool;

    /// The `WebGLRenderingContext.scissor()` method of the WebGL API sets a scissor box, which limits
    /// the drawing to a specified rectangle.
    #[wasm_bindgen(method)]
    pub fn scissor(this: &WebGL2RenderingContext, x: i32, y: i32, width: u32, height: u32);

    /// The `WebGLRenderingContext.viewport()` method of the WebGL API sets the viewport, which
    /// specifies the affine transformation of x and y from normalized device coordinates to window
    /// coordinates.
    #[wasm_bindgen(method)]
    pub fn viewport(this: &WebGL2RenderingContext, x: i32, y: i32, width: u32, height: u32);

    /// The `WebGLRenderingContext.activeTexture()` method of the WebGL API specifies which texture
    /// unit to make active.
    #[wasm_bindgen(method, js_name = activeTexture)]
    pub fn active_texture(this: &WebGL2RenderingContext, texture: TextureUnit);

    /// The `WebGLRenderingContext.blendColor()` method of the WebGL API is used to set the source and
    /// destination blending factors.
    #[wasm_bindgen(method, js_name = blendColor)]
    pub fn blend_color(this: &WebGL2RenderingContext, red: f32, green: f32, blue: f32, alpha: f32);

    /// The `WebGLRenderingContext.blendEquation()` method of the WebGL API is used to set both the RGB
    /// blend equation and alpha blend equation to a single equation.
    ///
    /// The blend equation determines how a new pixel is combined with a pixel already in the
    /// WebGLFramebuffer.
    #[wasm_bindgen(method, js_name = blendEquation)]
    pub fn blend_equation(this: &WebGL2RenderingContext, mode: BlendEquation);

    /// The `WebGLRenderingContext.blendEquationSeparate()` method of the WebGL API is used to set
    /// the RGB blend equation and alpha blend equation separately.
    ///
    /// The blend equation determines how a new pixel is combined with a pixel already in the
    /// WebGLFramebuffer.
    #[wasm_bindgen(method, js_name = blendEquationSeparate)]
    pub fn blend_equation_separate(
        this: &WebGL2RenderingContext,
        mode_rgb: BlendEquation,
        mode_alpha: BlendEquation,
    );

    /// The `WebGLRenderingContext.blendFunc()` method of the WebGL API defines which function is used
    /// for blending pixel arithmetic.
    #[wasm_bindgen(method, js_name = blendFunc)]
    pub fn blend_func(this: &WebGL2RenderingContext, sfactor: BlendMode, dfactor: BlendMode);

    /// The `WebGLRenderingContext.blendFuncSeparate()` method of the WebGL API defines which function
    /// is used for blending pixel arithmetic for RGB and alpha components separately.
    #[wasm_bindgen(method, js_name = blendFuncSeparate)]
    pub fn blend_func_separate(
        this: &WebGL2RenderingContext,
        src_rgb: BlendMode,
        dst_rgb: BlendMode,
        src_alpha: BlendMode,
        dst_alpha: BlendMode,
    );

    /// The `WebGLRenderingContext.clearColor()` method of the WebGL API specifies the color values
    /// used when clearing color buffers.
    ///
    /// This specifies what color values to use when calling the clear() method. The values are clamped
    /// between 0 and 1.
    #[wasm_bindgen(method, js_name = clearColor)]
    pub fn clear_color(this: &WebGL2RenderingContext, red: f32, green: f32, blue: f32, alpha: f32);

    /// The `WebGLRenderingContext.clearDepth()` method of the WebGL API specifies the clear value for
    /// the depth buffer.
    ///
    /// This specifies what depth value to use when calling the clear() method. The value is clamped
    /// between 0 and 1.
    #[wasm_bindgen(method, js_name = clearDepth)]
    pub fn clear_depth(this: &WebGL2RenderingContext, depth: f32);

    /// The `WebGLRenderingContext.clearStencil()` method of the WebGL API specifies the clear value
    /// for the stencil buffer.
    ///
    /// This specifies what stencil value to use when calling the clear() method.
    #[wasm_bindgen(method, js_name = clearStencil)]
    pub fn clear_stencil(this: &WebGL2RenderingContext, s: i32);

    /// The `WebGLRenderingContext.colorMask()`  method of the WebGL API sets which color components
    /// to enable or to disable when drawing or rendering to a WebGLFramebuffer.
    #[wasm_bindgen(method, js_name = colorMask)]
    pub fn color_mask(
        this: &WebGL2RenderingContext,
        red: bool,
        green: bool,
        blue: bool,
        alpha: bool,
    );

    /// The `WebGLRenderingContext.cullFace()` method of the WebGL API specifies whether or not
    /// front- and/or back-facing polygons can be culled.
    #[wasm_bindgen(method, js_name = cullFace)]
    pub fn cull_face(this: &WebGL2RenderingContext, mode: Culling);

    /// The `WebGLRenderingContext.depthFunc()` method of the WebGL API specifies a function that
    /// compares incoming pixel depth to the current depth buffer value.
    #[wasm_bindgen(method, js_name = depthFunc)]
    pub fn depth_func(this: &WebGL2RenderingContext, func: DepthTest);

    /// The `WebGLRenderingContext.depthMask()` method of the WebGL API sets whether writing
    /// into the depth buffer is enabled or disabled.
    #[wasm_bindgen(method, js_name = depthMask)]
    pub fn depth_mask(this: &WebGL2RenderingContext, flag: bool);

    /// The `WebGLRenderingContext.depthRange()` method of the WebGL API specifies the depth
    /// range mapping from normalized device coordinates to window or viewport coordinates.
    #[wasm_bindgen(method, js_name = depthRange)]
    pub fn depth_range(this: &WebGL2RenderingContext, z_near: f32, z_far: f32);

    /// The `WebGLRenderingContext.disable()` method of the WebGL API disables specific WebGL
    /// capabilities for this context.
    #[wasm_bindgen(method)]
    pub fn disable(this: &WebGL2RenderingContext, cap: Flag);

    /// The `WebGLRenderingContext.enable()` method of the WebGL API enables specific WebGL
    /// capabilities for this context.
    #[wasm_bindgen(method)]
    pub fn enable(this: &WebGL2RenderingContext, cap: Flag);

    /// The `WebGLRenderingContext.frontFace()` method of the WebGL API specifies whether polygons
    /// are front- or back-facing by setting a winding orientation.
    #[wasm_bindgen(method, js_name = frontFace)]
    pub fn front_face(this: &WebGL2RenderingContext, mode: FrontFaceDirection);

    /// The `WebGLRenderingContext.getParameter()` method of the WebGL API returns a value for the
    /// passed parameter name.
    //#[wasm_bindgen(method, js_name = getParameter)]
    // TODO save for later, this is a very convoluted method
    //pub fn get_parameter(this: &WebGL2RenderingContext, pname: )

    /// The `WebGLRenderingContext.getError()` method of the WebGL API returns error information.
    #[wasm_bindgen(method, js_name = getError)]
    pub fn get_error(this: &WebGL2RenderingContext) -> Error;

    /// The `WebGLRenderingContext.hint()` method of the WebGL API specifies hints for certain behaviors.
    /// The interpretation of these hints depend on the implementation.
    #[wasm_bindgen(method)]
    pub fn hint(this: &WebGL2RenderingContext, target: HintTarget, mode: HintMode);

    /// The `WebGLRenderingContext.isEnabled()` method of the WebGL API tests whether a specific WebGL
    /// capability is enabled or not for this context.
    ///
    /// By default, all capabilities except `gl.DITHER` are disabled.
    #[wasm_bindgen(method, js_name = isEnabled)]
    pub fn is_enabled(this: &WebGL2RenderingContext, cap: Flag);

    /// The `WebGLRenderingContext.lineWidth()` method of the WebGL API sets the line width of rasterized lines.
    #[wasm_bindgen(method, js_name = lineWidth)]
    pub fn line_width(this: &WebGL2RenderingContext, width: f32);

    /// The `WebGLRenderingContext.pixelStorei()` method of the WebGL API specifies the pixel storage modes.
    #[wasm_bindgen(method, js_name = pixelStorei)]
    pub fn pixel_storei(this: &WebGL2RenderingContext, pname: PixelStorageMode, param: i32);

    /// The `WebGLRenderingContext.polygonOffset()` method of the WebGL API specifies the scale factors and
    /// units to calculate depth values.
    ///
    /// The offset is added before the depth test is performed and before the value is written into the depth buffer.
    #[wasm_bindgen(method, js_name = polygonOffset)]
    pub fn polygon_offset(this: &WebGL2RenderingContext, factor: f32, units: f32);

    /// The `WebGLRenderingContext.sampleCoverage()` method of the WebGL API specifies multi-sample coverage parameters
    /// for anti-aliasing effects.
    #[wasm_bindgen(method, js_name = sampleCoverage)]
    pub fn sample_coverage(this: &WebGL2RenderingContext, value: f32, invert: bool);

    /// The `WebGLRenderingContext.stencilFunc()` method of the WebGL API sets the front and back function and
    /// reference value for stencil testing.
    ///
    /// Stencilling enables and disables drawing on a per-pixel basis. It is typically used in multipass rendering
    /// to achieve special effects.
    #[wasm_bindgen(method, js_name = stencilFunc)]
    pub fn stencil_func(
        this: &WebGL2RenderingContext,
        func: StencilTest,
        reference: i32,
        mask: u32,
    );

    /// The `WebGLRenderingContext.stencilFuncSeparate()` method of the WebGL API sets the front and/or back
    /// function and reference value for stencil testing.
    ///
    /// Stencilling enables and disables drawing on a per-pixel basis. It is typically used in multipass rendering to achieve special effects.
    #[wasm_bindgen(method, js_name = stencilFuncSeparate)]
    pub fn stencil_func_separate(
        this: &WebGL2RenderingContext,
        face: Culling,
        func: StencilTest,
        reference: i32,
        mask: u32,
    );

    /// The `WebGLRenderingContext.stencilMask()` method of the WebGL API controls enabling and disabling
    /// of both the front and back writing of individual bits in the stencil planes.
    ///
    /// The `WebGLRenderingContext.stencilMaskSeparate()` method can set front and back stencil writemasks
    /// to different values.
    #[wasm_bindgen(method, js_name = stencilMask)]
    pub fn stencil_mask(this: &WebGL2RenderingContext, mask: u32);

    /// The `WebGLRenderingContext.stencilMaskSeparate()` method of the WebGL API controls enabling and
    /// disabling of front and/or back writing of individual bits in the stencil planes.
    ///
    /// The `WebGLRenderingContext.stencilMask()` method can set both, the front and back stencil writemasks
    /// to one value at the same time.
    #[wasm_bindgen(method, js_name = stencilMaskSeparate)]
    pub fn stencil_mask_separate(this: &WebGL2RenderingContext, face: Culling, mask: u32);

    /// The `WebGLRenderingContext.stencilOp()` method of the WebGL API sets both the front and back-facing
    /// stencil test actions.
    #[wasm_bindgen(method, js_name = stencilOp)]
    pub fn stencil_op(
        this: &WebGL2RenderingContext,
        fail: StencilAction,
        zfail: StencilAction,
        zpass: StencilAction,
    );

    /// The `WebGLRenderingContext.stencilOpSeparate()` method of the WebGL API sets the front and/or
    /// back-facing stencil test actions.
    #[wasm_bindgen(method, js_name = stencilOpSeparate)]
    pub fn stencil_op_separate(
        this: &WebGL2RenderingContext,
        face: Culling,
        fail: StencilAction,
        zfail: StencilAction,
        zpass: StencilAction,
    );

    /// The `WebGLRenderingContext.bindBuffer()` method of the WebGL API binds a given WebGLBuffer to a target.
    #[wasm_bindgen(method, js_name = bindBuffer)]
    pub fn bind_buffer(this: &WebGL2RenderingContext, target: BufferKind, buffer: WebGLBuffer);

    /// TODO maybe add a method for every buffer type

    /// The `WebGLRenderingContext.bufferData()` method of the WebGL API initializes and creates the
    /// buffer object's data store.
    #[wasm_bindgen(method, js_name = bufferData)]
    pub fn buffer_data(
        this: &WebGL2RenderingContext,
        target: BufferKind,
        srcData: Vec<u8>,
        usage: DataHint,
        srcOffset: u32,
        length: u32,
    );

    /// The `WebGLRenderingContext.bufferSubData()` method of the WebGL API updates a subset of a
    /// buffer object's data store.
    #[wasm_bindgen(method, js_name = bufferSubData)]
    pub fn buffer_sub_data(
        this: &WebGL2RenderingContext,
        target: BufferKind,
        dst_byte_offset: u32,
        srcData: Vec<u8>,
        srcOffset: u32,
        length: u32,
    );

    /// The `WebGLRenderingContext.createBuffer()` method of the WebGL API creates and initializes a
    /// WebGLBuffer storing data such as vertices or colors.
    #[wasm_bindgen(method, js_name = createBuffer)]
    pub fn create_buffer(this: &WebGL2RenderingContext) -> WebGLBuffer;

    /// The `WebGLRenderingContext.deleteBuffer()` method of the WebGL API deletes a given WebGLBuffer.
    /// This method has no effect if the buffer has already been deleted.
    #[wasm_bindgen(method, js_name = deleteBuffer)]
    pub fn delete_buffer(this: &WebGL2RenderingContext, buffer: WebGLBuffer);

    /// The `WebGLRenderingContext.getBufferParameter()` method of the WebGL API returns information about the buffer.
    #[wasm_bindgen(method, js_name = getBufferParameter)]
    fn get_buffer_parameter_size(
        this: &WebGL2RenderingContext,
        target: BufferKind,
        pname: BufferParameter,
    ) -> i32;
    #[wasm_bindgen(method, js_name = getBufferParameter)]
    fn get_buffer_parameter_usage(
        this: &WebGL2RenderingContext,
        target: BufferKind,
        pname: BufferParameter,
    ) -> DataHint;

    /// The `WebGLRenderingContext.isBuffer()` method of the WebGL API returns true if the passed
    /// WebGLBuffer is valid and false otherwise.
    #[wasm_bindgen(method, js_name = isBuffer)]
    pub fn is_buffer(this: &WebGL2RenderingContext, buffer: WebGLBuffer) -> bool;

    /// The WebGLRenderingContext.bindFramebuffer() method of the WebGL API binds a given
    /// WebGLFramebuffer to a target.
    #[wasm_bindgen(method, js_name = bindFramebuffer)]
    pub fn bind_framebuffer(
        this: &WebGL2RenderingContext,
        target: FramebufferKind,
        framebuffer: WebGLFramebuffer,
    );

    /// The `WebGLRenderingContext.checkFramebufferStatus()` method of the WebGL API returns the completeness
    /// status of the WebGLFramebuffer object.
    #[wasm_bindgen(method, js_name = checkFramebufferStatus)]
    pub fn check_framebuffer_status(this: &WebGL2RenderingContext, target: FramebufferKind);

    /// The `WebGLRenderingContext.createFramebuffer()` method of the WebGL API creates and initializes a
    /// WebGLFramebuffer object.
    #[wasm_bindgen(method, js_name = createFramebuffer)]
    pub fn create_framebuffer(this: &WebGL2RenderingContext) -> WebGLFramebuffer;

    /// The `WebGLRenderingContext.deleteFramebuffer()` method of the WebGL API deletes a given WebGLFramebuffer object.
    /// This method has no effect if the frame buffer has already been deleted.
    #[wasm_bindgen(method, js_name = deleteFramebuffer)]
    pub fn delete_framebuffer(this: &WebGL2RenderingContext, framebuffer: WebGLFramebuffer);

    /// The `WebGLRenderingContext.framebufferRenderbuffer()` method of the WebGL API attaches a WebGLRenderbuffer object
    /// to a WebGLFramebuffer object.
    #[wasm_bindgen(method, js_name = framebufferRenderbuffer)]
    pub fn framebuffer_renderbuffer(
        this: &WebGL2RenderingContext,
        target: FramebufferKind,
        attachment: Attachment,
        renderbuffertarget: RenderbufferKind,
        renderbuffer: WebGLRenderbuffer,
    );

    /// The `WebGLRenderingContext.framebufferTexture2D()` method of the WebGL API attaches a texture to a
    /// WebGLFramebuffer.
    #[wasm_bindgen(method, js_name = framebufferTexture2D)]
    pub fn framebuffer_texture_2d(
        this: &WebGL2RenderingContext,
        target: FramebufferKind,
        attachment: Attachment,
        textarget: TextureBindPoint,
        texture: WebGLTexture,
        level: i32,
    );

    // TODO getFramebufferAttachmentParameter()
    // later because of awful return structure

    /// The `WebGLRenderingContext.isFramebuffer()` method of the WebGL API returns true if the passed
    /// WebGLFramebuffer is valid and false otherwise.
    #[wasm_bindgen(method, js_name = isFramebuffer)]
    pub fn is_framebuffer(this: &WebGL2RenderingContext, framebuffer: WebGLFramebuffer) -> bool;

    /// The `WebGLRenderingContext.readPixels()` method of the WebGL API reads a block of pixels from a
    /// specified rectangle of the current color framebuffer into an ArrayBufferView object.
    // TODO rework because of variability of pixels datatype
    #[wasm_bindgen(method, js_name = readPixels)]
    pub fn read_pixels(
        this: &WebGL2RenderingContext,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        format: ColorFormat,
        pixel_type: PixelType,
        pixels: Vec<u8>,
        dstOffset: i32,
    );

    /// The `WebGLRenderingContext.bindRenderbuffer()` method of the WebGL API binds a given WebGLRenderbuffer
    /// to a target, which must be `gl.RENDERBUFFER`.
    #[wasm_bindgen(method, js_name = bindRenderbuffer)]
    pub fn bind_renderbuffer(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        renderbuffer: WebGLRenderbuffer,
    );

    /// The `WebGLRenderingContext.createRenderbuffer()` method of the WebGL API creates and initializes
    /// a WebGLRenderbuffer object.
    #[wasm_bindgen(method, js_name = createRenderbuffer)]
    pub fn create_renderbuffer(this: &WebGL2RenderingContext) -> WebGLRenderbuffer;

    /// The `WebGLRenderingContext.deleteRenderbuffer()` method of the WebGL API deletes a given WebGLRenderbuffer
    /// object. This method has no effect if the render buffer has already been deleted.
    #[wasm_bindgen(method, js_name = deleteRenderbuffer)]
    pub fn delete_renderbuffer(this: &WebGL2RenderingContext, renderbuffer: WebGLRenderbuffer);

    /// The `WebGLRenderingContext.getRenderbufferParameter()` method of the WebGL API returns information
    /// about the renderbuffer.
    #[wasm_bindgen(method, js_name = getRenderbufferParameter)]
    pub fn get_renderbuffer_parameter(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        pname: RenderbufferParameter,
    ) -> i32;
    // TODO add pub method get_renderbuffer_format to call this function
    #[wasm_bindgen(method, js_name = getRenderbufferParameter)]
    fn get_renderbuffer_parameter_format(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        pname: i32,
    ) -> RenderbufferFormat;

    /// The `WebGLRenderingContext.isRenderbuffer()` method of the WebGL API returns true if the passed
    /// WebGLRenderbuffer is valid and false otherwise.
    #[wasm_bindgen(method, js_name = isRenderbuffer)]
    pub fn is_renderbuffer(this: &WebGL2RenderingContext, renderbuffer: WebGLRenderbuffer) -> bool;

    /// The `WebGLRenderingContext.renderbufferStorage()` method of the WebGL API creates and initializes
    /// a renderbuffer object's data store.
    #[wasm_bindgen(method, js_name = renderbufferStorage)]
    pub fn renderbuffer_storage(
        this: &WebGL2RenderingContext,
        target: RenderbufferKind,
        internalFormat: RenderbufferFormat,
        width: u32,
        height: u32,
    );
}

/// WebGLContextAttributes
#[wasm_bindgen]
extern "C" {
    pub type WebGLContextAttributes;
}

/// WebGLBuffer
#[wasm_bindgen]
extern "C" {
    pub type WebGLBuffer;
}

/// WebGLFramebuffer
#[wasm_bindgen]
extern "C" {
    pub type WebGLFramebuffer;
}

/// WebGLRenderbuffer
#[wasm_bindgen]
extern "C" {
    pub type WebGLRenderbuffer;
}

/// WebGLTexture
#[wasm_bindgen]
extern "C" {
    pub type WebGLTexture;
}

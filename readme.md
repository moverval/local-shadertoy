# Local Shadertoy

A program to run and develop your shadertoy shaders locally!

The project is under active development. It is possible to run shadertoy shaders, but iChannels are currently not supported.

I am adding them in future.

## How to use

> You need to have `rust` installed.

Clone this repository and run

```
cargo install --path .
```

After the installation, the `shadertoy` command should be available (make sure, cargo binaries are available in `$PATH`).

Run a shader with the command

```
shadertoy -s shader.glsl
```

Like mentioned above, shaders are compatible with the `shadertoy.com` style. This project supports live reloading. Happy coding!

## I am new to shader programming, how can I start?

Get to know the [GLSL Website](https://www.khronos.org/opengl/wiki/Core_Language_(GLSL)). This is the ground truth to know about, but in my opinion not the most efficient way to learn shader programming.

I recommend to follow this [blog](https://clauswilke.com/art/post/shaders) and use [shadertoy.com](https://shadertoy.com) for writing your first shaders. You can also use this program to run your shadertoy shaders locally
and use your favourite development environment for it.

### Minimal Shader

```glsl
// Frag Color is the color to paint the pixel in, fragCoord is the coordinate of the pixel to paint
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    // Just paint every pixel green
    fragColor = vec4(vec3(0., 1., 0.), 1.);
}
```

#### Official Shadertoy.com Example

```glsl
void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    // Output to screen
    fragColor = vec4(col,1.0);
}
```

### Exposed Variables
- `iResolution`: A `vec2` for the window width and height
- `iTime`: A floating point number which holds the time the program runs in seconds
- `iMouse`: A `vec4` which holds the x and y coordinates of the mouse in the first two dimensions when clicked
- `iFrame`: Holds the count of the frame that will be displayed (integer)
- `iFrameRate`: Holds the frame rate of the window (float)

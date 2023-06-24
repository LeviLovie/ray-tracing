# Ray Tracing
My realisation of ray tracing in rust

## Running
I'm building project only for mac os, and to run my app, you need to fix it (idk why):
```
xattr -d com.apple.quarantine ray-tracing
```
If you using other OS, you need to build it by yourself.

## v0.0.1
![](https://github.com/LeviiLovie/ray-tracing/blob/main/versions/v0.0.1.png)
[v0.0.1](https://github.com/LeviiLovie/ray-tracing/releases/tag/v0.0.1) Renderer image.

Here I realised simple window creator, and filling VRAM with simple gradient.

## v0.0.2
[v0.0.2](https://github.com/LeviiLovie/ray-tracing/releases/tag/v0.0.2)

Here I made system to store objects for render.

## v0.0.3
![](https://github.com/LeviiLovie/ray-tracing/blob/main/versions/v0.0.3.png)
[v0.0.3](https://github.com/LeviiLovie/ray-tracing/releases/tag/v0.0.3) Renderer image.

Here I made processing pixels like usual shader. Example of random color for every pixel, shown on the screenshot:
```
use crate::window;
use rand::Rng;

pub fn ProcessPixel(SizeY: usize, SizeX: usize, PosY: usize, PosX: usize) -> window::Color {
    return window::Color::new(
        rand::thread_rng().gen_range(0..100) as f64 / 100.0,
        rand::thread_rng().gen_range(0..100) as f64 / 100.0,
        rand::thread_rng().gen_range(0..100) as f64 / 100.0,
    ) 
}
```

## v0.0.4
![](https://github.com/LeviiLovie/ray-tracing/blob/main/versions/v0.0.4.png)
[v0.0.4](https://github.com/LeviiLovie/ray-tracing/releases/tag/v0.0.4) Renderer image.

I made simple renderer. But I'm declaring every object, when rendering pixel. I will optimise it later.

## v0.0.5
[v0.0.5](https://github.com/LeviiLovie/ray-tracing/releases/tag/v0.0.5)

Performace analyser:
```
Booting v0.0.5
Window: `Ray Tracing` - 1000:1000 with upscale 1
Processing screen (it can take a few seconds)
Process Screen took 104.253406ms
```

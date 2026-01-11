# Changelog

## [0.2.0](https://github.com/burdockcascade/breeze/compare/breeze-v0.1.1...breeze-v0.2.0) (2026-01-11)


### ✨ Features

* add fullscreen toggle functionality and window context management ([d3131a2](https://github.com/burdockcascade/breeze/commit/d3131a2f92066923cdc10e7fb47ac2877a3732f7))
* add line drawing functionality to shapes module ([2903641](https://github.com/burdockcascade/breeze/commit/290364157fce026a6b352846c1c75f5eafde783c))
* implement analog clock with hands and digital display ([b169785](https://github.com/burdockcascade/breeze/commit/b1697857c2faff43ea026ac30cf232e2c83ee502))
* implement background clearing functionality in draw context ([e9ea744](https://github.com/burdockcascade/breeze/commit/e9ea74475182d774f685fe4c7f86a37023d62d9b))
* implement Bunnymark game with bunny physics and rendering ([141dfaf](https://github.com/burdockcascade/breeze/commit/141dfaf0ede2073b38642e9efcdc2a9625f59560))
* implement layered rendering with camera context ([#4](https://github.com/burdockcascade/breeze/issues/4)) ([c9b9657](https://github.com/burdockcascade/breeze/commit/c9b9657daf4a13f3541ed8074f4b661b81a83ebe))
* introduce StableId component for improved entity management and optimize sprite/text rendering processes ([84bfcd6](https://github.com/burdockcascade/breeze/commit/84bfcd6ac6a79152efae3a80d9d4252e36fdbc9d))
* optimize entity visibility management by limiting reserve count and despawning excess entities ([7ea2dac](https://github.com/burdockcascade/breeze/commit/7ea2dac32a2d5484dd690e0947882898c71ba1d4))
* optimize rendering updates by conditionally modifying properties in engine and text modules ([3c48640](https://github.com/burdockcascade/breeze/commit/3c48640dad86cec0c283ca415dfc6a3a97ff0366))
* rename sprite rendering methods to draw ([5669247](https://github.com/burdockcascade/breeze/commit/56692476e1b3f3f7ce7721fdac3e3d52ddcb9254))


### ♻️ Code Refactoring

* disable LogPlugin in runner for improved performance ([038af14](https://github.com/burdockcascade/breeze/commit/038af14d8e2cebf2ac3833f5774f751cf85dc7a5))
* move run logic into engine.rs ([daeb304](https://github.com/burdockcascade/breeze/commit/daeb30432d839a6afade35fa6e668978b84b487d))
* remove debug print statement and adjust text drawing position in Bunnymark ([077eda5](https://github.com/burdockcascade/breeze/commit/077eda578eb3fa1cbec86ed95a7f3ad65aede575))
* remove debug print statement from update function in hello_music.rs ([51286ec](https://github.com/burdockcascade/breeze/commit/51286ec76809876f5f8d2859cddae20f5b6ef71e))
* remove unused CameraContext struct and its methods for cleaner code ([47ee4f7](https://github.com/burdockcascade/breeze/commit/47ee4f7d5e5708eea0adfe3752170bd30e83c79d))
* reorganize module structure and rename files for clarity ([59fe3ef](https://github.com/burdockcascade/breeze/commit/59fe3ef1a92c76ccfaf6b9666d34b5f7db08ccf1))
* update render functions to use existing flat_commands vector for efficiency ([a80e5ec](https://github.com/burdockcascade/breeze/commit/a80e5ecadae973f69a15742cc499a03931351a27))


### 📚 Documentation

* add badges for project status, license, version, tests, and documentation to README ([3e4f9f2](https://github.com/burdockcascade/breeze/commit/3e4f9f29223ae02746b6d618dd1f09c7c4895e8d))
* add license section to README ([8286f3c](https://github.com/burdockcascade/breeze/commit/8286f3c54b6f02b757a82a826e06abde9ae5ecd3))
* update documentation badges for tests and docs links in README ([58ec556](https://github.com/burdockcascade/breeze/commit/58ec556240c75dc54d5ae31e23442822c8699971))

## [0.1.1](https://github.com/burdockcascade/breeze/compare/breeze-v0.1.0...breeze-v0.1.1) (2026-01-10)


### 🐛 Bug Fixes

* add license to Cargo.toml ([716de3b](https://github.com/burdockcascade/breeze/commit/716de3b049ea5f71fda0a4dcfc5723405e2fa335))

## 0.1.0 (2026-01-10)


### ✨ Features

* add audio handling system with AudioContext and integrate into game loop ([f24df95](https://github.com/burdockcascade/breeze/commit/f24df95e622bf67cb491558057deb43f463ac985))
* add input handling system with InputContext and integrate into game loop ([4aa6248](https://github.com/burdockcascade/breeze/commit/4aa62489b0acd778b5dd59a84ceccddd541e0ce5))
* implement basic game engine structure and initial window setup ([ad8a810](https://github.com/burdockcascade/breeze/commit/ad8a8103dffde9cb76c214350ba965fe0f43e431))
* implement sprite rendering system with SpriteContext and SpriteQueue ([f2d1964](https://github.com/burdockcascade/breeze/commit/f2d1964014d115d582c52163416718fbb1a0c410))
* implement text rendering system with TextQueue and TextContext ([232c436](https://github.com/burdockcascade/breeze/commit/232c436f06c0070fbbc342f436f93e59ed17bc9a))
* integrate shape drawing capabilities with ShapeContext and ShapePainter ([5583766](https://github.com/burdockcascade/breeze/commit/558376632441fbb9dfd7bdfd91de9094a04ea14a))


### 📚 Documentation

* add initial README with project overview and example usage ([d1b45c5](https://github.com/burdockcascade/breeze/commit/d1b45c5c2c4991014feaa5b11245725f5686f21d))
* correct casing in example usage for consistency ([a768f5c](https://github.com/burdockcascade/breeze/commit/a768f5cb14ffd12fe1de09820fa0b17494dd8cca))
* update README to remove reference to 2D Physics/Collision preparation ([dbfca2a](https://github.com/burdockcascade/breeze/commit/dbfca2a363d6e47f03e6af78818e21ca2cafd45a))

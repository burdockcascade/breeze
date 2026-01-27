# Changelog

## [0.4.0](https://github.com/burdockcascade/breeze/compare/breeze-v0.3.0...breeze-v0.4.0) (2026-01-27)


### ✨ Features

* add Scene management ([556b938](https://github.com/burdockcascade/breeze/commit/556b938e66d32fc43f8977dce33a59adf01dc2f7))
* add SystemContext for system information and integrate into Context ([55fea71](https://github.com/burdockcascade/breeze/commit/55fea7139b16df6a1ad908fb0cee12d251a56e55))
* enhance SystemContext to include monitor information and update internal game loop for multi-monitor support ([b51146b](https://github.com/burdockcascade/breeze/commit/b51146be3880004119ecb7dc98b72a1bf270901c))
* update geometry commands to support optional textures and improve material caching ([91914f3](https://github.com/burdockcascade/breeze/commit/91914f3ab914b82008e73de93f476fb465c1a551))


### ♻️ Code Refactoring

* clean up comments in hello_light.rs for clarity and conciseness ([73d5bfa](https://github.com/burdockcascade/breeze/commit/73d5bfabb44cfd25e8e550648b1e7a452a89df2d))

## [0.3.0](https://github.com/burdockcascade/breeze/compare/breeze-v0.2.2...breeze-v0.3.0) (2026-01-19)


### ✨ Features

* add basic 3d shapes ([58bcff1](https://github.com/burdockcascade/breeze/commit/58bcff1cb734cea7d7bf7718cfce691f39e298bc))
* add basic support for point and directional lights in shape rendering ([b53c911](https://github.com/burdockcascade/breeze/commit/b53c91158a4519f3b74955153c19f2b64ffe8549))
* add FPS display to UI and integrate FPS resource into context ([d091170](https://github.com/burdockcascade/breeze/commit/d0911709238c44dd62968f017ee3fbbd9e34170c))
* add shadows support to point and directional lights in the rendering system ([3e5b070](https://github.com/burdockcascade/breeze/commit/3e5b070ca24ef6fa653e630eb0b0925beb86ab50))
* add unified graphics command system with new commands and renderer ([cafad02](https://github.com/burdockcascade/breeze/commit/cafad02a1fa3aef65093888ddfd3ea25bb77f43c))
* enhance light and text rendering systems with mutable access for improved processing ([1bfe90d](https://github.com/burdockcascade/breeze/commit/1bfe90daa85e52f29cec10a2375a19af938b9d2f))
* implement FPS monitoring and display in the UI ([dc32f26](https://github.com/burdockcascade/breeze/commit/dc32f26d7bdbb752e102496e506c7c1f6aeae4e6))
* implement material caching for 2D and 3D geometry rendering ([4cf0b43](https://github.com/burdockcascade/breeze/commit/4cf0b432b9321842d6c748e1826a180408f7d8d8))
* optimize geometry processing and entity management in the renderer ([5579b56](https://github.com/burdockcascade/breeze/commit/5579b566faa4e89fd83892aeabb2508ac48ec4ed))
* refactor main function to use Breeze builder for configuration in multiple examples ([6bc1920](https://github.com/burdockcascade/breeze/commit/6bc19202894452cd95ba3ed3b94697ab82f24a98))
* refactor rendering helpers to support entity recycling and cleanup ([7cf0129](https://github.com/burdockcascade/breeze/commit/7cf0129b6560ff1146569f31fbf8b8f3ba218933))
* reorganize geometry and rendering systems for improved material caching and command processing ([cc3b53c](https://github.com/burdockcascade/breeze/commit/cc3b53cba3eb0e5c5b5f668e283e835acc4d9a8d))


### ♻️ Code Refactoring

* add documentation for shape drawing methods in shapes.rs ([a2bcbaf](https://github.com/burdockcascade/breeze/commit/a2bcbaf4bcb5c01d6d2547ccf4faf91e4e6176b9))
* clean out comments ([6f580ad](https://github.com/burdockcascade/breeze/commit/6f580adbfc6eba0f81d72353760b936a55f224b0))
* clean up comments and improve documentation in geometry, lights, sprite, text, and context modules ([d6199b5](https://github.com/burdockcascade/breeze/commit/d6199b58c7ee0a5bea42cb6b454b30c47a1efacf))
* clear background in draw method of hello_text.rs ([4a4eace](https://github.com/burdockcascade/breeze/commit/4a4eaceb489d217998dd47d78518a3d2488d88ce))
* rename shapes module to geometry and update related drawing methods ([2dcebdb](https://github.com/burdockcascade/breeze/commit/2dcebdbd826fc86140065d640866581f111e512a))


### 📚 Documentation

* add documentation comments for asset loading and context methods ([0a6b955](https://github.com/burdockcascade/breeze/commit/0a6b955b4d888843f1b89c496fb7be25c879cf05))

## [0.2.2](https://github.com/burdockcascade/breeze/compare/breeze-v0.2.1...breeze-v0.2.2) (2026-01-14)


### ♻️ Code Refactoring

* restructure shape rendering system with new command queue and global resources ([b062c2b](https://github.com/burdockcascade/breeze/commit/b062c2b0b258e62a3797eee80abc07b176ab29e5))
* update imports to use bevy::prelude for consistency ([28f887b](https://github.com/burdockcascade/breeze/commit/28f887bba5a24fa822b0c99a77079d236fc80d35))
* use default constructor for Camera2d instances in camera.rs ([c8319b2](https://github.com/burdockcascade/breeze/commit/c8319b249c44817c85f6fecdcfced7071adecffd))

## [0.2.1](https://github.com/burdockcascade/breeze/compare/breeze-v0.2.0...breeze-v0.2.1) (2026-01-13)


### ♻️ Code Refactoring

* update Cargo.toml with improved metadata and exclude unnecessary files ([257432a](https://github.com/burdockcascade/breeze/commit/257432a341a62f69690c1be06bfea163f2b51954))

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

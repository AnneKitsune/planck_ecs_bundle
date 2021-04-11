<img src="repo/splash.png" alt="Planck ECS" />
<a href="https://crates.io/crates/planck_ecs">
    <img src="https://img.shields.io/crates/v/planck_ecs.svg" alt="Planck ECS" />
</a>

# Planck ECS Bundles

Support an Open Source Developer! :hearts:  
[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

Depends on:
* [world_dispatcher](https://github.com/jojolepro/world_dispatcher): the `System` part of the ECS.

Read the [documentation](https://docs.rs/planck_ecs_bundle).

# Features

* Adds Bundle, a trait that creates a group of ECS systems and can add them to a dispatcher.

# Usage
Add the following to you Cargo.toml file:
```
planck_ecs_bundle = "*"
```

Use it like so:
```rust
use world_dispatcher::*;
use planck_ecs_bundle::*;
struct TestBundle;
impl Bundle for TestBundle {
    fn systems() -> Vec<System> {
        vec![
            (|| {Ok(())}).system(),
            (|| {Ok(())}).system(),
            (|| {println!("hello!"); Ok(())}).system(),
        ]
    }
}
fn main() {
    let mut builder = DispatcherBuilder::default();
    builder = TestBundle::insert(builder);
}
```

### Maintainer Information

* Maintainer: Jojolepro
* Contact: jojolepro [at] jojolepro [dot] com
* Website: [jojolepro.com](https://jojolepro.com)
* Patreon: [patreon](https://patreon.com/jojolepro)


use briar::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
            GamePlugins,
        ))
        .run();
}

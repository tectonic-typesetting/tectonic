use crate::c_api::dvi::DviCtx;
use crate::c_api::engine::EngineCtx;
use crate::c_api::font::FontCtx;
use crate::c_api::hash::HashCtx;
use crate::c_api::inputs::FileCtx;
use crate::c_api::output::OutputCtx;
use crate::c_api::pool::StringPool;
use crate::c_api::scaled_math::MathCtx;
use crate::c_api::synctex::SynctexCtx;
use crate::make_token;
use tectonic_bridge_core::CoreBridgeState;

type TokenCell<T> = crate::token::TokenCell<T, GlobalToken>;

make_token!(GlobalToken, local);

thread_local! {
    pub(crate) static ALL_CTX: (
        TokenCell<EngineCtx>,
        TokenCell<StringPool>,
        TokenCell<HashCtx>,
        TokenCell<FileCtx>,
        TokenCell<OutputCtx>,
        TokenCell<DviCtx>,
        TokenCell<FontCtx>,
        TokenCell<SynctexCtx>,
        TokenCell<MathCtx>,
    )= (
        TokenCell::new(EngineCtx::new()),
        TokenCell::new(StringPool::new()),
        TokenCell::new(HashCtx::new()),
        TokenCell::new(FileCtx::new()),
        TokenCell::new(OutputCtx::new()),
        TokenCell::new(DviCtx::new()),
        TokenCell::new(FontCtx::new()),
        TokenCell::new(SynctexCtx::new()),
        TokenCell::new(MathCtx::new()),
    );
}

#[non_exhaustive]
pub struct Globals<'a, 'b> {
    pub state: &'a mut CoreBridgeState<'b>,
    pub engine: &'a mut EngineCtx,
    pub strings: &'a mut StringPool,
    pub hash: &'a mut HashCtx,
    pub files: &'a mut FileCtx,
    pub out: &'a mut OutputCtx,
    pub dvi: &'a mut DviCtx,
    pub fonts: &'a mut FontCtx,
    pub synctex: &'a mut SynctexCtx,
    pub math: &'a mut MathCtx,
}

impl Globals<'_, '_> {
    pub fn token<T>(f: impl FnOnce(&mut GlobalToken) -> T) -> T {
        GlobalToken::with(f)
    }

    pub fn with<T>(f: impl for<'a, 'b> FnOnce(&mut Globals<'a, 'b>) -> T) -> T {
        ALL_CTX.with(
            |(engine, strings, hash, files, out, dvi, fonts, synctex, math)| {
                Globals::token(|token| unsafe {
                    let (engine, strings, hash, files, out, dvi, fonts, synctex, math) =
                        TokenCell::get_many_unchecked(
                            (engine, strings, hash, files, out, dvi, fonts, synctex, math),
                            token,
                        );
                    CoreBridgeState::with_global_state(|state| {
                        let mut globals = Globals {
                            state,
                            engine,
                            strings,
                            hash,
                            files,
                            out,
                            dvi,
                            fonts,
                            synctex,
                            math,
                        };
                        f(&mut globals)
                    })
                })
            },
        )
    }
}

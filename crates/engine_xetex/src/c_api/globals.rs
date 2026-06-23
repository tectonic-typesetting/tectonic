use crate::c_api::dvi::{DviCtx, DVI_CTX};
use crate::c_api::engine::{EngineCtx, ENGINE_CTX};
use crate::c_api::hash::{HashCtx, HASH_CTX};
use crate::c_api::inputs::{FileCtx, FILE_CTX};
use crate::c_api::output::{OutputCtx, OUTPUT_CTX};
use crate::c_api::pool::{StringPool, STRING_POOL};
use tectonic_bridge_core::CoreBridgeState;

#[non_exhaustive]
pub struct Globals<'a, 'b> {
    pub state: &'a mut CoreBridgeState<'b>,
    pub engine: &'a mut EngineCtx,
    pub strings: &'a mut StringPool,
    pub hash: &'a mut HashCtx,
    pub files: &'a mut FileCtx,
    pub out: &'a mut OutputCtx,
    pub dvi: &'a mut DviCtx,
}

impl Globals<'_, '_> {
    pub fn with<T>(f: impl for<'a, 'b> FnOnce(&mut Globals<'a, 'b>) -> T) -> T {
        CoreBridgeState::with_global_state(|state| {
            ENGINE_CTX.with_borrow_mut(|engine| {
                STRING_POOL.with_borrow_mut(|strings| {
                    HASH_CTX.with_borrow_mut(|hash| {
                        FILE_CTX.with_borrow_mut(|files| {
                            OUTPUT_CTX.with_borrow_mut(|out| {
                                DVI_CTX.with_borrow_mut(|dvi| {
                                    let mut globals = Globals {
                                        state,
                                        engine,
                                        strings,
                                        hash,
                                        files,
                                        out,
                                        dvi,
                                    };
                                    f(&mut globals)
                                })
                            })
                        })
                    })
                })
            })
        })
    }
}

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use variadics_please::all_tuples_enumerated;

/// Creates a new token for use with [`TokenCell`]
#[macro_export]
macro_rules! make_token {
    ($name:ident) => {
        pub struct $name($crate::token::LockToken);

        // SAFETY: This type contains a unique `LockToken`, created only once per program.
        unsafe impl $crate::token::Token for $name {}

        impl $name {
            /// # Safety
            ///
            /// This function should only be used once, to conjure the unique lock token.
            const unsafe fn conjure() -> Self {
                // SAFETY: Delegated to caller
                unsafe { $name($crate::token::LockToken::conjure()) }
            }
        }
    };
    ($name:ident, local) => {
        make_token!($name);

        impl $name {
            pub fn with<T>(f: impl FnOnce(&mut Self) -> T) -> T {
                __TOKEN_VAR.with(|(borrowed, token)| {
                    if borrowed.get() {
                        panic!("Attempted to borrow token while already borrowed")
                    }
                    borrowed.set(true);
                    let out = f(unsafe { &mut *token.get() });
                    borrowed.set(false);
                    out
                })
            }
        }

        thread_local! {
            static __TOKEN_VAR: (std::cell::Cell<bool>, std::cell::UnsafeCell<$name>) = (std::cell::Cell::new(false), std::cell::UnsafeCell::new(unsafe { $name::conjure() }));
        }
    };
    ($name:ident, sync) => {
        make_token!($name);

        pub fn with<T>(f: impl FnOnce(&mut Self) -> T) -> T {
            f(&mut __TOKEN_VAR.try_lock().unwrap())
        }

        static __TOKEN_VAR: std::sync::Mutex<$name> = std::sync::Mutex::new(unsafe { $name::conjure() });
    };
}

pub unsafe trait Token {}

pub struct LockToken(());

impl LockToken {
    /// # Safety
    ///
    /// This function should only be used once per lock token type, to conjure the unique lock token.
    pub const unsafe fn conjure() -> LockToken {
        LockToken(())
    }
}

pub trait CellMany {
    type Out<'a>
    where
        Self: 'a;

    fn assert_unique(&self);
    unsafe fn borrow_all<'a>(self) -> Self::Out<'a>;
}

macro_rules! impl_many {
    ($(( $n:tt, $T:ident )),*) => {
        impl<TOK: $crate::token::Token, $($T),*> CellMany for ($( &TokenCell<$T, TOK>, )*) {
            type Out<'a> = ($(&'a mut $T,)*)
            where
                Self: 'a;

            fn assert_unique(&self) {
                let addrs = [
                    $( (self.$n as *const TokenCell<$T, TOK>).addr() ),*
                ];
                for (idx, addr) in addrs.iter().enumerate() {
                    assert!(!addrs[idx+1..].contains(addr));
                }
            }

            unsafe fn borrow_all<'a>(self) -> Self::Out<'a>
            where
                Self: 'a,
            {
                unsafe {
                    ($(self.$n.borrow_unchecked_mut(),)*)
                }
            }
        }
    };
}

all_tuples_enumerated!(impl_many, 1, 15, T);

pub struct TokenCell<T, TOK: Token>(UnsafeCell<T>, PhantomData<TOK>);

impl<T, TOK: Token> TokenCell<T, TOK> {
    pub const fn new(val: T) -> TokenCell<T, TOK> {
        TokenCell(UnsafeCell::new(val), PhantomData)
    }

    unsafe fn borrow_unchecked<'a>(&self) -> &'a T {
        unsafe { &*self.0.get() }
    }

    unsafe fn borrow_unchecked_mut<'a>(&self) -> &'a mut T {
        unsafe { &mut *self.0.get() }
    }

    pub fn borrow<'a>(&'a self, _token: &'a TOK) -> &'a T
    where
        T: 'a,
    {
        unsafe { self.borrow_unchecked() }
    }

    pub fn borrow_mut<'a, 'b>(&'a self, _token: &'b mut TOK) -> &'b mut T
    where
        'b: 'a,
        T: 'a,
    {
        // SAFETY: Only one LockToken exists, and it can only be acquired uniquely per-thread
        unsafe { self.borrow_unchecked_mut() }
    }
}

impl<TOK: Token> TokenCell<(), TOK> {
    pub fn get_many<'a, T: CellMany + 'a>(vals: T, _token: &'a mut TOK) -> T::Out<'a> {
        vals.assert_unique();
        unsafe { vals.borrow_all() }
    }

    pub unsafe fn get_many_unchecked<'a, T: CellMany + 'a>(
        vals: T,
        _token: &'a mut TOK,
    ) -> T::Out<'a> {
        unsafe { vals.borrow_all() }
    }
}

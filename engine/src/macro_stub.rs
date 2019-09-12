macro_rules! no_mangle_extern_fn {
    ($(
        $(#[$meta:meta])*
        pub fn $symbol:ident => $extern_symbol:ident($($argname:ident: $argtype:ty),*)
                                  -> $rettype:ty;
    )*) => {
        extern "C" {
            $(
                #[no_mangle]
                $(#[$meta])*
                fn $extern_symbol($($argname: $argtype),*) -> $rettype;
            )*
        }
    };
}

macro_rules! forward_stub_fn {
    ($(
        $(#[$meta:meta])*
        pub fn $symbol:ident => $extern_symbol:ident($($argname:ident: $argtype:ty),*)
                                  -> $rettype:ty;
    )*) => {
        $(
            #[allow(unused_variables)]
            $(#[$meta])*
            pub unsafe fn $symbol($($argname: $argtype),*) -> $rettype {
                $extern_symbol($($argname),*)
            }
        )*
    };
}
macro_rules! extern_and_forward_stub {
    ($(
        $(#[$meta:meta])*
        pub fn $symbol:ident => $extern_symbol:ident($($argname:ident: $argtype:ty),*)
                                  -> $rettype:ty;
    )*) => {
        no_mangle_extern_fn!($(
            $(#[$meta])*
            pub fn $symbol => $extern_symbol($($argname : $argtype),*)
                                    -> $rettype;
        )*);
        forward_stub_fn!($(
            $(#[$meta])*
            pub fn $symbol => $extern_symbol($($argname : $argtype),*)
                                    -> $rettype;
        )*);
    };
}

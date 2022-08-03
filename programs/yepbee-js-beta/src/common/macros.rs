#[macro_export]
macro_rules! payload {
    (
        #[derive($($ident:ident),*)]
        $i:item
    ) => {
        #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, $($ident),*)]
        $i
    };
    (
            $i:item
    ) => {
        #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
        $i
    };
}

pub use payload;

#[macro_export]
macro_rules! require_unique_arrays {
    ($a:expr, $error:expr) => {
        for (i, a) in $a.iter().enumerate() {
            if $a.iter().skip(i + 1).any(|b| b == a) {
                return Err($error.into());
            }
        }
    };
}

pub use require_unique_arrays;

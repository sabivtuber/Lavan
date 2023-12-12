pub(crate) mod assoc {
    macro_rules! val {
        ($ident:ident) => {
            <$ident::Output as $crate::data::traits::Data>::Value
        };
        ($ident:ident<$param:ty>) => {
			<$ident::Output as $crate::data::traits::Data>::WithVal<$param>
		};
    }

    macro_rules! err {
        ($ident:ident) => {
            <$ident::Output as $crate::data::traits::Exceptional>::Error
        };
		($ident:ident<$param:ty>) => {
			<$ident::Output as $crate::data::traits::Exceptional>::WithErr<$param>
		};
    }

    pub(crate) use err;
    pub(crate) use val;
}

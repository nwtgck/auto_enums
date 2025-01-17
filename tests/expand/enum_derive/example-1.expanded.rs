fn foo(x: i32) -> impl Iterator<Item = i32> {
    enum __Enum1<__T1, __T2> {
        __T1(__T1),
        __T2(__T2),
    }
    impl<__T1, __T2> ::core::iter::Iterator for __Enum1<__T1, __T2>
    where
        __T1: ::core::iter::Iterator,
        __T2: ::core::iter::Iterator<Item = <__T1 as ::core::iter::Iterator>::Item>,
    {
        type Item = <__T1 as ::core::iter::Iterator>::Item;
        #[inline]
        fn next(&mut self) -> ::core::option::Option<Self::Item> {
            match self {
                __Enum1::__T1(x) => ::core::iter::Iterator::next(x),
                __Enum1::__T2(x) => ::core::iter::Iterator::next(x),
            }
        }
        #[inline]
        fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
            match self {
                __Enum1::__T1(x) => ::core::iter::Iterator::size_hint(x),
                __Enum1::__T2(x) => ::core::iter::Iterator::size_hint(x),
            }
        }
        #[inline]
        fn count(self) -> usize {
            match self {
                __Enum1::__T1(x) => ::core::iter::Iterator::count(x),
                __Enum1::__T2(x) => ::core::iter::Iterator::count(x),
            }
        }
        #[inline]
        fn last(self) -> ::core::option::Option<Self::Item> {
            match self {
                __Enum1::__T1(x) => ::core::iter::Iterator::last(x),
                __Enum1::__T2(x) => ::core::iter::Iterator::last(x),
            }
        }
        #[inline]
        #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
        fn collect<__U: ::core::iter::FromIterator<Self::Item>>(self) -> __U {
            match self {
                __Enum1::__T1(x) => ::core::iter::Iterator::collect(x),
                __Enum1::__T2(x) => ::core::iter::Iterator::collect(x),
            }
        }
        #[inline]
        fn fold<__U, __F>(self, init: __U, f: __F) -> __U
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __U,
        {
            match self {
                __Enum1::__T1(x) => ::core::iter::Iterator::fold(x, init, f),
                __Enum1::__T2(x) => ::core::iter::Iterator::fold(x, init, f),
            }
        }
        #[inline]
        fn find<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
        where
            __P: ::core::ops::FnMut(&Self::Item) -> bool,
        {
            match self {
                __Enum1::__T1(x) => ::core::iter::Iterator::find(x, predicate),
                __Enum1::__T2(x) => ::core::iter::Iterator::find(x, predicate),
            }
        }
        #[inline]
        fn find_map<__U, __F>(&mut self, f: __F) -> ::core::option::Option<__U>
        where
            __F: ::core::ops::FnMut(Self::Item) -> ::core::option::Option<__U>,
        {
            match self {
                __Enum1::__T1(x) => ::core::iter::Iterator::find_map(x, f),
                __Enum1::__T2(x) => ::core::iter::Iterator::find_map(x, f),
            }
        }
    }
    match x {
        0 => __Enum1::__T1(1..10),
        _ => {
            __Enum1::__T2(
                <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([5, 10]))
                    .into_iter(),
            )
        }
    }
}

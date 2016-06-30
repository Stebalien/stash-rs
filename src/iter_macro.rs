macro_rules! impl_iter {
    (@item_identity, $i:item) => {
        $i
    };
    ($name:ident, ($($tparm:tt)*), $item:ty, $fun:expr) => {
        impl_iter! {
            @item_identity,
            impl $($tparm)* Iterator for $name $($tparm)* {
                type Item = $item;

                fn next(&mut self) -> Option<$item> {
                    let item = (&mut self.inner).filter_map($fun).next();
                    if item.is_some() {
                        self.len -= 1;
                    }
                    item
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    (self.len, Some(self.len))
                }
            }
        }

        impl_iter! {
            @item_identity,
            impl $($tparm)* ExactSizeIterator for $name $($tparm)* {
                fn len(&self) -> usize {
                    self.len
                }
            }
        }

        impl_iter! {
            @item_identity,
            impl $($tparm)* DoubleEndedIterator for $name $($tparm)* {
                fn next_back(&mut self) -> Option<$item> {
                    let item = (&mut self.inner).rev().filter_map($fun).next();
                    if item.is_some() {
                        self.len -= 1;
                    }
                    item
                }
            }
        }
    }
}

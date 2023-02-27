macro_rules! impl_iter {
    (@item_identity, $i:item) => {
        $i
    };
    ($name:ident, ($($tparm:tt)*), $item:ty, $fun:expr, ($($wh_clause:tt)*)) => {
        impl_iter! {
            @item_identity,
            impl $($tparm)* Iterator for $name $($tparm)* $($wh_clause)* {
                type Item = $item;

                fn next(&mut self) -> Option<Self::Item> {
                    let item = (&mut self.inner).find_map($fun);
                    if item.is_some() {
                        self.len -= 1;
                    }
                    item
                }

                fn count(self) -> usize {
                    self.len()
                }

                fn last(mut self) -> Option<Self::Item> {
                    self.next_back()
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    (self.len, Some(self.len))
                }
            }
        }

        impl_iter! {
            @item_identity,
            impl $($tparm)* ExactSizeIterator for $name $($tparm)* $($wh_clause)* {
                fn len(&self) -> usize {
                    self.len
                }
            }
        }

        impl_iter! {
            @item_identity,
            impl $($tparm)* DoubleEndedIterator for $name $($tparm)* $($wh_clause)* {
                fn next_back(&mut self) -> Option<Self::Item> {
                    let item = (&mut self.inner).rev().find_map($fun);
                    if item.is_some() {
                        self.len -= 1;
                    }
                    item
                }
            }
        }
    }
}

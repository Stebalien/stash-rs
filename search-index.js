var searchIndex = {};
searchIndex["stash"] = {"doc":"Stash is a library for efficiently storing maps of keys to values when one doesn't care what the keys are but wants blazing<sup>†</sup> fast `O(1)` insertions, deletions, and lookups.","items":[[3,"Stash","stash","An `O(1)` amortized table that reuses keys.",null,null],[3,"UniqueStash","","An `O(1)` amortized table that does not reuse keys.",null,null],[3,"Tag","","A versioned index into a `UniqueStash`.",null,null],[0,"index","","",null,null],[8,"Index","stash::index","Every index type to be used with Stash needs to implement this trait",null,null],[10,"from_usize","","Create an index from `usize`.",0,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[10,"into_usize","","Turn this index into `usize`",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[0,"stash","stash","",null,null],[3,"Extend","stash::stash","",null,null],[3,"Iter","","Iterator over the `(index, &value)` pairs.",null,null],[3,"IterMut","","Iterator over the `(index, &mut value)` pairs.",null,null],[3,"IntoIter","","Iterator over the `(index, value)` pairs.",null,null],[3,"Values","","Iterator over references to the values in the stash.",null,null],[3,"ValuesMut","","Iterator over mutable references to the values in the stash.",null,null],[3,"IntoValues","","Iterator over values in the stash.",null,null],[3,"Stash","","An `O(1)` amortized table that reuses keys.",null,null],[11,"drop","","",1,{"inputs":[{"name":"self"}],"output":null}],[11,"next","","",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",1,null],[11,"next_back","","",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",2,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",2,null],[11,"len","","",2,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",3,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",3,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",3,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",3,null],[11,"len","","",3,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",3,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",4,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",4,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",4,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",4,null],[11,"len","","",4,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",4,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",5,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",5,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",5,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",5,null],[11,"len","","",5,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",5,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",6,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",6,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",6,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",6,null],[11,"len","","",6,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",6,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",7,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",7,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",7,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",7,null],[11,"len","","",7,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",7,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"clone","","",8,{"inputs":[{"name":"self"}],"output":{"name":"stash"}}],[11,"new","","Constructs a new, empty `Stash<V, usize>`.",8,{"inputs":[],"output":{"name":"self"}}],[11,"with_capacity","","Constructs a new, empty `Stash<V, usize>` with the specified capacity.",8,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[11,"capacity","","Returns the number of elements the stash can hold without reallocating.",8,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"len","","The number of items in the stash.",8,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"reserve","","Reserves capacity for at least `additional` more elements to be put into the given `Stash<T>`. The collection may reserve more space to avoid frequent reallocations.",8,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"reserve_exact","","Reserves the minimum capacity for exactly `additional` more elements to be put into the given `Stash<T>`. Does nothing if the capacity is already sufficient.",8,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"put","","Put a value into the stash.",8,{"inputs":[{"name":"self"},{"name":"v"}],"output":{"name":"ix"}}],[11,"extend","","Put all items in the iterator into the stash.",8,{"inputs":[{"name":"self"},{"name":"i"}],"output":{"name":"extend"}}],[11,"iter","","Iterate over the items in this `Stash<V>`.",8,{"inputs":[{"name":"self"}],"output":{"name":"iter"}}],[11,"iter_mut","","Mutably iterate over the items in this `Stash<V>`.",8,{"inputs":[{"name":"self"}],"output":{"name":"itermut"}}],[11,"values","","Iterate over the values in this `Stash<V>` by reference.",8,{"inputs":[{"name":"self"}],"output":{"name":"values"}}],[11,"values_mut","","Mutably iterate over the values in this `Stash<V>` by reference.",8,{"inputs":[{"name":"self"}],"output":{"name":"valuesmut"}}],[11,"into_values","","Iterate over the values in this `Stash<V>` by value.",8,{"inputs":[{"name":"self"}],"output":{"name":"intovalues"}}],[11,"is_empty","","Check if this `Stash<V>` is empty.",8,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"take","","Take an item from a slot (if non empty).",8,{"inputs":[{"name":"self"},{"name":"ix"}],"output":{"name":"option"}}],[11,"take_unchecked","","Take an item from a slot (if non empty) without bounds or empty checking. So use it very carefully!",8,{"inputs":[{"name":"self"},{"name":"ix"}],"output":{"name":"v"}}],[11,"get","","Get a reference to the value at `index`.",8,{"inputs":[{"name":"self"},{"name":"ix"}],"output":{"name":"option"}}],[11,"get_unchecked","","Get a reference to the value at `index` without bounds or empty checking. So use it very carefully!",8,{"inputs":[{"name":"self"},{"name":"ix"}],"output":{"name":"v"}}],[11,"get_mut","","Get a mutable reference to the value at `index`.",8,{"inputs":[{"name":"self"},{"name":"ix"}],"output":{"name":"option"}}],[11,"get_unchecked_mut","","Get a mutable reference to the value at `index` without bounds or empty checking. So use it very carefully!",8,{"inputs":[{"name":"self"},{"name":"ix"}],"output":{"name":"v"}}],[11,"clear","","Clear the stash. Cleared stash will give the same keys as a new stash for subsequent puts.",8,{"inputs":[{"name":"self"}],"output":null}],[11,"into_iter","","",8,null],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"index","","",8,{"inputs":[{"name":"self"},{"name":"ix"}],"output":{"name":"v"}}],[11,"index_mut","","",8,{"inputs":[{"name":"self"},{"name":"ix"}],"output":{"name":"v"}}],[11,"default","","",8,{"inputs":[],"output":{"name":"self"}}],[0,"unique_stash","stash","",null,null],[3,"TagParseError","stash::unique_stash","",null,null],[3,"Tag","","A versioned index into a `UniqueStash`.",null,null],[3,"Extend","","The iterator produced by `Unique::extend`.",null,null],[3,"Iter","","Iterator over the `(index, &value)` pairs.",null,null],[3,"IterMut","","Iterator over the `(index, &mut value)` pairs.",null,null],[3,"IntoIter","","Iterator over the `(index, value)` pairs.",null,null],[3,"Values","","Iterator over references to the values in the stash.",null,null],[3,"ValuesMut","","Iterator over mutable references to the values in the stash.",null,null],[3,"IntoValues","","Iterator over values in the stash.",null,null],[3,"UniqueStash","","An `O(1)` amortized table that does not reuse keys.",null,null],[11,"fmt","","",9,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",9,{"inputs":[{"name":"self"}],"output":{"name":"tagparseerror"}}],[11,"eq","","",9,{"inputs":[{"name":"self"},{"name":"tagparseerror"}],"output":{"name":"bool"}}],[11,"description","","",9,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"fmt","","",9,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",10,{"inputs":[{"name":"self"}],"output":{"name":"tag"}}],[11,"eq","","",10,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"bool"}}],[11,"ne","","",10,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"bool"}}],[11,"cmp","","",10,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"ordering"}}],[11,"partial_cmp","","",10,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"option"}}],[11,"lt","","",10,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"bool"}}],[11,"le","","",10,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"bool"}}],[11,"gt","","",10,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"bool"}}],[11,"ge","","",10,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"bool"}}],[11,"hash","","",10,null],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_str","","",10,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"drop","","",11,{"inputs":[{"name":"self"}],"output":null}],[11,"next","","",11,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",11,null],[11,"next_back","","",11,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",12,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",12,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",12,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",12,null],[11,"len","","",12,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",12,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",13,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",13,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",13,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",13,null],[11,"len","","",13,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",13,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",14,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",14,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",14,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",14,null],[11,"len","","",14,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",14,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",15,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",15,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",15,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",15,null],[11,"len","","",15,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",15,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",16,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",16,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",16,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",16,null],[11,"len","","",16,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",16,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",17,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"count","","",17,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"last","","",17,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",17,null],[11,"len","","",17,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"next_back","","",17,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"clone","","",18,{"inputs":[{"name":"self"}],"output":{"name":"uniquestash"}}],[11,"new","","Constructs a new, empty `UniqueStash<T>`.",18,{"inputs":[],"output":{"name":"self"}}],[11,"with_capacity","","Constructs a new, empty `UniqueStash<T>` with the specified capacity.",18,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[11,"capacity","","Returns the number of elements the stash can hold without reallocating.",18,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"len","","The number of items in the stash.",18,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"reserve","","Reserves capacity for at least `additional` more elements to be put into the given `UniqueStash<T>`. The collection may reserve more space to avoid frequent reallocations.",18,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"reserve_exact","","Reserves the minimum capacity for exactly `additional` more elements to be put into the given `UniqueStash<T>`. Does nothing if the capacity is already sufficient.",18,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"put","","Put a value into the stash.",18,{"inputs":[{"name":"self"},{"name":"v"}],"output":{"name":"tag"}}],[11,"extend","","Put all items in the iterator into the stash.",18,{"inputs":[{"name":"self"},{"name":"i"}],"output":{"name":"extend"}}],[11,"iter","","Iterate over the items in this `UniqueStash<V>`.",18,{"inputs":[{"name":"self"}],"output":{"name":"iter"}}],[11,"iter_mut","","Mutably iterate over the items in this `UniqueStash<V>`.",18,{"inputs":[{"name":"self"}],"output":{"name":"itermut"}}],[11,"values","","Iterate over the values in this `UniqueStash<V>` by reference.",18,{"inputs":[{"name":"self"}],"output":{"name":"values"}}],[11,"values_mut","","Mutably iterate over the values in this `UniqueStash<V>` by reference.",18,{"inputs":[{"name":"self"}],"output":{"name":"valuesmut"}}],[11,"into_values","","Iterate over the values in this `UniqueStash<V>` by value.",18,{"inputs":[{"name":"self"}],"output":{"name":"intovalues"}}],[11,"is_empty","","Check if this `UniqueStash<V>` is empty.",18,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"take","","Take an item from a slot (if non empty).",18,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"option"}}],[11,"get","","Get a reference to the value at `index`.",18,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"option"}}],[11,"get_mut","","Get a mutable reference to the value at `index`.",18,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"option"}}],[11,"clear","","Clear the UniqueStash.",18,{"inputs":[{"name":"self"}],"output":null}],[11,"into_iter","","",18,null],[11,"fmt","","",18,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"index","","",18,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"v"}}],[11,"index_mut","","",18,{"inputs":[{"name":"self"},{"name":"tag"}],"output":{"name":"v"}}],[11,"default","","",18,{"inputs":[],"output":{"name":"self"}}],[8,"Index","stash","Every index type to be used with Stash needs to implement this trait",null,null],[10,"from_usize","","Create an index from `usize`.",0,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[10,"into_usize","","Turn this index into `usize`",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}]],"paths":[[8,"Index"],[3,"Extend"],[3,"Values"],[3,"ValuesMut"],[3,"IntoValues"],[3,"Iter"],[3,"IterMut"],[3,"IntoIter"],[3,"Stash"],[3,"TagParseError"],[3,"Tag"],[3,"Extend"],[3,"Values"],[3,"ValuesMut"],[3,"IntoValues"],[3,"Iter"],[3,"IterMut"],[3,"IntoIter"],[3,"UniqueStash"]]};
searchIndex["unreachable"] = {"doc":"unreachable","items":[[5,"unreachable","unreachable","Hint to the optimizer that any code path which calls this function is statically unreachable and can be removed.",null,null],[8,"UncheckedOptionExt","","An extension trait for `Option<T>` providing unchecked unwrapping methods.",null,null],[10,"unchecked_unwrap","","Get the value out of this Option without checking for None.",0,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[10,"unchecked_unwrap_none","","Assert that this Option is a None to the optimizer.",0,{"inputs":[{"name":"self"}],"output":null}],[8,"UncheckedResultExt","","An extension trait for `Result<T, E>` providing unchecked unwrapping methods.",null,null],[10,"unchecked_unwrap_ok","","Get the value out of this Result without checking for Err.",1,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[10,"unchecked_unwrap_err","","Get the error out of this Result without checking for Ok.",1,{"inputs":[{"name":"self"}],"output":{"name":"e"}}]],"paths":[[8,"UncheckedOptionExt"],[8,"UncheckedResultExt"]]};
searchIndex["void"] = {"doc":"Void","items":[[4,"Void","void","The empty type for cases which can't occur.",null,null],[5,"unreachable","","A safe version of `intrinsincs::unreachable`.",null,null],[8,"ResultVoidExt","","Extensions to `Result<T, Void>`",null,null],[10,"void_unwrap","","Get the value out of a wrapper.",0,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[8,"ResultVoidErrExt","","Extensions to `Result<Void, E>`",null,null],[10,"void_unwrap_err","","Get the error out of a wrapper.",1,{"inputs":[{"name":"self"}],"output":{"name":"e"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"void"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",2,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"name":"option"}}]],"paths":[[8,"ResultVoidExt"],[8,"ResultVoidErrExt"],[4,"Void"]]};
initSearch(searchIndex);

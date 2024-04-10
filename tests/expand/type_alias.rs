#[tsify::declare]
type TypeAlias<T, U> = Foo<T, i32, U>;
#[tsify::declare(type_suffix = "Special")]
type SuffixedTypeAlias = u32;
#[tsify::declare(type_suffix = "Special")]
type SuffixedTypeAlias<AndThis> = ThisShouldBeSuffixedToo<AndThis>;

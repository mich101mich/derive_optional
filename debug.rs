impl < T > HasChangedReordered < T >
{
    #[doc = "Converts from `&HasChangedReordered<T>` to `Option<&T>`."]
    #[inline] pub fn as_option_ref(& self) -> :: std :: option :: Option < & T
    >
    {
        match * self
        {
            HasChangedReordered :: Changed(ref x) => :: std :: option ::
            Option :: Some(x), _ => :: std :: option :: Option :: None,
        }
    }
    #[doc =
    "Converts from `&mut HasChangedReordered<T>` to `Option<&mut T>`."]
    #[inline] pub fn as_option_mut(& mut self) -> :: std :: option :: Option <
    & mut T >
    {
        match * self
        {
            HasChangedReordered :: Changed(ref mut x) => :: std :: option ::
            Option :: Some(x), _ => :: std :: option :: Option :: None,
        }
    }
    #[doc =
    "Returns `NoChange` if the `HasChangedReordered` is a `NoChange`, otherwise returns `optb`. Equivalent to `Option::and`."]
    #[inline] pub fn and < U > (self, optb : HasChangedReordered < U >) ->
    HasChangedReordered < U >
    {
        match self
        {
            HasChangedReordered :: Changed(_) => optb, _ =>
            HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Returns `HasChangedReordered` if the `NoChange` is a `HasChangedReordered`, otherwise calls `f` and returns the result. Equivalent to `Option::and_then`."]
    #[inline] pub fn and_then < U, F > (self, f : F) -> HasChangedReordered <
    U > where F : FnOnce(T) -> HasChangedReordered < U > ,
    {
        match self
        {
            HasChangedReordered :: Changed(x) => f(x), _ =>
            HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Returns a `Changed` if the `HasChangedReordered` is a `Changed` and the contained value satisfies the predicate `pred`, otherwise returns `NoChange`. Equivalent to `Option::filter`."]
    #[inline] pub fn filter < P > (self, pred : P) -> Self where P :
    FnOnce(& T) -> bool,
    {
        match self
        {
            HasChangedReordered :: Changed(x) if pred(& x) =>
            HasChangedReordered :: Changed(x), _ => HasChangedReordered ::
            NoChange,
        }
    }
    #[doc =
    "Returns the `HasChangedReordered` if it is a `Changed`, otherwise returns `optb`. Equivalent to `Option::or`."]
    #[inline] pub fn or(self, optb : Self) -> Self
    {
        match self
        {
            HasChangedReordered :: Changed(x) => HasChangedReordered ::
            Changed(x), _ => optb,
        }
    }
    #[doc =
    "Returns the `HasChangedReordered` if it is a `Changed`, otherwise calls `f` and returns the result. Equivalent to `Option::or_else`."]
    #[inline] pub fn or_else < F > (self, f : F) -> Self where F : FnOnce() ->
    Self,
    {
        match self
        {
            HasChangedReordered :: Changed(x) => HasChangedReordered ::
            Changed(x), _ => f(),
        }
    }
    #[doc =
    "Returns `Changed` if exactly one of `self` or `optb` is a `Changed`, otherwise returns `NoChange`. Equivalent to `Option::xor`."]
    #[inline] pub fn xor(self, optb : Self) -> Self
    {
        match (self, optb)
        {
            (HasChangedReordered :: Changed(x), HasChangedReordered ::
            NoChange) |
            (HasChangedReordered :: NoChange, HasChangedReordered ::
            Changed(x)) => HasChangedReordered :: Changed(x), _ =>
            HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Inserts a value into the `HasChangedReordered`, then returns a mutable reference to it. Equivalent to `Option::insert`."]
    #[inline] pub fn insert(& mut self, x : T) -> & mut T
    {
        * self = HasChangedReordered :: Changed(x); match self
        {
            HasChangedReordered :: Changed(ref mut x) => x, _ => unsafe
            { :: std :: hint :: unreachable_unchecked() },
        }
    }
    #[doc =
    "Returns a mutable reference to the contained value, inserting the provided value if empty. Equivalent to `Option::get_or_insert`."]
    #[inline] pub fn get_or_insert(& mut self, value : T) -> & mut T
    {
        match self
        {
            HasChangedReordered :: Changed(ref mut x) => x, _ =>
            self.insert(value),
        }
    }
    #[doc =
    "Returns a mutable reference to the contained value, inserting the default value if empty. Equivalent to `Option::get_or_insert_default`."]
    #[inline] pub fn get_or_insert_default(& mut self) -> & mut T where T : ::
    std :: default :: Default,
    { self.get_or_insert_with(:: std :: default :: Default :: default) }
    #[doc =
    "Returns a mutable reference to the contained value, inserting the result of the provided function if empty. Equivalent to `Option::get_or_insert_with`."]
    #[inline] pub fn get_or_insert_with < F > (& mut self, f : F) -> & mut T
    where F : FnOnce() -> T,
    {
        match self
        {
            HasChangedReordered :: Changed(ref mut x) => x, _ =>
            self.insert(f()),
        }
    }
    #[doc =
    "Returns the contained `Changed` value, consuming `self`. Equivalent to `Option::expect`.\n\n# Panics\n\nPanics if the value is a `NoChange` with a custom panic message provided by `msg`."]
    #[inline] pub fn expect(self, msg : & str) -> T
    {
        match self
        { HasChangedReordered :: Changed(x) => x, _ => panic! ("{}", msg), }
    }
    #[doc =
    "Returns the contained `Changed` value, consuming `self`. Equivalent to `Option::unwrap`.\n\n# Panics\n\nPanics if the value is a `NoChange`."]
    #[inline] pub fn unwrap(self) -> T
    {
        match self
        {
            HasChangedReordered :: Changed(x) => x, _ => panic!
            ("{}",
            "called `HasChangedReordered::unwrap()` on a `NoChange` value"),
        }
    }
    #[doc =
    "Returns the contained `Changed` value or a provided default. Equivalent to `Option::unwrap_or`."]
    #[inline] pub fn unwrap_or(self, default : T) -> T
    { match self { HasChangedReordered :: Changed(x) => x, _ => default, } }
    #[doc =
    "Returns the contained `Changed` value or computes it from a closure. Equivalent to `Option::unwrap_or_else`."]
    #[inline] pub fn unwrap_or_else < F > (self, f : F) -> T where F :
    FnOnce() -> T,
    { match self { HasChangedReordered :: Changed(x) => x, _ => f(), } }
    #[doc =
    "Returns the contained `Changed` value or its default. Equivalent to `Option::unwrap_or_default`."]
    #[inline] pub fn unwrap_or_default(self) -> T where T : :: std :: default
    :: Default,
    {
        match self
        {
            HasChangedReordered :: Changed(x) => x, _ => :: std :: default ::
            Default :: default(),
        }
    }
    #[doc =
    "Returns the contained `Changed` value without checking, consuming `self`. Equivalent to `Option::unwrap_unchecked`.\n            \n# Safety\n\nThe caller must guarantee that the value is a `Changed`. Otherwise, undefined behavior occurs."]
    pub unsafe fn unwrap_unchecked(self) -> T
    {
        match self
        {
            HasChangedReordered :: Changed(x) => x, _ => unsafe
            { :: std :: hint :: unreachable_unchecked() },
        }
    }
    #[doc =
    "Returns an iterator over the possibly contained value. Equivalent to `Option::iter`."]
    #[inline] pub fn iter(& self) -> :: std :: option :: IntoIter < & T >
    { self.as_option_ref().into_iter() }
    #[doc =
    "Returns a mutable iterator over the possibly contained value. Equivalent to `Option::iter_mut`."]
    #[inline] pub fn iter_mut(& mut self) -> :: std :: option :: IntoIter < &
    mut T > { self.as_option_mut().into_iter() }
    #[doc =
    "Takes the actual value out of the `HasChangedReordered`, leaving a `NoChange` in its place. Equivalent to `Option::take`."]
    #[inline] pub fn take(& mut self) -> Self { :: std :: mem :: take(self) }
    #[doc =
    "Replaces the actual value in the `HasChangedReordered` with the provided one, returning the old value, if any. Equivalent to `Option::replace`."]
    #[inline] pub fn replace(& mut self, x : T) -> Self
    { :: std :: mem :: replace(self, HasChangedReordered :: Changed(x)) }
    #[doc =
    "Returns `true` if the `HasChangedReordered` contains the given value. Equivalent to `Option::contains`."]
    #[inline] pub fn contains(& self, x : & T) -> bool where T : :: std :: cmp
    :: PartialEq,
    {
        match self
        {
            HasChangedReordered :: Changed(y) => :: std :: cmp :: PartialEq ::
            eq(x, y), _ => false,
        }
    }
    #[doc =
    "zips `self` with another `HasChangedReordered` and returns the pair of contained values if both are `Changed`s. Equivalent to `Option::zip`."]
    #[inline] pub fn zip < U > (self, other : HasChangedReordered < U >) ->
    HasChangedReordered < (T, U) >
    {
        match (self, other)
        {
            (HasChangedReordered :: Changed(x), HasChangedReordered ::
            Changed(y)) => HasChangedReordered :: Changed((x, y)), _ =>
            HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "zips `self` with another `HasChangedReordered` and returns the result of the provided function if both are `Changed`s. Equivalent to `Option::zip_with`."]
    #[inline] pub fn zip_with < U, F, R >
    (self, other : HasChangedReordered < U > , f : F) -> HasChangedReordered <
    R > where F : FnOnce(T, U) -> R,
    {
        match (self, other)
        {
            (HasChangedReordered :: Changed(x), HasChangedReordered ::
            Changed(y)) => HasChangedReordered :: Changed(f(x, y)), _ =>
            HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Returns `true` if the `HasChangedReordered` is a `Changed` value. Equivalent to `Option::is_some`."]
    #[inline] pub fn is_changed(& self) -> bool
    { matches! (* self, HasChangedReordered :: Changed(_)) }
    #[doc =
    "Returns `true` if the `HasChangedReordered` is a `Changed` value and the predicate `f` returns `true`."]
    #[inline] pub fn is_changed_and < F > (self, f : F) -> bool where F :
    FnOnce(T) -> bool,
    { match self { HasChangedReordered :: Changed(x) => f(x), _ => false, } }
    #[doc =
    "Returns `true` if the `HasChangedReordered` is a `NoChange` value. Equivalent to `Option::is_none`."]
    #[inline] pub fn is_no_change(& self) -> bool
    { matches! (* self, HasChangedReordered :: NoChange) }
    #[doc =
    "Returns `true` if the `HasChangedReordered` is a `NoChange` value or the predicate `f` returns `true`."]
    #[inline] pub fn is_no_change_or < F > (self, f : F) -> bool where F :
    FnOnce(T) -> bool,
    {
        match self
        {
            HasChangedReordered :: NoChange => true, HasChangedReordered ::
            Changed(x) => f(x),
        }
    }
    #[doc =
    "Converts from `&HasChangedReordered<T>` to `HasChangedReordered<&T>`. Equivalent to `Option::as_ref`."]
    #[inline] pub fn as_ref < 'a > (& 'a self) -> HasChangedReordered < & 'a T
    >
    {
        match * self
        {
            HasChangedReordered :: Changed(ref x) => HasChangedReordered ::
            Changed(x), _ => HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Converts from `&mut HasChangedReordered<T>` to `HasChangedReordered<&mut T>`. Equivalent to `Option::as_mut`."]
    #[inline] pub fn as_mut < 'a > (& 'a mut self) -> HasChangedReordered < &
    'a mut T >
    {
        match * self
        {
            HasChangedReordered :: Changed(ref mut x) => HasChangedReordered
            :: Changed(x), _ => HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Converts from `Pin<&HasChangedReordered<T>>` to `HasChangedReordered<Pin<&T>>`. Equivalent to `Option::as_pin_ref`."]
    #[inline] pub fn as_pin_ref < 'a >
    (self : :: std :: pin :: Pin < & 'a Self >) -> HasChangedReordered < ::
    std :: pin :: Pin < & 'a T > >
    {
        match :: std :: pin :: Pin :: get_ref(self)
        {
            HasChangedReordered :: Changed(ref x) => unsafe
            {
                HasChangedReordered ::
                Changed(:: std :: pin :: Pin :: new_unchecked(x))
            }, _ => HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Converts from `Pin<&mut HasChangedReordered<T>>` to `HasChangedReordered<Pin<&mut T>>`. Equivalent to `Option::as_pin_mut`."]
    #[inline] pub fn as_pin_mut < 'a >
    (self : :: std :: pin :: Pin < & 'a mut Self >) -> HasChangedReordered <
    :: std :: pin :: Pin < & 'a mut T > >
    {
        unsafe
        {
            match :: std :: pin :: Pin :: get_unchecked_mut(self)
            {
                HasChangedReordered :: Changed(ref mut x) =>
                HasChangedReordered ::
                Changed(:: std :: pin :: Pin :: new_unchecked(x)), _ =>
                HasChangedReordered :: NoChange,
            }
        }
    }
    #[doc =
    "Returns a slice of the contained value, if any. Equivalent to `Option::as_slice`."]
    #[inline] pub fn as_slice(& self) -> & [T]
    {
        match * self
        {
            HasChangedReordered :: Changed(ref x) => :: std :: slice ::
            from_ref(x), _ => & [],
        }
    }
    #[doc =
    "Returns a mutable slice of the contained value, if any. Equivalent to `Option::as_mut_slice`."]
    #[inline] pub fn as_mut_slice(& mut self) -> & mut [T]
    {
        match * self
        {
            HasChangedReordered :: Changed(ref mut x) => :: std :: slice ::
            from_mut(x), _ => & mut [],
        }
    }
    #[doc =
    "Maps an `HasChangedReordered<T>` to `HasChangedReordered<U>` by applying a function to a contained value. Equivalent to `Option::map`."]
    #[inline] pub fn map < U, F > (self, f : F) -> HasChangedReordered < U >
    where F : FnOnce(T) -> U,
    {
        match self
        {
            HasChangedReordered :: Changed(x) => HasChangedReordered ::
            Changed(f(x)), _ => HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Applies a function to the contained value (if any). Equivalent to `Option::inspect`."]
    #[inline] pub fn inspect < F > (self, f : F) -> Self where F :
    FnOnce(& T),
    { if let HasChangedReordered :: Changed(ref x) = self { f(x); } self }
    #[doc =
    "Applies a function to the contained value (if any), or returns the provided default (if not). Equivalent to `Option::map_or`."]
    #[inline] pub fn map_or < U, F > (self, default : U, f : F) -> U where F :
    FnOnce(T) -> U,
    {
        match self
        { HasChangedReordered :: Changed(x) => f(x), _ => default, }
    }
    #[doc =
    "Applies a function to the contained value (if any), or calls the provided function to compute a default (if not). Equivalent to `Option::map_or_else`."]
    #[inline] pub fn map_or_else < U, D, F > (self, default : D, f : F) -> U
    where D : FnOnce() -> U, F : FnOnce(T) -> U,
    {
        match self
        { HasChangedReordered :: Changed(x) => f(x), _ => default(), }
    }
    #[doc =
    "Transforms the `HasChangedReordered<T>` into a `Result<T, E>`, mapping `Changed(x)` to `Ok(x)` and `NoChange` to `Err(err)`. Equivalent to `Option::ok_or`."]
    #[inline] pub fn ok_or < E > (self, err : E) -> :: std :: result :: Result
    < T, E >
    {
        match self
        {
            HasChangedReordered :: Changed(x) => :: std :: result :: Result ::
            Ok(x), _ => :: std :: result :: Result :: Err(err),
        }
    }
    #[doc =
    "Transforms the `HasChangedReordered<T>` into a `Result<T, E>`, mapping `Changed(x)` to `Ok(x)` and `NoChange` to `Err(err())`. Equivalent to `Option::ok_or_else`."]
    #[inline] pub fn ok_or_else < E, F > (self, err : F) -> :: std :: result
    :: Result < T, E > where F : FnOnce() -> E,
    {
        match self
        {
            HasChangedReordered :: Changed(x) => :: std :: result :: Result ::
            Ok(x), _ => :: std :: result :: Result :: Err(err()),
        }
    }
    #[doc =
    "Creates a `HasChangedReordered<&T::Target>` from an `&HasChangedReordered<T>`. Equivalent to `Option::as_deref`."]
    #[inline] pub fn as_deref < 'a > (& 'a self) -> HasChangedReordered < & 'a
    < T as :: std :: ops :: Deref > :: Target > where T : :: std :: ops ::
    Deref,
    {
        match self
        {
            HasChangedReordered :: Changed(ref x) => HasChangedReordered ::
            Changed(x.deref()), _ => HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Creates a `HasChangedReordered<&mut T::Target>` from an `&mut HasChangedReordered<T>`. Equivalent to `Option::as_deref_mut`."]
    #[inline] pub fn as_deref_mut < 'a > (& 'a mut self) ->
    HasChangedReordered < & 'a mut < T as :: std :: ops :: Deref > :: Target >
    where T : :: std :: ops :: DerefMut,
    {
        match self
        {
            HasChangedReordered :: Changed(ref mut x) => HasChangedReordered
            :: Changed(x.deref_mut()), _ => HasChangedReordered :: NoChange,
        }
    }
} #[automatically_derived] impl < T > :: std :: convert :: From < T > for
HasChangedReordered < T >
{
    #[doc = "Moves the value into a `Changed`."] fn from(x : T) -> Self
    { HasChangedReordered :: Changed(x) }
} #[automatically_derived] impl < T > :: std :: convert :: From < :: std ::
option :: Option < T >> for HasChangedReordered < T >
{
    fn from(src : :: std :: option :: Option < T >) -> Self
    {
        match src
        {
            :: std :: option :: Option :: Some(x) => HasChangedReordered ::
            Changed(x), _ => HasChangedReordered :: NoChange,
        }
    }
} #[automatically_derived] impl < T > :: std :: convert :: From <
HasChangedReordered < T > > for :: std :: option :: Option < T >
{
    fn from(src : HasChangedReordered < T >) -> Self
    {
        match src
        {
            HasChangedReordered :: Changed(x) => :: std :: option :: Option ::
            Some(x), _ => :: std :: option :: Option :: None,
        }
    }
} #[automatically_derived] impl < T > :: std :: default :: Default for
HasChangedReordered < T >
{
    #[doc = "Returns a `NoChange` value. Equivalent to `Option::default`."] fn
    default() -> Self { HasChangedReordered :: NoChange }
} #[automatically_derived] impl < T > :: std :: iter :: IntoIterator for
HasChangedReordered < T >
{
    type Item = T; type IntoIter = :: std :: option :: IntoIter < T > ;
    #[doc =
    "Returns an iterator over the possibly contained value. Equivalent to `Option::into_iter`."]
    fn into_iter(self) -> Self :: IntoIter
    { :: std :: option :: Option :: < T > :: from(self).into_iter() }
} #[automatically_derived] impl < T, U > HasChangedReordered < (T, U) >
{
    #[doc =
    "Converts a tuple of `HasChangedReordered<(T, U)>` into a tuple of (HasChangedReordered<T>, HasChangedReordered<U>). Equivalent to `Option::unzip`."]
    pub fn unzip(self) ->
    (HasChangedReordered < T > , HasChangedReordered < U >)
    {
        match self
        {
            HasChangedReordered :: Changed((x, y)) =>
            (HasChangedReordered :: Changed(x), HasChangedReordered ::
            Changed(y)), _ =>
            (HasChangedReordered :: NoChange, HasChangedReordered ::
            NoChange),
        }
    }
} impl < 'a, T > HasChangedReordered < & 'a T >
{
    #[doc =
    "Copies the contained value into a new `HasChangedReordered<#some_ty>` if it implements `Copy`. Equivalent to `Option::copied`."]
    pub fn copied(& self) -> HasChangedReordered < T > where T : Copy
    {
        match self
        {
            HasChangedReordered :: Changed(x) => HasChangedReordered ::
            Changed(* * x), _ => HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Clones the contained value into a new `HasChangedReordered<#some_ty>` if it implements `Clone`. Equivalent to `Option::cloned`."]
    pub fn cloned(& self) -> HasChangedReordered < T > where T : Clone
    {
        match self
        {
            HasChangedReordered :: Changed(x) => HasChangedReordered ::
            Changed((* x).clone()), _ => HasChangedReordered :: NoChange,
        }
    }
} impl < 'a, T > HasChangedReordered < & 'a mut T >
{
    #[doc =
    "Copies the contained value into a new `HasChangedReordered<#some_ty>` if it implements `Copy`. Equivalent to `Option::copied`."]
    pub fn copied(& self) -> HasChangedReordered < T > where T : Copy
    {
        match self
        {
            HasChangedReordered :: Changed(x) => HasChangedReordered ::
            Changed(* * x), _ => HasChangedReordered :: NoChange,
        }
    }
    #[doc =
    "Clones the contained value into a new `HasChangedReordered<#some_ty>` if it implements `Clone`. Equivalent to `Option::cloned`."]
    pub fn cloned(& self) -> HasChangedReordered < T > where T : Clone
    {
        match self
        {
            HasChangedReordered :: Changed(x) => HasChangedReordered ::
            Changed((* x).clone()), _ => HasChangedReordered :: NoChange,
        }
    }
} #[automatically_derived] impl < T, E > HasChangedReordered < Result < T, E
>>
{
    #[doc =
    "Transposes a `HasChangedReordered<Result<T, E>>` into a `Result<HasChangedReordered<T>, E>`. Equivalent to `Option::transpose`."]
    pub fn transpose(self) -> Result < HasChangedReordered < T > , E >
    {
        match self
        {
            HasChangedReordered :: Changed(Ok(x)) =>
            Ok(HasChangedReordered :: Changed(x)), HasChangedReordered ::
            Changed(Err(e)) => Err(e), _ =>
            Ok(HasChangedReordered :: NoChange),
        }
    }
} #[automatically_derived] impl < T > HasChangedReordered <
HasChangedReordered < T >>
{
    #[doc =
    "Flattens a double `HasChangedReordered<HasChangedReordered<T>>` into a single `HasChangedReordered<T>`. Equivalent to `Option::flatten`."]
    pub fn flatten(self) -> HasChangedReordered < T >
    {
        match self
        {
            HasChangedReordered :: Changed(x) => x, _ => HasChangedReordered
            :: NoChange,
        }
    }
} #[automatically_derived] impl < T > HasChangedReordered < :: std :: option
:: Option < T >>
{
    #[doc =
    "Flattens a `HasChangedReordered<Option<T>>` into a single `HasChangedReordered<T>`. Similar to `Option::flatten`."]
    pub fn flatten(self) -> HasChangedReordered < T >
    {
        match self
        {
            HasChangedReordered :: Changed(x) => x.into(), _ =>
            HasChangedReordered :: NoChange,
        }
    }
}
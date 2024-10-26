+++
title = "My Haskell Notes"
date = 2024-10-26
[taxonomies]
tags=["haskell", "notes", "personal"]
+++

## type ambiguity with multi type param typeclasses
when we introduce multiple params to typeclasses, haskell is more likely to raise "ambiguity" errors. 
consider the following example:

```hs
class Foo a b where
  get :: a -> b

instance Foo Int [Int] where
  get x = []

instance Foo Int String where
  get x = []

foo :: (Foo a b) => a -> b
foo a = get a
```

now if want to invoke call 
```hs
foo 2
```

we get the following error:
```
• Could not deduce ‘Foo a0 b’
    from the context: (Foo a b, Num a)
    bound by the inferred type for ‘it’:
                forall {a} {b}. (Foo a b, Num a) => b
    at <interactive>:1:1-5
    The type variable ‘a0’ is ambiguous
    Potentially matching instances:
    instance [safe] Foo Int [Int] -- Defined at app/Main.hs:22:10
    instance [safe] Foo String Int -- Defined at app/Main.hs:25:10
• In the ambiguity check for the inferred type for ‘it’
    To defer the ambiguity check to use sites, enable AllowAmbiguousTypes
    When checking the inferred type
    it :: forall {a} {b}. (Foo a b, Num a) => b
```

this happens because when you call `foo 2`, haskell tries to resolve `Foo a b`, which I believe happens before the function
is actually evaluted, so it has solid types to operate on. so it sees `a` in `get :: a -> b`, and it knows its `Int` or 
something. 
nah dumbass! `a` is still not a solid type, haskell just puts a `Num` constraint on it, and that's what it only knows. So both `a` and `b` are ambigious. let's be more explicit here

```hs
foo (2 :: Int)
```

Then haskell looks for a `Foo Int b` instance, do we have one? No. So haskell is confused about which instance of
`Foo a b` you actually want to use, which the error message helpfully provides for you. To fix such an error, you have to be 
more explicit about the type. i.e 

```hs
(foo (2 :: Int) :: [Int])
```

tada, now haskell knows that you meant the `Foo Int [Int]` instance

## functional deps
with the multi type param extensions, you sometimes want one type to decide the other. e.g 
```hs
class Foo a b where
  get :: a -> b
```

this means we can do stuff like 

```hs
instance Foo Int M
  ... 
  
instance Foo String M
  ...
```

this is fine, but sometimes it isn't desirable. we don't want multiple `b`s for one `a`. we only want 1:1
so how do we communicate this to haskell? functional deps.
(enable the ext)

```hs
class Foo a b | a -> b where
  get :: a -> b
```

that's it, now if you try the snippet earlier, haskell would complain, you would have to only keep one of them.
think if `b` is some certain certain, it doesn't make sense for a state to be associated with different types.

now let's see a more complicated example. the `megaparsec` library provides a function called `single`, it's type signature is:

```hs
single :: MonadParsec e s m => Token s -> m (Token s)
```
roughly, this means that `e` `s` and `m` are related through `MonadParsec`. that's the signature of the class

```hs
class (Stream s, MonadPlus m) => MonadParsec e s m | m -> e s where 
   ...
```

you see there is a functional dep, which says "m determines e and s". yes so that means we only need to tell it about `m`, it will take care of `e` and `s`
if we try evaluate `single '\n'`, we would get an ambiguity error, because we don't know `m`, so we don't know `s` which is the first argument of `single`.
it tries to match `Char` (what we gave it) against what it knows i.e `Token s`. Nothing it can do abaout it. let's help it out and give it an explicit type

note: it knows `Token s ~ Char`, but nothing about `s`

```hs
(single '\n') :: Parsec Void String Char
```
so because we know `Token s ~ Char` (hint from '\n'), we observe how haskell matches the types

```hs
m Char
Parsec Void String Char
```

it sees that the `Char` is constant in both sides, so it attempts to match `m` with `Parsec Void String`, and it should work out.
now we know `m ~ Parsec Void String`, remember when I said that's what we only need? yes, so because of the functional dependency, you can rest assured that
there is only one instance of `MonadParsec` where `m ~ Parsec Void String`, which means there are only one `s` and `e` that can fit in here, and those are 
the `Void` and `String`. So, ultimately haskell knows that we need the `MonadParsec Void String (ParsecT Void String Identity)`. And if we hoogle it up: 

```hs
 (Ord e, Stream s) => MonadParsec e s (ParsecT e s m)
```

we can see that there is indeed an instance with the types we employed.


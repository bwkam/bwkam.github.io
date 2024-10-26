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


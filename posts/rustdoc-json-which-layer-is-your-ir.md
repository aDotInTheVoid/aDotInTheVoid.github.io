# Rustdoc Json Design Notes: Tension, and What layer IR are we:

## Intro

- What's rustdoc-json
- Who uses it
    - People that look like a typechecker
        - CSC
    - People that look like a AST pretty-printer
- Why do compilers have different IR's
    - Desugar
    - Handler less cases in 


## State of rustdoc-json today

### What rustdoc-json was designed for initially

(TODO: Ask julia, maybe tyler)

### What rustdoc-json is used for today

- https://github.com/stars/aDotInTheVoid/lists/rustdoc-json
- the changelog release-notes thing.

(TODO: Figure out if anyone uses the it for docs in anger)


## Choices we gotta make

I swear there were loads of these.

### Where bounds vs bounds on ParamDefs

- `fn foo<T: Debug>()` and `fn foo<T>() where T: Debug`
- To the type-checker they're the same
- So rustc when lowering from hir->middle makes them the same.
- Some users for rustdoc json don't care, but some do.
    - Because some people care, the people who don't have to pay for both of them

### Desugaring of `async Fn`.

- `fn foo() -> impl Future<Output=i32>` and `async fn foo() -> i32` are kinda the same.
- The compiler doesn't need to know about the latter in the `middle::ty` IR
- But rustdoc want's to present these differently
- But CSC wan't to know you can change between them.
    - Rustc doesn't need logic for this in the typechecker, it all happens in lowering!
    - Rustdoc-JSON was is great to mean you don't need to re-impl parse/expand/nameres, but you still gotta lower

### Auto-trait leakage via impl Trait in return position

### Variance

### Outlives bounds

### Keyword and attr docs

### Fully qualified path

(ask jyn more)


## Should rustdoc-json include URL's?

## Should rustdoc-json include stability info?

## Something about re-exports and middle->clean

- Usually, rustdoc on the local crate runs on HIR
- But when re-exporting, we only have middle from metadata
- Demo this somehow
- In practice, this doen't matter for rustdoc-json, because we don't inline

## Conclusion

- Different analises want different views of source code
- IR's offer those different views
- Inside rustc, there's many IR's, so you can usually do what you want
    - Not always, theres some exaples from clippy IIRC
- Rustdoc JSON only gets 1 IR, but serves some users who want different things.
- Many tools would rather look at a middle/ty like IR, but that's a pain
    - Rustdoc JSON got adoption not by being the most suitable, but by being easy to use
    - https://alona.page/talks/rustdoc-json.pdf
- Maybe `rustc_public` can solve this

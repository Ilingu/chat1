# [chat1] 🐈

#### A bare bones implementation of the sha1 algorithm in rust

## Motivation

Wanted to understand how "one way function" works and more specifically where the entropy in information is lost.

So I read the white paper and set myself a deadline of 2h to understand precisly how it works then 1h to implement it. This little 'quick coding' exercise was fun, I more or less respected the deadlines (I had a lot of endianness/bytes manipulation problems), and I totally see myself doing this more often as I seem to be more productive that way.

This introduce me into a hole new concept, that improved my understanding of bitwise operation and how we could use them to
gain entropy which then we can lose thanks to wrapping operation. I also learn a lot about how a stream of bytes works and how it should be handled in rust, more generally this made me more conformtable with endianness and an obscure part of rust

I then make a CLI out of it because why not?

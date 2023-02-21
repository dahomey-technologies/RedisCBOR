# RedisCBOR

RedisCBOR is a [Redis](https://redis.io/) module that implements [CBOR](https://cbor.io/) as a native data type. 
It allows storing, updating and fetching CBOR documents from Redis keys (documents).

It is based on [RedisJson](https://redis.io/docs/stack/json/) for its concepts, its commands and their syntax.

## Primary features:

* Full support of the CBOR standard
* [CBORPath](https://github.com/dahomey-technologies/cborpath-rs) syntax for selecting elements inside documents
* Documents are stored as raw CBOR binary data, allowing reduced memory footprint
* Typed atomic operations for all CBOR types

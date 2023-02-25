# CBOR.ARRAPPEND

Append the `CBOR` values into the array at `path` after the last element in it, at `key`.

### Syntax
```bash
CBOR.ARRAPPEND key path value [value ...]
```

## Required arguments

<details open><summary><code>key</code></summary> 
the key to modify.
</details>

<details open><summary><code>path</code></summary> 
the CBORPath to specify.
</details>

<details open><summary><code>value</code></summary> 
one or more values to append to one or more arrays. 
</details>

## Return value 

`CBOR.ARRAPEND` returns an [array](/docs/reference/protocol-spec/#resp-arrays) of integer replies for each path, the array's new size, or `nil`, if the matching CBOR value is not an array. 
For more information about replies, see [Redis serialization protocol specification](/docs/reference/protocol-spec). 

## Examples

Create a document with a sub-array

```bash
# path: ["$"] 
# value: {"foo":["a","b","c"]}
redis> CBOR.SET key "\x81\x61$" "\xa1\x63foo\x83\x61a\x61b\x61c"
OK
```

Add `"d"` to the end of the sub-array. `CBOR.ARRAPEND` returns the array's new size.

```bash
# path: ["$", "foo"] 
# value: "d"
redis> CBOR.ARRAPPEND key "\x82\x61$\x63foo" "\x61d"
1) (integer) 4
```

Return the new document version

```bash
# value: {"foo":["a","b","c","d"]}
redis> CBOR.GET key
"\x81\xa1cfoo\x84aaabacad"
```

## See also

[`CBOR.ARRINDEX`](cbor.arrindex.md) | [`CBOR.ARRINSERT`](cbor.arrinsert.md) | [`CBOR.ARRLEN`](cbor.arrlen.md)

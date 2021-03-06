# Tobu (Work in Progress)

`tobu` is a pro**tobu**f data format for [`serde`] with first class support for reflection.

This library is meant for academic purposes. It's primary feature is a full write up of my journey
through Rust and creating this library. While correctness is important, do not expect production
level maturity. Speed and benchmarking are important too, but only to the point of quenching my
curiosity.

## Motivation

There are already Protobuf implementations out in the wild, most notably [`prost`], the library
powering [`tonic`]. Why make another?

To get a better grasp of Rust, I felt the need to work on a project and took personal interest in
[`protobuf`] and [`gRPC`]. In particular, I wanted to try making a server that supports the
[`transcoding`] `google.api.http` annotation.

```proto
// transcode between HTTP GET /v1/shelves/{shelf} and GetShelf gRPC 
// where the field shelf in GetShelfRequest maps to {shelf} in the URI. 
// Note: All protobuf messages have a JSON encoding.
rpc GetShelf(GetShelfRequest) returns (GetShelfResponse) {
  option (google.api.http) = { get: "/v1/shelves/{shelf}" };
}

message GetShelfRequest {
  int64 shelf = 1;
}
```

With this option, gRPC servers could effortlessly add an analog HTTP/JSON interface to their API.
This is especially important for public facing servers since HTTP/JSON is so widespread.

Feeling stoked, I investigated what it would take to support it. What a rabbit hole that turned
out to be! The annotation is a custom option. Custom options are `proto2` extensions that require
working with descriptors. Working with descriptors requires reflection. Oh, and there is a
standard JSON wire format that needs to be supported too. This one little line  ended up traversing
much of the Protobuf feature set.

Browsing the Rust Protobuf ecosystem, I felt that reflection was a critical missing piece of
functionality. In addition to reflection, I also felt libraries like prost were missing that
distinctive Rust flavor that Serde provides. Serialization libraries should support
`#[derive(Serialize, Deserialize)]`. While some libraries did, they "cheated" by having each
generated message implement their own custom serialization without actually making a data format. 

There is a reason why other Protobuf libraries choose not to implement a Serde data format:
Serialization requires external metadata, like field numbers among other things. In other words, 
first class support for reflection is needed.

## Implementation

What follows is my documented experience writing the library before you. Please bear with me as this
project is quite the learning process. I started it right after reading [`The Rust Book`]. 

###  Reflection primer

To a user, reflection is a standardized way to [`describe`] and manipulate messages generated by
the Protobuf compiler. Any valid `.proto` file can be translated into a `FileDescriptorProto`
Protobuf message and back without losing any important information. Even comments are preserved.
Descriptors make it possible to write a single interface that can validate, manipulate, extend, or
even dynamically create any message object.

Of particular interest is the ability for third parties to extend messages with additional fields.
For example, custom options, like the `google.api.http` annotation, can be created by adding an
extension to `google.protobuf.MessageOptions`. 

For myself, the developer of this library, I would like to create a Protobuf compiler plugin that
is capable of generating both messages and descriptors in Rust and a library that provides a
reflective interface to utilize them. Piece of cake right? Right... Let's begin.

### Deep dive into `protobuf-go`

Since other programming languages have Protobuf libraries with support for reflection, let's use
them as an example. After a little digging, I found [`protobuf-go`], a relatively recent library
implemented in a compiled language with clear thought put into the reflection interface. Once I saw
it, I immediately wanted to model my own interface after it.

Though the interface is nice, the devil is in the details, as they say. Hidden underneath that
interface was an implementation that even a fledgling Rustacean such as myself could tell would not
translate to Rust without a world of hurt. Take for example:

```proto
syntax = "proto3";

message Simple {
    bool simple_bool = 1;
}
```

Running the `.proto` above through `protoc` with the Go Protobuf plugin produces the struct:

```go
type Simple struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	SimpleBool bool
}
```

`SimpleBool` is public data; the rest is private reflection state. `state` is interesting in
particular because it contains a single pointer:

```go
type MessageState struct {
	atomicMessageInfo *MessageInfo
}
```

and, you see, `state` happens to always be the first field in the struct. This is not by
coincidence, but to fully understand it, we need to dig deeper. To access the reflective interface,
users call `ProtoReflect`. Every generated message's `ProtoReflect` implementation looks similar. 
Here is what it looks like:

```go
var file_simple_proto_msgTypes = make([]protoimpl.MessageInfo, 1)

func (x *Simple) ProtoReflect() protoreflect.Message {
	mi := &file_simple_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

func (ms *messageState) LoadMessageInfo() *MessageInfo {
	return (*MessageInfo)(atomic.LoadPointer((*unsafe.Pointer)(unsafe.Pointer(&ms.atomicMessageInfo))))
}

func (ms *messageState) StoreMessageInfo(mi *MessageInfo) {
	atomic.StorePointer((*unsafe.Pointer)(unsafe.Pointer(&ms.atomicMessageInfo)), unsafe.Pointer(mi))
}
```

If we wipe off all the `atomic` and `unsafe.Pointer` gore, this function does two things:

1. It casts `*Simple` to a type called `*MessageState` which implements the reflective interface
   `protoreflect.Message`.
1. It assigns some static descriptor data to `atomicMessageInfo` if it has not been assigned
   already.

With this critical piece of information in mind, let's look at an example of reflection in action:

```go
func TestReflection(t *testing.T) {
	s := proto3.Simple{SimpleBool: true}
	r := s.ProtoReflect()
	v := r.Get(r.Descriptor().Fields().ByName("simple_bool")).Bool()
	if v != s.SimpleBool {
		t.Errorf("field simple_bool = %v, want %v", v, s.SimpleBool)
	}
}
```

Everything up through the call to `ProtoReflect` so far is clear. We now know that the descriptor
information, a.k.a. `MessageInfo`, can be accessed through the `atomicMessageInfo` pointer. And we
know that our `Simple` object `s` is masquerading as a `MessageState`. But, that's only the tip of
the iceberg. We have effectively just erased our type. So, then how does a generic type like
`MessageState` magically access concrete data like `s.SimpleBool` using only a descriptor? 

The process is complicated so let's do a little backtracking. What is this descriptor thing? Within
the generated Go code, looking very conspicuous, is a random array of bytes:

```go
var file_simple_proto_rawDesc = []byte{
	0x0a, 0x0c, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13,
	//...
}
```

That random array of bytes turns out to be a serialized `FileDescriptorProto` containing all the
static information for `Simple`. These byes get lazily deserialized in various stages as needed
during runtime. For example, calling `Message::Descriptor` would initialize less static data than
calling `Message::Get`. In Rust, this is equivalent to wrapping the fields and nested fields within
`MessageInfo` in layers of [`sync::OnceCell<T>`]. 

Speaking of which, let's take a look at the fields in `MessageInfo`. For the purpose of explanation,
some liberties were taken to simplify the code.

```go
type MessageInfo struct {
	Desc         pref.MessageDescriptor
	fields       map[pref.FieldNumber]*fieldInfo
	extensionMap func(pointer) *map[pref.FieldNumber]ExtensionField
	coderFields  map[pref.FieldNumber]*coderFieldInfo
}

type fieldInfo struct {
	fieldDesc pref.FieldDescriptor
	has       func(pointer) bool
	clear     func(pointer)
	get       func(pointer) pref.Value
	set       func(pointer, pref.Value)
}

type coderFieldInfo struct {
	funcs      pointerCoderFuncs
	mi         *MessageInfo      // non-null when field is of type Message
	num        pref.FieldNumber  // unique to each message
	offset     offset            // offset in bytes from beginning of struct
}

type pointerCoderFuncs struct {
	size      func(p pointer, f *coderFieldInfo) int
	marshal   func(b []byte, p pointer, f *coderFieldInfo) ([]byte, error)
	unmarshal func(b []byte, p pointer, wtyp protowire.Type, f *coderFieldInfo) error
}
```

As expected, `MessageInfo.Desc` and `fieldInfo.fieldDesc` contain de-serialized descriptor data, but
the other fields are more interesting. They are function objects that take a `pointer` which, as you
may have guessed, is `*MessageState`. By indexing some [`Offset`] into `*MessageState` and casting
the data to a [`Value`], the reflective interface is able to access and modify a concrete message's
data, like it did for `Simple` in the test above. 

Even though I have some familiarity with Go, this all sounded somewhat nuts to do, especially with
generated code. Go, however, has powerful reflection capabilities built into the language, so this
practice becomes somewhat reasonable. I would not try this with C/C++ and Rust similarly does not
have such reflective capabilities. I don't think that is a bad thing though; Rust was built on a
completely different foundation of thought.

### Back to Rust with a glance at `Pin`

For the moment, let's forget about Serde. How should reflection be implemented? After we figure out
reflection, we can get it working with Serde.

A Rust analog to the implementation above would involve gratuitous amounts of [`mem::transmute`].
Just a brief glance at the documentation and a shallow gaze into the [`nomicon`] and I am feeling
just a little uncomfortable. Since Rust doesn't have the same reflective capabilities as Go, it
truly feels like casting off into a sea of bytes and praying for correctness. The point of this
project is to learn and Rust seems to be telling me to find another way. Let's take the hint.

How about making our own type like `MessageState` - let's call it `Reflection` - that holds a
reference to the other data in the struct. `Reflection` would also contain references to generated
descriptor data, but let's leave that out for now. To reflect, just return `reflect`, a field that
all generated Protobuf messages could have in common.


```rust
// Imagine this contains a reference to static descriptor info too.
pub struct Reflection<'a> {
    pub fields: Vec<Value<'a>>,
}

// Imagine more variants here.
pub enum Value<'a> {
    Bool(&'a bool),
    I32(&'a i32),
}

// A generated message with internal message state for reflection.
pub struct Simple<'a> {
    reflect: Reflection<'a>,
    pub simple_bool: bool,
}
```

Of course, it's not quite this easy. References can never be null so it's not possible to make a
struct that points to itself like this:

```rust
impl<'a> Simple<'a> {
    pub fn new() -> Self {
        Simple {
            reflect: Reflection {
                fields: vec![Value::Bool(/* ??? */)],
            },
            simple_bool: false,
        }
    }
}
```

Thankfully there is a type for that, and looking at the standard library, all the cool containers
are using it!

```rust
use std::ptr::NonNull;

pub enum Value {
    Bool(NonNull<bool>),
    I32(NonNull<i32>),
}

impl Simple {
    pub fn new() -> Self {
        let mut s = Simple {
            reflect: Reflection {
                fields: vec![Value::Bool(NonNull::dangling())],
            },
            simple_bool: false,
        };
        s.reflect.fields[0] = Value::Bool(NonNull::from(&s.simple_bool));
        s
    }
    
    pub fn reflect(&self) -> &Reflection {
        &self.reflect
    }

    pub fn reflect_mut(&mut self) -> &mut Reflection {
        &mut self.reflect
    }
}

```

It's very nice how the lifetime went away too. Borrowing `reflect`, which is bound to the lifetime
of the object holding it, should always live long enough. Okay, so this code is a little clever, but
it compiles! Does it work? Nope! Check out this test:

```rust
#[test]
fn it_works() {
    let s = Simple::new();
    
    // If it wasn't for return value optimization, we would already be 
    // in trouble, but let's explicitly force a move to prove a point.
    let s = s;

    let v = match s.reflect().fields.get(0) {
        Some(Value::Bool(v)) => v,
        _ => unreachable!(),
    };

    assert_eq!(format!("{:p}", v.as_ptr()), format!("{:p}", &s.simple_bool));
}
```

```
thread 'it_works' panicked at 'assertion failed: `(left == right)`
  left: `"0x7fc75c46f578"`,
 right: `"0x7fc75c46f598"`'
```

Coming from C++, one of the nice things about Rust is how natural moving data feels. I was just
sitting here taking it for granted... until this panic happened. 

Okay so now what? Surely I am not the only one who wants to do this kind of thing. Sure enough, I
found something called [`pin`]. The documentation even has an example for my use case, something
called self-referential structs. I had never seen anything like `Pin` before, but thanks to
[`Boats`], [`Jon`], and the [`async-book`], I was finally able to wrap my head around it! 

As magical as it is, `Pin` won't actually solve the problem. Yes, it's important to use it when
writing self-referential structs, but it's purpose is to turn runtime bugs into compile time errors;
it doesn't tell the compiler to prevent moves from happening. I'd say it's more of a safety `Pin`
(sorry!). 

Ultimately, the problem is much more fundamental. I am using the stack. Things on the stack get
moved and fall out of scope. So, I need to use the heap. It's not immediately obvious, but heap
allocation and `Pin` are a package deal. Rust drops little hints like `Box` having it's own
[constructor](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.pin) specifically for
pinning objects to the heap. Also, in the example from the `pin` documentation , the constructor for
the self-referential struct returns a `Pin<Box<Self>>`.

```rust
fn new(data: String) -> Pin<Box<Self>> { /* ... */ }
```

That's not a coincidence; it forces heap allocation. Heap allocated objects are accessed indirectly
through pointers, and those pointers are free to move around all they want. 

By the way, objects can be pinned to the stack, but it's incredibly limiting, so the use cases for
it are rare. It took me far too long to come to terms with that.

Unfortunately, allocating a type like `Simple` on the heap feels a little heavy and users would have
to interact with a `Box` instead of the type directly, all for a feature most will never use. 

For those who want a more complete example of the code above, take a look at the prototype
[here](https://github.com/ajguerrer/tobu-self-ref).

As a last gasp, I found another interesting concept called [`rel-ptr`]. This library could be used
in a very similar way to [`Offset`] with no `transmute` necessary. However, concerns have been
raised that the implementation is [unsound](https://github.com/RustyYato/rel-ptr/issues/3). I think
I would want support from the language for something like that anyway. Let's keep searching for
alternatives.

### Type conversion: heavy and light

How about something a little more conventional? Let's make a conversion trait, much like [`From`],
but just a little fancier.

```rust
pub trait Reflect: Sized {
    fn reflect(self) -> Reflection<Self>;
}

pub struct Reflection<T> {
    message: Message,
    _marker: PhantomData<T>,
}

pub struct Message {
    fields: Vec<Option<Value>>,
}
```

Damn, that's fancy. What does [`PhantomData`] do? When a type is generic over type parameter `T`,
the compiler rightfully gets mad if the type doesn't actually make use of that `T`. However, there
are some use cases where keeping that information around is important. `Reflection` uses it to track
which concrete message type to go back to, via `absorb`, the aptly named opposite of `reflect`.

```rust
impl<T> Reflection<T> {
    pub fn absorb(self) -> Result<T, ReflectionError>
    where
        Message: TryInto<T, Error = ReflectionError>,
    {
        self.message.try_into()
    }
}

impl<T> Reflect for T
where
    T: Into<Message>,
{
    fn reflect(self) -> Reflection<Self> {
        Reflection::new(self)
    }
}
```

Assuming `Simple` can be converted `Into`/`From`  a `Message`, the reflective interface looks like:

```rust
let s: Simple = Simple::new();
let r: Reflection<Simple> = s.reflect();
let s: Simple = r.absorb().unwrap();
```

All that's left is to implement the actual conversion. As you may have guessed from the section
title, I found two type conversion candidates to compare: heavy and light.

Before getting into it, let's flex the Protobuf type system a little more by making a new message 
`Complex`.

```proto
syntax = "proto2";

message Complex {
  optional Enum optional_enum = 1;
  repeated bytes repeated_bytes = 2;
  map<int32, Nested> map_message = 3;

  enum Enum { 
    ZERO = 0;
    ONE = 1;
    TEN = 10;
  }

  message Nested {
    optional string optional_string = 1;
  }
}
``` 

Lets also fully flesh out `Value`, but this time without the references.

```rust
use std::collections::HashMap;

pub enum Value {
    Bool(bool),
    Bytes(Vec<u8>),
    Enum(Enum),
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    Message(Message),
    String(String),
    U32(u32),
    U64(u64),
    List(Vec<Value>),
    Map(HashMap<Value, Value>),
}
```

This design won't go out of it's way to prevent invalid types around `List` and `Map`. For example,
it's possible to create a list of maps, which isn't definable in Protobuf syntax. Also it's really
nice when the template type parameters of `List` and `Map` match the actual type that you want
because then the per-element conversion `Into`/`From` a `Value` is no longer necessary. Here is a
definition that utilizes the type system better:

```rust
#[derive(Debug, Clone)]
pub enum Value {
    Bool(Rule<bool>),
    Bytes(Rule<Vec<u8>>),
    Enum(Rule<Enum>),
    F32(Rule<f32>),
    F64(Rule<f64>),
    I32(Rule<i32>),
    I64(Rule<i64>),
    Message(Rule<Message>),
    String(Rule<String>),
    U32(Rule<u32>),
    U64(Rule<u64>),
}

#[derive(Debug, Clone)]
pub enum Rule<T> {
    Singular(T),
    Repeated(Vec<T>),
    Map(Key<T>),
}

#[derive(Debug, Clone)]
pub enum Key<T> {
    Bool(HashMap<bool, T>),
    I32(HashMap<i32, T>),
    I64(HashMap<i64, T>),
    String(HashMap<String, T>),
    U32(HashMap<u32, T>),
    U64(HashMap<u64, T>),
}
```

Granted, `Value` becomes less intuitive, but let's give it a try. Note, it is still possible for
`absorb` to fail. For example, `Enum` values may not match a valid variant or the `field`s in
`Message` may not match the descriptor. This is why `absorb` returns a `Result`. Without further
ado, on to conversion.

#### The heavy conversion

Welcome to the "heavy" conversion. This is the most direct approach. Compiling our Protobuf messages
would produce Rust structs that look like:

```rust
pub struct Simple {
    pub simple_bool: bool,
}

pub struct Complex {
    pub optional_enum: Option<ComplexEnum>,
    pub repeated_bytes: Vec<Vec<u8>>,
    pub map_message: HashMap<i32, ComplexNested>,
}

pub enum ComplexEnum {
    One = 1,
    Two = 2,
    Ten = 10,
}

pub struct ComplexNested {
    pub optional_string: Option<String>,
}
```

Each struct has fields that are intuitive enough to be made public; this won't be the case for the
light conversion. But as a trade-off, the fields need to be fully processed to produce a
`Reflection`. 

```rust

impl From<Complex> for Message {
    fn from(m: Complex) -> Self {
        Message {
            fields: vec![
                m.optional_enum
                    .map(|v| Value::Enum(Rule::Singular(Enum { number: v as i32 }))),
                Some(Value::Bytes(Rule::Repeated(m.repeated_bytes))),
                Some(Value::Message(Rule::Map(Key::I32(
                    m.map_message
                        .into_iter()
                        .map(|(k, v)| (k, v.into()))
                        .collect(),
                )))),
            ],
        }
    }
}

impl TryFrom<Message> for Complex {
    type Error = AbsorbError;

    fn try_from(m: Message) -> Result<Self, Self::Error> {
        let mut fields = m.fields.into_iter();
        if fields.len() != 3 {
            return Err(AbsorbError::invalid_length(3, fields.len()));
        }

        Ok(Complex {
            optional_enum: fields
                .next()
                .unwrap()
                .map(|v| match v {
                    Value::Enum(Rule::Singular(v)) => ComplexEnum::new(v.number)
                        .ok_or_else(|| AbsorbError::invalid_enum("ComplexEnum", &v)),
                    v => Err(AbsorbError::invalid_type("optional_enum", &v)),
                })
                .transpose()?,
            repeated_bytes: match fields.next().unwrap() {
                Some(Value::Bytes(Rule::Repeated(v))) => Ok(v),
                Some(v) => Err(AbsorbError::invalid_type("repeated_bytes", &v)),
                None => Err(AbsorbError::not_optional("repeated_bytes")),
            }?,
            map_message: match fields.next().unwrap() {
                Some(Value::Message(Rule::Map(Key::I32(v)))) => {
                    v.into_iter().map(|(k, v)| Ok((k, v.try_into()?))).collect()
                }
                Some(v) => Err(AbsorbError::invalid_type("map_message", &v)),
                None => Err(AbsorbError::not_optional("map_message")),
            }?,
        })
    }
}
```

Heavy indeed, but iterators and match syntax really pull some weight. `AbsorbError` is a
[`thiserror`] enum with constructors to make the code a little cleaner.

#### The light conversion

A lighter approach, at least as far as the reflection part is concerned, would be to not do any
conversion at all! Instead, we could generate types that start in an already converted format.

How would that look? Every concrete message would be a struct with a single private `Message`
field. Field data would need to be accessed through special methods that can perform the
conversion between a `Value` and the type users want.

In true Rust fashion, each field would have a `mut` and non-`mut` pair of access methods.

- `field(&self) -> &Field`
- `field_mut(&mut self) -> &mut Field`

For messages defined with "proto2" syntax, each field would have a couple of additional methods for
dealing with nullability:

- `has_field(&self) -> bool`
- `clear_field(&mut self)`

Now, here would be the part where I show you a clean, simple conversion. Except, there is a little
trouble with regard to validation. `Message`s and `Enum`s require a costly conversion just to be
validated and subsequently discarded. Take for example, the `map_message` field:

```rust
match &m.fields[2].get_or_insert(Value::Message(Rule::Map(Key::I32(HashMap::new())))) {
    Value::Message(Rule::Map(Key::I32(v))) => v
        .values()
        .map(|v| ComplexNested::try_from(v.clone()))
        .collect::<Result<_, _>>(),
    v => return Err(AbsorbError::invalid_type("map_message", v)),
};
```

The problem is `TryFrom` does not borrow. So, we need to `clone` every single value element in the
map.  To avoid the `clone`, messages could implement an additional [`AsRef`] conversion, but the
documentation does not recommend using it for conversions that are costly or can fail. Ultimately,
the message just needs to be validated, so let's make a `validate` method that takes a reference to
a `Message` and spits out an `AbsorbError` if something goes wrong.

```rust
#[repr(transparent)]
pub struct Complex {
    inner: Message,
}

impl Complex {
    fn validate(m: &Message) -> Option<AbsorbError> {
        if m.fields.len() != 3 {
            return Some(AbsorbError::invalid_length(3, m.fields.len()));
        }

        match &m.fields[0] {
            Some(Value::Enum(Rule::Singular(v))) => ComplexEnum::validate(v),
            Some(v) => Some(AbsorbError::invalid_type("optional_enum", v)),
            None => None,
        }?;

        match &m.fields[1] {
            Some(Value::Bytes(Rule::Repeated(_))) => None,
            Some(v) => Some(AbsorbError::invalid_type("repeated_bytes", v)),
            None => Some(AbsorbError::not_optional("repeated_bytes")),
        }?;

        match &m.fields[2] {
            Some(Value::Message(Rule::Map(Key::I32(v)))) => {
                v.values().find_map(ComplexNested::validate)
            }
            Some(v) => Some(AbsorbError::invalid_type("map_message", v)),
            None => Some(AbsorbError::not_optional("map_message")),
        }
    }
}

impl TryFrom<Message> for Complex {
    type Error = AbsorbError;

    fn try_from(m: Message) -> Result<Self, Self::Error> {
        if let Some(err) = Self::validate(&m) {
            return Err(err);
        }

        Ok(Complex { inner: m })
    }
}
```

All that's left is to make access methods. This is mostly just a ton of boilerplate (feel free
to take a look at the implementation
[here](https://github.com/ajguerrer/tobu-conversion/blob/main/src/light/complex.rs)), with the
exception of `Message` and `Enum` fields which need to be represented as concrete types.

```rust
#[repr(transparent)]
pub struct Complex {
    inner: Message,
}

#[repr(i32)]
pub enum ComplexEnum {
    One = 1,
    Two = 2,
    Ten = 10,
}

#[repr(transparent)]
pub struct Enum {
    pub number: i32,
}

impl Complex {
    pub fn optional_enum(&self) -> ComplexEnum {
        match &self.inner.fields[0] {
            Some(Value::Enum(Rule::Singular(v))) => unsafe {
                // Safety: ComplexEnum is repr(i32) and
                // Enum is a repr(transparent) wrapper around i32
                *(v as *const Enum as *const ComplexEnum)
            },
            Some(_) => unreachable!(),
            None => ComplexEnum::default(),
        }
    }

    pub fn map_message(&self) -> &HashMap<i32, ComplexNested> {
        match &self.inner.fields[2] {
            Some(Value::Message(Rule::Map(Key::I32(v)))) => unsafe {
                // Safety: ComplexNested is a repr(transparent) wrapper around a Message
                &*(v as *const HashMap<i32, Message> as *const HashMap<i32, ComplexNested>)
            },
            _ => unreachable!(),
        }
    }

}
```

Yep, `unsafe`! As noted in the comment, Rust has some ability to control the layout of types. Here,
concrete `Messages` and `Enums` are [`newtype`]s that can take advantage of `repr(transparent)`.
This will guarantee the newtype and the type it wraps have the same representation. Therefore, this
bit of `unsafe` code is sound. In fact, David Tolnay, a person much smarter than myself, has a crate
for that called [`ref-cast`]. 

#### Comparing heavy and light

And now for the results! To benchmark, I used the popular crate [`criterion`]. I won't go into
detail about the implementation of the benchmark because it's fairly dry and the criterion API is
quite simple. Feel free to take a look at the benches
[here](https://github.com/ajguerrer/tobu-conversion/tree/main/benches). I split the benchmarks into
groups so that heavy and light implementations can be side-by-side compared based on 5 categories,
per message:

- `new`: Construct a default concrete message.
- `access`: Access all the fields of the message without mutation.
- `mutate`: Mutate all the fields of the message.
- `reflect`: Reflect a non-empty message. See
  [implementation](https://github.com/ajguerrer/tobu-conversion/tree/main/benches) for more details.
- `absorb`: Absorb a reflection of a non-empty message. See
  [implementation](https://github.com/ajguerrer/tobu-conversion/tree/main/benches) for more details.

```
test complex/new/heavy ... bench:          23 ns/iter (+/- 6)
test complex/new/light ... bench:          67 ns/iter (+/- 8)

test complex/access/heavy ... bench:           5 ns/iter (+/- 1)
test complex/access/light ... bench:          14 ns/iter (+/- 4)

test complex/mutate/heavy ... bench:         338 ns/iter (+/- 23)
test complex/mutate/light ... bench:         458 ns/iter (+/- 22)

test complex/reflect/heavy ... bench:         392 ns/iter (+/- 61)
test complex/reflect/light ... bench:          14 ns/iter (+/- 1)

test complex/absorb/heavy ... bench:         462 ns/iter (+/- 120)
test complex/absorb/light ... bench:          23 ns/iter (+/- 51)

test simple/new/heavy ... bench:           2 ns/iter (+/- 0)
test simple/new/light ... bench:          38 ns/iter (+/- 1)

test simple/access/heavy ... bench:           1 ns/iter (+/- 0)
test simple/access/light ... bench:           3 ns/iter (+/- 1)

test simple/mutate/heavy ... bench:           0 ns/iter (+/- 0)
test simple/mutate/light ... bench:          49 ns/iter (+/- 16)

test simple/reflect/heavy ... bench:          39 ns/iter (+/- 28)
test simple/reflect/light ... bench:          14 ns/iter (+/- 2)

test simple/absorb/heavy ... bench:          62 ns/iter (+/- 7)
test simple/absorb/light ... bench:          21 ns/iter (+/- 1)
```

It comes as no surprise that there is no clear winner here. It'a trade-off between `new`, `access`,
`mutate` and `reflect`, `absorb`. Light makes the reflection API seemingly constant-time with
impressive performance gains, especially for messages with increasingly large amounts of data. But,
oof! It really does pay for it by adding non-trivial overhead to what I anticipate to be the far
more commonly used methods.

[`prost`]: https://github.com/danburkert/prost
[`tonic`]: https://github.com/hyperium/tonic
[`protobuf`]: https://github.com/protocolbuffers/protobuf
[`grpc`]: https://github.com/grpc/grpc
[`serde`]: https://github.com/serde-rs/serde
[`transcoding`]: https://cloud.google.com/endpoints/docs/grpc/transcoding
[`The Rust Book`]: https://doc.rust-lang.org/book/
[`describe`]: https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/descriptor.proto
[`protobuf-go`]: https://github.com/protocolbuffers/protobuf-go
[`sync::OnceCell<T>`]: https://docs.rs/once_cell
[`Offset`]: https://golang.org/pkg/reflect/#StructField
[`Value`]: https://golang.org/pkg/reflect/#Value
[`mem::transmute`]: https://doc.rust-lang.org/std/mem/fn.transmute.html
[`nomicon`]: https://doc.rust-lang.org/nomicon/transmutes.html
[`pin`]: https://doc.rust-lang.org/std/pin/
[`Boats`]: https://without.boats/
[`Jon`]: https://www.youtube.com/c/JonGjengset
[`async-book`]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[`rel-ptr`]: https://github.com/RustyYato/rel-ptr
[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`PhantomData`]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
[`thiserror`]: https://docs.rs/thiserror
[`AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
[`ref-cast`]: https://github.com/dtolnay/ref-cast
[`newtype`]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
[`criterion`]: https://github.com/bheisler/criterion.rs

# Reelang

## Types

There are only 11 types. With their array equivalant

Unsigned

- u8
- u16
- u32
- u64

Signed

- i8
- i16
- i32
- i64

Floats

- f32
- f64

Extra

- boolean

## Operators

There are all basic arithmatic operators for all number types

### Arithmetic

- Add `+`
- Sub `-`
- Mul `*`
- Div `/`
- Exponent `**`
- Modulus `%`

### Assign

- Assign `=`
- AddAssign `+=`
- SubAssign `-=`
- MulAssign `*=`
- DivAssign `/=`
- ExponentAssign `**=`
- ModulusAssign `%=`

### Comparison

- Equal `==`
- Not Equal `!=`
- Greater Than `>`
- Lower Than `<`
- Greater Equal `>=`
- Lower Equal `<=`

### Logical

- And `&&`
- Or `||`
- Not `!`

### Bitwise

- And `&`
- Or `|`
- Not `~`
- Xor `^`
- Lshift `<<`
- Rshift `>>`
- URshift `>>>`

## Declarations

### Function

```rust
fn add(a: u8, b: u8): u8 {
  a + b
}
```

### Constants

```ts
const name: u8 = 5;
const array: u8[2] = [1, 2];
```

### Variable

```ts
let name: u8 = 5;
let array: u8[10];
let array2: u8[2] = [1, 2];
```

## Control Flow

### While

```ts
while (boolean) {
  break;
}
```

### Do-while

```ts
do {
  break;
} while (boolean);
```

### For

```ts
for (let i: u8 = 0; i < 10; i += 1) {
  break;
}
```

### If

```ts
if (condition) {
}
```

### If else

```ts
if (condition) {
} else {
}
```

### If else if else

```ts
if (condition) {
} else if (condition) {
} else {
}
```

### Match

```rust
match name {
  8 => true,
  _ => false,
}
```

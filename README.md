> Funksiyaga biror o'zgaruvchi (`mut` qilingan) berilayotgan bo'lsa, \
> `&` qilib havola berilganda, pointer ga ham `mut` berish kerak. \
> Sababi `&variable` pointerlar ham default o'zgarmas bo'ladi. \
> Shuning uchun `&mut variable` qilib beriladi

---

> Rust da funksiya `fn` default holatda **expression** qaytaradi
> ```rust
> fn add_two(x: isize) -> isize {
>     x + 2
> }
> ```
> shunda `x + 2;` dan keyin `;` qo'yadigan bo'lsak, u holda **statement** \
> ga aylanib qoladi va hech nima qaytarmaydi, bu holda xatolik bo'ladi \
> **Statement** - ba'zi amallarni bajaradigan va qiymat qaytarmaydigan ko'rsatmalardir.

---

> ```rust
> let x = if (2 + 3) == 5 { 5 } else { 3 };
> ```
> Python dagi **ternary operator** ga o'xsharkan


---

## loop

```rust
fn main() {
    let mut hisoblagich = 0;

    let natija = loop {
        hisoblagich += 1;

        if hisoblagich == 10 {
            break hisoblagich * 2;
        }
    };

    println!("Natija: {natija}");
}
```

> `break <qiymat>` - kalit so'zdan keyin qiymat berilsa, uni qaytaradi

```rust
fn main() {
    let mut hisob = 0;
    'hisoblash: loop {
        println!("hisob = {hisob}");
        let mut qolgan = 10;

        'inner_loop: loop {
            println!("qolgan = {qolgan}");
            if qolgan == 9 {
                break 'inner_loop;
            }
            if hisob == 2 {
                break 'hisoblash;
            }
            qolgan -= 1;
        }

        hisob += 1;
    }
    println!("Yakuniy hisob = {hisob}");
}
```

> `loop` larni nomlashimiz va ularda ixtiyoriy holat chiqishimiz mn break bilan

> break orqali loop nomini yozib unda chiqishimiz va bir vaqtda qiymat ham qaytarishimiz mn
> ```rust
> break 'loop_name <returning_value>;
> ```

---

### range

> `1..5` - [1:5) \
> `1..=5` - [1:5] ya'ni 5 ham kiradi \
> `(1..5).rev()` - (5:1] *teskari tartib* \
> `(1..=5).rev()` - [5:1] *teskari tartib*, 5 ham kiradi

---

## Stack, Heap

> **Stack** - da qat'iy malumotlar, ya'ni aniq o'lchamdagi, qat'iy belgilangan hajmdagi ma'lumotlar saqlanadi.

> **Heap** - noma'lum o'lchamli yoki o'zgarishi mumkin bo'lgan o'lchamdagi ma'lumotlar saqlanadi

---

## String

> Rustda **String** static `str` dan farqli ravishda o'zgarib turadi
> ```rust
> pub struct String {
>     vec: Vec<u8>,
> }
> ```
> - Ya'ni unda `value` qiymat
> - `len` - uzunlik
> - `capacity` - sig'im
> - `ptr` - heap xotiraga ko'rsatkich (pointer)

Misol ko'rib chiqaylik:

```rust
let x = 5;
let y = x;
println!("x = {} va y = {}", x, y);
```

Natija:

```text
x = 5 va y = 5
```

Ya'ni **Stack** dagi xotira copy qilindi.

> Agar biz **Heap** xotirali Reference Type ga (String misol uchun) murojaat qiladigan bo'lsak

```rust
let x = String::from("Hello, World");
let y = x;

print!("x={x} va y={y}", x = x, y = y);
```

Natija:

```text
5 |     let x = String::from("Hello, World");
  |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
6 |     let y = x;
  |             - value moved here
7 |
8 |     print!("x={x} va y={y}", x=x, y=y);
  |                                ^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args` which comes from the expansion of the macro `print` 
  (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
6 |     let y = x.clone();
  |              ++++++++
```

Bu holatda `let y = x;` qilinganda, stack dagi ma'lumotlar nusxalanadi, lekin `ptr` yani
heap xotiraga bo'lgan murojaat ham nusxalanadi va bitta heap xotiraga murojaat bo'p qoladi
lekin (deepcopy bo'lmaganligi uchun) _heap xotirani o'zi nusxalanmaydi_.
Rust da default holatda _deepcopy_ qilmaydi, shuning uchun, har qanday avtomatik nusxa ko'chirish
runtimening ishlashi nuqtai nazaridan arzon deb taxmin qilish mumkin

Note: funksiyaga ham reference type dagi o'zgaruvchini bersak, ownerligini ham berib yuborgan bo'lamiz.
Buning oldini olish uchun (ya'ni Ownership likni bermasdan turib undan foydalanmoqchi bo'lsak)
`&` orqali faqat pointerini berib ishlatishimiz kk. Bu holatda biz faqat qiymati bilan ishlaymiz.
Ownerlik esa o'zida qoladi.

#### Reference

> faqat bitta `mut` reference (havola yoki pointer) bo'lishi mn.

> xoxlagancha immutable (read only) reference qo'llashimiz mumkin.

> lekin mutable va immutable ni aralashtirib qo'llab bo'lmaydi

> References - har doim yaroqli bo'lishi kk. Bo'lmasam `dangle` holatiga tushadi

#### Dangling References

```rust
fn main() {
    let dangle_reference = dangle();
}

fn dangle() -> &String {
    let s = String::from("salom");
    &s
}
```

Natija:

```text
  | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
8 | fn dangle() -> &'static String {
  |                 +++++++
```

Ya'ni tagi yo'q xotiraga pointer qilishni oldini oladi (kompilyator)

---

## Slice

> Slice lar bu aniq xotiraga ega bo'lmagan (`&` yani ptr), array (yoki Vector) larning qismlar (yoki to'liq holatini)ini
> ifodalaydi. \

```rust
let s = "Hello World";
let my_slice = & s[0..5]; // Hello
```

> slice dagi oraliqni ko'rinishlari
> - `[0..5]` - [0; 5)
> - `[..5]` - [0; 5) bu ham yuqoridagiga ekvivalent
> - `[2..]` - [2; last) 2 dan array (vec, str, String, ..) ni oxirigacha
> - `[..]` - [first; last] boshidan oxirigacha yani toliq holatda boglanadi

---

## Struct

E'lon qilish

```rust
struct User {
    is_active: bool,
    full_name: String,
    username: String,
    id: usize,
}
```

Object olish

```rust
fn main() {
    let user = User {
        is_active: true,
        full_name: String::from("Solijonov Otabek Solijon o'g'li"),
        username: String::from("mrsolijonov"),
        id: 1,
    };
}
```

Object oladigan funksiya

```rust
fn new_user(full_name: String, username: String) -> User {
    User {
        is_active: true,
        full_name: full_name,
        username: username,
        id: 1,
    }
}
```

> > Attributlar bir xil nomli bolganda **Shorthand** dan foydalanishimiz mumkin
>
> ```rust
> fn new_user(full_name: String, username: String) -> User {
>     User {
>         is_active: true,
>         full_name,
>         username,
>         id: 1,
>     }
> }
> ```

> Struct update

```rust
fn main() {
    let user1 = User {
        is_active: true,
        full_name: String::from("Solijonov Otabek Solijon o'g'li"),
        username: String::from("mrsolijonov"),
        id: 1,
    };

    let user2 = User {
        is_active: user1.is_active,
        full_name: String::from("Askarov Abror"),
        username: user1.username, // moved [can't use user1.username]
        id: user1.id,
    };

    let user3 = User {
        full_name: String::from("Mamatov Rashid og'a"),
        ..user2  // moved [also can't use user2.username]
        // id and is_active are also moved, but they are in stack memory not heap
    };
}
```

### Tuple Struct

> Tuple larni nomlab bir biridan ajratish uchun foydalaniladi.

```rust
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let red = Color(255, 0, 0);
    let begin_at = Point(2, 6, 3); // not the same with Color {}

    println!("{}", red.0)  // call by index
}
```

Note: Ikkita bir xil tuple ham bir biriga teng bo'lmayda (boshqa nomda chunki)


---

### Non field Struct

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

> Bunda hech qanday ma'lumotlar saqlanmaydi lekin, `trait` lardan foydalanish mumkin

---

### `impl` - Implementatsiya

> Bu struct yoki trait ga tegish bo'lgan funksiyalar to'plami

```rust
struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn new(height: i32, width: i32) -> Self {
        // Constructor
        // ::new() - because self not called
        Self {
            height,
            width,
        }
    }
    fn area(self: &Self) -> i32 {
        self.height * self.width
    }

    fn perimeter(&self) -> i32 {
        2 * (self.height + self.width)
    }

    fn can_catch(&self, another_rectangle: &Rectangle) -> bool {
        self.height >= another_rectangle.height && self.width >= another_rectangle.width ||
            self.height >= another_rectangle.width && self.width >= another_rectangle.height
    }
}

fn main() {
    let rec1 = Rectangle {
        height: 15,
        width: 10,
    };

    let rec2 = Rectangle::new(10, 14);
    println!("{}", rec1.can_catch(&rec2));
}
```

> method larga `.` nuqta orqali murojaat qilish mumkin

> `&self` - bu `self: &Self` ning qisqartmasi

> [!NOTE] <br> o'qish (`&self`), o'zgartirish (`&mut self`) yoki iste'mol qilish (`self`)

#### Associate Functions (Bog'langan funksiyalar)

> `self` siz ishlatilgan (asosan konstruktorlarda) funksiyalar `::` orqali chaqiriladi





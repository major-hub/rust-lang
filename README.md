--- 

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
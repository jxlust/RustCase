## 整数除法

- usize 取用的都是向下取整
- 负数 是向上。
- 综合来说：貌似是向零取整

```js
 11 / 4 = 2
 -7 / 8 = 0
```

- 无符号整数溢出问题

```rust
let a:usize = 1;
// "a - 9" is error overflow
println!("{}",(a.wrapping_sub(9)));// 1 - 9
```

- 负数对正数取模

```rust
// (-7) % 8
println!("{}",(-7) % 8 )// -7
```

- 无符号溢出取整问题

```rust
let a:usize = 1;
let mod_a = (a.wrapping_sub(2)) % 8; // 7
```
>这里说明一下，因为无符号溢出了就是由负数的补码存储。
>计算机负数也都是存的补码形式，可以参考网站或者我的[二进制相关知识](https://github.com/jxlust/WebHtml/blob/main/%E7%B3%BB%E7%BB%9F%E6%93%8D%E4%BD%9C/%E4%BA%8C%E8%BF%9B%E5%88%B6%E7%9B%B8%E5%85%B3%E7%9F%A5%E8%AF%86.md)
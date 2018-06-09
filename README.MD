#Day 20: Linearithmic multiplication
<br>
10 days ago I used Karatsuba to multiply two numbers. Another possibility is to use Fourier transform.
<br>
While convolution in time domain takes O(n²) operations, it can be done in O(n) operations as point-wise multiplication in frequency domain. Conversions between time and frequency take O(n.log(n)) operations which is also the final complexity.
<br>
The very same idea is successfully used in computer graphics when you need to apply large 2D kernels on large images.
<br>

Result:
```

fn main() {
    let result = mult("987324", "23487");
    assert_eq!("23189278788", &result);
}

```

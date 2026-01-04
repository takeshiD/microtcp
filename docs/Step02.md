これはインターフェースとしてopen,close,outputのメソッドを持つことを約束するということなので、Rustではtraitで実装すれば十分です。

```rust
pub trait NetDeviceOperation {
    fn open(&mut self) -> Result<u32, String>;
    fn close(&mut self) -> Result<u32, String>;
    fn output(&self, ty: NetDeviceType, data: &[u8], dst: &[u8]) -> Result<u32, String>;
}
```

これにともなって`NetDeviceArray`も少し修正します。NetDeviceのnameメンバを直接使っていたりするので先程のtraitに加えます。

```diff
pub trait NetDeviceOperation {
    fn open(&mut self) -> Result<u32, String>;
    fn close(&mut self) -> Result<u32, String>;
    fn output(&self, ty: NetDeviceType, data: &[u8], dst: &[u8]) -> Result<u32, String>;
+   fn with_name(&self, name: impl AsRef<str>) -> Self;
+   fn name(&self) -> &[u8];
}
```

これでNetDevice、はたまた別のデバイスであってもNetDeviceOperationトレイトを実装すれば問題ありません。

書籍では、つまりC言語では関数ポインタを工夫してインターフェースを定義していますが、Rustでは型安全かつ実装少なく定義出来ることに驚きます。

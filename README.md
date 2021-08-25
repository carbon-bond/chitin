# 幾丁

幾丁是[碳鍵](https://github.com/carbon-bond/carbonbond)論壇的 RPC 框架。

以 Rust 撰寫靜態型別的 API ，自動生成 TypeScript 對應的請求／響應程式碼，讓編譯器在編譯期爲你檢查型別，媽媽再也不擔心你的 payload 送錯啦！

## 起源

[碳鍵](https://github.com/carbon-bond/carbonbond)的 API 定義經歷了幾個階段：

1. 刀耕火種時期的口頭／文件約定
2. 嘗試 GraphQL/protobuf/Swagger ，使用這些工具的定義文件 *.gql, *.proto 定義 API ，再由這些文件生成 Rust/TypeScript 程式碼。

第一階段的口頭／文件約定有非常顯著的缺點
- 這些約定沒有強制力，能否正確實作只能仰賴開發者的細心。
- Rust 跟 TypeScript 這兩端的型別定義往往十分相似，但仍要各寫一次

於是我們嘗試遷移向第二時期，使用 protobuf/graphql/Swagger 來生成 API ，然而這些框架都不能完全避開下列缺點。
- 對 TypeScript/Rust 的支援並不友好（時爲 2020），會生成冗餘、不易閱讀的程式碼
- 不支援在 Rust/TypeScript 中廣泛使用的 sum type

舉例來說：
TODO: 補個 protobuf 的例子

於是[碳鍵論壇](https://github.com/carbon-bond)決定自己搞一個。

## 示例
```sh
./examples/codegen.sh server
./examples/codegen.sh client
```
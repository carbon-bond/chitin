# 幾丁

幾丁是[碳鍵](https://github.com/carbon-bond/carbonbond)論壇的 RPC 框架。

以 Rust 撰寫靜態型別的 API ，自動生成 TypeScript 對應的請求／響應程式碼，讓編譯器在編譯期爲你檢查型別，媽媽再也不擔心你的 payload 送錯啦！

## 示例
```sh
./examples/codegen.sh server
./examples/codegen.sh client
```

## 源碼結構
```
.
├── chitin         # 導出下面三個模組的外曝介面
├── chitin-core    # 由 ChitinEntry 生成出 Rust 的 trait 以及 TypeScript 的虛類
├── chitin-derive  # 由 Rust 寫的 API 定義解析出 ChitinEntry
├── chitin-model   # 由 Rust 寫的結構／enum 生成 TypeScript 的對應型別
└── examples       # 範例
```

## 文件

[碳鍵文件](./doc/../文件.md)
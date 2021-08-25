本範例示範如何以幾丁質創建簡單的應用程式

## 執行

### 啓動伺服器
```
cargo run
```

### 啓動客戶端
```
cd client
yarn
yarn start      # 看到提示字元後，可打 help 來看有哪些指令
```

## 源碼結構
```
.
├── build.rs          # cargo 在編譯源碼前會先執行該腳本，codegen 指令皆位於此
├── client
│   ├── api_trait.ts  # 生成的 typescript 程式碼
│   ├── api.ts        # 實作 api_trait.ts 中的抽象類
│   ├── index.ts      # 客戶端入口，定義命令行指令
└── src
    ├── api_trait.rs  # 生成的 rust 程式碼
    ├── api.rs        # 實作 api_trait.rs 中的 trait
    ├── main.rs       # 主程式入口，包含 hyper 路由定義
    ├── model.rs      # 在 API 中會用到的結構
    └── query.rs      # 幾丁質路由定義
```
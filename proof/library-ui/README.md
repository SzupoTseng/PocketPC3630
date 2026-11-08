# Web 圖書館 UI 首版實作與驗證

日期：2026-11-08。圖書館與個人程式庫新增介紹／操作入口、操作需求篩選、七種方案介紹預覽。指定目錄項提供操作與保存說明；未知軟體明示資料待補。推薦不代表指定平台／版本操作已通過，目錄版本不會自動套到本機安裝。

已匯入的軟體可套用六種 Web 按鍵盤／螢幕鍵盤顯示方案，自訂目前是選擇顯示按鍵。每套配置在各自安裝目錄 `web-controls.json`，重載保留、可恢復預設；不修改遊戲進度、`game.json`、全域設定，也不包含在進度 ZIP。遊玩中亦可開設定，但設定視窗不會暫停客體。

Device Emulator 面板只提供示意與限制介紹，套用停用。任意按鍵重映射、手把、相對滑鼠、IME、硬體鍵路由及 Rust 共用控制模型仍未實作；Windows／Android 原生介面沒有因這次 Web 改動而更新。

## 實際檢查

- Node：10 項測試通過，涵蓋配置驗證、同步佇列等待／失敗恢復、多來源持鍵與延遲釋放清理，及既有音訊／啟動工具回歸。
- 真正 Windows Chrome + 專用 profile + 現有 WASM：圖書館搜尋、操作需求篩選、七種預覽不送客體按鍵、原生對話框關閉與焦點、兩套程式配置隔離、自訂按鍵、重設、重載、寫入失敗回復、目錄載入失敗／重試、繁中／英文。
- 設定前後比較實際匯入進度及 `game.json` 的 SHA-256，保持一致。
- 真正建立／結束同一軟體的 Web session 十次，逐次送實體鍵盤事件：共十次 Down／Up，無殘留 pressed keys、延遲 KeyUp 或定期保存計時器。
- 390×844 手機與 844×390 橫向尺寸以 Chrome emulation 檢查對話框無橫向溢出，並人工查看截圖。此項不是手機實機、手指觸控或螢幕閱讀器驗收。
- EQ2 的世界畫面、正常退出與頁面重載讀回另外執行實際瀏覽器回歸；結果與音訊回呼資料見彙整 JSON。不據此宣稱完整戰鬥／任務或其他軟體操作已驗收。

彙整與產物雜湊：[library-ui-verification.json](../../tools/eq-validation/library-ui-verification.json)。原始測試庫、進度、音訊及瀏覽器 profile 保留在 ignored `target/eq-validation`，未公開部署、未操作 Git。

## 畫面

- [圖書館卡片與操作入口](desktop-library.png)
- [軟體介紹及保存說明](desktop-introduction.png)
- [裝置面板介紹預覽](device-panel-preview.png)
- [手機操作設定](mobile-controls.png)
- [橫向操作設定](landscape-controls.png)

預覽中的「程式畫面區」是明確標示的示意，不是真實客體截圖；裝置按鈕位置與鍵碼未取得完整參照，不能作為原裝置行為證據。

## 重跑

```bash
node --test tools/eq-validation/*.test.mjs
node tools/eq-validation/library-ui-browser.mjs docs CHROME_PATH ISOLATED_PROFILE OUTPUT_DIR EP1_PACKAGE EP2_PACKAGE EP2_PROGRESS
```

瀏覽器工具使用自己的 loopback port 8183、CDP 9341 及新建 profile，測試後關閉自己啟動的 Chrome。素材須使用本機持有副本。核心遊玩回歸使用既有 `browser-smoke.mjs`；兩者的原始報告均以實際產物雜湊核對。

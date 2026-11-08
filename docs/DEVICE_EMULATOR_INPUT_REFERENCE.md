# Device Emulator 輸入參照

2026-11-08：使用既有使用者持有的 PPC2003 ROM，另開隔離的冷開機實例、共享目錄及狀態檔。未載入既有 EQ、序號或 saved state。此參照不是 iPAQ 3630 實機。

客體回報 CE 4.21 build 1088；skin 為 `Pocket_PC/PPC_2003_SE.xml`，358×557，顯示區 `(61,85,240,320)`。ROM、模擬器、XML、圖片及自製 probe 的 SHA-256，與每筆客體訊息保存在 [量測資料](../tools/wince-input-test/reference/ppc2003-input-observations.json)。只保存功能量測及雜湊，不散布 Microsoft 映像或 skin 圖片。

## 單次按鍵結果

每個按鈕一次，主機按住 150 ms 後放開。probe 收到以下 `WM_KEYDOWN`／`WM_KEYUP` 配對：

| Skin 按鈕 | Skin scan code | 客體 VK | XML 策略 |
| --- | --- | --- | --- |
| record | 0x44 | F10，0x79 | hold |
| rocker-up／down | 0x48／0x50 | Up／Down，0x26／0x28 | hold |
| softkey 1／2 | 0x3B／0x3C | F1／F2，0x70／0x71 | click |
| softkey 3／4 | 0x42／0x43 | F8／F9，0x77／0x78 | click |
| dpad-up／down | 0x48／0x50 | Up／Down，0x26／0x28 | hold |
| dpad-left／right | 0x4B／0x4D | Left／Right，0x25／0x27 | hold |
| dpad-action | 0x1C | Enter，0x0D | click |

這些是客體實際收到的鍵碼，不能直接把 skin scan code 當 VK。尤其這份 skin 的四個快捷鍵不是 CE App1～4。既有通用 CE Action／App1～6 方案應保留獨立用途；不能藉這組結果全域改寫 GAPI 或其他軟體鍵碼。

## 三秒長按

每種一次，最後明確送出主機放開：

- softkey 1：一組 F1 down/up，客體 tick 相差 1 ms；不是三秒客體持續按住。
- dpad-action：一組 Enter down/up，客體 tick 相差 1 ms。
- dpad-up：22 筆 down（含重複）及一筆 up；首次重複約 375 ms，後續 down 的 lParam previous-state bit 已設置。
- record：一組 F10 down/up，客體 tick 相差約 2,903 ms，未觀察到重複。

主機 hold 時長不等於客體 down/up 間隔；產品面板需要按鈕各自的 click／hold 行為。這四次量測尚不能建立完整重複頻率、負載或取消行為保證。

## 未完成的比對

單次階段 App1～6 已關聯主視窗 A，呼叫回傳成功；但本 skin 送出的都是方向／F 鍵，因此**不能用這批事件驗證 `SHSetAppKeyWndAssoc` 路由**。成功呼叫後的舊 GetLastError 值也不是失敗證據。PocketHLE 該 API 目前仍為成功 stub，不能標示已支援。

尚待：A／B 視窗 App 鍵路由、GAPI 查詢前後、20 次重複、滑出／失焦／背景取消、多來源合併、三平台同 probe 比對及真實軟體回歸。Power 未測，產品 Power／Reset 未提供。完整 DE-01～08 維持未驗收。

probe 的建置及執行限制見 [README](../tools/wince-input-test/README.md)。CLI 僅完成 probe 啟動和明確輸出目錄測試，沒有冒充硬體鍵比對通過。

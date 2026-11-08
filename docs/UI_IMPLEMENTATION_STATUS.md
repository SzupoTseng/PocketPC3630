# 三平台 UI 實作與驗收狀態

更新：2026-11-08。**新版尚未完成完整驗收，不能標示「所有 UI 已完成」。** 本文件與 `ui_TODO.md` 分開記錄已寫程式、已建置及實際遊戲測試，避免把原始碼或舊版本畫面當作新版驗收結果。

## 已加入原始碼

| 範圍 | 共用／Windows | Web | Android |
| --- | --- | --- | --- |
| 操作配置 | Rust version 1；產品→全域→每軟體；舊 Web 配置讀取；明確重設 | Rust 匯出 API；序列化整筆寫入／IDBFS／失敗回復 | JNI 共用配置；原生設定頁 |
| 七種方案 | 觸控筆、方向、完整 Pocket PC、文字、桌面、自訂、通用 CE 硬體鍵 | 七種可預覽／套用 | 七種可選；直橫向布局拖曳 |
| 按鍵／手把 | 可見鍵、原始 CE VK、鍵盤重映射；Windows XInput；各來源合併放鍵 | 標準 Gamepad API、鍵盤／觸控／手把獨立來源 | 裝置 ID／按鍵／軸分開追蹤；移除裝置放鍵 |
| 顯示 | 透明度、位置／尺寸、直橫向布局、放大與客體焦點 X/Y | 同組配置；縮放／旋轉後逆向觸控 | GL UV 放大／焦點與觸控使用同組座標；最小 44 dp |
| 遊戲中選單 | 暫停確認、繼續、配置、音量、顯示、文字、備份與返回；可改鍵盤快捷鍵 | 固定觸控入口；設定／失焦／背景原因合併 | Back 先開選單；全螢幕浮動入口；等待核心確認暫停 |
| 暫停 | runner 阻止客體執行；共用 guest clock；音訊 transport 凍結 PCM／voice 游標 | 同步停止下一個 slice | JNI 回報實際暫停狀態；背景／失焦政策 |
| 文字 | 獨立 UTF-16→WM_CHAR；Edit surrogate pair；主機文字不經遊戲鍵綁定 | 主機 IME 文字框、確認送入、客體退格 | 系統 IME、確認送入、客體退格 |
| 更新進度保護 | CAB／EXE／ZIP／RAR 共用替換交易；保留 saves、registry、操作、別名與舊 EQ 檔案 | 使用同一核心 | 使用同一核心 |
| 個人遊戲庫 | 搜尋、最近遊玩、別名／收藏、有效方案／繼承來源 | 同左；索引以安裝 id 驗證，容許點號 | 背景讀取、搜尋／排序、別名／收藏、避免重複啟動 |
| 公開圖書館 | 1,203 筆既有目錄、需求篩選、介紹、保存說明、七方案預覽；明確選本機軟體才開草稿 | 同一份推薦／步驟資料；目錄版本不自動綁定 | 原生目錄、搜尋／篩選、介紹、預覽、本機選擇 |
| 診斷與語意 | FPS／backend 預設不顯示；一般操作與進階診斷分層；備份不稱即時存檔 | FPS 可選；寫入失敗不返回／不報成功 | 一般／進階設定分組；備份只含已寫進度 |

原生目錄沒有內嵌全部截圖，會明示畫面占位，提供使用者主動開啟既有目錄畫面的連結。這不是本次驗收截圖。需求建議是入門提示，未比對的本機版本保持未驗證。

Windows 手把目前限 XInput；Linux 桌面手把未提供。Windows／Web／Android 預設可用左搖桿按下開啟／關閉選單，明確綁定 PadL3 後保留該自訂用途；Android 另支援系統控制器選單鍵。Web 亦保留 Start＋Select。客體 Esc 只在使用者明確改作主機快捷鍵時才被攔截。

## 尚不能列為完成

- Windows、WASM、兩核心 APK 已完成本輪建置與雜湊綁定；完整軟體／控制器／機型矩陣仍未完成。
- Android 150 ms 正常短點的選角／Options／存檔退出已實測；不同時長的重複量測、拖曳及 CT／APP／LIB 完整矩陣仍待完成。
- 通用 CE Action／App1～6 可以送原始鍵碼，但 **不是指定 Device Emulator 機型／OS／skin 的驗證結果**。SHSetAppKeyWndAssoc 的 shell 關聯仍為既有 stub；已建立隔離的 PPC2003 CE 4.21 build 1088／Pocket PC 2003 SE skin 參照，12 個按鈕完成單次實測；DE-01～08、產品面板及完整硬體路由仍未完成。Power／Reset 不提供。
- 相對滑鼠、游標擷取、完整主機選單手把導覽及 ARM64 實機／輔助工具驗收尚未完成；沒有將設定欄位當成已支援能力。
- 客體即時存檔、倒帶與任意位置續玩仍是獨立核心專案；ZIP 備份不能代替。

## 目前驗證

Linux Node 可執行的 19 項測試通過：`library-ui.test.mjs` 8、`controls-host.test.mjs` 7、`smoke.test.mjs` 3、`web-audio.test.mjs` 1。涵蓋資料遷移與別名、配置繼承、寫入失敗／重疊交易回復、巢狀暫停原因、純預覽、九組旋轉／放大座標、音訊重開清理及測試工具契約。它們使用主機邏輯與部分替身，**不是遊戲／真實手把／實機通過**。

最新 workspace 為 612 passed／12 ignored／0 failed，Clippy 通過（`tests-present.log`／`clippy-present.log`）；包含診斷限制及完整畫面邊界／逾時更新測試。Windows release、WASM、Unicorn／Interpreter APK 已完成最終建置；原生完整畫面策略已包含在本輪實測產物。凍結站台 `target/ui-validation/web-limits-acceptance` 的真實 Chrome 驗證通過 35 項檢查；EP1／EP2／EP3 各自完成進世界、移動、遊戲內存檔與重載進度，音訊探針捕獲非零輸出。這些結果只對應報告內的產物雜湊，不能代替後續修改的驗證。

Windows 實際操作已確認全域繼承及 EP2 獨立覆寫，12 個進度檔的雜湊未因設定操作改變。Android 已確認新版設定介面與核心確認後的「Paused」選單。Windows 遊戲測試發現舊 `max_slices=50000000` 會在一般遊玩自行結束，因此新增預設關閉的 `apply_run_limits`，保留原有診斷數值；Windows／Android 共用同一決策。Windows 結束狀態與診斷明細分離，Web 選單補上橫向捲動及可見寫入錯誤。

Windows 新增 F12／選單截圖，直接讀渲染器像素。GPU 截圖已確認全螢幕暫停選單及 F11 返回；Windows GDI 視窗擷取曾留下舊畫面，不能據此判定選單沒有繪製。原生畫面另改成優先使用完整 frame/copy 邊界，100 ms fallback 保留沒有 EndDraw 的程式更新；最新版 Windows 已逐章確認 EP1／EP2／EP3 世界及暫停，包含全螢幕置中選單與選單內 F11 返回，見 [實測畫面](../proof/ui-shared/README.md)。Android 最終 Unicorn APK 已逐章完成 EP1／EP2／EP3 選角、世界、暫停確認、Options、遊戲內存檔退出及結束後返回圖書館；Interpreter APK 完成 EP2 同流程，之後恢復安裝 Unicorn。正常點擊為 150 ms，並等待實際畫面狀態。先前 40 ms 測試的藍色門檻設錯，結果列為無法判定，不能據此聲稱遊戲漏接短點。

新版 Web 已通過安全預檢並同步至本機 `docs/`（未推送／發布）。**完整軟體矩陣仍待完成；本輪 EQ 正常流程及新版畫面策略已有實測。** WSL 啟動 Windows 工具偶爾仍回報 `UtilBindVsockAnyPort: socket failed 1`，已成功啟動的工作以實際完成 log 為準。

## 本輪產物與參照

- Windows：`a7a6e5b42e650a8ead1829e031c064b99e8c2ad20b76e8a6f371c038610dbc21`，三章逐章通過；F12 預設存入圖書館 `screenshots/` 已實測。
- Android Unicorn：`cf2485c0176a5b2d16d976d4657dcdbbdac9f96477db421f6f8161681f32f3c7`；Interpreter：`95a000021c532e3405cf7eab66b0fba410845b98e6c960c491b198e60d889c04`。這是 x86_64 模擬器實測，不能代替 ARM64 實機。
- APK 包裝安全測試 4 項通過，涵蓋建置失敗後還原預設 APK 及拒絕過期 Kotlin／新 JNI 混包。
- 詳細產物與報告索引：[機器可讀驗證紀錄](../tools/eq-validation/ui-implementation-verification.json)。實際畫面：[畫面索引](../proof/ui-shared/README.md)。
- Device Emulator：獨立冷開機映像及共享目錄，未使用既有 EQ／序號資料；自製 [輸入 probe](../tools/wince-input-test/README.md) 量測 12 個按鈕。觀察到快捷鍵為 F1／F2／F8／F9，錄音 F10，方向為箭頭，中央為 Enter；它們不是 App1～6。詳見 [參照紀錄](DEVICE_EMULATOR_INPUT_REFERENCE.md)。

## 可重現檢查

在工作目錄的 Windows CMD 執行（不操作 Git）：

```bat
tools\eq-validation\check-ui.cmd
tools\eq-validation\build-ui-wasm.cmd
tools\eq-validation\build-ui-android.cmd
```

結果放 `target/ui-validation/`。建置 Web 會產生完整本機測試站台，**不自動覆寫 `docs/`**；目前 `docs/` 已由 35 項檢查及三章世界測試通過的凍結站台手動同步。新版瀏覽器驗證使用 `library-ui-browser.mjs`，需要指定隔離 profile 與使用者自己的素材／進度。驗證成功後才用 `prepare-pages.mjs` 同步；它會先檢查全部檔案與新 WASM exports，拒絕將新 UI 與舊核心混用。

XInput 的 ABI 與載入方式依 Microsoft 的 [XInputGetState](https://learn.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)、[XINPUT_GAMEPAD](https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad) 與 [版本說明](https://learn.microsoft.com/en-us/windows/win32/xinput/xinput-versions) 核對；這不代替實體控制器測試。

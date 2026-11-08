# EQ 模擬改善

## 2026-11-07：原版註冊畫面的版權與註冊商標符號

修正 `crates/pocket-kernel/src/font.rs` 的控制項文字繪製：© 與 ® 保留各自字形，不再退化成問號。UTF-8 與 UTF-16 路徑共用同一組字形，維持 6×8 字格，其他不支援字元仍使用原有問號替代。未變更遊戲註冊邏輯。

測試涵蓋兩個符號與問號的像素差異、兩條文字路徑一致性、UTF-16 NUL 結尾與字寬相符，以及其他字元的既有替代行為。

驗證：Windows Rust 工具鏈已完成兩個新增的字型回歸測試（2 passed），`rustfmt --check` 通過。`cargo test -p pocket-kernel --offline` 全部 121 個測試通過。尚未以原版 EQ 或 Device Emulator 重新比對畫面，亦未執行 157 程式回歸或跨平台建置。

後續工作：依 Device Emulator 參考逐項處理主機控制項 TrueType 字型、實際字寬換行，以及註冊對話框的全螢幕版面。


## 2026-11-07：靜態文字換行與重新建立實測基準

- 曾準備靜態文字長字串斷行變更；因尚未與真機行為比對，已撤回該變更及其測試，維持原有控制項行為。
- `tools/eq-validation/`：新增無輸入的 WASM 基準測試工具、工具自身測試、EP1 光碟來源紀錄與三部驗收清單。工具測試與 JS 語法檢查通過。
- 實跑既有 docs WASM：EP1 本機候選 CAB、EP1 光碟內 CAB、EP3 候選 CAB，皆成功匯入與啟動，20 秒後停在註冊畫面，無執行錯誤。已人工查看最終截圖。
- EP1 光碟 CAB 與本機候選 CAB 不同，驗收結果分開保留。EP2 因現有素材曾被標記為修改版，尚未執行。
- 此次基準不含目前 Rust 修改，不能作為修正後驗收。所有遊戲資料與截圖留在被忽略的 target/eq-validation/。
- 下一個驗證門檻：在可用 Rust 建置環境執行核心測試並重建 WASM，重跑相同素材，與原版 Device Emulator 比較；有自有註冊碼後才繼續遊玩及存讀檔。


## 2026-11-07：註冊互動測試與合成畫面計數

- 已實測既有 WASM：EP1 光碟版與 EP3 的空白註冊碼 OK 點擊，畫面保留在註冊對話框、按鈕取得焦點，沒有執行錯誤。這只能證明輸入抵達控制項，不能證明成功註冊或完整驗證錯誤碼訊息。
- EP1 本機候選版、EP1 光碟版與 EP3 的 Cancel 會正常退出，最後錯誤為空。工具已區分正常退出與崩潰。
- 測試工具新增可重跑、僅含點擊與截圖的時序情境，不接受鍵盤內容；報告記錄實際事件時間及載入的 EXE／DLL SHA-256。新情境解析測試與工具契約測試已通過。
- `KernelState::composite_controls` 的註解原本明定合成不得增加 frame_counter，但 `Surface::fill_rect` 仍會間接增加計數。此問題與註冊畫面靜止卻回報大量 presentationEvents 相符。已修改為保留進入時的計數，保留所有像素寫入；輸入與 guest 的新 dirty 標記仍可觀察。
- 已加入控制項及 modal-only 兩個回歸測試，檢查有繪出像素、重複合成不增加計數、guest 新畫面仍增加計數。這次核心修改及測試尚未編譯，不能宣稱效能改善已驗證。
- 建置限制再確認：目前無 Linux Rust 工具鏈；Windows interop 於沙箱內回報 socket failed；下載 Linux Rust 的 DNS 解析也失敗。尚未產生修正後 WASM。


## 2026-11-07：阻塞狀態複核

- 再次嘗試 Windows `cmd.exe /c ver`，仍於沙箱內回報 `UtilBindVsockAnyPort: socket failed 1`；Linux PATH 無 cargo／rustc。
- 現有 `pocket_kernel` 測試執行檔早於合成計數修改，不能用它證明新修改通過。
- 發現本機 `target/eq-validation/packages/ep2.zip`；靜態核對其 `EQ_E2.cab` 的 SHA-256 為 `d718ce2e…`，與先前標記的 `work/cabs/E2.cab` 完全一致，未增加獨立的原版來源證據。詳見 `tools/eq-validation/ep2-source.json`。
- 尚無新的自有註冊碼持有情況確認；註冊後遊玩、音效、存讀檔及 Windows／Android 驗收均未完成。
- 在可用 Rust 建置環境、EP2 原版來源與自有註冊條件補齊前，完整目標處於受阻狀態。修正、測試與三部驗收範圍全部保留，未縮減目標，未提交或發布。


## 2026-11-07：Windows 工具鏈重新確認可用

- 重新單獨呼叫 `cmd.exe /c ver` 及 Windows cargo，皆成功；cargo 版本為 1.97.1。先前 socket 錯誤不再是所有 Windows 呼叫的持續阻塞。
- 本次觀察：混合 Linux 準備指令與 Windows 執行的同一命令仍出現 socket 錯誤；將 Windows 呼叫獨立成一個命令後可用。這是可重現的呼叫方式差異，未證明底層根因。
- 最新原始碼 `cargo test -p pocket-kernel --offline`：123 個測試全部通過，包含控制項與 modal 合成計數兩個新增測試。
- 最新原始碼 `cargo fmt --all -- --check`：通過。
- WASM 重建正在執行；成品獨立放在 target/eq-validation/current-wasm，不覆蓋原有 docs 基準。
- EP2 來源與自有註冊碼確認仍待解決；不得將工具鏈恢復等同三部完整驗收已完成。


## 2026-11-07：修正後 WASM 實測

- WASM release 重建成功（3 分 10 秒），成品位於 target/eq-validation/current-wasm；Windows CLI release 也已建置成功（5 分 18 秒）。
- 同一份 EP1 光碟套件，20 秒呈現事件從 2,088 次降為 2 次；最終畫面與舊基準逐像素完全相同。這是閒置註冊畫面的呈現事件比較，不是遊玩 FPS 測試。
- 同一份 EP3 套件，20 秒呈現事件從 2,052 次降為 1 次；已查看新版截圖，© 與 ® 顯示各自字形，不再是問號。
- 新版 EP1 光碟版 Cancel 與 EP3 空白碼 OK 流程皆未出現執行錯誤；互動結果另行保留原始報告。
- pocket-kernel Clippy 一般 -D warnings 因既有未識別 lint 失敗；依相容方式加 -A unknown_lints 後通過。未宣稱全 workspace Clippy 已通過。
- 工具鏈阻礙已解除；下一步是 Device Emulator 參照比對及原生 Windows 畫面檢查。完整遊玩、EP2 原版與註冊條件仍未完成。


## 2026-11-07：原生參照與音訊測試發現

- 已啟動 Pocket PC 2003 Device Emulator，安裝同一份 EP1 光碟 CAB，取得實際註冊畫面。與 PocketHLE 比較後，標題列、背景、字體和控制項位置仍有明顯差異，不能視為顯示正常。
- 新增 tools/wince-dialog-test 獨立 ARM 探針，保存相同執行檔在 Device Emulator 與 PocketHLE 的結果。已確認 GWL_STYLE、初始輸入焦點、caption 幾何、字體相關 DLU 與 CE masked system colors 存在差異；尚未套用這些版面修正。
- 原生 Windows CLI 與新版 WASM 的 EP1／EP3 註冊最終畫面逐像素一致；此結果僅驗證兩個後端一致，不能代替原生裝置正確性。
- 全 workspace 測試發現 rmp3 0.3.1 處理非音訊資料時因未初始化 frame metadata／unchecked slice 而中止程序。已將相同版本置於 third_party/rmp3，以零初始化 metadata 與受檢查切片修復，並透過 Cargo patch 同時套用 FMOD 與 MAS1 路徑。
- FMOD 四個針對性測試已通過，涵蓋既有 WAV、零資料、截斷標頭與含前後雜訊的有效 MP3。全 workspace 重跑中；現有 WASM／CLI release 尚未包含這次音訊修正。
- 依使用者要求，修改與測試均留在本機，不進行提交或推送。
- 音訊修正後 `cargo test --workspace --offline` 已全部通過（exit 0），包含新增 MAS1 垃圾資料後接有效 MP3 的復原測試；`cargo fmt --all -- --check` 也已通過。這仍不等同 EQ 三部完整遊玩或所有顯示驗收。


## 2026-11-07：CE 標記系統色值

- 依已保存的 Pocket PC 2003 Device Emulator 探針，補上 GetSysColor 的 0x40000005、0x4000000f、0x40000012，分別回傳白色、0x00c0ccd9、黑色；保留未標記索引的既有行為。尚未擴充 GetSysColorBrush。
- 新增經 API 呼叫的回歸測試；cargo test -p pocket-winceapi --offline 的 167 個單元測試與 5 個註冊整合測試通過，cargo fmt --all -- --check 通過。既有 modeless 初始焦點測試亦通過。
- 此次尚未重建 release／WASM 或重跑遊戲畫面，不能據此宣稱 EQ 註冊版面已正確。caption 幾何、字型、DLU 與完整遊玩驗收仍待處理。未提交或推送。


## 2026-11-07：初始焦點與對話框樣式實測

- 已用重新建置的原生 CLI 重跑同一份 ARM 探針。WM_INITDIALOG 的建議控制項與返回 TRUE 後的 edit 焦點符合 Device Emulator；返回 FALSE 保留應用自行設定焦點的 API 單元測試通過。
- DialogPanel 現在保留 template style／ex_style，GetWindowLongW 可讀回，SetWindowLongW 可交換對話框 metadata，visible 位元依目前 panel 狀態回傳。CE 的 DS_3DLOOK 位元依參照加入。控制項與獨立 guest window 的 style 查詢也讀取既有狀態。
- 已保存 reference/pocketpc3630-focus-colors.txt 與 pocketpc3630-styles.txt。新增 compare.py，列出每項仍不一致的觀察，不把探針通過數當成遊戲相容率。caption、字體度量及 child 初始化期間 visible 位元仍不同。
- 重建 Windows CLI release 成功，EP1 光碟版以 Unicorn、message-budget 0、50,000 slices 重跑，人工查看截圖；畫面與先前 EP1 native 最終畫面逐像素一致。樣式查詢修正沒有解決註冊版面，不能宣稱顯示已正常。此次由刻意 slice 上限終止，不是完整遊玩驗收。
- 新增樣式 API 回歸測試正在隨全 workspace 執行；第一次針對性測試使用了 parser 不接受的零控制項 fixture，已改成含一個 static 的有效 template。完整結果以下續記。
- 更新結果：修正後全 workspace 測試 exit 0，共 582 passed、11 ignored（保留既有忽略項目）；新樣式 API 測試通過。cargo fmt --all -- --check exit 0。驗證資料保存於 tools/eq-validation/dialog-verification-2026-11-07.json。
- EP3 同樣以新版原生 CLI 實跑、人工查看最終截圖，與先前 native 畫面逐像素一致。EP1／EP3 仍只有註冊階段，未擴大宣稱遊玩或正常版面。
- cargo clippy --workspace --all-targets --offline exit 0；仍有既有 unknown clippy::chunks_exact_to_as_chunks 警告，不宣稱無警告。WASM release 尚未重建包含此次焦點／樣式變更。


## 2026-11-07：使用者授權套用既有遊戲 patch

- 使用者明確要求「用 patch 檔覆蓋，避免序號問題」，本次開始另外測試本機既有修改版。此授權取代先前僅測原版的範圍限制；歷史原版註冊結果仍保留，修改版不作為原版註冊通過證據。
- EP1：從使用者本機 Pocket EQ.zip 的 Episode 1/crack/PocketEQ.exe 取得既有替換檔；SHA-256 ad702136fca20a4548a526ed56471ef7d9877e36a07540485e22139193fe5e02。套用於 target/eq-validation/modified-library 獨立副本，匯入後再次核對 EXE 雜湊。
- 發現 CLI run ZIP 優先載入 CAB，忽略根目錄替換 EXE；第一次 ZIP 直跑未套到 patch，不列入修改版成功證據。改用 Library import（會套用附加檔），再明確執行核對過的 EXE，設定相同 InstallDir 登錄值與 guest mount。
- EP1 修改版已實際通過開場，顯示四個「Tap here to create a character」欄位；已人工查看截圖 target/eq-validation/ep1-patched-menu-native/final.png。2,000,000 slices 到上限結束，尚非完整遊玩／存讀檔驗收。
- EP2 依此次授權使用已識別的本機修改套件，已到 emodiv 標誌；延長互動測試中。EP3 尚未找到對應 patch，未將 EP1 替換檔套到其他版本。
- 來源與實際啟動方式另記 tools/eq-validation/modified-package-sources.json；所有遊戲 binary 保留在 target，原始套件未覆寫，未操作 Git。
- 延長互動結果：EP1 已點入角色建立畫面，人工查看職業圖示、說明區、手寫名字區與難度按鈕；EP2 已進入四欄角色選擇畫面。截圖分別保存 ep1-patched-create-native/final.png 與 ep2-modified-menu-native/final.png。仍需完成建立角色、進入世界、操作／戰鬥、音訊、存讀檔及原生裝置參照比對。


## 2026-11-07：EP1／EP2 實際遊玩與存檔目錄修正

- 依使用者最新要求，優先測 EP1／EP2，EP3 保留待測。兩部均使用已記錄的本機素材；EP1 套用使用者授權的既有替換檔，EP2 沿用目前不需序號的套件。
- 改以持續開啟的 Windows 原生 CLI 視窗逐步操作，並用 PID 限定的控制工具取得實際顯示截圖。早期使用 stride=50 的最後一張抽樣圖誤判為停在載入；真正最終 framebuffer 顯示已回到建立角色，已撤回卡載入判斷。之後依每一步即時截圖確認，不用抽樣尾圖推論最終狀態。
- EP1：已進入 Freeport 世界、瀏覽教學各頁，角色移動與場景捲動有效；背包／裝備、物品說明、法術書、任務記錄、地圖、設定音量與製作名單已逐項截圖檢視。退出世界顯示 Level 1 角色，重新啟動程序後載入同一角色並恢復場景位置。
- EP2：已進入 Qeynos 世界、瀏覽教學各頁，移動與場景捲動有效；背包／裝備、法術書、任務記錄、地圖與設定已截圖檢視。退出世界後顯示 Level 12 角色，重新啟動程序後成功讀回並恢復先前場景位置。
- 以上只能證明已走過的介面與城鎮流程，尚未完成戰鬥、商店、任務推進、死亡／復活、長時間遊玩及同素材 Device Emulator 遊玩畫面對照。截圖人工初查未見整片缺圖或文字越界，不等於逐像素符合裝置參照。
- EP2 讀取六個未找到的怪物音效（Rat_Att/Hit/Die、Croc_Att/Hit/Die）；已確認目前匯入素材沒有這些檔案，不直接歸因 FMOD 解碼。錄製的 PCM 非靜音；尚非聽感或完整音效驗收。
- 實測發現指定存檔目錄仍是空的，.sav/.nam/.css 寫入 extracted：安裝 mount 與 save mount prefix 相同，原本 stable sort 讓先註冊的安裝 mount 優先。新增 save_overlay metadata，僅在 prefix 長度相同時優先 save mount，保持其他 mount 排序；非截斷更新先複製既有檔到存檔層。
- 新增兩個回歸測試涵蓋相同 prefix 的讀寫覆蓋、素材讀取 fallthrough、既有資料的非截斷更新不變動安裝檔。全 workspace：584 passed、11 ignored；fmt exit 0；Clippy exit 0，仍有既有 unknown lint 警告。
- 新版 CLI release 已建置成功。EP1 實跑讀回角色、移動、退出後，darkheart.css 與 pocketeqdarkheart0.sav 寫入指定 play-saves/ep1；安裝目錄原有 .sav/.nam/.css 的 SHA-256 保持不變。EP2 同修正的實跑續驗中。
- 新版 CLI 的 EP2 實跑也確認 eqppce2.css 與 pocketeqe2_2.sav 寫入指定 play-saves/ep2，安裝目錄原有存檔／名字／角色列表檔雜湊保持不變。兩部 overlay 的 .sav 都已不同於安裝目錄舊副本。
- EP2 額外完成 gnoll in stocks 的 NPC 對話，四頁文字已留圖；任務目標更新為 Speak with Captain of the Guards。此為第一段任務推進，不代表整條任務完成。正在以新程序確認此次 overlay 存檔的任務進度。
- EP2 已在全新程序讀回 overlay 存檔，NPC 旁的位置與更新後的 Speak with Captain of the Guards 任務目標都保留（ep2-overlay-reloaded-world.png／ep2-overlay-reloaded-quest.png），證明讀取的不是安裝目錄內舊進度。
- EP1 也已在全新程序從 overlay 讀回更新後位置，截圖 ep1-overlay-reloaded-world.png；兩部安裝目錄舊存檔皆維持原雜湊。新原生視窗已正常關閉。此次僅改善存檔優先順序，未宣稱完整戰鬥／商店／死亡復活或整個 EP1／EP2 已完成。


## 2026-11-07：EP3 進入世界

- 找到本機 Pocket EQ.zip／Episode 3 安裝說明附帶的註冊資料；僅在本機測試啟動設定中使用，未把內容寫入程式碼或報告。遊戲 EXE 未修改。
- 修正原生 display 鍵盤轉送：以 held-key snapshot 產生一次性按下／放開事件，避免依賴 minifb 更新週期的 pressed/released 狀態。EP3 註冊欄位已能正常顯示輸入；CPU slice 數不造成重複按鍵的測試通過。
- 啟動器現會還原 CAB 的所有字串登錄值。追查發現 CAB 的 Q-Q-T 擁有者文字原始內容沒有反斜線，匯入後卻被當作路徑加上反斜線；這會使註冊檢查不匹配。pocket-cab 已將宏替換與路徑正規化分開，保留一般 REG_SZ、空字串、URL 與相對文字，並加入解析回歸測試。pocket-cab 的 17 個單元測試與 2 個解壓整合測試通過。
- 使用正確擁有者及本機隨附碼初始化遊戲的 RegistrationE3 設定後，EP3 可啟動進入選角。此方法測試的是已提供註冊設定的啟動流程；MFC 註冊對話框的姓名顯示／初始化仍待另外修正，不能宣稱該 GUI 已正常完成註冊。
- 已建立 25 級 Warrior，進入 Butcherblock Mtns 世界、檢視教學、移動並開啟地圖；世界與地圖均已人工查看。退出後寫出 pocketeqe3_0.sav，選角頁顯示角色。
- 用修正後 release CLI 全新匯入 EP3 到 ep3-clean-library，確認擁有者不再被加反斜線，遊戲 EXE 與原先匯入副本逐位元相同。
- 乾淨安裝的讀檔驗證發現角色清單仍綁定舊安裝目錄的 pockethle-registry.dat，即使 .sav 已在存檔資料夾也看不到角色。現正補上 mount_save_dir 同時接管 registry snapshot：首次接管遷移舊資料；已存在的存檔 snapshot 優先，避免刪除角色後被舊安裝資料復活。補上兩個回歸測試，重建後續驗乾淨副本讀檔。
- Registry snapshot 修正的四個 persistence 測試通過；完整 workspace 最終 588 passed、11 ignored，fmt exit 0，Clippy exit 0（僅既有 unknown lint 警告）。release CLI 已重建。
- 實跑先從舊安裝讀取並遷移 Char0Class／Char0Diff／Char0File／Char0Level／Char0Zone 到 play-saves/ep3/pockethle-registry.dat。再以 ep3-clean-library 全新素材啟動，已看到相同 Butcherblock Mtns Level 25 Warrior 角色清單，正在讀入世界。
- 最終實測：乾淨副本已從共同存檔目錄成功讀回世界，角色仍在 Butcherblock Mtns，25 級 Warrior、先前位置及狀態保留；ep3-clean-reloaded-world.png 已人工查看。此次「EP3 進入世界」已達成，遊戲 EXE 無修改。目前原生測試視窗保留在世界內供檢視，PID 由 native-ui-state.json 記錄。完整三部戰鬥／任務驗收、註冊 GUI 修正與多後端驗證仍未完成；未操作 Git。


## 2026-11-07：Web 與 Android 更新及實跑

- 依使用者要求同步測 Web／Android。以目前 CAB 字串、存檔 overlay、registry snapshot 與 MP3 安全修正重建 WASM；`docs/pockethle.js`／`docs/pockethle.wasm` 已更新並核對與測試模組雜湊相同。未部署公開網站，未操作 Git。
- 真正 Windows Chrome UI 已逐部匯入 EP1、EP2、EP3 套件與原生存檔，進入 Freeport／Qeynos／Butcherblock Mtns，點擊移動、退出存檔、重新整理頁面後從 IndexedDB 還原遊戲並再次進世界。三部世界／重讀世界截圖均已人工查看，沒有未捕捉的 Runtime exception。console 仍有 HLE stub 與缺少部分音效等警告；不將這些警告隱藏或當成完整 API／音效驗收。
- 新增實際 Chromium UI 驗證工具 `tools/eq-validation/browser-smoke.mjs`，使用獨立 profile、loopback server 與 CDP；`smoke.mjs` 可於 boot 前匯入給定 save ZIP，並有順序契約測試。存檔、遊戲素材與註冊資料均維持在 ignored target；報告僅記錄步驟、雜湊與截圖路徑。
- Android 以 NDK r26d／Gradle 8.7／JDK 21 實際編出 ARM64、x86_64 JNI；補齊 x86_64 的 NDK CMake wrapper，app 固定 ndkVersion 26.3.11579264。實際 clean assembleDebug 成功，兩個 ABI 的 debug APK 為 29,726,965 bytes，已安裝至本次專用 Android 14／API 34 x86_64 模擬器。ARM64 僅編譯通過，尚未實機驗證。
- Android EP3 已經真正 SAF 匯入套件與三個存檔檔案，看到原有 25 級 Warrior，進入 Butcherblock Mtns，觸控移動與場景捲動有效；退出遊戲存檔後，force-stop 並重新啟動 app，可讀回世界與更新位置。透過 Settings 關閉 FPS／backend status 疊字後再截圖檢查底部畫面；未改變產品預設值。
- Android EP1 已進入 Freeport，人工查看世界畫面；已核對裝置內 EXE 雜湊與使用者授權替換檔相同。EP1／EP2 的其餘 Android 操作及顯示驗證持續進行，結果記錄於 `tools/eq-validation/cross-platform-verification.json`。
- 目前通過的跨平台進世界／存讀檔流程不等於完整三部遊戲驗收：戰鬥、商店、死亡復活、長時間遊玩、完整任務、音效聽測與同素材 CE 裝置參照仍待完成；EP3 原版註冊 GUI 的 MFC owner 初始化／呈現也仍需另外修正。

- Android 補充：EP1 已測直向／橫向／帶虛擬按鍵全螢幕，維持遊戲 240×320 比例；橫向觸控移動有效，旋回直向無需重新啟動 guest，並成功退出存檔。Android EP2 也已人工查看 Qeynos 世界，force-stop 後可重讀匯入角色；Options 的 Exit 另確認寫出與匯入 ZIP 不同的 SAV，正在用新程序續驗此次更新。
- Android 操作限制：150 ms 測試注入短點偶爾未被 EP2 接收；改用 500 ms 按住後 Options 可正常開啟。未因此宣稱短觸控可靠性已修復；報告保留此現象，需另追蹤客體輪詢與輸入交付時序。
- 後續檢查：cargo fmt --all -- --check 成功，Windows Node 的 smoke／scenario 共 5 個測試通過，browser-smoke 語法檢查成功。上一輪 runtime 變更的完整 Rust workspace 為 588 passed、11 ignored；此次其後只有建置設定、Web artifact、測試工具與文件變更。

- Android EP2 更新存檔續驗完成：Options Exit 寫出的 SAV 雜湊已不同於原匯入進度；新程序再次讀回同一 12 級 Wizard 與 Qeynos 世界，最終 android-ep2-reloaded-world-guest.png 已人工查看。此項不包含整套戰鬥／任務驗收。

- Android EP1 最終新程序讀檔也已完成：遊戲庫第一次自動點擊未啟動 Activity，人工重試後成功啟動，未見 app crash；讀回相同角色、Freeport 與移動後位置，android-ep1-reloaded-world-guest.png 已人工查看。Android 三部的基本進世界與重啟讀檔均已有實際畫面證據，但不將自動點擊成功當成遊戲行為通過。
- 本機 Web 預覽使用目前 docs，僅綁定 127.0.0.1:8000；進程與網址記錄在 ignored target/eq-validation/web-preview-state.json。APK 位於 frontends/pocket-android/app/build/outputs/apk/debug/app-debug.apk；版本 0.6.0。未公開部署、未動 Git。


## 2026-11-07～08：三平台音效補全

- FMOD 狀態改為每個 guest 獨立，補上播放中音量、逐聲道暫停／恢復、指定串流停止與釋放；自動分配真正空閒聲道，避免多次音效搶走循環音樂。同串流重播只保留一個聲道，AudioEngine 銷毀會停止背景輸出。
- 從使用者本機既有各部素材補入 EP2 缺少的 6 個及 EP3 缺少的 15 個 WAV；檔案來源與雜湊見 tools/eq-validation/audio-materials.json。三部共 199 個音訊檔均已實際解碼並送入共用混音器，輸出非靜音。這不等於戰鬥中每種音效都已觸發驗收。
- 新增音效專用匯入補丁，僅增加缺少的 Sounds/*.wav；重複匯入無副作用，遇到既有不同音效則拒絕。補丁不重裝遊戲，不覆寫存檔、設定或登錄資料。補丁 ZIP 留在 ignored target/eq-validation/audio-packages。
- Web 退出／重新啟動時關閉舊 AudioContext、回呼及 WASM 緩衝，避免多個消費者混用同一音訊游標。Android 使用裝置最小值與 100 ms 下限配置緩衝，記錄非靜音 PCM、播放進度及輸出錯誤。
- 最新完整 Rust workspace：598 passed、0 failed、12 ignored；另行啟用本機素材測試成功（199 檔）；Clippy 成功，僅既有 unknown lint 警告；Node 測試 6 passed。
- 新版實際 Chrome：EP1、EP2、EP3 均已進世界、退出、重新載入，六張世界畫面已人工查看；Web Audio 回呼均有非靜音輸出，沒有缺少音效或未捕捉例外。EP2／EP3 已透過正常 UI 匯入音效專用補丁，前後存檔及 manifest 雜湊相同。WAV 為最多十秒的有聲片段，非連續錄音。
- 新版 Windows CLI 三部均已進世界，cpal 裝置開啟及非靜音混音輸出已確認，三張世界畫面已人工查看；EP2／EP3 補丁連續匯入兩次，進度與設定保持一致。Android Unicorn 三部亦已進世界、人工查看畫面，AudioTrack 接收非靜音 PCM 且播放位置前進，沒有缺少音效及輸出錯誤。Android Interpreter（--no-default-features，對應正式發行 CPU 配置的 debug 簽署 APK）三部也已逐部進世界，人工查看畫面，確認 AudioTrack 非靜音輸出與播放進度，EP2／EP3 補丁保留存檔和設定；裝置安裝 APK 雜湊與測試指定檔相同。彙整證據、版本雜湊及限制見 tools/eq-validation/audio-verification.json。聲音輸出路徑驗證不等於人工聽測；Android ARM64 仍需實機，Windows egui 啟動器亦未完成這輪 UI 音效驗證。完整戰鬥、任務、商店及死亡復活驗收仍待完成。未操作 Git。

- 2026-11-08 最終產物：Windows target/release/pockethle.exe；Web docs/pockethle.js／pockethle.wasm；Android target/eq-validation/audio-artifacts/pockethle-audio-unicorn-debug.apk 與 pockethle-audio-interpreter-debug.apk。預設 JNI 與 app-debug.apk 已還原為 Unicorn，新版兩個 ABI 已核對；測試裝置最後恢復預設 APK，保留進度。既有遊戲請返回遊戲庫匯入 ep2-audio-patch.zip／ep3-audio-patch.zip，無須重新匯入完整遊戲。
